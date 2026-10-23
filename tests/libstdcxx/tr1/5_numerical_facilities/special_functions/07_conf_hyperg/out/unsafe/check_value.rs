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
pub struct testcase_conf_hyperg_double_ {
    pub f0: (),
    pub a: (),
    pub c: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_14: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 3_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_16: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 4_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_18: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_20: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 6_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_22: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 7_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_24: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 8_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_26: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 9_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_28: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0_f64,
                c: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_30: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.18354081260932842_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.19419827762834704_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.20700192122398287_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.22280243801078498_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.24300035416182644_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.27004644161220326_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.308508322553671_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.36743360905415834_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.46575960759364043_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6450352704491501_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.753387654377091_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.441523869125334_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.3801013214774045_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 16.843983681258987_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 40.07844550407642_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 98.03333969781269_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 244.33254130132138_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 617.0640304056244_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1573.6049422133694_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4042.755430890411_f64,
                a: 0.5_f64,
                c: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-11_f64 });
pub static mut data012_32: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.34751307955387056_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.36515709992587503_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.385752760726423_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4102024146138289_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.43982706745912625_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4766310911434693_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5237776118026086_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.586472996475084_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6736700229433487_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8014560736340217_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.3281918274866849_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.9052621465543667_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.9805776178019903_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.090678729317165_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.418565045042598_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 18.627776225142014_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 38.82351306969962_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 84.21528770042696_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 188.31125697734257_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 431.0259017395232_f64,
                a: 0.5_f64,
                c: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data013_34: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.44148780381255504_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4615489003015372_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4845452077181575_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.511241319179763_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5426968203238793_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5804188816496212_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6266137193204989_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6846131564463674_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7596197536913264_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8600470272655335_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2039946674617061_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.516175047025178_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.0187596221024697_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.8698033217756134_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.382118604314445_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.1913541951514235_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 12.620107286909638_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 23.47892648303636_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 45.85298186074905_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 93.13726509924584_f64,
                a: 0.5_f64,
                c: 3_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_36: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.507231430752982_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5281542002616678_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5518165151642677_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5788476728788237_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.610088283242754_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6466845185365926_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.690234798673865_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7430236597586141_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8084040275320187_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8914381440030124_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1467204168940972_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.3525055369951857_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.6530571499633475_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.1112387416058045_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.84104803362782_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.055056222185471_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.160103904477858_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.953803414426451_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 17.034704868473916_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 30.67144532542843_f64,
                a: 0.5_f64,
                c: 4_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_38: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.5571523916238331_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5782313526951898_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6018168855679725_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6284268814782993_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6587343448952188_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6936287273193257_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.734307416181532_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7824150359387054_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8402601334525486_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9111597643320869_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1141687602185972_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2651443108002267_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.4712624889419719_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.7626460645467978_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.1901779328181084_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.842179697945709_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.876035458620354_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.579294015654554_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8.48984290024633_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 13.636227878037948_f64,
                a: 0.5_f64,
                c: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data016_40: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.5968711191949919_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6177498227805703_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6409074448512445_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6667732279286019_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.69589293014101_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7289704003257105_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7669275540820718_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8109924455910189_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8628310240127653_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9247480922397641_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.093291259462882_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2115798426781204_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.3654106750890422_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.5711704305419896_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.8549798357448213_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.2595503871694826_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.856503877287693_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.768932573631784_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.213473855469953_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.580156554535286_f64,
                a: 0.5_f64,
                c: 6_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_41: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_42: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.6294673695375408_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6499583096482705_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6725191039627635_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6975087059608364_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7253753917485644_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.756685884348355_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7921662345887965_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8327601049132689_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8797132337587894_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9346979484015023_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.0788040971101556_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.175638551679476_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2970810749099917_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.4529009687665237_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.6579437149144023_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.9353010489337754_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.3217458547039813_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.877225460764602_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.7017478151936585_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.965950064855224_f64,
                a: 0.5_f64,
                c: 7_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler017_43: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data018_44: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.6568257438960127_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6768310608444045_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6987188488313648_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7227920113126842_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7494231555364722_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.779075557638195_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8123319225847639_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8499343852125205_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8928409587146189_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9423064123103875_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.0681796709163929_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1499542693515108_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.249685095671268_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.373611912726657_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.5308465522192733_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.7349787653671505_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.006718899603938_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.378325520430694_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.901155874625575_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.6587666457431234_f64,
                a: 0.5_f64,
                c: 8_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler018_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data019_46: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.6801865406347545_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6996587009453866_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7208470102094278_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7440092863582257_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7694585931917298_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.797578682701247_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8288447664979425_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8638518021485514_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9033535161271631_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9483169759447369_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.0600626000640645_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1307298999505393_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.215034109277418_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.317179802300684_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.4431045594091672_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.6013540635087158_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.8044714074708206_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.0712406108144257_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.4303714711293143_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.9266541358556295_f64,
                a: 0.5_f64,
                c: 9_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler019_47: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data020_48: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.700409544611041_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7193302573765444_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7398199575861503_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7620998527275505_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7864355396308798_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.813148605106268_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8426319656522667_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8753703779849664_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9119681856815145_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9531873178622932_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.0536628587304602_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1158225648376323_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1886686247111011_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2751576744751334_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.3793478044961116_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.5069047234443802_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.6658803233122232_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.8679295659745196_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.130243295552205_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.4786679001777303_f64,
                a: 0.5_f64,
                c: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler020_49: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data021_50: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 4.539992976248485e-05_f64,
                a: 1_f64,
                c: 1_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00012340980408667956_f64,
                a: 1_f64,
                c: 1_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00033546262790251185_f64,
                a: 1_f64,
                c: 1_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0009118819655545162_f64,
                a: 1_f64,
                c: 1_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0024787521766663585_f64,
                a: 1_f64,
                c: 1_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.006737946999085467_f64,
                a: 1_f64,
                c: 1_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.018315638888734182_f64,
                a: 1_f64,
                c: 1_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.049787068367863944_f64,
                a: 1_f64,
                c: 1_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1353352832366127_f64,
                a: 1_f64,
                c: 1_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3678794411714423_f64,
                a: 1_f64,
                c: 1_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.7182818284590455_f64,
                a: 1_f64,
                c: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.38905609893065_f64,
                a: 1_f64,
                c: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 20.085536923187668_f64,
                a: 1_f64,
                c: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 54.598150033144236_f64,
                a: 1_f64,
                c: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 148.4131591025766_f64,
                a: 1_f64,
                c: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 403.4287934927351_f64,
                a: 1_f64,
                c: 1_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1096.6331584284585_f64,
                a: 1_f64,
                c: 1_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2980.9579870417283_f64,
                a: 1_f64,
                c: 1_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8103.083927575384_f64,
                a: 1_f64,
                c: 1_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 22026.465794806718_f64,
                a: 1_f64,
                c: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler021_51: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data022_52: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.09999546000702375_f64,
                a: 1_f64,
                c: 2_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.11109739891065704_f64,
                a: 1_f64,
                c: 2_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.12495806717151219_f64,
                a: 1_f64,
                c: 2_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1427268740049208_f64,
                a: 1_f64,
                c: 2_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.16625354130388895_f64,
                a: 1_f64,
                c: 2_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1986524106001829_f64,
                a: 1_f64,
                c: 2_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.24542109027781644_f64,
                a: 1_f64,
                c: 2_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3167376438773787_f64,
                a: 1_f64,
                c: 2_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.43233235838169365_f64,
                a: 1_f64,
                c: 2_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6321205588285577_f64,
                a: 1_f64,
                c: 2_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.7182818284590455_f64,
                a: 1_f64,
                c: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.194528049465325_f64,
                a: 1_f64,
                c: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.361845641062556_f64,
                a: 1_f64,
                c: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 13.399537508286059_f64,
                a: 1_f64,
                c: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 29.48263182051532_f64,
                a: 1_f64,
                c: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 67.07146558212251_f64,
                a: 1_f64,
                c: 2_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 156.51902263263693_f64,
                a: 1_f64,
                c: 2_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 372.49474838021604_f64,
                a: 1_f64,
                c: 2_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 900.231547508376_f64,
                a: 1_f64,
                c: 2_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2202.546579480672_f64,
                a: 1_f64,
                c: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler022_53: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data023_54: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.18000090799859525_f64,
                a: 1_f64,
                c: 3_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.19753391135318732_f64,
                a: 1_f64,
                c: 3_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.21876048320712196_f64,
                a: 1_f64,
                c: 3_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2449351788557369_f64,
                a: 1_f64,
                c: 3_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.27791548623203705_f64,
                a: 1_f64,
                c: 3_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.32053903575992687_f64,
                a: 1_f64,
                c: 3_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3772894548610918_f64,
                a: 1_f64,
                c: 3_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4555082374150809_f64,
                a: 1_f64,
                c: 3_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5676676416183064_f64,
                a: 1_f64,
                c: 3_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7357588823428847_f64,
                a: 1_f64,
                c: 3_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 3_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.436563656918091_f64,
                a: 1_f64,
                c: 3_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.194528049465325_f64,
                a: 1_f64,
                c: 3_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.5745637607083705_f64,
                a: 1_f64,
                c: 3_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.1997687541430295_f64,
                a: 1_f64,
                c: 3_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11.393052728206127_f64,
                a: 1_f64,
                c: 3_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 22.023821860707507_f64,
                a: 1_f64,
                c: 3_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 44.43400646646769_f64,
                a: 1_f64,
                c: 3_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 92.87368709505401_f64,
                a: 1_f64,
                c: 3_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 199.8292327796391_f64,
                a: 1_f64,
                c: 3_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 440.30931589613436_f64,
                a: 1_f64,
                c: 3_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler023_55: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data024_56: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.24599972760042138_f64,
                a: 1_f64,
                c: 4_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.26748869621560417_f64,
                a: 1_f64,
                c: 4_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2929648187973293_f64,
                a: 1_f64,
                c: 4_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.32359920906182715_f64,
                a: 1_f64,
                c: 4_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.36104225688398156_f64,
                a: 1_f64,
                c: 4_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4076765785440439_f64,
                a: 1_f64,
                c: 4_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.46703290885418114_f64,
                a: 1_f64,
                c: 4_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5444917625849192_f64,
                a: 1_f64,
                c: 4_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6484985375725405_f64,
                a: 1_f64,
                c: 4_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7927233529713461_f64,
                a: 1_f64,
                c: 4_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 4_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.3096909707542714_f64,
                a: 1_f64,
                c: 4_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.7917920741979876_f64,
                a: 1_f64,
                c: 4_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.5745637607083705_f64,
                a: 1_f64,
                c: 4_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.8998265656072717_f64,
                a: 1_f64,
                c: 4_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.2358316369236775_f64,
                a: 1_f64,
                c: 4_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10.511910930353745_f64,
                a: 1_f64,
                c: 4_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 18.614574199914728_f64,
                a: 1_f64,
                c: 4_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 34.45263266064527_f64,
                a: 1_f64,
                c: 4_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 66.27641092654633_f64,
                a: 1_f64,
                c: 4_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 131.79279476884014_f64,
                a: 1_f64,
                c: 4_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler024_57: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data025_58: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.3016001089598315_f64,
                a: 1_f64,
                c: 5_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3255605794597313_f64,
                a: 1_f64,
                c: 5_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3535175906013356_f64,
                a: 1_f64,
                c: 5_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3865147376789559_f64,
                a: 1_f64,
                c: 5_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.42597182874401246_f64,
                a: 1_f64,
                c: 5_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.47385873716476473_f64,
                a: 1_f64,
                c: 5_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5329670911458189_f64,
                a: 1_f64,
                c: 5_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6073443165534412_f64,
                a: 1_f64,
                c: 5_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.703002924854919_f64,
                a: 1_f64,
                c: 5_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8291065881146157_f64,
                a: 1_f64,
                c: 5_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2387638830170857_f64,
                a: 1_f64,
                c: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.5835841483959754_f64,
                a: 1_f64,
                c: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.099418347611161_f64,
                a: 1_f64,
                c: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.899826565607273_f64,
                a: 1_f64,
                c: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.188665309538943_f64,
                a: 1_f64,
                c: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.341273953569168_f64,
                a: 1_f64,
                c: 5_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10.065470971379845_f64,
                a: 1_f64,
                c: 5_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 16.726316330322632_f64,
                a: 1_f64,
                c: 5_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 29.011738189576135_f64,
                a: 1_f64,
                c: 5_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 52.31711790753606_f64,
                a: 1_f64,
                c: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler025_59: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data026_60: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.3491999455200842_f64,
                a: 1_f64,
                c: 6_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3746885669668158_f64,
                a: 1_f64,
                c: 6_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.40405150587416555_f64,
                a: 1_f64,
                c: 6_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4382037588007456_f64,
                a: 1_f64,
                c: 6_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4783568093799898_f64,
                a: 1_f64,
                c: 6_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5261412628352351_f64,
                a: 1_f64,
                c: 6_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5837911360677266_f64,
                a: 1_f64,
                c: 6_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6544261390775982_f64,
                a: 1_f64,
                c: 6_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7424926878627024_f64,
                a: 1_f64,
                c: 6_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8544670594269214_f64,
                a: 1_f64,
                c: 6_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 6_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1938194150854282_f64,
                a: 1_f64,
                c: 6_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.4589603709899384_f64,
                a: 1_f64,
                c: 6_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.832363912685268_f64,
                a: 1_f64,
                c: 6_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.37478320700909_f64,
                a: 1_f64,
                c: 6_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.1886653095389423_f64,
                a: 1_f64,
                c: 6_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.451061627974306_f64,
                a: 1_f64,
                c: 6_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.47533640812846_f64,
                a: 1_f64,
                c: 6_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.828947706451636_f64,
                a: 1_f64,
                c: 6_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 15.562076771986721_f64,
                a: 1_f64,
                c: 6_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 25.65855895376798_f64,
                a: 1_f64,
                c: 6_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler026_61: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data027_62: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.39048003268794934_f64,
                a: 1_f64,
                c: 7_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4168742886887892_f64,
                a: 1_f64,
                c: 7_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4469613705943759_f64,
                a: 1_f64,
                c: 7_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.48153963531364674_f64,
                a: 1_f64,
                c: 7_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5216431906200103_f64,
                a: 1_f64,
                c: 7_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5686304845977178_f64,
                a: 1_f64,
                c: 7_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6243132958984103_f64,
                a: 1_f64,
                c: 7_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6911477218448039_f64,
                a: 1_f64,
                c: 7_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7725219364118928_f64,
                a: 1_f64,
                c: 7_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8731976434384715_f64,
                a: 1_f64,
                c: 7_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 7_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1629164905125695_f64,
                a: 1_f64,
                c: 7_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.376881112969815_f64,
                a: 1_f64,
                c: 7_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.664727825370536_f64,
                a: 1_f64,
                c: 7_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.062174810513636_f64,
                a: 1_f64,
                c: 7_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.626398371446729_f64,
                a: 1_f64,
                c: 7_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.4510616279743087_f64,
                a: 1_f64,
                c: 7_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.693145492681546_f64,
                a: 1_f64,
                c: 7_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.621710779838747_f64,
                a: 1_f64,
                c: 7_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.708051181324505_f64,
                a: 1_f64,
                c: 7_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 14.795135372260791_f64,
                a: 1_f64,
                c: 7_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler027_63: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data028_64: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.4266639771184355_f64,
                a: 1_f64,
                c: 8_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.45354221990871935_f64,
                a: 1_f64,
                c: 8_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.483908800729921_f64,
                a: 1_f64,
                c: 8_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5184603646863535_f64,
                a: 1_f64,
                c: 8_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5580829442766547_f64,
                a: 1_f64,
                c: 8_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.603917321563195_f64,
                a: 1_f64,
                c: 8_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.657451732177782_f64,
                a: 1_f64,
                c: 8_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7206553156954576_f64,
                a: 1_f64,
                c: 8_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7961732225583753_f64,
                a: 1_f64,
                c: 8_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8876164959306991_f64,
                a: 1_f64,
                c: 8_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 8_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1404154335879861_f64,
                a: 1_f64,
                c: 8_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.3190838953943527_f64,
                a: 1_f64,
                c: 8_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.5510315925312508_f64,
                a: 1_f64,
                c: 8_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.8588059183988626_f64,
                a: 1_f64,
                c: 8_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.276957720025422_f64,
                a: 1_f64,
                c: 8_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.8595718993033583_f64,
                a: 1_f64,
                c: 8_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.693145492681539_f64,
                a: 1_f64,
                c: 8_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.918996932358905_f64,
                a: 1_f64,
                c: 8_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.772928696585724_f64,
                a: 1_f64,
                c: 8_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.65659476058258_f64,
                a: 1_f64,
                c: 8_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler028_65: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data029_66: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.45866881830525147_f64,
                a: 1_f64,
                c: 9_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.48574024897002727_f64,
                a: 1_f64,
                c: 9_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5160911992700791_f64,
                a: 1_f64,
                c: 9_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5503310117870245_f64,
                a: 1_f64,
                c: 9_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5892227409644605_f64,
                a: 1_f64,
                c: 9_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6337322854988879_f64,
                a: 1_f64,
                c: 9_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6850965356444361_f64,
                a: 1_f64,
                c: 9_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7449191581454464_f64,
                a: 1_f64,
                c: 9_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.815307109766499_f64,
                a: 1_f64,
                c: 9_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8990680325544067_f64,
                a: 1_f64,
                c: 9_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 9_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1233234687038898_f64,
                a: 1_f64,
                c: 9_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.276335581577411_f64,
                a: 1_f64,
                c: 9_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.4694175800833351_f64,
                a: 1_f64,
                c: 9_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.7176118367977251_f64,
                a: 1_f64,
                c: 9_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.0431323520406743_f64,
                a: 1_f64,
                c: 9_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.479429199071145_f64,
                a: 1_f64,
                c: 9_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.077880563064615_f64,
                a: 1_f64,
                c: 9_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.918996932358891_f64,
                a: 1_f64,
                c: 9_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.131492174742854_f64,
                a: 1_f64,
                c: 9_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.925275808466047_f64,
                a: 1_f64,
                c: 9_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler029_67: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data030_68: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.4871980635252735_f64,
                a: 1_f64,
                c: 10_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5142597510299729_f64,
                a: 1_f64,
                c: 10_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5443974008211611_f64,
                a: 1_f64,
                c: 10_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5781458419881114_f64,
                a: 1_f64,
                c: 10_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6161658885533093_f64,
                a: 1_f64,
                c: 10_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6592818861020016_f64,
                a: 1_f64,
                c: 10_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7085327948000189_f64,
                a: 1_f64,
                c: 10_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7652425255636607_f64,
                a: 1_f64,
                c: 10_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8311180060507546_f64,
                a: 1_f64,
                c: 10_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.9083877070103394_f64,
                a: 1_f64,
                c: 10_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 1_f64,
                c: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1099112183350075_f64,
                a: 1_f64,
                c: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2435101170983485_f64,
                a: 1_f64,
                c: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.408252740250006_f64,
                a: 1_f64,
                c: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.6146266327948817_f64,
                a: 1_f64,
                c: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.877638233673215_f64,
                a: 1_f64,
                c: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.219143798606718_f64,
                a: 1_f64,
                c: 10_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.6715607239402184_f64,
                a: 1_f64,
                c: 10_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.2838715489037495_f64,
                a: 1_f64,
                c: 10_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.131492174742869_f64,
                a: 1_f64,
                c: 10_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.332748227619445_f64,
                a: 1_f64,
                c: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler030_69: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data031_70: std::cell::LazyCell<[testcase_conf_hyperg_double_; 20]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -0.0004085993678623636_f64,
                a: 2_f64,
                c: 1_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0009872784326934365_f64,
                a: 2_f64,
                c: 1_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.002348238395317583_f64,
                a: 2_f64,
                c: 1_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.005471291793327097_f64,
                a: 2_f64,
                c: 1_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.012393760883331793_f64,
                a: 2_f64,
                c: 1_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.026951787996341868_f64,
                a: 2_f64,
                c: 1_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.05494691666620255_f64,
                a: 2_f64,
                c: 1_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.09957413673572789_f64,
                a: 2_f64,
                c: 1_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.1353352832366127_f64,
                a: 2_f64,
                c: 1_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.436563656918091_f64,
                a: 2_f64,
                c: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 22.16716829679195_f64,
                a: 2_f64,
                c: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 80.34214769275067_f64,
                a: 2_f64,
                c: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 272.9907501657212_f64,
                a: 2_f64,
                c: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 890.4789546154595_f64,
                a: 2_f64,
                c: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2824.0015544491457_f64,
                a: 2_f64,
                c: 1_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8773.065267427668_f64,
                a: 2_f64,
                c: 1_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 26828.621883375556_f64,
                a: 2_f64,
                c: 1_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 81030.83927575384_f64,
                a: 2_f64,
                c: 1_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 242291.1237428739_f64,
                a: 2_f64,
                c: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler031_71: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data032_72: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 4.539992976248485e-05_f64,
                a: 2_f64,
                c: 2_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00012340980408667956_f64,
                a: 2_f64,
                c: 2_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00033546262790251185_f64,
                a: 2_f64,
                c: 2_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0009118819655545162_f64,
                a: 2_f64,
                c: 2_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0024787521766663585_f64,
                a: 2_f64,
                c: 2_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.006737946999085467_f64,
                a: 2_f64,
                c: 2_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.018315638888734182_f64,
                a: 2_f64,
                c: 2_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.049787068367863944_f64,
                a: 2_f64,
                c: 2_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1353352832366127_f64,
                a: 2_f64,
                c: 2_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3678794411714423_f64,
                a: 2_f64,
                c: 2_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.7182818284590455_f64,
                a: 2_f64,
                c: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.38905609893065_f64,
                a: 2_f64,
                c: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 20.085536923187668_f64,
                a: 2_f64,
                c: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 54.598150033144236_f64,
                a: 2_f64,
                c: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 148.4131591025766_f64,
                a: 2_f64,
                c: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 403.4287934927351_f64,
                a: 2_f64,
                c: 2_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1096.6331584284585_f64,
                a: 2_f64,
                c: 2_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2980.9579870417283_f64,
                a: 2_f64,
                c: 2_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8103.083927575384_f64,
                a: 2_f64,
                c: 2_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 22026.465794806718_f64,
                a: 2_f64,
                c: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler032_73: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data033_74: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.019990012015452252_f64,
                a: 2_f64,
                c: 3_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.02466088646812675_f64,
                a: 2_f64,
                c: 3_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.03115565113590242_f64,
                a: 2_f64,
                c: 3_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.04051856915410464_f64,
                a: 2_f64,
                c: 3_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.05459159637574086_f64,
                a: 2_f64,
                c: 3_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.07676578544043897_f64,
                a: 2_f64,
                c: 3_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.11355272569454114_f64,
                a: 2_f64,
                c: 3_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1779670503396765_f64,
                a: 2_f64,
                c: 3_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.296997075145081_f64,
                a: 2_f64,
                c: 3_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5284822353142309_f64,
                a: 2_f64,
                c: 3_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 3_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.0000000000000004_f64,
                a: 2_f64,
                c: 3_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.194528049465326_f64,
                a: 2_f64,
                c: 3_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.149127521416741_f64,
                a: 2_f64,
                c: 3_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 20.59930626242909_f64,
                a: 2_f64,
                c: 3_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 47.57221091282452_f64,
                a: 2_f64,
                c: 3_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 112.11910930353754_f64,
                a: 2_f64,
                c: 3_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 268.60403879880613_f64,
                a: 2_f64,
                c: 3_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 652.1158096653782_f64,
                a: 2_f64,
                c: 3_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1600.633862237113_f64,
                a: 2_f64,
                c: 3_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3964.783843065209_f64,
                a: 2_f64,
                c: 3_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler033_75: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data034_76: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.04800326879494294_f64,
                a: 2_f64,
                c: 4_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.05762434162835353_f64,
                a: 2_f64,
                c: 4_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.07035181202670733_f64,
                a: 2_f64,
                c: 4_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.08760711844355668_f64,
                a: 2_f64,
                c: 4_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1116619449281481_f64,
                a: 2_f64,
                c: 4_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.14626395019169272_f64,
                a: 2_f64,
                c: 4_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.19780254687491297_f64,
                a: 2_f64,
                c: 4_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.27754118707540437_f64,
                a: 2_f64,
                c: 4_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.40600584970983794_f64,
                a: 2_f64,
                c: 4_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6218299410859616_f64,
                a: 2_f64,
                c: 4_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 4_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.6903090292457281_f64,
                a: 2_f64,
                c: 4_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.9999999999999987_f64,
                a: 2_f64,
                c: 4_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.574563760708369_f64,
                a: 2_f64,
                c: 4_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10.79965313121455_f64,
                a: 2_f64,
                c: 4_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 21.707494910771032_f64,
                a: 2_f64,
                c: 4_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 45.04764372141505_f64,
                a: 2_f64,
                c: 4_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 96.07287099957367_f64,
                a: 2_f64,
                c: 4_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 209.71579596387159_f64,
                a: 2_f64,
                c: 4_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 466.9348764858249_f64,
                a: 2_f64,
                c: 4_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1057.3423581507243_f64,
                a: 2_f64,
                c: 4_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler034_77: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data035_78: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.0791985835221914_f64,
                a: 2_f64,
                c: 5_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.09327304648322253_f64,
                a: 2_f64,
                c: 5_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.11130650338531098_f64,
                a: 2_f64,
                c: 5_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1348526232104402_f64,
                a: 2_f64,
                c: 5_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.16625354130388895_f64,
                a: 2_f64,
                c: 5_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.20913010268188095_f64,
                a: 2_f64,
                c: 5_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2692303619792681_f64,
                a: 2_f64,
                c: 5_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3559341006793529_f64,
                a: 2_f64,
                c: 5_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4849853757254047_f64,
                a: 2_f64,
                c: 5_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6835736475415372_f64,
                a: 2_f64,
                c: 5_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.5224722339658285_f64,
                a: 2_f64,
                c: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.4164158516040235_f64,
                a: 2_f64,
                c: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.000000000000001_f64,
                a: 2_f64,
                c: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.899826565607272_f64,
                a: 2_f64,
                c: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 12.377330619077886_f64,
                a: 2_f64,
                c: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 23.023821860707503_f64,
                a: 2_f64,
                c: 5_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 44.26188388551938_f64,
                a: 2_f64,
                c: 5_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 87.63158165161316_f64,
                a: 2_f64,
                c: 5_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 178.0704291374568_f64,
                a: 2_f64,
                c: 5_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 370.2198253527524_f64,
                a: 2_f64,
                c: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler035_79: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data036_80: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.11120076271882003_f64,
                a: 2_f64,
                c: 6_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.12904862943139384_f64,
                a: 2_f64,
                c: 6_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.15138192951001525_f64,
                a: 2_f64,
                c: 6_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.17975865319179699_f64,
                a: 2_f64,
                c: 6_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.21643190620010283_f64,
                a: 2_f64,
                c: 6_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.264728634482884_f64,
                a: 2_f64,
                c: 6_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3296709114581884_f64,
                a: 2_f64,
                c: 6_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4190170264568135_f64,
                a: 2_f64,
                c: 6_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5450438728237857_f64,
                a: 2_f64,
                c: 6_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.727664702865393_f64,
                a: 2_f64,
                c: 6_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 6_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.4185417547437151_f64,
                a: 2_f64,
                c: 6_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.0820792580201224_f64,
                a: 2_f64,
                c: 6_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.167636087314732_f64,
                a: 2_f64,
                c: 6_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.999999999999998_f64,
                a: 2_f64,
                c: 6_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8.18866530953894_f64,
                a: 2_f64,
                c: 6_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 13.902123255948611_f64,
                a: 2_f64,
                c: 6_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 24.426009224385382_f64,
                a: 2_f64,
                c: 6_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 44.315790825806545_f64,
                a: 2_f64,
                c: 6_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 82.81038385993361_f64,
                a: 2_f64,
                c: 6_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 158.95135372260788_f64,
                a: 2_f64,
                c: 6_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler036_81: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data037_82: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.14279950968075855_f64,
                a: 2_f64,
                c: 7_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.163759958356948_f64,
                a: 2_f64,
                c: 7_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.18950218227311263_f64,
                a: 2_f64,
                c: 7_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.22152437623624174_f64,
                a: 2_f64,
                c: 7_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.26192490317988687_f64,
                a: 2_f64,
                c: 7_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3136951540228214_f64,
                a: 2_f64,
                c: 7_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3811803369143073_f64,
                a: 2_f64,
                c: 7_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.47081822524156886_f64,
                a: 2_f64,
                c: 7_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5923464451167507_f64,
                a: 2_f64,
                c: 7_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.7608141393691709_f64,
                a: 2_f64,
                c: 7_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 7_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.348334037949722_f64,
                a: 2_f64,
                c: 7_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.8693566610905543_f64,
                a: 2_f64,
                c: 7_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.670544349258928_f64,
                a: 2_f64,
                c: 7_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.937825189486365_f64,
                a: 2_f64,
                c: 7_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.000000000000002_f64,
                a: 2_f64,
                c: 7_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.451061627974312_f64,
                a: 2_f64,
                c: 7_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 15.386290985363091_f64,
                a: 2_f64,
                c: 7_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 25.86513233951624_f64,
                a: 2_f64,
                c: 7_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 44.83220472529802_f64,
                a: 2_f64,
                c: 7_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 79.97567686130395_f64,
                a: 2_f64,
                c: 7_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler037_83: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data038_84: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.17337636610503362_f64,
                a: 2_f64,
                c: 8_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.19686670136921_f64,
                a: 2_f64,
                c: 8_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.22527678978110538_f64,
                a: 2_f64,
                c: 8_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.26001525907740475_f64,
                a: 2_f64,
                c: 8_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.303004668680144_f64,
                a: 2_f64,
                c: 8_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.356909462804855_f64,
                a: 2_f64,
                c: 8_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4254826782221804_f64,
                a: 2_f64,
                c: 8_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5141021587408818_f64,
                a: 2_f64,
                c: 8_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6306142195329979_f64,
                a: 2_f64,
                c: 8_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.786684528485106_f64,
                a: 2_f64,
                c: 8_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 8_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2979228320600693_f64,
                a: 2_f64,
                c: 8_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.7236644184225898_f64,
                a: 2_f64,
                c: 8_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.3469052224062485_f64,
                a: 2_f64,
                c: 8_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.282388163202275_f64,
                a: 2_f64,
                c: 8_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.723042279974578_f64,
                a: 2_f64,
                c: 8_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.000000000000001_f64,
                a: 2_f64,
                c: 8_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10.693145492681536_f64,
                a: 2_f64,
                c: 8_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 16.83799386471781_f64,
                a: 2_f64,
                c: 8_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 27.31878608975717_f64,
                a: 2_f64,
                c: 8_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 45.62637904233032_f64,
                a: 2_f64,
                c: 8_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler038_85: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data039_86: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.20263008881072142_f64,
                a: 2_f64,
                c: 9_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.22815601647956382_f64,
                a: 2_f64,
                c: 9_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2586320109488156_f64,
                a: 2_f64,
                c: 9_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2953658349816557_f64,
                a: 2_f64,
                c: 9_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3401043674620142_f64,
                a: 2_f64,
                c: 9_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3952125740133439_f64,
                a: 2_f64,
                c: 9_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4639381079112034_f64,
                a: 2_f64,
                c: 9_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5508084185455355_f64,
                a: 2_f64,
                c: 9_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6622360121015094_f64,
                a: 2_f64,
                c: 9_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.807455739564746_f64,
                a: 2_f64,
                c: 9_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 9_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2600591877766618_f64,
                a: 2_f64,
                c: 9_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.6183220921129462_f64,
                a: 2_f64,
                c: 9_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.1223296796666578_f64,
                a: 2_f64,
                c: 9_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.8471644896068233_f64,
                a: 2_f64,
                c: 9_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.9137352959186495_f64,
                a: 2_f64,
                c: 9_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.520570800928854_f64,
                a: 2_f64,
                c: 9_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.999999999999998_f64,
                a: 2_f64,
                c: 9_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11.918996932358892_f64,
                a: 2_f64,
                c: 9_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 18.262984349485706_f64,
                a: 2_f64,
                c: 9_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 28.77582742539814_f64,
                a: 2_f64,
                c: 9_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler039_87: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data040_88: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.23043485654507717_f64,
                a: 2_f64,
                c: 10_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2575842324904634_f64,
                a: 2_f64,
                c: 10_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2896415868614212_f64,
                a: 2_f64,
                c: 10_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3278123701783314_f64,
                a: 2_f64,
                c: 10_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.37367756025366927_f64,
                a: 2_f64,
                c: 10_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.42933548067397925_f64,
                a: 2_f64,
                c: 10_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4976064623997737_f64,
                a: 2_f64,
                c: 10_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5823322187997332_f64,
                a: 2_f64,
                c: 10_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6888199394924538_f64,
                a: 2_f64,
                c: 10_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.8245106369069453_f64,
                a: 2_f64,
                c: 10_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 2_f64,
                c: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.230621471654947_f64,
                a: 2_f64,
                c: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.5389392974099088_f64,
                a: 2_f64,
                c: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.95873629874997_f64,
                a: 2_f64,
                c: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.5414934688204727_f64,
                a: 2_f64,
                c: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.3670852989803555_f64,
                a: 2_f64,
                c: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.561712402786565_f64,
                a: 2_f64,
                c: 10_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.3284392760597825_f64,
                a: 2_f64,
                c: 10_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.000000000000004_f64,
                a: 2_f64,
                c: 10_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 13.131492174742865_f64,
                a: 2_f64,
                c: 10_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 19.665496455238888_f64,
                a: 2_f64,
                c: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler040_89: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data041_90: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.0004993992273873331_f64,
                a: 5_f64,
                c: 1_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0005707703439008925_f64,
                a: 5_f64,
                c: 1_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0032428054030576147_f64,
                a: 5_f64,
                c: 1_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.007864981952907702_f64,
                a: 5_f64,
                c: 1_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.012393760883331793_f64,
                a: 5_f64,
                c: 1_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.008703181540485393_f64,
                a: 5_f64,
                c: 1_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.01831563888883202_f64,
                a: 5_f64,
                c: 1_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0684572190058147_f64,
                a: 5_f64,
                c: 1_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.045111761078875295_f64,
                a: 5_f64,
                c: 1_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.22992465073215118_f64,
                a: 5_f64,
                c: 1_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 23.671704256164183_f64,
                a: 5_f64,
                c: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 199.50451467112745_f64,
                a: 5_f64,
                c: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1232.7498286606428_f64,
                a: 5_f64,
                c: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6460.781087255402_f64,
                a: 5_f64,
                c: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 30480.352550691663_f64,
                a: 5_f64,
                c: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 133534.93064609537_f64,
                a: 5_f64,
                c: 1_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 553479.8936684965_f64,
                a: 5_f64,
                c: 1_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2196966.036449753_f64,
                a: 5_f64,
                c: 1_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8422142.857223663_f64,
                a: 5_f64,
                c: 1_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 31373029.447069697_f64,
                a: 5_f64,
                c: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler041_91: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-10_f64 });
pub static mut data042_92: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -0.00025726626865408083_f64,
                a: 5_f64,
                c: 2_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00029309828470586396_f64,
                a: 5_f64,
                c: 2_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.000111820875967504_f64,
                a: 5_f64,
                c: 2_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0006459163922677825_f64,
                a: 5_f64,
                c: 2_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0024787521766663585_f64,
                a: 5_f64,
                c: 2_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.005334208040942662_f64,
                a: 5_f64,
                c: 2_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.006105212962902297_f64,
                a: 5_f64,
                c: 2_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00622338354598232_f64,
                a: 5_f64,
                c: 2_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0451117610788718_f64,
                a: 5_f64,
                c: 2_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.015328310048810216_f64,
                a: 5_f64,
                c: 2_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8.268107228229598_f64,
                a: 5_f64,
                c: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 46.79735529322744_f64,
                a: 5_f64,
                c: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 223.45159827046285_f64,
                a: 5_f64,
                c: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 964.5673172522146_f64,
                a: 5_f64,
                c: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3889.6615448133625_f64,
                a: 5_f64,
                c: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 14926.865359231202_f64,
                a: 5_f64,
                c: 2_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 55151.50925929789_f64,
                a: 5_f64,
                c: 2_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 197736.87980710136_f64,
                a: 5_f64,
                c: 2_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 691800.7903167484_f64,
                a: 5_f64,
                c: 2_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2371516.15057419_f64,
                a: 5_f64,
                c: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler042_93: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-10_f64 });
pub static mut data043_94: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.00012106647936662628_f64,
                a: 5_f64,
                c: 3_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00021596715715168925_f64,
                a: 5_f64,
                c: 3_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00033546262790251185_f64,
                a: 5_f64,
                c: 3_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0003799508189810484_f64,
                a: 5_f64,
                c: 3_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0_f64,
                a: 5_f64,
                c: 3_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0016844867497713668_f64,
                a: 5_f64,
                c: 3_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.006105212962911393_f64,
                a: 5_f64,
                c: 3_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.012446767091965986_f64,
                a: 5_f64,
                c: 3_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0_f64,
                a: 5_f64,
                c: 3_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.15328310048810098_f64,
                a: 5_f64,
                c: 3_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 3_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.75699319980333_f64,
                a: 5_f64,
                c: 3_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 19.7041495971484_f64,
                a: 5_f64,
                c: 3_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 75.32076346195376_f64,
                a: 5_f64,
                c: 3_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 272.9907501657212_f64,
                a: 5_f64,
                c: 3_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 952.3177709081999_f64,
                a: 5_f64,
                c: 3_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3227.430347941881_f64,
                a: 5_f64,
                c: 3_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10692.17329467747_f64,
                a: 5_f64,
                c: 3_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 34777.8431821535_f64,
                a: 5_f64,
                c: 3_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 111417.40400416154_f64,
                a: 5_f64,
                c: 3_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 352423.4527169075_f64,
                a: 5_f64,
                c: 3_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler043_95: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-11_f64 });
pub static mut data044_96: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -6.809989464372728e-05_f64,
                a: 5_f64,
                c: 4_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00015426225510834944_f64,
                a: 5_f64,
                c: 4_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00033546262790251185_f64,
                a: 5_f64,
                c: 4_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0006839114741658872_f64,
                a: 5_f64,
                c: 4_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0012393760883331792_f64,
                a: 5_f64,
                c: 4_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0016844867497713668_f64,
                a: 5_f64,
                c: 4_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0_f64,
                a: 5_f64,
                c: 4_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.012446767091965986_f64,
                a: 5_f64,
                c: 4_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.06766764161830635_f64,
                a: 5_f64,
                c: 4_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2759095808785817_f64,
                a: 5_f64,
                c: 4_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 4_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.3978522855738067_f64,
                a: 5_f64,
                c: 4_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11.083584148395975_f64,
                a: 5_f64,
                c: 4_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 35.14968961557842_f64,
                a: 5_f64,
                c: 4_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 109.19630006628847_f64,
                a: 5_f64,
                c: 4_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 333.92960798079736_f64,
                a: 5_f64,
                c: 4_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1008.5719837318378_f64,
                a: 5_f64,
                c: 4_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3015.741185678261_f64,
                a: 5_f64,
                c: 4_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8942.873961125184_f64,
                a: 5_f64,
                c: 4_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 26335.02276462_f64,
                a: 5_f64,
                c: 4_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 77092.63028182351_f64,
                a: 5_f64,
                c: 4_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler044_97: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000014e-11_f64 });
pub static mut data045_98: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 4.539992976248485e-05_f64,
                a: 5_f64,
                c: 5_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00012340980408667956_f64,
                a: 5_f64,
                c: 5_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00033546262790251185_f64,
                a: 5_f64,
                c: 5_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0009118819655545162_f64,
                a: 5_f64,
                c: 5_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0024787521766663585_f64,
                a: 5_f64,
                c: 5_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.006737946999085467_f64,
                a: 5_f64,
                c: 5_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.018315638888734182_f64,
                a: 5_f64,
                c: 5_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.049787068367863944_f64,
                a: 5_f64,
                c: 5_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1353352832366127_f64,
                a: 5_f64,
                c: 5_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3678794411714423_f64,
                a: 5_f64,
                c: 5_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.7182818284590455_f64,
                a: 5_f64,
                c: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.38905609893065_f64,
                a: 5_f64,
                c: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 20.085536923187668_f64,
                a: 5_f64,
                c: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 54.598150033144236_f64,
                a: 5_f64,
                c: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 148.4131591025766_f64,
                a: 5_f64,
                c: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 403.4287934927351_f64,
                a: 5_f64,
                c: 5_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1096.6331584284585_f64,
                a: 5_f64,
                c: 5_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2980.9579870417283_f64,
                a: 5_f64,
                c: 5_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8103.083927575384_f64,
                a: 5_f64,
                c: 5_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 22026.465794806718_f64,
                a: 5_f64,
                c: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler045_99: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data046_100: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.0011648967743076429_f64,
                a: 5_f64,
                c: 6_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0019205128456127479_f64,
                a: 5_f64,
                c: 6_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0032972446271226324_f64,
                a: 5_f64,
                c: 6_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0059047424914709015_f64,
                a: 5_f64,
                c: 6_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.011033078698817415_f64,
                a: 5_f64,
                c: 6_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.02148505785349585_f64,
                a: 5_f64,
                c: 6_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.04349567165860857_f64,
                a: 5_f64,
                c: 6_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.09122802739566811_f64,
                a: 5_f64,
                c: 6_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.19744881503891684_f64,
                a: 5_f64,
                c: 6_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.43918161928124544_f64,
                a: 5_f64,
                c: 6_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 6_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.3226822806570357_f64,
                a: 5_f64,
                c: 6_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.486320123663313_f64,
                a: 5_f64,
                c: 6_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 13.144500379942246_f64,
                a: 5_f64,
                c: 6_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 31.873916035045458_f64,
                a: 5_f64,
                c: 6_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 78.08628695159632_f64,
                a: 5_f64,
                c: 6_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 192.98291046720357_f64,
                a: 5_f64,
                c: 6_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 480.548772048884_f64,
                a: 5_f64,
                c: 6_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1204.4605636118315_f64,
                a: 5_f64,
                c: 6_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3036.132904835058_f64,
                a: 5_f64,
                c: 6_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7691.640655546505_f64,
                a: 5_f64,
                c: 6_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler046_101: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000006e-11_f64 });
pub static mut data047_102: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.0036308901122103932_f64,
                a: 5_f64,
                c: 7_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00553273360192294_f64,
                a: 5_f64,
                c: 7_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.008676785265660346_f64,
                a: 5_f64,
                c: 7_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.014030481266326614_f64,
                a: 5_f64,
                c: 7_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.023426839582149212_f64,
                a: 5_f64,
                c: 7_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0404276819945128_f64,
                a: 5_f64,
                c: 7_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.07212378417759376_f64,
                a: 5_f64,
                c: 7_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1329585740959674_f64,
                a: 5_f64,
                c: 7_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.2529899131989388_f64,
                a: 5_f64,
                c: 7_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.4960243723933782_f64,
                a: 5_f64,
                c: 7_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 7_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.068107249881924_f64,
                a: 5_f64,
                c: 7_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.376881112969814_f64,
                a: 5_f64,
                c: 7_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.456636847199222_f64,
                a: 5_f64,
                c: 7_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 20.811741224531826_f64,
                a: 5_f64,
                c: 7_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 46.556488803696276_f64,
                a: 5_f64,
                c: 7_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 105.66804767556316_f64,
                a: 5_f64,
                c: 7_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 242.93097638084427_f64,
                a: 5_f64,
                c: 7_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 564.8980438088736_f64,
                a: 5_f64,
                c: 7_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1326.9606865425994_f64,
                a: 5_f64,
                c: 7_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3145.3685154983905_f64,
                a: 5_f64,
                c: 7_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler047_103: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-12_f64 });
pub static mut data048_104: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.007529529383140611_f64,
                a: 5_f64,
                c: 8_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.010936052508673187_f64,
                a: 5_f64,
                c: 8_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.01624745425364972_f64,
                a: 5_f64,
                c: 8_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.024729468107576008_f64,
                a: 5_f64,
                c: 8_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.038615775445860964_f64,
                a: 5_f64,
                c: 8_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.061937865588523586_f64,
                a: 5_f64,
                c: 8_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.10213565389690644_f64,
                a: 5_f64,
                c: 8_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.17324118243379236_f64,
                a: 5_f64,
                c: 8_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.30228316551605494_f64,
                a: 5_f64,
                c: 8_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5423874880220383_f64,
                a: 5_f64,
                c: 8_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 8_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.8922997283093959_f64,
                a: 5_f64,
                c: 8_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.669974283112627_f64,
                a: 5_f64,
                c: 8_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.283184235996094_f64,
                a: 5_f64,
                c: 8_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 14.76467653066477_f64,
                a: 5_f64,
                c: 8_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 30.522558591756702_f64,
                a: 5_f64,
                c: 8_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 64.23614709373022_f64,
                a: 5_f64,
                c: 8_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 137.4050303288333_f64,
                a: 5_f64,
                c: 8_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 298.2915388482877_f64,
                a: 5_f64,
                c: 8_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 656.2938935500275_f64,
                a: 5_f64,
                c: 8_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1461.618310143373_f64,
                a: 5_f64,
                c: 8_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler048_105: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data049_106: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.012801285049305222_f64,
                a: 5_f64,
                c: 9_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.017955923031350202_f64,
                a: 5_f64,
                c: 9_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.02566165037109072_f64,
                a: 5_f64,
                c: 9_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.03741461671020431_f64,
                a: 5_f64,
                c: 9_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.055720934057414885_f64,
                a: 5_f64,
                c: 9_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.084862956151756_f64,
                a: 5_f64,
                c: 9_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1323063517016232_f64,
                a: 5_f64,
                c: 9_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.21132914572142125_f64,
                a: 5_f64,
                c: 9_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.34601808641639625_f64,
                a: 5_f64,
                c: 9_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.5809218096571088_f64,
                a: 5_f64,
                c: 9_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 9_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.7643922061378634_f64,
                a: 5_f64,
                c: 9_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.188801009633245_f64,
                a: 5_f64,
                c: 9_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.898119492947927_f64,
                a: 5_f64,
                c: 9_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11.152835510393174_f64,
                a: 5_f64,
                c: 9_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 21.533483453443495_f64,
                a: 5_f64,
                c: 9_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 42.39714599535572_f64,
                a: 5_f64,
                c: 9_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 85.01089140485998_f64,
                a: 5_f64,
                c: 9_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 173.3622586873996_f64,
                a: 5_f64,
                c: 9_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 359.10444177844266_f64,
                a: 5_f64,
                c: 9_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 754.6484437196141_f64,
                a: 5_f64,
                c: 9_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler049_107: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data050_108: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.01931373116184047_f64,
                a: 5_f64,
                c: 10_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.02636108577518393_f64,
                a: 5_f64,
                c: 10_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.03655677207071192_f64,
                a: 5_f64,
                c: 10_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.05156393404834414_f64,
                a: 5_f64,
                c: 10_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.07405662579452184_f64,
                a: 5_f64,
                c: 10_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.10841132531381448_f64,
                a: 5_f64,
                c: 10_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.16192115120742598_f64,
                a: 5_f64,
                c: 10_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.24696279814742444_f64,
                a: 5_f64,
                c: 10_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.38492640633381947_f64,
                a: 5_f64,
                c: 10_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.6134562822972327_f64,
                a: 5_f64,
                c: 10_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 5_f64,
                c: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.6675470647226096_f64,
                a: 5_f64,
                c: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.8442428103603667_f64,
                a: 5_f64,
                c: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.9603804008438415_f64,
                a: 5_f64,
                c: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8.840595307162479_f64,
                a: 5_f64,
                c: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 16.089667272320337_f64,
                a: 5_f64,
                c: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 29.876575194426895_f64,
                a: 5_f64,
                c: 10_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 56.54671985643232_f64,
                a: 5_f64,
                c: 10_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 108.97420168465273_f64,
                a: 5_f64,
                c: 10_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 213.6060904583292_f64,
                a: 5_f64,
                c: 10_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 425.4132388063717_f64,
                a: 5_f64,
                c: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler050_109: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data051_110: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.0006715506365396127_f64,
                a: 10_f64,
                c: 1_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0007155564890525868_f64,
                a: 10_f64,
                c: 1_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0035372078786207375_f64,
                a: 10_f64,
                c: 1_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.004788400557471437_f64,
                a: 10_f64,
                c: 1_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0024787521766663585_f64,
                a: 10_f64,
                c: 1_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.01813682724252288_f64,
                a: 10_f64,
                c: 1_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.009968617568012997_f64,
                a: 10_f64,
                c: 1_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.052832081031434205_f64,
                a: 10_f64,
                c: 1_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0010979582061523968_f64,
                a: 10_f64,
                c: 1_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.11394854824644536_f64,
                a: 10_f64,
                c: 1_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 131.63017574352625_f64,
                a: 10_f64,
                c: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2431.291369875548_f64,
                a: 10_f64,
                c: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 27127.32889979105_f64,
                a: 10_f64,
                c: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 232066.4997783557_f64,
                a: 10_f64,
                c: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1674401.379493153_f64,
                a: 10_f64,
                c: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10707495.82038634_f64,
                a: 10_f64,
                c: 1_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 62515499.24281551_f64,
                a: 10_f64,
                c: 1_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 339773485.0093745_f64,
                a: 10_f64,
                c: 1_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1742442474.2135217_f64,
                a: 10_f64,
                c: 1_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8514625476.546278_f64,
                a: 10_f64,
                c: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler051_111: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data052_112: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -0.00014116415550486912_f64,
                a: 10_f64,
                c: 2_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00016988130843806985_f64,
                a: 10_f64,
                c: 2_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.661920970339138e-05_f64,
                a: 10_f64,
                c: 2_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0007258291964636574_f64,
                a: 10_f64,
                c: 2_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0012039653429522313_f64,
                a: 10_f64,
                c: 2_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0006145071537002133_f64,
                a: 10_f64,
                c: 2_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.005355789996035497_f64,
                a: 10_f64,
                c: 2_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0007890361281514147_f64,
                a: 10_f64,
                c: 2_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.023725444715554326_f64,
                a: 10_f64,
                c: 2_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.05729766902438476_f64,
                a: 10_f64,
                c: 2_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 34.43211665963657_f64,
                a: 10_f64,
                c: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 432.53475371634494_f64,
                a: 10_f64,
                c: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3789.1768909683506_f64,
                a: 10_f64,
                c: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 27089.676185774806_f64,
                a: 10_f64,
                c: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 169243.72183073507_f64,
                a: 10_f64,
                c: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 959019.4013539733_f64,
                a: 10_f64,
                c: 2_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5043073.345829796_f64,
                a: 10_f64,
                c: 2_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 24989309.81928148_f64,
                a: 10_f64,
                c: 2_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 117948708.50540228_f64,
                a: 10_f64,
                c: 2_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 534524325.6981042_f64,
                a: 10_f64,
                c: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler052_113: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data053_114: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 1.497316907510523e-05_f64,
                a: 10_f64,
                c: 3_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.7627971015476266e-05_f64,
                a: 10_f64,
                c: 3_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.596479408428118e-05_f64,
                a: 10_f64,
                c: 3_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.5479477810339013e-05_f64,
                a: 10_f64,
                c: 3_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0003541074538094798_f64,
                a: 10_f64,
                c: 3_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0007839399313861014_f64,
                a: 10_f64,
                c: 3_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0003811720262558433_f64,
                a: 10_f64,
                c: 3_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.004534179440644753_f64,
                a: 10_f64,
                c: 3_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0031029253652133403_f64,
                a: 10_f64,
                c: 3_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.02848713706161134_f64,
                a: 10_f64,
                c: 3_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 3_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 15.691485606063274_f64,
                a: 10_f64,
                c: 3_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 141.71088859081416_f64,
                a: 10_f64,
                c: 3_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 997.5517779931373_f64,
                a: 10_f64,
                c: 3_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6038.632428092606_f64,
                a: 10_f64,
                c: 3_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 32946.95242543715_f64,
                a: 10_f64,
                c: 3_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 166431.66712118863_f64,
                a: 10_f64,
                c: 3_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 791818.3027206131_f64,
                a: 10_f64,
                c: 3_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3589678.0198700386_f64,
                a: 10_f64,
                c: 3_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 15637649.698874988_f64,
                a: 10_f64,
                c: 3_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 65871447.34667836_f64,
                a: 10_f64,
                c: 3_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler053_115: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000013e-09_f64 });
pub static mut data054_116: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 6.9661267889527014e-06_f64,
                a: 10_f64,
                c: 4_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -3.0301514396282942e-06_f64,
                a: 10_f64,
                c: 4_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -3.7983599138168025e-05_f64,
                a: 10_f64,
                c: 4_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -9.361566012116387e-05_f64,
                a: 10_f64,
                c: 4_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -7.082149076189594e-05_f64,
                a: 10_f64,
                c: 4_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0003069286372764626_f64,
                a: 10_f64,
                c: 4_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.001065989564952783_f64,
                a: 10_f64,
                c: 4_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0004223010263345605_f64,
                a: 10_f64,
                c: 4_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.010168047735237568_f64,
                a: 10_f64,
                c: 4_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.03690351470878207_f64,
                a: 10_f64,
                c: 4_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 4_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.338475643321399_f64,
                a: 10_f64,
                c: 4_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 63.90556137202139_f64,
                a: 10_f64,
                c: 4_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 370.0849845672878_f64,
                a: 10_f64,
                c: 4_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1922.952621749354_f64,
                a: 10_f64,
                c: 4_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9245.038001435149_f64,
                a: 10_f64,
                c: 4_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 41898.961838459785_f64,
                a: 10_f64,
                c: 4_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 181211.14084739226_f64,
                a: 10_f64,
                c: 4_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 754384.2557069231_f64,
                a: 10_f64,
                c: 4_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3042060.4915799876_f64,
                a: 10_f64,
                c: 4_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11939626.42440293_f64,
                a: 10_f64,
                c: 4_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler054_117: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.0000000000000026e-09_f64 });
pub static mut data055_118: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -6.245492983198973e-06_f64,
                a: 10_f64,
                c: 5_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -1.1459481808048817e-05_f64,
                a: 10_f64,
                c: 5_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -8.164698880166951e-06_f64,
                a: 10_f64,
                c: 5_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.124040067177509e-05_f64,
                a: 10_f64,
                c: 5_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00014164298152379191_f64,
                a: 10_f64,
                c: 5_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00023172833594738382_f64,
                a: 10_f64,
                c: 5_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00036825094062005215_f64,
                a: 10_f64,
                c: 5_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0030227862937631683_f64,
                a: 10_f64,
                c: 5_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0002864238798658492_f64,
                a: 10_f64,
                c: 5_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.10617896040159878_f64,
                a: 10_f64,
                c: 5_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.480369496602827_f64,
                a: 10_f64,
                c: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 35.201619637445276_f64,
                a: 10_f64,
                c: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 171.58787257237464_f64,
                a: 10_f64,
                c: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 775.8714886720568_f64,
                a: 10_f64,
                c: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3317.407101977368_f64,
                a: 10_f64,
                c: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 13578.260535269774_f64,
                a: 10_f64,
                c: 5_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 53651.761875039716_f64,
                a: 10_f64,
                c: 5_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 205900.60390283042_f64,
                a: 10_f64,
                c: 5_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 770979.4961233422_f64,
                a: 10_f64,
                c: 5_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2826613.234853133_f64,
                a: 10_f64,
                c: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler055_119: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000013e-09_f64 });
pub static mut data056_120: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 9.60845074338303e-07_f64,
                a: 10_f64,
                c: 6_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.713112755417473e-06_f64,
                a: 10_f64,
                c: 6_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.3074149009167486e-05_f64,
                a: 10_f64,
                c: 6_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.010591978133289e-05_f64,
                a: 10_f64,
                c: 6_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -1.0325734976052423e-20_f64,
                a: 10_f64,
                c: 6_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00029188857701064686_f64,
                a: 10_f64,
                c: 6_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.001065989564952783_f64,
                a: 10_f64,
                c: 6_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00044452739614164207_f64,
                a: 10_f64,
                c: 6_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.020049671590609285_f64,
                a: 10_f64,
                c: 6_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.17092282236966808_f64,
                a: 10_f64,
                c: 6_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 6_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.9520550902714575_f64,
                a: 10_f64,
                c: 6_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 22.206263831706924_f64,
                a: 10_f64,
                c: 6_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 93.07494342084284_f64,
                a: 10_f64,
                c: 6_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 371.2096444052399_f64,
                a: 10_f64,
                c: 6_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1424.6976175888547_f64,
                a: 10_f64,
                c: 6_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5302.207000190233_f64,
                a: 10_f64,
                c: 6_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 19239.311823447424_f64,
                a: 10_f64,
                c: 6_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 68341.22199921592_f64,
                a: 10_f64,
                c: 6_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 238389.83519072225_f64,
                a: 10_f64,
                c: 6_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 818592.0409667853_f64,
                a: 10_f64,
                c: 6_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler056_121: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000012e-08_f64 });
pub static mut data057_122: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 3.963485931645503e-06_f64,
                a: 10_f64,
                c: 7_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.4074930030956985e-06_f64,
                a: 10_f64,
                c: 7_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -5.3248036175001926e-06_f64,
                a: 10_f64,
                c: 7_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -5.066010919747312e-05_f64,
                a: 10_f64,
                c: 7_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0001770537269047399_f64,
                a: 10_f64,
                c: 7_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00034759250392107574_f64,
                a: 10_f64,
                c: 7_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0002907244268053043_f64,
                a: 10_f64,
                c: 7_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.007112438338266279_f64,
                a: 10_f64,
                c: 7_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.046185850628367824_f64,
                a: 10_f64,
                c: 7_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.22919473120601758_f64,
                a: 10_f64,
                c: 7_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 7_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 4.034275412078106_f64,
                a: 10_f64,
                c: 7_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 15.423188523958418_f64,
                a: 10_f64,
                c: 7_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 56.66990774756521_f64,
                a: 10_f64,
                c: 7_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 201.9264913924223_f64,
                a: 10_f64,
                c: 7_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 702.0178001994894_f64,
                a: 10_f64,
                c: 7_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2391.7564185640726_f64,
                a: 10_f64,
                c: 7_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8011.5144629634615_f64,
                a: 10_f64,
                c: 7_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 26450.087535814702_f64,
                a: 10_f64,
                c: 7_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 86239.96465776658_f64,
                a: 10_f64,
                c: 7_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 278127.8339645833_f64,
                a: 10_f64,
                c: 7_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler057_123: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000004e-06_f64 });
pub static mut data058_124: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -5.044436640276097e-06_f64,
                a: 10_f64,
                c: 8_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -1.5426225510834945e-05_f64,
                a: 10_f64,
                c: 8_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -3.7273625322501334e-05_f64,
                a: 10_f64,
                c: 8_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -6.332513649684159e-05_f64,
                a: 10_f64,
                c: 8_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0_f64,
                a: 10_f64,
                c: 8_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0006550781804666425_f64,
                a: 10_f64,
                c: 8_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.004070141975274263_f64,
                a: 10_f64,
                c: 8_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.01867015063794898_f64,
                a: 10_f64,
                c: 8_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.07518626846478484_f64,
                a: 10_f64,
                c: 8_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.28101901756151837_f64,
                a: 10_f64,
                c: 8_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 8_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.4356061998579603_f64,
                a: 10_f64,
                c: 8_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11.494087265003234_f64,
                a: 10_f64,
                c: 8_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 37.66038173097688_f64,
                a: 10_f64,
                c: 8_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 121.32922229587608_f64,
                a: 10_f64,
                c: 8_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 385.4619548914142_f64,
                a: 10_f64,
                c: 8_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1210.2863804782053_f64,
                a: 10_f64,
                c: 8_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3762.0609740531836_f64,
                a: 10_f64,
                c: 8_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11592.614394051165_f64,
                a: 10_f64,
                c: 8_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 35450.992183142305_f64,
                a: 10_f64,
                c: 8_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 107684.94388572175_f64,
                a: 10_f64,
                c: 8_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler058_125: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000013e-09_f64 });
pub static mut data059_126: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -5.044436640276097e-06_f64,
                a: 10_f64,
                c: 9_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0_f64,
                a: 10_f64,
                c: 9_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.7273625322501334e-05_f64,
                a: 10_f64,
                c: 9_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00020264043678989247_f64,
                a: 10_f64,
                c: 9_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0008262507255554529_f64,
                a: 10_f64,
                c: 9_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.002994643110704652_f64,
                a: 10_f64,
                c: 9_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.010175354938185657_f64,
                a: 10_f64,
                c: 9_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0331913789119093_f64,
                a: 10_f64,
                c: 9_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.10526077585069878_f64,
                a: 10_f64,
                c: 9_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.32700394770794866_f64,
                a: 10_f64,
                c: 9_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 9_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.020313142732273_f64,
                a: 10_f64,
                c: 9_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.031068565359684_f64,
                a: 10_f64,
                c: 9_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 26.780715897583555_f64,
                a: 10_f64,
                c: 9_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 78.86399449231945_f64,
                a: 10_f64,
                c: 9_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 230.8649141595636_f64,
                a: 10_f64,
                c: 9_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 672.3813224878918_f64,
                a: 10_f64,
                c: 9_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1949.5700594283705_f64,
                a: 10_f64,
                c: 9_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5630.698419967709_f64,
                a: 10_f64,
                c: 9_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 16206.167855150768_f64,
                a: 10_f64,
                c: 9_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 46500.31667792529_f64,
                a: 10_f64,
                c: 9_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler059_127: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000013e-09_f64 });
pub static mut data060_128: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 4.539992976248485e-05_f64,
                a: 10_f64,
                c: 10_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00012340980408667956_f64,
                a: 10_f64,
                c: 10_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00033546262790251185_f64,
                a: 10_f64,
                c: 10_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0009118819655545162_f64,
                a: 10_f64,
                c: 10_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0024787521766663585_f64,
                a: 10_f64,
                c: 10_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.006737946999085467_f64,
                a: 10_f64,
                c: 10_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.018315638888734182_f64,
                a: 10_f64,
                c: 10_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.049787068367863944_f64,
                a: 10_f64,
                c: 10_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.1353352832366127_f64,
                a: 10_f64,
                c: 10_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.3678794411714423_f64,
                a: 10_f64,
                c: 10_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 10_f64,
                c: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.7182818284590455_f64,
                a: 10_f64,
                c: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.38905609893065_f64,
                a: 10_f64,
                c: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 20.085536923187668_f64,
                a: 10_f64,
                c: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 54.598150033144236_f64,
                a: 10_f64,
                c: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 148.4131591025766_f64,
                a: 10_f64,
                c: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 403.4287934927351_f64,
                a: 10_f64,
                c: 10_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1096.6331584284585_f64,
                a: 10_f64,
                c: 10_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2980.9579870417283_f64,
                a: 10_f64,
                c: 10_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8103.083927575384_f64,
                a: 10_f64,
                c: 10_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 22026.465794806718_f64,
                a: 10_f64,
                c: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler060_129: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data061_130: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 0.00018021852293239455_f64,
                a: 20_f64,
                c: 1_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0017726368057851866_f64,
                a: 20_f64,
                c: 1_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0005828004038232928_f64,
                a: 20_f64,
                c: 1_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.004965771702059014_f64,
                a: 20_f64,
                c: 1_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0012360336087128597_f64,
                a: 20_f64,
                c: 1_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.014898894139255305_f64,
                a: 20_f64,
                c: 1_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.013800784612552078_f64,
                a: 20_f64,
                c: 1_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.012192213426039619_f64,
                a: 20_f64,
                c: 1_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.05031124677313621_f64,
                a: 20_f64,
                c: 1_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.025985814502838493_f64,
                a: 20_f64,
                c: 1_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1563.6577385252017_f64,
                a: 20_f64,
                c: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 86377.09191076609_f64,
                a: 20_f64,
                c: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2216718.878997925_f64,
                a: 20_f64,
                c: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 38045018.52064716_f64,
                a: 20_f64,
                c: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 504376263.68346804_f64,
                a: 20_f64,
                c: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5565635666.797205_f64,
                a: 20_f64,
                c: 1_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 53451562646.54452_f64,
                a: 20_f64,
                c: 1_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 460009135340.3383_f64,
                a: 20_f64,
                c: 1_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3620401937301.4907_f64,
                a: 20_f64,
                c: 1_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 26446266822604.152_f64,
                a: 20_f64,
                c: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler061_131: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data062_132: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 6.664768199268407e-05_f64,
                a: 20_f64,
                c: 2_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -3.724825327022715e-05_f64,
                a: 20_f64,
                c: 2_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00024392611307344034_f64,
                a: 20_f64,
                c: 2_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.4034559592246202e-05_f64,
                a: 20_f64,
                c: 2_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0008164596058484307_f64,
                a: 20_f64,
                c: 2_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0005132638711646204_f64,
                a: 20_f64,
                c: 2_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.002178627985633392_f64,
                a: 20_f64,
                c: 2_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.006102938062517997_f64,
                a: 20_f64,
                c: 2_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.011834301617155166_f64,
                a: 20_f64,
                c: 2_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.03762201697368106_f64,
                a: 20_f64,
                c: 2_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 303.10954080179823_f64,
                a: 20_f64,
                c: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11508.923130556599_f64,
                a: 20_f64,
                c: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 234541.8602346128_f64,
                a: 20_f64,
                c: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3398931.2897027107_f64,
                a: 20_f64,
                c: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 39382712.287920594_f64,
                a: 20_f64,
                c: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 388350500.3708794_f64,
                a: 20_f64,
                c: 2_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3385284070.552718_f64,
                a: 20_f64,
                c: 2_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 26751585258.405773_f64,
                a: 20_f64,
                c: 2_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 195061928138.27676_f64,
                a: 20_f64,
                c: 2_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1329571695324.3132_f64,
                a: 20_f64,
                c: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler062_133: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000017e-10_f64 });
pub static mut data063_134: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -8.66719623185057e-06_f64,
                a: 20_f64,
                c: 3_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -1.8205565180535425e-05_f64,
                a: 20_f64,
                c: 3_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.562058871792763e-05_f64,
                a: 20_f64,
                c: 3_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.653276737310376e-05_f64,
                a: 20_f64,
                c: 3_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -5.270860038017211e-05_f64,
                a: 20_f64,
                c: 3_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00028546308121326275_f64,
                a: 20_f64,
                c: 3_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0005649074602625627_f64,
                a: 20_f64,
                c: 3_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -5.060258887546835e-07_f64,
                a: 20_f64,
                c: 3_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0021376080642211692_f64,
                a: 20_f64,
                c: 3_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0028873127225376104_f64,
                a: 20_f64,
                c: 3_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 3_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 106.38207299128948_f64,
                a: 20_f64,
                c: 3_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2880.573473283132_f64,
                a: 20_f64,
                c: 3_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 47353.75696516572_f64,
                a: 20_f64,
                c: 3_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 584732.279781489_f64,
                a: 20_f64,
                c: 3_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5957333.110132098_f64,
                a: 20_f64,
                c: 3_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 52725595.633352734_f64,
                a: 20_f64,
                c: 3_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 418560160.0336955_f64,
                a: 20_f64,
                c: 3_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3045067611.31509_f64,
                a: 20_f64,
                c: 3_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 20614600690.354652_f64,
                a: 20_f64,
                c: 3_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 131344201933.74118_f64,
                a: 20_f64,
                c: 3_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler063_135: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000007e-09_f64 });
pub static mut data064_136: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -1.1286669552452397e-06_f64,
                a: 20_f64,
                c: 4_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.95951887851377e-06_f64,
                a: 20_f64,
                c: 4_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8.694015305279005e-06_f64,
                a: 20_f64,
                c: 4_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -1.085881401806751e-05_f64,
                a: 20_f64,
                c: 4_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -4.1826023828710966e-05_f64,
                a: 20_f64,
                c: 4_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.645589362243632e-05_f64,
                a: 20_f64,
                c: 4_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.00014238710517977906_f64,
                a: 20_f64,
                c: 4_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0007179629470086613_f64,
                a: 20_f64,
                c: 4_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0020884061677332645_f64,
                a: 20_f64,
                c: 4_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.012768833157321973_f64,
                a: 20_f64,
                c: 4_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 4_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 50.65991693465774_f64,
                a: 20_f64,
                c: 4_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1014.313444233591_f64,
                a: 20_f64,
                c: 4_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 13665.584449611577_f64,
                a: 20_f64,
                c: 4_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 145123.62797278623_f64,
                a: 20_f64,
                c: 4_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1308144.4519382305_f64,
                a: 20_f64,
                c: 4_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10438124.578674613_f64,
                a: 20_f64,
                c: 4_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 75719160.52442463_f64,
                a: 20_f64,
                c: 4_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 508510905.9631025_f64,
                a: 20_f64,
                c: 4_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3203200954.5618095_f64,
                a: 20_f64,
                c: 4_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 19111993543.12469_f64,
                a: 20_f64,
                c: 4_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler064_137: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000007e-09_f64 });
pub static mut data065_138: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 8.475564345567097e-07_f64,
                a: 20_f64,
                c: 5_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8.57210618625657e-07_f64,
                a: 20_f64,
                c: 5_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -2.82287008375556e-06_f64,
                a: 20_f64,
                c: 5_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -6.6486802159657585e-06_f64,
                a: 20_f64,
                c: 5_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1816828026110384e-05_f64,
                a: 20_f64,
                c: 5_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.6173872819745774e-05_f64,
                a: 20_f64,
                c: 5_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0001148193428729667_f64,
                a: 20_f64,
                c: 5_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.2650647218867087e-07_f64,
                a: 20_f64,
                c: 5_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0010626537950495965_f64,
                a: 20_f64,
                c: 5_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.008549901120564194_f64,
                a: 20_f64,
                c: 5_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 29.12663780880938_f64,
                a: 20_f64,
                c: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 446.2691498351806_f64,
                a: 20_f64,
                c: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5005.647016485638_f64,
                a: 20_f64,
                c: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 46145.715220935184_f64,
                a: 20_f64,
                c: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 370342.18574452243_f64,
                a: 20_f64,
                c: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2676402.7371661114_f64,
                a: 20_f64,
                c: 5_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 17803174.102030005_f64,
                a: 20_f64,
                c: 5_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 110674464.6359731_f64,
                a: 20_f64,
                c: 5_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 650149739.3422849_f64,
                a: 20_f64,
                c: 5_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3639417243.515066_f64,
                a: 20_f64,
                c: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler065_139: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000005e-07_f64 });
pub static mut data066_140: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -1.9022359545310003e-08_f64,
                a: 20_f64,
                c: 6_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -7.45338096562347e-07_f64,
                a: 20_f64,
                c: 6_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -9.785242035872408e-07_f64,
                a: 20_f64,
                c: 6_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.018156986674634e-06_f64,
                a: 20_f64,
                c: 6_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7.981691070145728e-06_f64,
                a: 20_f64,
                c: 6_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -2.013316315396607e-05_f64,
                a: 20_f64,
                c: 6_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -4.7462368393259685e-05_f64,
                a: 20_f64,
                c: 6_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.0003191086993896482_f64,
                a: 20_f64,
                c: 6_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.001038052846805644_f64,
                a: 20_f64,
                c: 6_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.008475209755865106_f64,
                a: 20_f64,
                c: 6_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 6_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 19.002159564861383_f64,
                a: 20_f64,
                c: 6_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 229.93981298721295_f64,
                a: 20_f64,
                c: 6_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2180.3120758940972_f64,
                a: 20_f64,
                c: 6_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 17610.73251030529_f64,
                a: 20_f64,
                c: 6_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 126633.20907014767_f64,
                a: 20_f64,
                c: 6_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 832692.8301687413_f64,
                a: 20_f64,
                c: 6_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5097225.094065165_f64,
                a: 20_f64,
                c: 6_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 29414585.342530526_f64,
                a: 20_f64,
                c: 6_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 161513229.8813897_f64,
                a: 20_f64,
                c: 6_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 849871092.1095955_f64,
                a: 20_f64,
                c: 6_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler066_141: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000002e-07_f64 });
pub static mut data067_142: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -1.7754301607387138e-07_f64,
                a: 20_f64,
                c: 7_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -6.212860508947127e-08_f64,
                a: 20_f64,
                c: 7_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.133887337253315e-07_f64,
                a: 20_f64,
                c: 7_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.665740026927318e-06_f64,
                a: 20_f64,
                c: 7_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -4.7904165143355465e-06_f64,
                a: 20_f64,
                c: 7_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -1.5503088351319618e-05_f64,
                a: 20_f64,
                c: 7_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.642510849695434e-05_f64,
                a: 20_f64,
                c: 7_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.108355234547901e-05_f64,
                a: 20_f64,
                c: 7_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0018058773247853388_f64,
                a: 20_f64,
                c: 7_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.03285014769697774_f64,
                a: 20_f64,
                c: 7_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 7_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 13.551527852090807_f64,
                a: 20_f64,
                c: 7_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 133.23579819973105_f64,
                a: 20_f64,
                c: 7_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1083.6769250393436_f64,
                a: 20_f64,
                c: 7_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 7739.141090563762_f64,
                a: 20_f64,
                c: 7_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 50175.328973240226_f64,
                a: 20_f64,
                c: 7_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 301599.4681410286_f64,
                a: 20_f64,
                c: 7_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1705051.1866143662_f64,
                a: 20_f64,
                c: 7_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9159788.235373314_f64,
                a: 20_f64,
                c: 7_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 47122070.3986653_f64,
                a: 20_f64,
                c: 7_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 233529421.53991735_f64,
                a: 20_f64,
                c: 7_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler067_143: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000001e-07_f64 });
pub static mut data068_144: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 4.4385719622857066e-08_f64,
                a: 20_f64,
                c: 8_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.7870855352561944e-07_f64,
                a: 20_f64,
                c: 8_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2.722170603702833e-07_f64,
                a: 20_f64,
                c: 8_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -1.5211293805365477e-06_f64,
                a: 20_f64,
                c: 8_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -4.297833653155391e-06_f64,
                a: 20_f64,
                c: 8_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.1339557446266733e-05_f64,
                a: 20_f64,
                c: 8_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.352636522065899e-05_f64,
                a: 20_f64,
                c: 8_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0002946105326951324_f64,
                a: 20_f64,
                c: 8_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00026793636646740143_f64,
                a: 20_f64,
                c: 8_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.06106125843445281_f64,
                a: 20_f64,
                c: 8_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 8_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10.312756690132913_f64,
                a: 20_f64,
                c: 8_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 84.47182485684642_f64,
                a: 20_f64,
                c: 8_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 597.4733566685499_f64,
                a: 20_f64,
                c: 8_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3805.978636410741_f64,
                a: 20_f64,
                c: 8_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 22386.06846164166_f64,
                a: 20_f64,
                c: 8_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 123573.63516975302_f64,
                a: 20_f64,
                c: 8_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 647514.2414157087_f64,
                a: 20_f64,
                c: 8_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3247628.2434586394_f64,
                a: 20_f64,
                c: 8_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 15690070.625286419_f64,
                a: 20_f64,
                c: 8_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 73379158.89332505_f64,
                a: 20_f64,
                c: 8_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler068_145: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000005e-07_f64 });
pub static mut data069_146: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: 7.397626357656857e-08_f64,
                a: 20_f64,
                c: 9_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -9.075323809254817e-09_f64,
                a: 20_f64,
                c: 9_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -5.554948497039669e-07_f64,
                a: 20_f64,
                c: 9_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -1.1104933512848787e-06_f64,
                a: 20_f64,
                c: 9_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 3.2483424385770483e-06_f64,
                a: 20_f64,
                c: 9_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.7493431113569438e-05_f64,
                a: 20_f64,
                c: 9_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -3.906611063611723e-05_f64,
                a: 20_f64,
                c: 9_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.0004035615549330851_f64,
                a: 20_f64,
                c: 9_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.003767153147053455_f64,
                a: 20_f64,
                c: 9_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.09094434448524844_f64,
                a: 20_f64,
                c: 9_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 9_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 8.239094295714972_f64,
                a: 20_f64,
                c: 9_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 57.46805456216671_f64,
                a: 20_f64,
                c: 9_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 358.00109079775746_f64,
                a: 20_f64,
                c: 9_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2051.3704389047_f64,
                a: 20_f64,
                c: 9_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 11012.59750306421_f64,
                a: 20_f64,
                c: 9_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 56082.11330893447_f64,
                a: 20_f64,
                c: 9_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 273348.4691886334_f64,
                a: 20_f64,
                c: 9_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1283674.4996444662_f64,
                a: 20_f64,
                c: 9_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5838026.873042533_f64,
                a: 20_f64,
                c: 9_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 25817349.9728597_f64,
                a: 20_f64,
                c: 9_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler069_147: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000004e-06_f64 });
pub static mut data070_148: std::cell::LazyCell<[testcase_conf_hyperg_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_conf_hyperg_double_ {
                f0: -4.115767779294494e-08_f64,
                a: 20_f64,
                c: 10_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -2.0187210039960914e-07_f64,
                a: 20_f64,
                c: 10_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -2.2272304939386817e-07_f64,
                a: 20_f64,
                c: 10_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1.292556821260617e-06_f64,
                a: 20_f64,
                c: 10_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5.574457377599621e-06_f64,
                a: 20_f64,
                c: 10_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -6.256827201178729e-06_f64,
                a: 20_f64,
                c: 10_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: -0.00011955177906335608_f64,
                a: 20_f64,
                c: 10_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 9.247540551699115e-05_f64,
                a: 20_f64,
                c: 10_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.010123531287569976_f64,
                a: 20_f64,
                c: 10_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 0.12118937229909534_f64,
                a: 20_f64,
                c: 10_f64,
                x: -1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1_f64,
                a: 20_f64,
                c: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 6.831985794241554_f64,
                a: 20_f64,
                c: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 41.35665814081522_f64,
                a: 20_f64,
                c: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 229.57496033810904_f64,
                a: 20_f64,
                c: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 1192.78305499695_f64,
                a: 20_f64,
                c: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 5878.600388721592_f64,
                a: 20_f64,
                c: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 27741.7493226739_f64,
                a: 20_f64,
                c: 10_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 126220.5459930588_f64,
                a: 20_f64,
                c: 10_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 556592.1088661245_f64,
                a: 20_f64,
                c: 10_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 2388555.2873243927_f64,
                a: 20_f64,
                c: 10_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_conf_hyperg_double_ {
                f0: 10008079.497419352_f64,
                a: 20_f64,
                c: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler070_149: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000001e-07_f64 });
pub unsafe fn test_150(data: *const [testcase_conf_hyperg_double_; 21], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 21_();
    let mut i: () = 0_();
    'loop_: while ((i) < (num_datum)) {
        let f: () = (unsafe {
            let ___a: () = (*data)[(i) as usize].a;
            let ___c: () = (*data)[(i) as usize].c;
            let ___x: () = (*data)[(i) as usize].x;
            conf_hyperg_151(___a, ___c, ___x)
        });
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
                __builtin_fprintf_152 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 3843_() ) .into() , ( c"void test(const testcase_conf_hyperg<Ret> (&)[Num], Ret) [Ret = double, Num = 21U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_153(data: *const [testcase_conf_hyperg_double_; 20], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 20_();
    let mut i: () = 0_();
    'loop_: while ((i) < (num_datum)) {
        let f: () = (unsafe {
            let ___a: () = (*data)[(i) as usize].a;
            let ___c: () = (*data)[(i) as usize].c;
            let ___x: () = (*data)[(i) as usize].x;
            conf_hyperg_151(___a, ___c, ___x)
        });
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
                __builtin_fprintf_152 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 3843_() ) .into() , ( c"void test(const testcase_conf_hyperg<Ret> (&)[Num], Ret) [Ret = double, Num = 20U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_15)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_17)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_19)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_21)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_23)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_25)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_27)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_29)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_31)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_33)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_35)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_37)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_38)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_39)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_40)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_41)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_42)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_43)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_44)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_45)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_46)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_47)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data020_48)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler020_49)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data021_50)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler021_51)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data022_52)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler022_53)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data023_54)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler023_55)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data024_56)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler024_57)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data025_58)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler025_59)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data026_60)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler026_61)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data027_62)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler027_63)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data028_64)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler028_65)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data029_66)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler029_67)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data030_68)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler030_69)),
        )
    });
    (unsafe {
        test_153(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data031_70)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler031_71)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data032_72)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler032_73)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data033_74)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler033_75)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data034_76)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler034_77)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data035_78)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler035_79)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data036_80)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler036_81)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data037_82)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler037_83)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data038_84)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler038_85)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data039_86)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler039_87)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data040_88)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler040_89)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data041_90)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler041_91)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data042_92)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler042_93)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data043_94)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler043_95)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data044_96)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler044_97)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data045_98)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler045_99)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data046_100)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler046_101)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data047_102)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler047_103)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data048_104)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler048_105)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data049_106)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler049_107)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data050_108)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler050_109)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data051_110)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler051_111)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data052_112)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler052_113)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data053_114)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler053_115)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data054_116)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler054_117)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data055_118)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler055_119)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data056_120)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler056_121)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data057_122)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler057_123)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data058_124)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler058_125)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data059_126)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler059_127)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data060_128)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler060_129)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data061_130)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler061_131)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data062_132)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler062_133)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data063_134)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler063_135)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data064_136)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler064_137)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data065_138)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler065_139)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data066_140)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler066_141)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data067_142)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler067_143)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data068_144)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler068_145)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data069_146)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler069_147)),
        )
    });
    (unsafe {
        test_150(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data070_148)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler070_149)),
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
        this.pid_ = ((unsafe { getpid_154() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_155(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_156(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) })
            == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_157(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_158(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_159(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_158(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_159(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_158((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_159(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_160(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_158(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_159(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_161(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_162(
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
    let mut w: () =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1({ wanted }, None);
    if unimplemented!() {
        (unsafe { __throw_runtime_error_3(s) });
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_163(
    mut name: *const (),
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut res: *const () = (unimplemented!()).cast_const();
    if !(res).is_null() {
        let mut preLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { res },
                None,
            );
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (unimplemented!()).cast_const() },
                None,
            );
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_152 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"LC_ALL for ".as_ptr() },
            None,
        );
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub unsafe fn run_tests_wrapped_env_164(
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
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { env },
            None,
        );
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
        if ((self.pid_) == (unsafe { getpid_154() })) {
            (unsafe { semctl_156(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_165(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_165(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_166(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_167));
}
static mut tmp_167: std::cell::LazyCell<::libc::tm> =
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
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
