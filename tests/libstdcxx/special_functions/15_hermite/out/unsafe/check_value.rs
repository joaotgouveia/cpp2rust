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
pub struct testcase_hermite_double_ {
    pub f0: (),
    pub n: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_9: std::cell::LazyCell<[testcase_hermite_double_; 201]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.299999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.29999999999999893_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: -0.09999999999999964_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.10000000000000142_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.7000000000000011_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 2.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 3.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 4.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 6.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 7.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 8.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.200000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.700000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 9.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_11: std::cell::LazyCell<[testcase_hermite_double_; 201]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: -20_f64,
                n: 1_(),
                x: -10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -19.8_f64,
                n: 1_(),
                x: -9.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -19.6_f64,
                n: 1_(),
                x: -9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -19.4_f64,
                n: 1_(),
                x: -9.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -19.2_f64,
                n: 1_(),
                x: -9.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -19_f64,
                n: 1_(),
                x: -9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18.8_f64,
                n: 1_(),
                x: -9.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18.6_f64,
                n: 1_(),
                x: -9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18.4_f64,
                n: 1_(),
                x: -9.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18.2_f64,
                n: 1_(),
                x: -9.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18_f64,
                n: 1_(),
                x: -9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -17.8_f64,
                n: 1_(),
                x: -8.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -17.6_f64,
                n: 1_(),
                x: -8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -17.4_f64,
                n: 1_(),
                x: -8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -17.2_f64,
                n: 1_(),
                x: -8.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -17_f64,
                n: 1_(),
                x: -8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -16.8_f64,
                n: 1_(),
                x: -8.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -16.6_f64,
                n: 1_(),
                x: -8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -16.4_f64,
                n: 1_(),
                x: -8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -16.2_f64,
                n: 1_(),
                x: -8.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -16_f64,
                n: 1_(),
                x: -8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15.8_f64,
                n: 1_(),
                x: -7.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15.6_f64,
                n: 1_(),
                x: -7.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15.399999999999999_f64,
                n: 1_(),
                x: -7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15.2_f64,
                n: 1_(),
                x: -7.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15_f64,
                n: 1_(),
                x: -7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -14.8_f64,
                n: 1_(),
                x: -7.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -14.6_f64,
                n: 1_(),
                x: -7.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -14.399999999999999_f64,
                n: 1_(),
                x: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -14.2_f64,
                n: 1_(),
                x: -7.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -14_f64,
                n: 1_(),
                x: -7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -13.8_f64,
                n: 1_(),
                x: -6.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -13.6_f64,
                n: 1_(),
                x: -6.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -13.399999999999999_f64,
                n: 1_(),
                x: -6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -13.2_f64,
                n: 1_(),
                x: -6.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -13_f64,
                n: 1_(),
                x: -6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -12.8_f64,
                n: 1_(),
                x: -6.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -12.6_f64,
                n: 1_(),
                x: -6.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -12.399999999999999_f64,
                n: 1_(),
                x: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -12.2_f64,
                n: 1_(),
                x: -6.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -12_f64,
                n: 1_(),
                x: -6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -11.799999999999999_f64,
                n: 1_(),
                x: -5.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -11.6_f64,
                n: 1_(),
                x: -5.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -11.4_f64,
                n: 1_(),
                x: -5.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -11.2_f64,
                n: 1_(),
                x: -5.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -11_f64,
                n: 1_(),
                x: -5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -10.799999999999999_f64,
                n: 1_(),
                x: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -10.6_f64,
                n: 1_(),
                x: -5.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -10.399999999999999_f64,
                n: 1_(),
                x: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -10.2_f64,
                n: 1_(),
                x: -5.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -10_f64,
                n: 1_(),
                x: -5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9.799999999999999_f64,
                n: 1_(),
                x: -4.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9.6_f64,
                n: 1_(),
                x: -4.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9.399999999999999_f64,
                n: 1_(),
                x: -4.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9.2_f64,
                n: 1_(),
                x: -4.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9_f64,
                n: 1_(),
                x: -4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.799999999999999_f64,
                n: 1_(),
                x: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.6_f64,
                n: 1_(),
                x: -4.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.399999999999999_f64,
                n: 1_(),
                x: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.2_f64,
                n: 1_(),
                x: -4.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8_f64,
                n: 1_(),
                x: -4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -7.799999999999999_f64,
                n: 1_(),
                x: -3.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -7.6_f64,
                n: 1_(),
                x: -3.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -7.399999999999999_f64,
                n: 1_(),
                x: -3.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -7.199999999999999_f64,
                n: 1_(),
                x: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -7_f64,
                n: 1_(),
                x: -3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.799999999999999_f64,
                n: 1_(),
                x: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.6_f64,
                n: 1_(),
                x: -3.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.399999999999999_f64,
                n: 1_(),
                x: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.199999999999999_f64,
                n: 1_(),
                x: -3.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6_f64,
                n: 1_(),
                x: -3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.799999999999999_f64,
                n: 1_(),
                x: -2.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.6_f64,
                n: 1_(),
                x: -2.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.399999999999999_f64,
                n: 1_(),
                x: -2.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.199999999999999_f64,
                n: 1_(),
                x: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5_f64,
                n: 1_(),
                x: -2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.799999999999999_f64,
                n: 1_(),
                x: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.6_f64,
                n: 1_(),
                x: -2.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.399999999999999_f64,
                n: 1_(),
                x: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.199999999999999_f64,
                n: 1_(),
                x: -2.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4_f64,
                n: 1_(),
                x: -2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.8000000000000007_f64,
                n: 1_(),
                x: -1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.599999999999998_f64,
                n: 1_(),
                x: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.3999999999999986_f64,
                n: 1_(),
                x: -1.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.1999999999999993_f64,
                n: 1_(),
                x: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3_f64,
                n: 1_(),
                x: -1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.8000000000000007_f64,
                n: 1_(),
                x: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.599999999999998_f64,
                n: 1_(),
                x: -1.299999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.3999999999999986_f64,
                n: 1_(),
                x: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.1999999999999993_f64,
                n: 1_(),
                x: -1.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2_f64,
                n: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.8000000000000007_f64,
                n: 1_(),
                x: -0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.5999999999999979_f64,
                n: 1_(),
                x: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.3999999999999986_f64,
                n: 1_(),
                x: -0.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.1999999999999993_f64,
                n: 1_(),
                x: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1_f64,
                n: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -0.7999999999999972_f64,
                n: 1_(),
                x: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -0.5999999999999979_f64,
                n: 1_(),
                x: -0.29999999999999893_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -0.3999999999999986_f64,
                n: 1_(),
                x: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -0.1999999999999993_f64,
                n: 1_(),
                x: -0.09999999999999964_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 0_f64,
                n: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 0.20000000000000284_f64,
                n: 1_(),
                x: 0.10000000000000142_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 0.40000000000000213_f64,
                n: 1_(),
                x: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 0.6000000000000014_f64,
                n: 1_(),
                x: 0.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 0.8000000000000007_f64,
                n: 1_(),
                x: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1_f64,
                n: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2000000000000028_f64,
                n: 1_(),
                x: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4000000000000021_f64,
                n: 1_(),
                x: 0.7000000000000011_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.6000000000000014_f64,
                n: 1_(),
                x: 0.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.8000000000000007_f64,
                n: 1_(),
                x: 0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2_f64,
                n: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.200000000000003_f64,
                n: 1_(),
                x: 1.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.400000000000002_f64,
                n: 1_(),
                x: 1.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.6000000000000014_f64,
                n: 1_(),
                x: 1.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.8000000000000007_f64,
                n: 1_(),
                x: 1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3_f64,
                n: 1_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.200000000000003_f64,
                n: 1_(),
                x: 1.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.400000000000002_f64,
                n: 1_(),
                x: 1.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.6000000000000014_f64,
                n: 1_(),
                x: 1.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.8000000000000007_f64,
                n: 1_(),
                x: 1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4_f64,
                n: 1_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.200000000000003_f64,
                n: 1_(),
                x: 2.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.400000000000002_f64,
                n: 1_(),
                x: 2.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.600000000000001_f64,
                n: 1_(),
                x: 2.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.800000000000001_f64,
                n: 1_(),
                x: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5_f64,
                n: 1_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.200000000000003_f64,
                n: 1_(),
                x: 2.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.400000000000002_f64,
                n: 1_(),
                x: 2.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.600000000000001_f64,
                n: 1_(),
                x: 2.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.800000000000001_f64,
                n: 1_(),
                x: 2.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6_f64,
                n: 1_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.200000000000003_f64,
                n: 1_(),
                x: 3.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.400000000000002_f64,
                n: 1_(),
                x: 3.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.600000000000001_f64,
                n: 1_(),
                x: 3.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.800000000000001_f64,
                n: 1_(),
                x: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7_f64,
                n: 1_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.200000000000003_f64,
                n: 1_(),
                x: 3.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.400000000000002_f64,
                n: 1_(),
                x: 3.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.600000000000001_f64,
                n: 1_(),
                x: 3.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.800000000000001_f64,
                n: 1_(),
                x: 3.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8_f64,
                n: 1_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.200000000000003_f64,
                n: 1_(),
                x: 4.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.400000000000002_f64,
                n: 1_(),
                x: 4.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.600000000000001_f64,
                n: 1_(),
                x: 4.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.8_f64,
                n: 1_(),
                x: 4.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9_f64,
                n: 1_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.200000000000003_f64,
                n: 1_(),
                x: 4.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.400000000000002_f64,
                n: 1_(),
                x: 4.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.600000000000001_f64,
                n: 1_(),
                x: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.8_f64,
                n: 1_(),
                x: 4.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10.200000000000003_f64,
                n: 1_(),
                x: 5.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10.400000000000002_f64,
                n: 1_(),
                x: 5.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10.600000000000001_f64,
                n: 1_(),
                x: 5.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10.8_f64,
                n: 1_(),
                x: 5.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 11_f64,
                n: 1_(),
                x: 5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 11.200000000000003_f64,
                n: 1_(),
                x: 5.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 11.400000000000002_f64,
                n: 1_(),
                x: 5.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 11.600000000000001_f64,
                n: 1_(),
                x: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 11.8_f64,
                n: 1_(),
                x: 5.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 12_f64,
                n: 1_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 12.200000000000003_f64,
                n: 1_(),
                x: 6.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 12.399999999999999_f64,
                n: 1_(),
                x: 6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 12.600000000000001_f64,
                n: 1_(),
                x: 6.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 12.800000000000004_f64,
                n: 1_(),
                x: 6.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 13_f64,
                n: 1_(),
                x: 6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 13.200000000000003_f64,
                n: 1_(),
                x: 6.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 13.399999999999999_f64,
                n: 1_(),
                x: 6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 13.600000000000001_f64,
                n: 1_(),
                x: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 13.800000000000004_f64,
                n: 1_(),
                x: 6.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 14_f64,
                n: 1_(),
                x: 7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 14.200000000000003_f64,
                n: 1_(),
                x: 7.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 14.399999999999999_f64,
                n: 1_(),
                x: 7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 14.600000000000001_f64,
                n: 1_(),
                x: 7.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 14.800000000000004_f64,
                n: 1_(),
                x: 7.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 15_f64,
                n: 1_(),
                x: 7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 15.200000000000003_f64,
                n: 1_(),
                x: 7.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 15.399999999999999_f64,
                n: 1_(),
                x: 7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 15.600000000000001_f64,
                n: 1_(),
                x: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 15.800000000000004_f64,
                n: 1_(),
                x: 7.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 16_f64,
                n: 1_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 16.200000000000003_f64,
                n: 1_(),
                x: 8.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 16.4_f64,
                n: 1_(),
                x: 8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 16.6_f64,
                n: 1_(),
                x: 8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 16.800000000000004_f64,
                n: 1_(),
                x: 8.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 17_f64,
                n: 1_(),
                x: 8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 17.200000000000003_f64,
                n: 1_(),
                x: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 17.4_f64,
                n: 1_(),
                x: 8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 17.6_f64,
                n: 1_(),
                x: 8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 17.800000000000004_f64,
                n: 1_(),
                x: 8.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 18_f64,
                n: 1_(),
                x: 9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 18.200000000000003_f64,
                n: 1_(),
                x: 9.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 18.400000000000006_f64,
                n: 1_(),
                x: 9.200000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 18.6_f64,
                n: 1_(),
                x: 9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 18.800000000000004_f64,
                n: 1_(),
                x: 9.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19_f64,
                n: 1_(),
                x: 9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19.200000000000003_f64,
                n: 1_(),
                x: 9.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19.400000000000006_f64,
                n: 1_(),
                x: 9.700000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19.6_f64,
                n: 1_(),
                x: 9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19.800000000000004_f64,
                n: 1_(),
                x: 9.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 20_f64,
                n: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_13: std::cell::LazyCell<[testcase_hermite_double_; 201]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 398_f64,
                n: 2_(),
                x: -10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 390.04_f64,
                n: 2_(),
                x: -9.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 382.1600000000001_f64,
                n: 2_(),
                x: -9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 374.35999999999996_f64,
                n: 2_(),
                x: -9.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 366.64_f64,
                n: 2_(),
                x: -9.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 359_f64,
                n: 2_(),
                x: -9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 351.44000000000005_f64,
                n: 2_(),
                x: -9.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 343.96000000000004_f64,
                n: 2_(),
                x: -9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 336.55999999999995_f64,
                n: 2_(),
                x: -9.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 329.23999999999995_f64,
                n: 2_(),
                x: -9.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 322_f64,
                n: 2_(),
                x: -9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 314.84000000000003_f64,
                n: 2_(),
                x: -8.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 307.76000000000005_f64,
                n: 2_(),
                x: -8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 300.75999999999993_f64,
                n: 2_(),
                x: -8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 293.84_f64,
                n: 2_(),
                x: -8.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 287_f64,
                n: 2_(),
                x: -8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 280.24_f64,
                n: 2_(),
                x: -8.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 273.56000000000006_f64,
                n: 2_(),
                x: -8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 266.96_f64,
                n: 2_(),
                x: -8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 260.44_f64,
                n: 2_(),
                x: -8.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 254_f64,
                n: 2_(),
                x: -8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 247.64000000000001_f64,
                n: 2_(),
                x: -7.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 241.35999999999999_f64,
                n: 2_(),
                x: -7.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 235.15999999999997_f64,
                n: 2_(),
                x: -7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 229.04_f64,
                n: 2_(),
                x: -7.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 223_f64,
                n: 2_(),
                x: -7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 217.04000000000002_f64,
                n: 2_(),
                x: -7.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 211.16_f64,
                n: 2_(),
                x: -7.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 205.35999999999996_f64,
                n: 2_(),
                x: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 199.64_f64,
                n: 2_(),
                x: -7.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 194_f64,
                n: 2_(),
                x: -7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 188.44000000000003_f64,
                n: 2_(),
                x: -6.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 182.95999999999998_f64,
                n: 2_(),
                x: -6.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 177.55999999999997_f64,
                n: 2_(),
                x: -6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 172.23999999999998_f64,
                n: 2_(),
                x: -6.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 167_f64,
                n: 2_(),
                x: -6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 161.84000000000003_f64,
                n: 2_(),
                x: -6.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 156.76_f64,
                n: 2_(),
                x: -6.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 151.75999999999996_f64,
                n: 2_(),
                x: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 146.83999999999997_f64,
                n: 2_(),
                x: -6.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 142_f64,
                n: 2_(),
                x: -6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 137.23999999999998_f64,
                n: 2_(),
                x: -5.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 132.56_f64,
                n: 2_(),
                x: -5.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 127.96000000000001_f64,
                n: 2_(),
                x: -5.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 123.43999999999998_f64,
                n: 2_(),
                x: -5.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 119_f64,
                n: 2_(),
                x: -5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 114.63999999999997_f64,
                n: 2_(),
                x: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 110.36_f64,
                n: 2_(),
                x: -5.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 106.15999999999997_f64,
                n: 2_(),
                x: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 102.03999999999999_f64,
                n: 2_(),
                x: -5.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 98_f64,
                n: 2_(),
                x: -5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 94.03999999999998_f64,
                n: 2_(),
                x: -4.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 90.16_f64,
                n: 2_(),
                x: -4.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 86.35999999999997_f64,
                n: 2_(),
                x: -4.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 82.63999999999999_f64,
                n: 2_(),
                x: -4.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 79_f64,
                n: 2_(),
                x: -4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 75.43999999999998_f64,
                n: 2_(),
                x: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 71.96_f64,
                n: 2_(),
                x: -4.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 68.55999999999997_f64,
                n: 2_(),
                x: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 65.24_f64,
                n: 2_(),
                x: -4.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 62_f64,
                n: 2_(),
                x: -4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 58.83999999999998_f64,
                n: 2_(),
                x: -3.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 55.76_f64,
                n: 2_(),
                x: -3.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 52.75999999999998_f64,
                n: 2_(),
                x: -3.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 49.83999999999999_f64,
                n: 2_(),
                x: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 47_f64,
                n: 2_(),
                x: -3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 44.23999999999999_f64,
                n: 2_(),
                x: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 41.559999999999995_f64,
                n: 2_(),
                x: -3.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 38.95999999999998_f64,
                n: 2_(),
                x: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 36.43999999999999_f64,
                n: 2_(),
                x: -3.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 34_f64,
                n: 2_(),
                x: -3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 31.639999999999986_f64,
                n: 2_(),
                x: -2.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 29.359999999999996_f64,
                n: 2_(),
                x: -2.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 27.159999999999986_f64,
                n: 2_(),
                x: -2.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 25.039999999999992_f64,
                n: 2_(),
                x: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 23_f64,
                n: 2_(),
                x: -2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 21.03999999999999_f64,
                n: 2_(),
                x: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19.159999999999997_f64,
                n: 2_(),
                x: -2.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 17.35999999999999_f64,
                n: 2_(),
                x: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 15.639999999999993_f64,
                n: 2_(),
                x: -2.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 14_f64,
                n: 2_(),
                x: -2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 12.440000000000005_f64,
                n: 2_(),
                x: -1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10.959999999999985_f64,
                n: 2_(),
                x: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.55999999999999_f64,
                n: 2_(),
                x: -1.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.239999999999995_f64,
                n: 2_(),
                x: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7_f64,
                n: 2_(),
                x: -1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.840000000000004_f64,
                n: 2_(),
                x: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.759999999999989_f64,
                n: 2_(),
                x: -1.299999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.7599999999999936_f64,
                n: 2_(),
                x: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.839999999999997_f64,
                n: 2_(),
                x: -1.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2_f64,
                n: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2400000000000024_f64,
                n: 2_(),
                x: -0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 0.5599999999999934_f64,
                n: 2_(),
                x: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -0.04000000000000403_f64,
                n: 2_(),
                x: -0.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -0.5600000000000016_f64,
                n: 2_(),
                x: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1_f64,
                n: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.3600000000000045_f64,
                n: 2_(),
                x: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.6400000000000026_f64,
                n: 2_(),
                x: -0.29999999999999893_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.8400000000000012_f64,
                n: 2_(),
                x: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.9600000000000002_f64,
                n: 2_(),
                x: -0.09999999999999964_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2_f64,
                n: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.9599999999999989_f64,
                n: 2_(),
                x: 0.10000000000000142_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.8399999999999983_f64,
                n: 2_(),
                x: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.6399999999999983_f64,
                n: 2_(),
                x: 0.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.359999999999999_f64,
                n: 2_(),
                x: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1_f64,
                n: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -0.5599999999999932_f64,
                n: 2_(),
                x: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -0.03999999999999404_f64,
                n: 2_(),
                x: 0.7000000000000011_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 0.5600000000000045_f64,
                n: 2_(),
                x: 0.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2400000000000024_f64,
                n: 2_(),
                x: 0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2_f64,
                n: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.8400000000000123_f64,
                n: 2_(),
                x: 1.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.7600000000000104_f64,
                n: 2_(),
                x: 1.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.760000000000008_f64,
                n: 2_(),
                x: 1.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.840000000000004_f64,
                n: 2_(),
                x: 1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7_f64,
                n: 2_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.240000000000018_f64,
                n: 2_(),
                x: 1.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.560000000000015_f64,
                n: 2_(),
                x: 1.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10.96000000000001_f64,
                n: 2_(),
                x: 1.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 12.440000000000005_f64,
                n: 2_(),
                x: 1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 14_f64,
                n: 2_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 15.640000000000025_f64,
                n: 2_(),
                x: 2.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 17.360000000000017_f64,
                n: 2_(),
                x: 2.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19.160000000000014_f64,
                n: 2_(),
                x: 2.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 21.040000000000006_f64,
                n: 2_(),
                x: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 23_f64,
                n: 2_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 25.04000000000003_f64,
                n: 2_(),
                x: 2.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 27.16000000000002_f64,
                n: 2_(),
                x: 2.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 29.360000000000017_f64,
                n: 2_(),
                x: 2.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 31.640000000000008_f64,
                n: 2_(),
                x: 2.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 34_f64,
                n: 2_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 36.44000000000003_f64,
                n: 2_(),
                x: 3.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 38.96000000000003_f64,
                n: 2_(),
                x: 3.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 41.56000000000002_f64,
                n: 2_(),
                x: 3.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 44.24000000000001_f64,
                n: 2_(),
                x: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 47_f64,
                n: 2_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 49.84000000000004_f64,
                n: 2_(),
                x: 3.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 52.76000000000003_f64,
                n: 2_(),
                x: 3.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 55.76000000000002_f64,
                n: 2_(),
                x: 3.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 58.84000000000001_f64,
                n: 2_(),
                x: 3.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 62_f64,
                n: 2_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 65.24000000000005_f64,
                n: 2_(),
                x: 4.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 68.56000000000003_f64,
                n: 2_(),
                x: 4.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 71.96000000000002_f64,
                n: 2_(),
                x: 4.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 75.44000000000001_f64,
                n: 2_(),
                x: 4.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 79_f64,
                n: 2_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 82.64000000000006_f64,
                n: 2_(),
                x: 4.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 86.36000000000004_f64,
                n: 2_(),
                x: 4.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 90.16000000000003_f64,
                n: 2_(),
                x: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 94.04000000000002_f64,
                n: 2_(),
                x: 4.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 98_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 102.04000000000006_f64,
                n: 2_(),
                x: 5.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 106.16000000000004_f64,
                n: 2_(),
                x: 5.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 110.36000000000003_f64,
                n: 2_(),
                x: 5.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 114.64000000000001_f64,
                n: 2_(),
                x: 5.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 119_f64,
                n: 2_(),
                x: 5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 123.44000000000007_f64,
                n: 2_(),
                x: 5.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 127.96000000000004_f64,
                n: 2_(),
                x: 5.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 132.56000000000003_f64,
                n: 2_(),
                x: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 137.24_f64,
                n: 2_(),
                x: 5.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 142_f64,
                n: 2_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 146.84000000000006_f64,
                n: 2_(),
                x: 6.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 151.75999999999996_f64,
                n: 2_(),
                x: 6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 156.76000000000005_f64,
                n: 2_(),
                x: 6.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 161.84000000000012_f64,
                n: 2_(),
                x: 6.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 167_f64,
                n: 2_(),
                x: 6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 172.24000000000007_f64,
                n: 2_(),
                x: 6.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 177.55999999999997_f64,
                n: 2_(),
                x: 6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 182.96000000000004_f64,
                n: 2_(),
                x: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 188.4400000000001_f64,
                n: 2_(),
                x: 6.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 194_f64,
                n: 2_(),
                x: 7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 199.64000000000007_f64,
                n: 2_(),
                x: 7.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 205.35999999999996_f64,
                n: 2_(),
                x: 7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 211.16000000000005_f64,
                n: 2_(),
                x: 7.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 217.04000000000013_f64,
                n: 2_(),
                x: 7.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 223_f64,
                n: 2_(),
                x: 7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 229.04000000000008_f64,
                n: 2_(),
                x: 7.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 235.15999999999997_f64,
                n: 2_(),
                x: 7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 241.36000000000004_f64,
                n: 2_(),
                x: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 247.64000000000013_f64,
                n: 2_(),
                x: 7.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 254_f64,
                n: 2_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 260.4400000000001_f64,
                n: 2_(),
                x: 8.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 266.96_f64,
                n: 2_(),
                x: 8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 273.56000000000006_f64,
                n: 2_(),
                x: 8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 280.2400000000001_f64,
                n: 2_(),
                x: 8.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 287_f64,
                n: 2_(),
                x: 8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 293.8400000000001_f64,
                n: 2_(),
                x: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 300.75999999999993_f64,
                n: 2_(),
                x: 8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 307.76000000000005_f64,
                n: 2_(),
                x: 8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 314.84000000000015_f64,
                n: 2_(),
                x: 8.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 322_f64,
                n: 2_(),
                x: 9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 329.2400000000001_f64,
                n: 2_(),
                x: 9.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 336.56000000000023_f64,
                n: 2_(),
                x: 9.200000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 343.96000000000004_f64,
                n: 2_(),
                x: 9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 351.44000000000017_f64,
                n: 2_(),
                x: 9.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 359_f64,
                n: 2_(),
                x: 9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 366.6400000000001_f64,
                n: 2_(),
                x: 9.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 374.36000000000024_f64,
                n: 2_(),
                x: 9.700000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 382.1600000000001_f64,
                n: 2_(),
                x: 9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 390.0400000000002_f64,
                n: 2_(),
                x: 9.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 398_f64,
                n: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_15: std::cell::LazyCell<[testcase_hermite_double_; 201]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: -3041200_f64,
                n: 5_(),
                x: -10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2889108.3196800007_f64,
                n: 5_(),
                x: -9.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2743131.8297600015_f64,
                n: 5_(),
                x: -9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2603085.202239999_f64,
                n: 5_(),
                x: -9.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2468786.872319999_f64,
                n: 5_(),
                x: -9.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2340059_f64,
                n: 5_(),
                x: -9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2216727.431680001_f64,
                n: 5_(),
                x: -9.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2098621.661760001_f64,
                n: 5_(),
                x: -9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1985574.7942399993_f64,
                n: 5_(),
                x: -9.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1877423.5043199996_f64,
                n: 5_(),
                x: -9.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1774008_f64,
                n: 5_(),
                x: -9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1675171.9836800008_f64,
                n: 5_(),
                x: -8.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1580762.6137600006_f64,
                n: 5_(),
                x: -8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1490630.466239999_f64,
                n: 5_(),
                x: -8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1404629.49632_f64,
                n: 5_(),
                x: -8.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1322617_f64,
                n: 5_(),
                x: -8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1244453.5756800002_f64,
                n: 5_(),
                x: -8.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1170003.0857600006_f64,
                n: 5_(),
                x: -8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1099132.6182399995_f64,
                n: 5_(),
                x: -8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1031712.4483199996_f64,
                n: 5_(),
                x: -8.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -967616_f64,
                n: 5_(),
                x: -8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -906719.8076800001_f64,
                n: 5_(),
                x: -7.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -848903.4777599999_f64,
                n: 5_(),
                x: -7.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -794049.6502399996_f64,
                n: 5_(),
                x: -7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -742043.96032_f64,
                n: 5_(),
                x: -7.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -692775_f64,
                n: 5_(),
                x: -7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -646134.2796800003_f64,
                n: 5_(),
                x: -7.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -602016.1897599999_f64,
                n: 5_(),
                x: -7.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -560317.9622399998_f64,
                n: 5_(),
                x: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -520939.6323199999_f64,
                n: 5_(),
                x: -7.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -483784_f64,
                n: 5_(),
                x: -7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -448756.5916800002_f64,
                n: 5_(),
                x: -6.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -415765.62176_f64,
                n: 5_(),
                x: -6.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -384721.95423999976_f64,
                n: 5_(),
                x: -6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -355539.06431999995_f64,
                n: 5_(),
                x: -6.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -328133_f64,
                n: 5_(),
                x: -6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -302422.3436800001_f64,
                n: 5_(),
                x: -6.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -278328.17376_f64,
                n: 5_(),
                x: -6.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -255774.02623999983_f64,
                n: 5_(),
                x: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -234685.8563199999_f64,
                n: 5_(),
                x: -6.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -214992_f64,
                n: 5_(),
                x: -6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -196623.1356799999_f64,
                n: 5_(),
                x: -5.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -179512.24576_f64,
                n: 5_(),
                x: -5.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -163594.57824000003_f64,
                n: 5_(),
                x: -5.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -148807.60831999997_f64,
                n: 5_(),
                x: -5.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -135091_f64,
                n: 5_(),
                x: -5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -122386.56767999992_f64,
                n: 5_(),
                x: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -110638.23776_f64,
                n: 5_(),
                x: -5.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -99792.01023999995_f64,
                n: 5_(),
                x: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -89795.92031999998_f64,
                n: 5_(),
                x: -5.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -80600_f64,
                n: 5_(),
                x: -5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -72156.23967999994_f64,
                n: 5_(),
                x: -4.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -64418.549759999994_f64,
                n: 5_(),
                x: -4.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -57342.72223999995_f64,
                n: 5_(),
                x: -4.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -50886.39231999997_f64,
                n: 5_(),
                x: -4.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -45009_f64,
                n: 5_(),
                x: -4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -39671.751679999965_f64,
                n: 5_(),
                x: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -34837.58175999999_f64,
                n: 5_(),
                x: -4.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -30471.114239999963_f64,
                n: 5_(),
                x: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -26538.62431999999_f64,
                n: 5_(),
                x: -4.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -23008_f64,
                n: 5_(),
                x: -4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -19848.70367999998_f64,
                n: 5_(),
                x: -3.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -17031.733759999996_f64,
                n: 5_(),
                x: -3.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -14529.586239999979_f64,
                n: 5_(),
                x: -3.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -12316.216319999992_f64,
                n: 5_(),
                x: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -10367_f64,
                n: 5_(),
                x: -3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8658.695679999993_f64,
                n: 5_(),
                x: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -7169.405759999996_f64,
                n: 5_(),
                x: -3.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5878.538239999991_f64,
                n: 5_(),
                x: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4766.7683199999965_f64,
                n: 5_(),
                x: -3.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3816_f64,
                n: 5_(),
                x: -3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3009.327679999996_f64,
                n: 5_(),
                x: -2.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2330.997759999999_f64,
                n: 5_(),
                x: -2.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1766.3702399999963_f64,
                n: 5_(),
                x: -2.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1301.8803199999984_f64,
                n: 5_(),
                x: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -925_f64,
                n: 5_(),
                x: -2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -624.1996799999986_f64,
                n: 5_(),
                x: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -388.9097599999998_f64,
                n: 5_(),
                x: -2.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -209.4822399999989_f64,
                n: 5_(),
                x: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -77.15231999999952_f64,
                n: 5_(),
                x: -2.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 16_f64,
                n: 5_(),
                x: -2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 77.08831999999984_f64,
                n: 5_(),
                x: -1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 112.45824000000026_f64,
                n: 5_(),
                x: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 127.72576000000002_f64,
                n: 5_(),
                x: -1.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 127.81567999999996_f64,
                n: 5_(),
                x: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 117_f64,
                n: 5_(),
                x: -1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 98.93632000000008_f64,
                n: 5_(),
                x: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 76.70623999999975_f64,
                n: 5_(),
                x: -1.299999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 52.85375999999984_f64,
                n: 5_(),
                x: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 29.423679999999933_f64,
                n: 5_(),
                x: -1.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8_f64,
                n: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -10.255679999999948_f64,
                n: 5_(),
                x: -0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -24.565760000000125_f64,
                n: 5_(),
                x: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -34.49824000000005_f64,
                n: 5_(),
                x: -0.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -39.92832000000001_f64,
                n: 5_(),
                x: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -41_f64,
                n: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -38.087679999999935_f64,
                n: 5_(),
                x: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -31.75775999999992_f64,
                n: 5_(),
                x: -0.29999999999999893_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -22.73023999999993_f64,
                n: 5_(),
                x: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -11.84031999999996_f64,
                n: 5_(),
                x: -0.09999999999999964_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 0_f64,
                n: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 11.840320000000162_f64,
                n: 5_(),
                x: 0.10000000000000142_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 22.73024000000011_f64,
                n: 5_(),
                x: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 31.757760000000054_f64,
                n: 5_(),
                x: 0.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 38.08768000000002_f64,
                n: 5_(),
                x: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 41_f64,
                n: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 39.92831999999996_f64,
                n: 5_(),
                x: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 34.498239999999925_f64,
                n: 5_(),
                x: 0.7000000000000011_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 24.565759999999916_f64,
                n: 5_(),
                x: 0.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10.255679999999948_f64,
                n: 5_(),
                x: 0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8_f64,
                n: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -29.423680000000317_f64,
                n: 5_(),
                x: 1.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -52.853760000000264_f64,
                n: 5_(),
                x: 1.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -76.70624000000018_f64,
                n: 5_(),
                x: 1.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -98.93632000000008_f64,
                n: 5_(),
                x: 1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -117_f64,
                n: 5_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -127.8156800000001_f64,
                n: 5_(),
                x: 1.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -127.72575999999992_f64,
                n: 5_(),
                x: 1.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -112.4582399999998_f64,
                n: 5_(),
                x: 1.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -77.08831999999984_f64,
                n: 5_(),
                x: 1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -16_f64,
                n: 5_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 77.15232000000162_f64,
                n: 5_(),
                x: 2.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 209.48224000000164_f64,
                n: 5_(),
                x: 2.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 388.9097600000015_f64,
                n: 5_(),
                x: 2.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 624.1996800000007_f64,
                n: 5_(),
                x: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 925_f64,
                n: 5_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1301.880320000006_f64,
                n: 5_(),
                x: 2.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1766.3702400000057_f64,
                n: 5_(),
                x: 2.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2330.9977600000047_f64,
                n: 5_(),
                x: 2.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3009.3276800000026_f64,
                n: 5_(),
                x: 2.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3816_f64,
                n: 5_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4766.768320000015_f64,
                n: 5_(),
                x: 3.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5878.538240000013_f64,
                n: 5_(),
                x: 3.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7169.405760000009_f64,
                n: 5_(),
                x: 3.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8658.695680000008_f64,
                n: 5_(),
                x: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10367_f64,
                n: 5_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 12316.216320000027_f64,
                n: 5_(),
                x: 3.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 14529.586240000028_f64,
                n: 5_(),
                x: 3.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 17031.73376000002_f64,
                n: 5_(),
                x: 3.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19848.703680000013_f64,
                n: 5_(),
                x: 3.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 23008_f64,
                n: 5_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 26538.624320000057_f64,
                n: 5_(),
                x: 4.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 30471.114240000043_f64,
                n: 5_(),
                x: 4.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 34837.58176000004_f64,
                n: 5_(),
                x: 4.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 39671.75168000001_f64,
                n: 5_(),
                x: 4.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 45009_f64,
                n: 5_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 50886.392320000086_f64,
                n: 5_(),
                x: 4.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 57342.722240000076_f64,
                n: 5_(),
                x: 4.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 64418.54976000005_f64,
                n: 5_(),
                x: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 72156.23968000003_f64,
                n: 5_(),
                x: 4.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 80600_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 89795.92032000014_f64,
                n: 5_(),
                x: 5.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 99792.0102400001_f64,
                n: 5_(),
                x: 5.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 110638.23776000006_f64,
                n: 5_(),
                x: 5.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 122386.56768000004_f64,
                n: 5_(),
                x: 5.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 135091_f64,
                n: 5_(),
                x: 5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 148807.6083200002_f64,
                n: 5_(),
                x: 5.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 163594.57824000015_f64,
                n: 5_(),
                x: 5.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 179512.2457600001_f64,
                n: 5_(),
                x: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 196623.13568000006_f64,
                n: 5_(),
                x: 5.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 214992_f64,
                n: 5_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 234685.8563200003_f64,
                n: 5_(),
                x: 6.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 255774.02623999983_f64,
                n: 5_(),
                x: 6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 278328.1737600002_f64,
                n: 5_(),
                x: 6.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 302422.34368000063_f64,
                n: 5_(),
                x: 6.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 328133_f64,
                n: 5_(),
                x: 6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 355539.06432000035_f64,
                n: 5_(),
                x: 6.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 384721.95423999976_f64,
                n: 5_(),
                x: 6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 415765.6217600002_f64,
                n: 5_(),
                x: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 448756.59168000077_f64,
                n: 5_(),
                x: 6.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 483784_f64,
                n: 5_(),
                x: 7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 520939.63232000044_f64,
                n: 5_(),
                x: 7.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 560317.9622399998_f64,
                n: 5_(),
                x: 7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 602016.1897600003_f64,
                n: 5_(),
                x: 7.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 646134.279680001_f64,
                n: 5_(),
                x: 7.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 692775_f64,
                n: 5_(),
                x: 7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 742043.9603200005_f64,
                n: 5_(),
                x: 7.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 794049.6502399996_f64,
                n: 5_(),
                x: 7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 848903.4777600003_f64,
                n: 5_(),
                x: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 906719.8076800012_f64,
                n: 5_(),
                x: 7.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 967616_f64,
                n: 5_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1031712.4483200011_f64,
                n: 5_(),
                x: 8.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1099132.6182399995_f64,
                n: 5_(),
                x: 8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1170003.0857600006_f64,
                n: 5_(),
                x: 8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1244453.5756800014_f64,
                n: 5_(),
                x: 8.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1322617_f64,
                n: 5_(),
                x: 8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1404629.4963200013_f64,
                n: 5_(),
                x: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1490630.466239999_f64,
                n: 5_(),
                x: 8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1580762.6137600006_f64,
                n: 5_(),
                x: 8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1675171.9836800022_f64,
                n: 5_(),
                x: 8.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1774008_f64,
                n: 5_(),
                x: 9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1877423.5043200015_f64,
                n: 5_(),
                x: 9.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1985574.7942400032_f64,
                n: 5_(),
                x: 9.200000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2098621.661760001_f64,
                n: 5_(),
                x: 9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2216727.4316800022_f64,
                n: 5_(),
                x: 9.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2340059_f64,
                n: 5_(),
                x: 9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2468786.872320002_f64,
                n: 5_(),
                x: 9.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2603085.2022400037_f64,
                n: 5_(),
                x: 9.700000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2743131.8297600015_f64,
                n: 5_(),
                x: 9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2889108.319680004_f64,
                n: 5_(),
                x: 9.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3041200_f64,
                n: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_17: std::cell::LazyCell<[testcase_hermite_double_; 201]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 8093278209760_f64,
                n: 10_(),
                x: -10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7282867075495.307_f64,
                n: 10_(),
                x: -9.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6545846221520.677_f64,
                n: 10_(),
                x: -9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5876279006180.638_f64,
                n: 10_(),
                x: -9.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5268651052510.467_f64,
                n: 10_(),
                x: -9.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4717844376391_f64,
                n: 10_(),
                x: -9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4219112842239.1147_f64,
                n: 10_(),
                x: -9.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3768058891466.047_f64,
                n: 10_(),
                x: -9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3360611490639.089_f64,
                n: 10_(),
                x: -9.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2993005247949.7607_f64,
                n: 10_(),
                x: -9.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2661760648224_f64,
                n: 10_(),
                x: -9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2363665358307.8496_f64,
                n: 10_(),
                x: -8.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2095756556225.3428_f64,
                n: 10_(),
                x: -8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1855304239034.7983_f64,
                n: 10_(),
                x: -8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1639795465805.4746_f64,
                n: 10_(),
                x: -8.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1446919493599_f64,
                n: 10_(),
                x: -8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1274553765769.7463_f64,
                n: 10_(),
                x: -8.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1120750713295.2778_f64,
                n: 10_(),
                x: -8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 983725331213.0719_f64,
                n: 10_(),
                x: -8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 861843493572.9005_f64,
                n: 10_(),
                x: -8.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 753610971616_f64,
                n: 10_(),
                x: -8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 657663121163.0245_f64,
                n: 10_(),
                x: -7.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 572755206432.8134_f64,
                n: 10_(),
                x: -7.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 497753328723.87476_f64,
                n: 10_(),
                x: -7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 431625929570.40063_f64,
                n: 10_(),
                x: -7.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 373435839135_f64,
                n: 10_(),
                x: -7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 322332841721.5573_f64,
                n: 10_(),
                x: -7.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 277546731384.0178_f64,
                n: 10_(),
                x: -7.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 238380831670.8996_f64,
                n: 10_(),
                x: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 204205954581.2473_f64,
                n: 10_(),
                x: -7.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 174454774816_f64,
                n: 10_(),
                x: -7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 148616596389.6723_f64,
                n: 10_(),
                x: -6.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 126232489621.23923_f64,
                n: 10_(),
                x: -6.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 106890777450.54652_f64,
                n: 10_(),
                x: -6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 90222850927.78798_f64,
                n: 10_(),
                x: -6.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 75899294599_f64,
                n: 10_(),
                x: -6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 63626303360.46811_f64,
                n: 10_(),
                x: -6.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 53142373179.80605_f64,
                n: 10_(),
                x: -6.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 44215248881.62544_f64,
                n: 10_(),
                x: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 36639112971.52768_f64,
                n: 10_(),
                x: -6.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 30232000224_f64,
                n: 10_(),
                x: -6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 24833423488.050575_f64,
                n: 10_(),
                x: -5.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 20302196869.444584_f64,
                n: 10_(),
                x: -5.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 16514443130.57936_f64,
                n: 10_(),
                x: -5.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 13361772808.732477_f64,
                n: 10_(),
                x: -5.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10749623191_f64,
                n: 10_(),
                x: -5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8595745900.089802_f64,
                n: 10_(),
                x: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6828832439.616082_f64,
                n: 10_(),
                x: -5.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5387267621.025891_f64,
                n: 10_(),
                x: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4218001347.152458_f64,
                n: 10_(),
                x: -5.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3275529760_f64,
                n: 10_(),
                x: -5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2520977273.0966735_f64,
                n: 10_(),
                x: -4.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1921271501.974427_f64,
                n: 10_(),
                x: -4.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1448403580.4203794_f64,
                n: 10_(),
                x: -4.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1078766805.4651456_f64,
                n: 10_(),
                x: -4.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 792566991_f64,
                n: 10_(),
                x: -4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 573298328.8199337_f64,
                n: 10_(),
                x: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 407278957.1437564_f64,
                n: 10_(),
                x: -4.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 283240820.637882_f64,
                n: 10_(),
                x: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 191968773.0386072_f64,
                n: 10_(),
                x: -4.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 125984224_f64,
                n: 10_(),
                x: -4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 79268966.16287735_f64,
                n: 10_(),
                x: -3.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 47025137.01603572_f64,
                n: 10_(),
                x: -3.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 25467573.27570921_f64,
                n: 10_(),
                x: -3.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 11645103.614666298_f64,
                n: 10_(),
                x: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3287599_f64,
                n: 10_(),
                x: -3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1324140.979837358_f64,
                n: 10_(),
                x: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3468342.2313268245_f64,
                n: 10_(),
                x: -3.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4074495.5241857003_f64,
                n: 10_(),
                x: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3800107.4878923763_f64,
                n: 10_(),
                x: -3.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3093984_f64,
                n: 10_(),
                x: -3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2247873.5653938125_f64,
                n: 10_(),
                x: -2.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1438117.197882981_f64,
                n: 10_(),
                x: -2.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -758781.9328103384_f64,
                n: 10_(),
                x: -2.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -247597.050124696_f64,
                n: 10_(),
                x: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 94135_f64,
                n: 10_(),
                x: -2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 286617.4739841031_f64,
                n: 10_(),
                x: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 360718.7974552577_f64,
                n: 10_(),
                x: -2.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 350419.8282674173_f64,
                n: 10_(),
                x: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 287863.09027338214_f64,
                n: 10_(),
                x: -2.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 200416_f64,
                n: 10_(),
                x: -2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 109249.22783242268_f64,
                n: 10_(),
                x: -1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 29012.09401589686_f64,
                n: 10_(),
                x: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -31740.330680422732_f64,
                n: 10_(),
                x: -1.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -69648.59783413768_f64,
                n: 10_(),
                x: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -85401_f64,
                n: 10_(),
                x: -1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -82507.67575285764_f64,
                n: 10_(),
                x: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -66123.41303306217_f64,
                n: 10_(),
                x: -1.299999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -42007.46514186222_f64,
                n: 10_(),
                x: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15676.055823257526_f64,
                n: 10_(),
                x: -1.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8224_f64,
                n: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 26314.366684262357_f64,
                n: 10_(),
                x: -0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 36668.34491637766_f64,
                n: 10_(),
                x: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 38802.82603509758_f64,
                n: 10_(),
                x: -0.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 33513.16789002236_f64,
                n: 10_(),
                x: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 22591_f64,
                n: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8467.690759782194_f64,
                n: 10_(),
                x: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6173.852487782552_f64,
                n: 10_(),
                x: -0.29999999999999893_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18778.85695754247_f64,
                n: 10_(),
                x: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -27256.158950297624_f64,
                n: 10_(),
                x: -0.09999999999999964_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -30240_f64,
                n: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -27256.158950297515_f64,
                n: 10_(),
                x: 0.10000000000000142_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18778.856957542288_f64,
                n: 10_(),
                x: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6173.8524877822965_f64,
                n: 10_(),
                x: 0.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8467.690759782456_f64,
                n: 10_(),
                x: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 22591_f64,
                n: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 33513.167890022516_f64,
                n: 10_(),
                x: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 38802.82603509762_f64,
                n: 10_(),
                x: 0.7000000000000011_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 36668.34491637756_f64,
                n: 10_(),
                x: 0.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 26314.366684262357_f64,
                n: 10_(),
                x: 0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8224_f64,
                n: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15676.05582325796_f64,
                n: 10_(),
                x: 1.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -42007.46514186269_f64,
                n: 10_(),
                x: 1.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -66123.41303306256_f64,
                n: 10_(),
                x: 1.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -82507.67575285764_f64,
                n: 10_(),
                x: 1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -85401_f64,
                n: 10_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -69648.59783413724_f64,
                n: 10_(),
                x: 1.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -31740.33068042186_f64,
                n: 10_(),
                x: 1.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 29012.094015898125_f64,
                n: 10_(),
                x: 1.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 109249.22783242268_f64,
                n: 10_(),
                x: 1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 200416_f64,
                n: 10_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 287863.09027338354_f64,
                n: 10_(),
                x: 2.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 350419.828267418_f64,
                n: 10_(),
                x: 2.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 360718.79745525745_f64,
                n: 10_(),
                x: 2.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 286617.4739841019_f64,
                n: 10_(),
                x: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 94135_f64,
                n: 10_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -247597.0501247038_f64,
                n: 10_(),
                x: 2.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -758781.9328103488_f64,
                n: 10_(),
                x: 2.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1438117.1978829878_f64,
                n: 10_(),
                x: 2.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2247873.565393821_f64,
                n: 10_(),
                x: 2.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3093984_f64,
                n: 10_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3800107.487892385_f64,
                n: 10_(),
                x: 3.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4074495.5241857045_f64,
                n: 10_(),
                x: 3.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3468342.2313268133_f64,
                n: 10_(),
                x: 3.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1324140.9798373245_f64,
                n: 10_(),
                x: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3287599_f64,
                n: 10_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 11645103.614666503_f64,
                n: 10_(),
                x: 3.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 25467573.27570951_f64,
                n: 10_(),
                x: 3.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 47025137.0160359_f64,
                n: 10_(),
                x: 3.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 79268966.16287765_f64,
                n: 10_(),
                x: 3.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 125984224_f64,
                n: 10_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 191968773.03860855_f64,
                n: 10_(),
                x: 4.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 283240820.6378839_f64,
                n: 10_(),
                x: 4.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 407278957.1437577_f64,
                n: 10_(),
                x: 4.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 573298328.8199356_f64,
                n: 10_(),
                x: 4.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 792566991_f64,
                n: 10_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1078766805.4651513_f64,
                n: 10_(),
                x: 4.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1448403580.4203873_f64,
                n: 10_(),
                x: 4.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1921271501.974432_f64,
                n: 10_(),
                x: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2520977273.0966806_f64,
                n: 10_(),
                x: 4.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3275529760_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4218001347.1524763_f64,
                n: 10_(),
                x: 5.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5387267621.025911_f64,
                n: 10_(),
                x: 5.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6828832439.616093_f64,
                n: 10_(),
                x: 5.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8595745900.089817_f64,
                n: 10_(),
                x: 5.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10749623191_f64,
                n: 10_(),
                x: 5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 13361772808.732529_f64,
                n: 10_(),
                x: 5.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 16514443130.579391_f64,
                n: 10_(),
                x: 5.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 20302196869.44462_f64,
                n: 10_(),
                x: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 24833423488.050617_f64,
                n: 10_(),
                x: 5.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 30232000224_f64,
                n: 10_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 36639112971.527824_f64,
                n: 10_(),
                x: 6.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 44215248881.62544_f64,
                n: 10_(),
                x: 6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 53142373179.80615_f64,
                n: 10_(),
                x: 6.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 63626303360.46833_f64,
                n: 10_(),
                x: 6.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 75899294599_f64,
                n: 10_(),
                x: 6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 90222850927.78821_f64,
                n: 10_(),
                x: 6.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 106890777450.54652_f64,
                n: 10_(),
                x: 6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 126232489621.23946_f64,
                n: 10_(),
                x: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 148616596389.67273_f64,
                n: 10_(),
                x: 6.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 174454774816_f64,
                n: 10_(),
                x: 7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 204205954581.2478_f64,
                n: 10_(),
                x: 7.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 238380831670.8996_f64,
                n: 10_(),
                x: 7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 277546731384.0183_f64,
                n: 10_(),
                x: 7.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 322332841721.5581_f64,
                n: 10_(),
                x: 7.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 373435839135_f64,
                n: 10_(),
                x: 7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 431625929570.4016_f64,
                n: 10_(),
                x: 7.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 497753328723.87476_f64,
                n: 10_(),
                x: 7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 572755206432.814_f64,
                n: 10_(),
                x: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 657663121163.0262_f64,
                n: 10_(),
                x: 7.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 753610971616_f64,
                n: 10_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 861843493572.9028_f64,
                n: 10_(),
                x: 8.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 983725331213.0719_f64,
                n: 10_(),
                x: 8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1120750713295.2778_f64,
                n: 10_(),
                x: 8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1274553765769.749_f64,
                n: 10_(),
                x: 8.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1446919493599_f64,
                n: 10_(),
                x: 8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1639795465805.4785_f64,
                n: 10_(),
                x: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1855304239034.7983_f64,
                n: 10_(),
                x: 8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2095756556225.3428_f64,
                n: 10_(),
                x: 8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2363665358307.854_f64,
                n: 10_(),
                x: 8.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2661760648224_f64,
                n: 10_(),
                x: 9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2993005247949.767_f64,
                n: 10_(),
                x: 9.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3360611490639.1025_f64,
                n: 10_(),
                x: 9.200000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3768058891466.047_f64,
                n: 10_(),
                x: 9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4219112842239.122_f64,
                n: 10_(),
                x: 9.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4717844376391_f64,
                n: 10_(),
                x: 9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5268651052510.479_f64,
                n: 10_(),
                x: 9.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5876279006180.66_f64,
                n: 10_(),
                x: 9.700000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6545846221520.677_f64,
                n: 10_(),
                x: 9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7282867075495.321_f64,
                n: 10_(),
                x: 9.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8093278209760_f64,
                n: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_19: std::cell::LazyCell<[testcase_hermite_double_; 201]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 3.653671097088803e+25_f64,
                n: 20_(),
                x: -10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.9174932703098834e+25_f64,
                n: 20_(),
                x: -9.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.3228769039548404e+25_f64,
                n: 20_(),
                x: -9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.8439144509496016e+25_f64,
                n: 20_(),
                x: -9.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.459197183454542e+25_f64,
                n: 20_(),
                x: -9.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1510764882450827e+25_f64,
                n: 20_(),
                x: -9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.05039412459916e+24_f64,
                n: 20_(),
                x: -9.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.091818891031215e+24_f64,
                n: 20_(),
                x: -9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.537653140520203e+24_f64,
                n: 20_(),
                x: -9.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.3084410724035914e+24_f64,
                n: 20_(),
                x: -9.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.3395360269524137e+24_f64,
                n: 20_(),
                x: -9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.5784910430831484e+24_f64,
                n: 20_(),
                x: -8.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.9828824261062853e+24_f64,
                n: 20_(),
                x: -8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.5184989558235974e+24_f64,
                n: 20_(),
                x: -8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1578391431515818e+24_f64,
                n: 20_(),
                x: -8.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.788674052502388e+23_f64,
                n: 20_(),
                x: -8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.639873380120607e+23_f64,
                n: 20_(),
                x: -8.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.991965653820119e+23_f64,
                n: 20_(),
                x: -8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.7339306542317994e+23_f64,
                n: 20_(),
                x: -8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.778075265395056e+23_f64,
                n: 20_(),
                x: -8.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.055402737399125e+23_f64,
                n: 20_(),
                x: -8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.5118471231121695e+23_f64,
                n: 20_(),
                x: -7.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.105221421838625e+23_f64,
                n: 20_(),
                x: -7.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.027526859471929e+22_f64,
                n: 20_(),
                x: -7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.790983293400904e+22_f64,
                n: 20_(),
                x: -7.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.1475563998692745e+22_f64,
                n: 20_(),
                x: -7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.947914972924925e+22_f64,
                n: 20_(),
                x: -7.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.0783192485964573e+22_f64,
                n: 20_(),
                x: -7.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4526171451238465e+22_f64,
                n: 20_(),
                x: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0059323685871305e+22_f64,
                n: 20_(),
                x: -7.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.897096560450233e+21_f64,
                n: 20_(),
                x: -7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.678441037952828e+21_f64,
                n: 20_(),
                x: -6.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.136726816041967e+21_f64,
                n: 20_(),
                x: -6.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.076483155841968e+21_f64,
                n: 20_(),
                x: -6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1355517474414813249536_f64,
                n: 20_(),
                x: -6.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 871249549700915789824_f64,
                n: 20_(),
                x: -6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 550332781337881083904_f64,
                n: 20_(),
                x: -6.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 340824121975337385984_f64,
                n: 20_(),
                x: -6.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 206318456487120863232_f64,
                n: 20_(),
                x: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 121587622122690281472_f64,
                n: 20_(),
                x: -6.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 69364200641629315072_f64,
                n: 20_(),
                x: -6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 37990093270095699968_f64,
                n: 20_(),
                x: -5.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19713403927925858304_f64,
                n: 20_(),
                x: -5.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9467379848853334016_f64,
                n: 20_(),
                x: -5.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4004640362878782464_f64,
                n: 20_(),
                x: -5.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1290768670572429312_f64,
                n: 20_(),
                x: -5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 85277679782819584_f64,
                n: 20_(),
                x: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -343595471790693760_f64,
                n: 20_(),
                x: -5.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -409708735015775616_f64,
                n: 20_(),
                x: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -334195856133482496_f64,
                n: 20_(),
                x: -5.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -225717767163827200_f64,
                n: 20_(),
                x: -5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -130511202035654928_f64,
                n: 20_(),
                x: -4.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -62555669197021992_f64,
                n: 20_(),
                x: -4.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -20974173561009776_f64,
                n: 20_(),
                x: -4.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 519073301695656_f64,
                n: 20_(),
                x: -4.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8939556693761121_f64,
                n: 20_(),
                x: -4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10070625675125174_f64,
                n: 20_(),
                x: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7940371996960715_f64,
                n: 20_(),
                x: -4.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4973623686173539_f64,
                n: 20_(),
                x: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2389023282480410_f64,
                n: 20_(),
                x: -4.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 619678016654336_f64,
                n: 20_(),
                x: -4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -339773491011950.5_f64,
                n: 20_(),
                x: -3.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -687467334428204.4_f64,
                n: 20_(),
                x: -3.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -663019129550482.2_f64,
                n: 20_(),
                x: -3.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -469585152350670.25_f64,
                n: 20_(),
                x: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -245659781875039_f64,
                n: 20_(),
                x: -3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -66042773886777.56_f64,
                n: 20_(),
                x: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 43442261337436.31_f64,
                n: 20_(),
                x: -3.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 87626632986465.44_f64,
                n: 20_(),
                x: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 85786202388511.38_f64,
                n: 20_(),
                x: -3.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 59990281399296_f64,
                n: 20_(),
                x: -3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 28343496696717.773_f64,
                n: 20_(),
                x: -2.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2296898915036.086_f64,
                n: 20_(),
                x: -2.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -13249381789941.586_f64,
                n: 20_(),
                x: -2.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18328180426561.06_f64,
                n: 20_(),
                x: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15799429886575_f64,
                n: 20_(),
                x: -2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9442592050214.242_f64,
                n: 20_(),
                x: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2602375356373.2393_f64,
                n: 20_(),
                x: -2.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2521759315047.877_f64,
                n: 20_(),
                x: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5027779307352.666_f64,
                n: 20_(),
                x: -2.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5080118660096_f64,
                n: 20_(),
                x: -2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3490463276000.5425_f64,
                n: 20_(),
                x: -1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1277254793997.0737_f64,
                n: 20_(),
                x: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -682119355279.2878_f64,
                n: 20_(),
                x: -1.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1851544254412.3203_f64,
                n: 20_(),
                x: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2085387081039_f64,
                n: 20_(),
                x: -1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1559402933581.5054_f64,
                n: 20_(),
                x: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -634265763221.8544_f64,
                n: 20_(),
                x: -1.299999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 295481874887.3343_f64,
                n: 20_(),
                x: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 924603483429.3424_f64,
                n: 20_(),
                x: -1.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1107214478336_f64,
                n: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 867235623835.1246_f64,
                n: 20_(),
                x: -0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 358848462745.1444_f64,
                n: 20_(),
                x: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -202944086511.71936_f64,
                n: 20_(),
                x: -0.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -617730863561.3262_f64,
                n: 20_(),
                x: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -759627879679_f64,
                n: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -607451728035.028_f64,
                n: 20_(),
                x: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -240424854484.41342_f64,
                n: 20_(),
                x: -0.29999999999999893_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 195759209122.61337_f64,
                n: 20_(),
                x: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 540334019322.52014_f64,
                n: 20_(),
                x: -0.09999999999999964_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 670442572800_f64,
                n: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 540334019322.51575_f64,
                n: 20_(),
                x: 0.10000000000000142_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 195759209122.60626_f64,
                n: 20_(),
                x: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -240424854484.4211_f64,
                n: 20_(),
                x: 0.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -607451728035.0327_f64,
                n: 20_(),
                x: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -759627879679_f64,
                n: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -617730863561.3212_f64,
                n: 20_(),
                x: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -202944086511.71027_f64,
                n: 20_(),
                x: 0.7000000000000011_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 358848462745.15466_f64,
                n: 20_(),
                x: 0.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 867235623835.1246_f64,
                n: 20_(),
                x: 0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1107214478336_f64,
                n: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 924603483429.3352_f64,
                n: 20_(),
                x: 1.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 295481874887.31995_f64,
                n: 20_(),
                x: 1.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -634265763221.8723_f64,
                n: 20_(),
                x: 1.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1559402933581.5054_f64,
                n: 20_(),
                x: 1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2085387081039_f64,
                n: 20_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1851544254412.3086_f64,
                n: 20_(),
                x: 1.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -682119355279.2595_f64,
                n: 20_(),
                x: 1.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1277254793997.1128_f64,
                n: 20_(),
                x: 1.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3490463276000.5425_f64,
                n: 20_(),
                x: 1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5080118660096_f64,
                n: 20_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5027779307352.644_f64,
                n: 20_(),
                x: 2.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2521759315047.807_f64,
                n: 20_(),
                x: 2.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2602375356373.297_f64,
                n: 20_(),
                x: 2.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9442592050214.303_f64,
                n: 20_(),
                x: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -15799429886575_f64,
                n: 20_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -18328180426561.04_f64,
                n: 20_(),
                x: 2.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -13249381789941.42_f64,
                n: 20_(),
                x: 2.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2296898915036.2812_f64,
                n: 20_(),
                x: 2.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 28343496696718.07_f64,
                n: 20_(),
                x: 2.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 59990281399296_f64,
                n: 20_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 85786202388511.64_f64,
                n: 20_(),
                x: 3.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 87626632986465.22_f64,
                n: 20_(),
                x: 3.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 43442261337435.67_f64,
                n: 20_(),
                x: 3.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -66042773886778.94_f64,
                n: 20_(),
                x: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -245659781875039_f64,
                n: 20_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -469585152350674.3_f64,
                n: 20_(),
                x: 3.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -663019129550484.6_f64,
                n: 20_(),
                x: 3.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -687467334428203.4_f64,
                n: 20_(),
                x: 3.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -339773491011946_f64,
                n: 20_(),
                x: 3.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 619678016654336_f64,
                n: 20_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2389023282480449_f64,
                n: 20_(),
                x: 4.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4973623686173592_f64,
                n: 20_(),
                x: 4.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7940371996960741_f64,
                n: 20_(),
                x: 4.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 10070625675125180_f64,
                n: 20_(),
                x: 4.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8939556693761121_f64,
                n: 20_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 519073301695404_f64,
                n: 20_(),
                x: 4.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -20974173561010304_f64,
                n: 20_(),
                x: 4.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -62555669197022528_f64,
                n: 20_(),
                x: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -130511202035655664_f64,
                n: 20_(),
                x: 4.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -225717767163827200_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -334195856133484160_f64,
                n: 20_(),
                x: 5.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -409708735015776128_f64,
                n: 20_(),
                x: 5.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -343595471790692160_f64,
                n: 20_(),
                x: 5.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 85277679782823936_f64,
                n: 20_(),
                x: 5.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1290768670572429312_f64,
                n: 20_(),
                x: 5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4004640362878845952_f64,
                n: 20_(),
                x: 5.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9467379848853415936_f64,
                n: 20_(),
                x: 5.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 19713403927925972992_f64,
                n: 20_(),
                x: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 37990093270095904768_f64,
                n: 20_(),
                x: 5.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 69364200641629315072_f64,
                n: 20_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 121587622122691559424_f64,
                n: 20_(),
                x: 6.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 206318456487120863232_f64,
                n: 20_(),
                x: 6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.40824121975339e+20_f64,
                n: 20_(),
                x: 6.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 550332781337886195712_f64,
                n: 20_(),
                x: 6.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 871249549700915789824_f64,
                n: 20_(),
                x: 6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1355517474414824259584_f64,
                n: 20_(),
                x: 6.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.076483155841968e+21_f64,
                n: 20_(),
                x: 6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3136726816041977511936_f64,
                n: 20_(),
                x: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4678441037952860553216_f64,
                n: 20_(),
                x: 6.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.897096560450233e+21_f64,
                n: 20_(),
                x: 7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0059323685871368e+22_f64,
                n: 20_(),
                x: 7.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4526171451238465e+22_f64,
                n: 20_(),
                x: 7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.0783192485964666e+22_f64,
                n: 20_(),
                x: 7.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.9479149729249434e+22_f64,
                n: 20_(),
                x: 7.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.1475563998692745e+22_f64,
                n: 20_(),
                x: 7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.790983293400938e+22_f64,
                n: 20_(),
                x: 7.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.027526859471929e+22_f64,
                n: 20_(),
                x: 7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1052214218386286e+23_f64,
                n: 20_(),
                x: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.511847123112176e+23_f64,
                n: 20_(),
                x: 7.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.055402737399125e+23_f64,
                n: 20_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.7780752653950703e+23_f64,
                n: 20_(),
                x: 8.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.7339306542317994e+23_f64,
                n: 20_(),
                x: 8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.991965653820119e+23_f64,
                n: 20_(),
                x: 8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.639873380120642e+23_f64,
                n: 20_(),
                x: 8.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.788674052502388e+23_f64,
                n: 20_(),
                x: 8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1578391431515869e+24_f64,
                n: 20_(),
                x: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.5184989558235974e+24_f64,
                n: 20_(),
                x: 8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.9828824261062853e+24_f64,
                n: 20_(),
                x: 8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.5784910430831597e+24_f64,
                n: 20_(),
                x: 8.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.3395360269524137e+24_f64,
                n: 20_(),
                x: 9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.3084410724036123e+24_f64,
                n: 20_(),
                x: 9.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.537653140520254e+24_f64,
                n: 20_(),
                x: 9.200000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.091818891031215e+24_f64,
                n: 20_(),
                x: 9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.050394124599195e+24_f64,
                n: 20_(),
                x: 9.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1510764882450827e+25_f64,
                n: 20_(),
                x: 9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.459197183454549e+25_f64,
                n: 20_(),
                x: 9.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.8439144509496166e+25_f64,
                n: 20_(),
                x: 9.700000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.3228769039548404e+25_f64,
                n: 20_(),
                x: 9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.9174932703098967e+25_f64,
                n: 20_(),
                x: 9.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.653671097088803e+25_f64,
                n: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_21: std::cell::LazyCell<[testcase_hermite_double_; 201]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 1.3516643049819314e+61_f64,
                n: 50_(),
                x: -10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.846670306226611e+60_f64,
                n: 50_(),
                x: -9.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.434481486374117e+60_f64,
                n: 50_(),
                x: -9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.673982806658579e+59_f64,
                n: 50_(),
                x: -9.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.619431296778013e+59_f64,
                n: 50_(),
                x: -9.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2454022521696361e+59_f64,
                n: 50_(),
                x: -9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.7546881918013145e+58_f64,
                n: 50_(),
                x: -9.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.650015847091727e+57_f64,
                n: 50_(),
                x: -9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.316320976952743e+56_f64,
                n: 50_(),
                x: -9.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9.786316537093047e+56_f64,
                n: 50_(),
                x: -9.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.056393090236891e+56_f64,
                n: 50_(),
                x: -9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.314593740632193e+56_f64,
                n: 50_(),
                x: -8.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.821077157764563e+56_f64,
                n: 50_(),
                x: -8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.008222530228956e+55_f64,
                n: 50_(),
                x: -8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.2392013720948442e+55_f64,
                n: 50_(),
                x: -8.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.5935887905806307e+54_f64,
                n: 50_(),
                x: -8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.432536504990938e+54_f64,
                n: 50_(),
                x: -8.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.23686412723009e+54_f64,
                n: 50_(),
                x: -8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0009712198862341e+54_f64,
                n: 50_(),
                x: -8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.0699480272357337e+53_f64,
                n: 50_(),
                x: -8.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.949279713268506e+52_f64,
                n: 50_(),
                x: -8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.998254410630006e+52_f64,
                n: 50_(),
                x: -7.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.567803133002378e+52_f64,
                n: 50_(),
                x: -7.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.807637174876232e+52_f64,
                n: 50_(),
                x: -7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.7887622198708925e+51_f64,
                n: 50_(),
                x: -7.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.580846769311333e+50_f64,
                n: 50_(),
                x: -7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0219511166806405e+51_f64,
                n: 50_(),
                x: -7.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.424126369454956e+50_f64,
                n: 50_(),
                x: -7.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.914389931509507e+50_f64,
                n: 50_(),
                x: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.317896624575665e+49_f64,
                n: 50_(),
                x: -7.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.471485598343656e+49_f64,
                n: 50_(),
                x: -7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.14282172720446e+49_f64,
                n: 50_(),
                x: -6.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.4864980414057334e+49_f64,
                n: 50_(),
                x: -6.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.068435547644688e+48_f64,
                n: 50_(),
                x: -6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.252923853017984e+47_f64,
                n: 50_(),
                x: -6.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.470966373984968e+48_f64,
                n: 50_(),
                x: -6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.7500305481204125e+48_f64,
                n: 50_(),
                x: -6.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.383455828514601e+47_f64,
                n: 50_(),
                x: -6.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.1477672402384868e+46_f64,
                n: 50_(),
                x: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.9304630401841983e+47_f64,
                n: 50_(),
                x: -6.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4355266959903589e+47_f64,
                n: 50_(),
                x: -6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.14822009055655e+46_f64,
                n: 50_(),
                x: -5.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.657735682746328e+45_f64,
                n: 50_(),
                x: -5.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.9676012349652066e+46_f64,
                n: 50_(),
                x: -5.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.3630554018675846e+46_f64,
                n: 50_(),
                x: -5.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.092049532809375e+45_f64,
                n: 50_(),
                x: -5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.368063888214557e+45_f64,
                n: 50_(),
                x: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.4465265559935436e+45_f64,
                n: 50_(),
                x: -5.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4270495629649337e+45_f64,
                n: 50_(),
                x: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.484551874333838e+44_f64,
                n: 50_(),
                x: -5.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.195392672127199e+44_f64,
                n: 50_(),
                x: -5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.4169399444879477e+44_f64,
                n: 50_(),
                x: -4.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4896819114978755e+44_f64,
                n: 50_(),
                x: -4.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.3078430866969463e+43_f64,
                n: 50_(),
                x: -4.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.844916863970072e+43_f64,
                n: 50_(),
                x: -4.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.918163970999746e+43_f64,
                n: 50_(),
                x: -4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.1434943490344838e+43_f64,
                n: 50_(),
                x: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1214652543461432e+43_f64,
                n: 50_(),
                x: -4.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.3843548994480566e+43_f64,
                n: 50_(),
                x: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.334979020530326e+42_f64,
                n: 50_(),
                x: -4.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9.55996406705539e+41_f64,
                n: 50_(),
                x: -4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.620255815828799e+42_f64,
                n: 50_(),
                x: -3.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.5206461734624493e+42_f64,
                n: 50_(),
                x: -3.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.7818051510730675e+41_f64,
                n: 50_(),
                x: -3.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.892137116533505e+41_f64,
                n: 50_(),
                x: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.205548576385277e+41_f64,
                n: 50_(),
                x: -3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.253576570728243e+41_f64,
                n: 50_(),
                x: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.9358941418244578e+41_f64,
                n: 50_(),
                x: -3.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.3076282847915616e+41_f64,
                n: 50_(),
                x: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.776497706663916e+41_f64,
                n: 50_(),
                x: -3.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.6751882008697154e+40_f64,
                n: 50_(),
                x: -3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.202538236999607e+41_f64,
                n: 50_(),
                x: -2.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.838303510355797e+40_f64,
                n: 50_(),
                x: -2.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.773360647963563e+39_f64,
                n: 50_(),
                x: -2.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.469681175871376e+40_f64,
                n: 50_(),
                x: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.371506248896345e+40_f64,
                n: 50_(),
                x: -2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.139008039057498e+40_f64,
                n: 50_(),
                x: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.6938519751181342e+40_f64,
                n: 50_(),
                x: -2.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.228450995295616e+40_f64,
                n: 50_(),
                x: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.096799428057053e+39_f64,
                n: 50_(),
                x: -2.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.412667799747298e+39_f64,
                n: 50_(),
                x: -2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.192683945403434e+40_f64,
                n: 50_(),
                x: -1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.54366542746976e+39_f64,
                n: 50_(),
                x: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.277949954255041e+39_f64,
                n: 50_(),
                x: -1.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.772053088969964e+39_f64,
                n: 50_(),
                x: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.68848511880343e+39_f64,
                n: 50_(),
                x: -1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.900500105255746e+38_f64,
                n: 50_(),
                x: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.1028320210430837e+39_f64,
                n: 50_(),
                x: -1.299999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.4780049977083965e+39_f64,
                n: 50_(),
                x: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.3484918476373692e+38_f64,
                n: 50_(),
                x: -1.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.6586815431645456e+39_f64,
                n: 50_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.722542947366143e+39_f64,
                n: 50_(),
                x: -0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.778544102494684e+38_f64,
                n: 50_(),
                x: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.841678437879016e+39_f64,
                n: 50_(),
                x: -0.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.2725918816693132e+39_f64,
                n: 50_(),
                x: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.7948375014926916e+38_f64,
                n: 50_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.3581645858906036e+39_f64,
                n: 50_(),
                x: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.0349661043040443e+39_f64,
                n: 50_(),
                x: -0.29999999999999893_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.504924881581704e+38_f64,
                n: 50_(),
                x: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.0564542132990048e+39_f64,
                n: 50_(),
                x: -0.09999999999999964_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.9607814681608194e+39_f64,
                n: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.0564542132989758e+39_f64,
                n: 50_(),
                x: 0.10000000000000142_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.504924881582024e+38_f64,
                n: 50_(),
                x: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.0349661043040497e+39_f64,
                n: 50_(),
                x: 0.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.358164585890575e+39_f64,
                n: 50_(),
                x: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.7948375014926916e+38_f64,
                n: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.2725918816693268e+39_f64,
                n: 50_(),
                x: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.8416784378789875e+39_f64,
                n: 50_(),
                x: 0.7000000000000011_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.778544102495173e+38_f64,
                n: 50_(),
                x: 0.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.722542947366143e+39_f64,
                n: 50_(),
                x: 0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.6586815431645456e+39_f64,
                n: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.348491847638007e+38_f64,
                n: 50_(),
                x: 1.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.4780049977084394e+39_f64,
                n: 50_(),
                x: 1.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.102832021043059e+39_f64,
                n: 50_(),
                x: 1.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.900500105255746e+38_f64,
                n: 50_(),
                x: 1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.68848511880343e+39_f64,
                n: 50_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.772053088969947e+39_f64,
                n: 50_(),
                x: 1.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.2779499542549072e+39_f64,
                n: 50_(),
                x: 1.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.543665427469911e+39_f64,
                n: 50_(),
                x: 1.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.192683945403434e+40_f64,
                n: 50_(),
                x: 1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.412667799747298e+39_f64,
                n: 50_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.096799428057363e+39_f64,
                n: 50_(),
                x: 2.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.2284509952956273e+40_f64,
                n: 50_(),
                x: 2.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.6938519751181172e+40_f64,
                n: 50_(),
                x: 2.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.1390080390575289e+40_f64,
                n: 50_(),
                x: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.371506248896345e+40_f64,
                n: 50_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.46968117587133e+40_f64,
                n: 50_(),
                x: 2.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.773360647964873e+39_f64,
                n: 50_(),
                x: 2.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.838303510355861e+40_f64,
                n: 50_(),
                x: 2.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2025382369996052e+41_f64,
                n: 50_(),
                x: 2.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.6751882008697154e+40_f64,
                n: 50_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.7764977066639554e+41_f64,
                n: 50_(),
                x: 3.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.3076282847915716e+41_f64,
                n: 50_(),
                x: 3.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.935894141824426e+41_f64,
                n: 50_(),
                x: 3.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.253576570728302e+41_f64,
                n: 50_(),
                x: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.205548576385277e+41_f64,
                n: 50_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.89213711653339e+41_f64,
                n: 50_(),
                x: 3.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.781805151073402e+41_f64,
                n: 50_(),
                x: 3.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.520646173462466e+42_f64,
                n: 50_(),
                x: 3.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.6202558158287927e+42_f64,
                n: 50_(),
                x: 3.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9.55996406705539e+41_f64,
                n: 50_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.334979020530477e+42_f64,
                n: 50_(),
                x: 4.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.3843548994480653e+43_f64,
                n: 50_(),
                x: 4.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.121465254346134e+43_f64,
                n: 50_(),
                x: 4.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.1434943490345182e+43_f64,
                n: 50_(),
                x: 4.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.918163970999746e+43_f64,
                n: 50_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.844916863970062e+43_f64,
                n: 50_(),
                x: 4.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.3078430866967542e+43_f64,
                n: 50_(),
                x: 4.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4896819114978953e+44_f64,
                n: 50_(),
                x: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.41693994448796e+44_f64,
                n: 50_(),
                x: 4.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.195392672127199e+44_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.4845518743340037e+44_f64,
                n: 50_(),
                x: 5.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4270495629649556e+45_f64,
                n: 50_(),
                x: 5.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.446526555993546e+45_f64,
                n: 50_(),
                x: 5.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.3680638882145392e+45_f64,
                n: 50_(),
                x: 5.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.092049532809375e+45_f64,
                n: 50_(),
                x: 5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.3630554018676023e+46_f64,
                n: 50_(),
                x: 5.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.9676012349652035e+46_f64,
                n: 50_(),
                x: 5.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.657735682746039e+45_f64,
                n: 50_(),
                x: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.148220090556615e+46_f64,
                n: 50_(),
                x: 5.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4355266959903589e+47_f64,
                n: 50_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.9304630401841966e+47_f64,
                n: 50_(),
                x: 6.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.1477672402384868e+46_f64,
                n: 50_(),
                x: 6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.383455828514698e+47_f64,
                n: 50_(),
                x: 6.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.7500305481204346e+48_f64,
                n: 50_(),
                x: 6.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.470966373984968e+48_f64,
                n: 50_(),
                x: 6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.252923853017179e+47_f64,
                n: 50_(),
                x: 6.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.068435547644688e+48_f64,
                n: 50_(),
                x: 6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.4864980414057495e+49_f64,
                n: 50_(),
                x: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.1428217272044745e+49_f64,
                n: 50_(),
                x: 6.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.471485598343656e+49_f64,
                n: 50_(),
                x: 7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.317896624576031e+49_f64,
                n: 50_(),
                x: 7.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.914389931509507e+50_f64,
                n: 50_(),
                x: 7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.424126369454993e+50_f64,
                n: 50_(),
                x: 7.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0219511166806373e+51_f64,
                n: 50_(),
                x: 7.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.580846769311333e+50_f64,
                n: 50_(),
                x: 7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.788762219871027e+51_f64,
                n: 50_(),
                x: 7.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.807637174876232e+52_f64,
                n: 50_(),
                x: 7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.567803133002397e+52_f64,
                n: 50_(),
                x: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.998254410629999e+52_f64,
                n: 50_(),
                x: 7.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.949279713268506e+52_f64,
                n: 50_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.0699480272358086e+53_f64,
                n: 50_(),
                x: 8.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0009712198862341e+54_f64,
                n: 50_(),
                x: 8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.23686412723009e+54_f64,
                n: 50_(),
                x: 8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.4325365049909476e+54_f64,
                n: 50_(),
                x: 8.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.5935887905806307e+54_f64,
                n: 50_(),
                x: 8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.2392013720948937e+55_f64,
                n: 50_(),
                x: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.008222530228956e+55_f64,
                n: 50_(),
                x: 8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.821077157764563e+56_f64,
                n: 50_(),
                x: 8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.314593740632248e+56_f64,
                n: 50_(),
                x: 8.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.056393090236891e+56_f64,
                n: 50_(),
                x: 9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -9.78631653709302e+56_f64,
                n: 50_(),
                x: 9.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.316320976954039e+56_f64,
                n: 50_(),
                x: 9.200000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.650015847091727e+57_f64,
                n: 50_(),
                x: 9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.7546881918013903e+58_f64,
                n: 50_(),
                x: 9.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2454022521696361e+59_f64,
                n: 50_(),
                x: 9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.6194312967780793e+59_f64,
                n: 50_(),
                x: 9.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.673982806658902e+59_f64,
                n: 50_(),
                x: 9.700000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.434481486374117e+60_f64,
                n: 50_(),
                x: 9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.8466703062266974e+60_f64,
                n: 50_(),
                x: 9.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.3516643049819314e+61_f64,
                n: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_23: std::cell::LazyCell<[testcase_hermite_double_; 201]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: -1.8738689223256612e+115_f64,
                n: 100_(),
                x: -10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.4232904120311186e+114_f64,
                n: 100_(),
                x: -9.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.376167665430519e+113_f64,
                n: 100_(),
                x: -9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.485743542757086e+113_f64,
                n: 100_(),
                x: -9.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.4904626130429828e+113_f64,
                n: 100_(),
                x: -9.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.226505176693479e+112_f64,
                n: 100_(),
                x: -9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.337422454114208e+112_f64,
                n: 100_(),
                x: -9.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.3977488190034162e+112_f64,
                n: 100_(),
                x: -9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.968603758339314e+111_f64,
                n: 100_(),
                x: -9.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.334133016654401e+111_f64,
                n: 100_(),
                x: -9.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.729413668304351e+110_f64,
                n: 100_(),
                x: -9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.6071876743328352e+110_f64,
                n: 100_(),
                x: -8.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.266977330029317e+110_f64,
                n: 100_(),
                x: -8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.8930624477368766e+109_f64,
                n: 100_(),
                x: -8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.5386102824684956e+109_f64,
                n: 100_(),
                x: -8.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.6203702280499544e+109_f64,
                n: 100_(),
                x: -8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2481735250539652e+108_f64,
                n: 100_(),
                x: -8.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.5611278359589723e+108_f64,
                n: 100_(),
                x: -8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.1534178973583771e+108_f64,
                n: 100_(),
                x: -8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.493259444695413e+106_f64,
                n: 100_(),
                x: -8.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.576150553544345e+107_f64,
                n: 100_(),
                x: -8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.344562292743773e+106_f64,
                n: 100_(),
                x: -7.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.7252309851017323e+106_f64,
                n: 100_(),
                x: -7.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.485043264812387e+106_f64,
                n: 100_(),
                x: -7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.748663881323885e+105_f64,
                n: 100_(),
                x: -7.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.477236447539496e+105_f64,
                n: 100_(),
                x: -7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.1375121759800508e+105_f64,
                n: 100_(),
                x: -7.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.9237691057265876e+104_f64,
                n: 100_(),
                x: -7.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.934880407487456e+104_f64,
                n: 100_(),
                x: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.2947583568692734e+104_f64,
                n: 100_(),
                x: -7.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0002273523275075e+104_f64,
                n: 100_(),
                x: -7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.4900499886057735e+103_f64,
                n: 100_(),
                x: -6.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.78177300191243e+102_f64,
                n: 100_(),
                x: -6.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.7849709684883083e+103_f64,
                n: 100_(),
                x: -6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.803904708558062e+102_f64,
                n: 100_(),
                x: -6.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.207247200204767e+102_f64,
                n: 100_(),
                x: -6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.3341289432999226e+102_f64,
                n: 100_(),
                x: -6.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4587350659146165e+101_f64,
                n: 100_(),
                x: -6.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -7.067213915070525e+101_f64,
                n: 100_(),
                x: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.8039133351601998e+101_f64,
                n: 100_(),
                x: -6.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.5170693933260738e+101_f64,
                n: 100_(),
                x: -6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.014140769096684e+101_f64,
                n: 100_(),
                x: -5.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.5140717605507886e+100_f64,
                n: 100_(),
                x: -5.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.637910259333053e+100_f64,
                n: 100_(),
                x: -5.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.293305767277913e+99_f64,
                n: 100_(),
                x: -5.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.93091864259704e+99_f64,
                n: 100_(),
                x: -5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.093567087905594e+99_f64,
                n: 100_(),
                x: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.8337906983006129e+99_f64,
                n: 100_(),
                x: -5.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.3096466887887237e+99_f64,
                n: 100_(),
                x: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.025923541660255e+97_f64,
                n: 100_(),
                x: -5.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.193193713059147e+98_f64,
                n: 100_(),
                x: -5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.5601172475776284e+98_f64,
                n: 100_(),
                x: -4.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.3193021196802698e+98_f64,
                n: 100_(),
                x: -4.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.654523801414265e+98_f64,
                n: 100_(),
                x: -4.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.244021894329917e+97_f64,
                n: 100_(),
                x: -4.6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.80738537561871e+97_f64,
                n: 100_(),
                x: -4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.213616391927072e+96_f64,
                n: 100_(),
                x: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.083750340495995e+97_f64,
                n: 100_(),
                x: -4.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.0988290456447506e+97_f64,
                n: 100_(),
                x: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0173847230741884e+97_f64,
                n: 100_(),
                x: -4.1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.707068274764355e+96_f64,
                n: 100_(),
                x: -4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.3947326745293985e+96_f64,
                n: 100_(),
                x: -3.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.1993887070961596e+96_f64,
                n: 100_(),
                x: -3.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.552176805050301e+94_f64,
                n: 100_(),
                x: -3.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.985272429624118e+96_f64,
                n: 100_(),
                x: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.784121344483892e+95_f64,
                n: 100_(),
                x: -3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.243311664494899e+95_f64,
                n: 100_(),
                x: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.2035951331058195e+95_f64,
                n: 100_(),
                x: -3.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.821201245456648e+95_f64,
                n: 100_(),
                x: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.5161870090563867e+95_f64,
                n: 100_(),
                x: -3.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.508454287119652e+94_f64,
                n: 100_(),
                x: -3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.078416074679753e+95_f64,
                n: 100_(),
                x: -2.8999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.6058116248679496e+94_f64,
                n: 100_(),
                x: -2.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1147132226581881e+95_f64,
                n: 100_(),
                x: -2.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.5607706742532875e+94_f64,
                n: 100_(),
                x: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.3758761713337664e+94_f64,
                n: 100_(),
                x: -2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.230322893257531e+94_f64,
                n: 100_(),
                x: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.16915015646855e+94_f64,
                n: 100_(),
                x: -2.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.2602488340116774e+94_f64,
                n: 100_(),
                x: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.052715503978761e+93_f64,
                n: 100_(),
                x: -2.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.2785574311661325e+94_f64,
                n: 100_(),
                x: -2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.8549527653152903e+93_f64,
                n: 100_(),
                x: -1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4787573463714434e+94_f64,
                n: 100_(),
                x: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.055407065477125e+93_f64,
                n: 100_(),
                x: -1.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.849682834697098e+93_f64,
                n: 100_(),
                x: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.840215189716951e+93_f64,
                n: 100_(),
                x: -1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.655546881992317e+93_f64,
                n: 100_(),
                x: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.462543712832261e+93_f64,
                n: 100_(),
                x: -1.299999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.7820042440391653e+93_f64,
                n: 100_(),
                x: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.581439334723589e+93_f64,
                n: 100_(),
                x: -1.0999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4487067293379347e+92_f64,
                n: 100_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.52683986789112e+93_f64,
                n: 100_(),
                x: -0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4120762149477404e+93_f64,
                n: 100_(),
                x: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.451076598114426e+93_f64,
                n: 100_(),
                x: -0.6999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.2242581581553176e+93_f64,
                n: 100_(),
                x: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.4129827902061037e+93_f64,
                n: 100_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.719542913975202e+93_f64,
                n: 100_(),
                x: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4235309630837636e+93_f64,
                n: 100_(),
                x: -0.29999999999999893_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.9850618739468043e+93_f64,
                n: 100_(),
                x: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.7017027479251074e+92_f64,
                n: 100_(),
                x: -0.09999999999999964_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.068518756254966e+93_f64,
                n: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.7017027479243444e+92_f64,
                n: 100_(),
                x: 0.10000000000000142_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.9850618739468293e+93_f64,
                n: 100_(),
                x: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4235309630836904e+93_f64,
                n: 100_(),
                x: 0.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.7195429139752497e+93_f64,
                n: 100_(),
                x: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.4129827902061037e+93_f64,
                n: 100_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.2242581581553936e+93_f64,
                n: 100_(),
                x: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.4510765981143827e+93_f64,
                n: 100_(),
                x: 0.7000000000000011_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4120762149478435e+93_f64,
                n: 100_(),
                x: 0.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.52683986789112e+93_f64,
                n: 100_(),
                x: 0.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4487067293379347e+92_f64,
                n: 100_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.5814393347236145e+93_f64,
                n: 100_(),
                x: 1.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.782004244039021e+93_f64,
                n: 100_(),
                x: 1.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.462543712832358e+93_f64,
                n: 100_(),
                x: 1.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.655546881992317e+93_f64,
                n: 100_(),
                x: 1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.840215189716951e+93_f64,
                n: 100_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.849682834696964e+93_f64,
                n: 100_(),
                x: 1.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.055407065477425e+93_f64,
                n: 100_(),
                x: 1.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.4787573463714363e+94_f64,
                n: 100_(),
                x: 1.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.8549527653152903e+93_f64,
                n: 100_(),
                x: 1.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.2785574311661325e+94_f64,
                n: 100_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.052715503978069e+93_f64,
                n: 100_(),
                x: 2.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.2602488340117204e+94_f64,
                n: 100_(),
                x: 2.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.1691501564685076e+94_f64,
                n: 100_(),
                x: 2.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.230322893257577e+94_f64,
                n: 100_(),
                x: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.3758761713337664e+94_f64,
                n: 100_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.560770674253508e+94_f64,
                n: 100_(),
                x: 2.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1147132226581835e+95_f64,
                n: 100_(),
                x: 2.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.6058116248681564e+94_f64,
                n: 100_(),
                x: 2.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.078416074679761e+95_f64,
                n: 100_(),
                x: 2.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.508454287119652e+94_f64,
                n: 100_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.5161870090564436e+95_f64,
                n: 100_(),
                x: 3.1000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.821201245456559e+95_f64,
                n: 100_(),
                x: 3.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.203595133105892e+95_f64,
                n: 100_(),
                x: 3.3000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -8.243311664494857e+95_f64,
                n: 100_(),
                x: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.784121344483892e+95_f64,
                n: 100_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.9852724296241191e+96_f64,
                n: 100_(),
                x: 3.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.552176805057605e+94_f64,
                n: 100_(),
                x: 3.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.1993887070961795e+96_f64,
                n: 100_(),
                x: 3.8000000000000007_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.394732674529333e+96_f64,
                n: 100_(),
                x: 3.9000000000000004_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.707068274764355e+96_f64,
                n: 100_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0173847230741732e+97_f64,
                n: 100_(),
                x: 4.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.0988290456448056e+97_f64,
                n: 100_(),
                x: 4.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.0837503404959957e+97_f64,
                n: 100_(),
                x: 4.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.213616391927759e+96_f64,
                n: 100_(),
                x: 4.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.80738537561871e+97_f64,
                n: 100_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.244021894329675e+97_f64,
                n: 100_(),
                x: 4.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.6545238014143033e+98_f64,
                n: 100_(),
                x: 4.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.319302119680255e+98_f64,
                n: 100_(),
                x: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.5601172475776894e+98_f64,
                n: 100_(),
                x: 4.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.193193713059147e+98_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.025923541663642e+97_f64,
                n: 100_(),
                x: 5.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.3096466887887553e+99_f64,
                n: 100_(),
                x: 5.200000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.8337906983005823e+99_f64,
                n: 100_(),
                x: 5.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.093567087905657e+99_f64,
                n: 100_(),
                x: 5.4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.93091864259704e+99_f64,
                n: 100_(),
                x: 5.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.293305767278417e+99_f64,
                n: 100_(),
                x: 5.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.637910259333072e+100_f64,
                n: 100_(),
                x: 5.700000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.5140717605507249e+100_f64,
                n: 100_(),
                x: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0141407690966954e+101_f64,
                n: 100_(),
                x: 5.9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.5170693933260738e+101_f64,
                n: 100_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.803913335160296e+101_f64,
                n: 100_(),
                x: 6.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -7.067213915070525e+101_f64,
                n: 100_(),
                x: 6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.458735065914455e+101_f64,
                n: 100_(),
                x: 6.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.334128943299976e+102_f64,
                n: 100_(),
                x: 6.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.207247200204767e+102_f64,
                n: 100_(),
                x: 6.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.803904708558293e+102_f64,
                n: 100_(),
                x: 6.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.7849709684883083e+103_f64,
                n: 100_(),
                x: 6.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.7817730019120996e+102_f64,
                n: 100_(),
                x: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.49004998860592e+103_f64,
                n: 100_(),
                x: 6.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0002273523275075e+104_f64,
                n: 100_(),
                x: 7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.2947583568693485e+104_f64,
                n: 100_(),
                x: 7.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.934880407487456e+104_f64,
                n: 100_(),
                x: 7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.923769105726468e+104_f64,
                n: 100_(),
                x: 7.300000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.137512175980108e+105_f64,
                n: 100_(),
                x: 7.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.477236447539496e+105_f64,
                n: 100_(),
                x: 7.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.7486638813241244e+105_f64,
                n: 100_(),
                x: 7.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.485043264812387e+106_f64,
                n: 100_(),
                x: 7.699999999999999_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.725230985101707e+106_f64,
                n: 100_(),
                x: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.344562292744032e+106_f64,
                n: 100_(),
                x: 7.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.576150553544345e+107_f64,
                n: 100_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.493259444694422e+106_f64,
                n: 100_(),
                x: 8.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.1534178973583771e+108_f64,
                n: 100_(),
                x: 8.2_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.5611278359589723e+108_f64,
                n: 100_(),
                x: 8.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2481735250541195e+108_f64,
                n: 100_(),
                x: 8.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.6203702280499544e+109_f64,
                n: 100_(),
                x: 8.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.5386102824684747e+109_f64,
                n: 100_(),
                x: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.8930624477368766e+109_f64,
                n: 100_(),
                x: 8.7_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.266977330029317e+110_f64,
                n: 100_(),
                x: 8.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.6071876743327728e+110_f64,
                n: 100_(),
                x: 8.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.729413668304351e+110_f64,
                n: 100_(),
                x: 9_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.334133016654443e+111_f64,
                n: 100_(),
                x: 9.100000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.968603758339104e+111_f64,
                n: 100_(),
                x: 9.200000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.3977488190034162e+112_f64,
                n: 100_(),
                x: 9.3_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -5.337422454114266e+112_f64,
                n: 100_(),
                x: 9.400000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.226505176693479e+112_f64,
                n: 100_(),
                x: 9.5_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.490462613043074e+113_f64,
                n: 100_(),
                x: 9.600000000000001_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.485743542757329e+113_f64,
                n: 100_(),
                x: 9.700000000000003_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.376167665430519e+113_f64,
                n: 100_(),
                x: 9.8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.423290412031299e+114_f64,
                n: 100_(),
                x: 9.900000000000002_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.8738689223256612e+115_f64,
                n: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_25: std::cell::LazyCell<[testcase_hermite_double_; 9]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 5324432_f64,
                n: 8_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 279702672_f64,
                n: 8_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3409634960_f64,
                n: 8_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 22149057680_f64,
                n: 8_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 99650305680_f64,
                n: 8_(),
                x: 12_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1040259450512_f64,
                n: 8_(),
                x: 16_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6326369025680_f64,
                n: 8_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9944083966401680_f64,
                n: 8_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2556417343865601536_f64,
                n: 8_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_26: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_27: std::cell::LazyCell<[testcase_hermite_double_; 9]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 54268892282368_f64,
                n: 18_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1350597603460566528_f64,
                n: 18_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1171930829027992535040_f64,
                n: 18_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1336099533194677e+23_f64,
                n: 18_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 3.9670688759298557e+24_f64,
                n: 18_(),
                x: 12_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.088730496796037e+26_f64,
                n: 18_(),
                x: 16_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.65259597588748e+28_f64,
                n: 18_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.697649796581238e+35_f64,
                n: 18_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.6014460548774723e+41_f64,
                n: 18_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_28: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_29: std::cell::LazyCell<[testcase_hermite_double_; 9]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 3.4488006171532706e+25_f64,
                n: 32_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 8.664682967536912e+29_f64,
                n: 32_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2445939522057109e+36_f64,
                n: 32_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.1381589300683108e+40_f64,
                n: 32_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.0979395756455302e+43_f64,
                n: 32_(),
                x: 12_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.202464606936125e+47_f64,
                n: 32_(),
                x: 16_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.675923373849773e+50_f64,
                n: 32_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.050082046753013e+63_f64,
                n: 32_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.189603191681971e+73_f64,
                n: 32_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_30: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data012_31: std::cell::LazyCell<[testcase_hermite_double_; 9]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: -9.55996406705539e+41_f64,
                n: 50_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -1.4355266959903589e+47_f64,
                n: 50_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.949279713268506e+52_f64,
                n: 50_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.3516643049819314e+61_f64,
                n: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.174655216978323e+66_f64,
                n: 50_(),
                x: 12_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.2608506104917666e+74_f64,
                n: 50_(),
                x: 16_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.4719364545684313e+79_f64,
                n: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 7.808162844440577e+99_f64,
                n: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.0588500951956526e+115_f64,
                n: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_32: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data013_33: std::cell::LazyCell<[testcase_hermite_double_; 9]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: -5.00163442490933e+65_f64,
                n: 72_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.1604343420664594e+70_f64,
                n: 72_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.130767357836452e+76_f64,
                n: 72_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.922745895091038e+83_f64,
                n: 72_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.276112165685176e+93_f64,
                n: 72_(),
                x: 12_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.405745239104926e+105_f64,
                n: 72_(),
                x: 16_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.553214433516885e+113_f64,
                n: 72_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 5.953644999483257e+143_f64,
                n: 72_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.153938420836477e+165_f64,
                n: 72_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_34: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_35: std::cell::LazyCell<[testcase_hermite_double_; 9]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: 7.484301596936389e+129_f64,
                n: 128_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.9427207779012687e+134_f64,
                n: 128_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -2.533207637423372e+140_f64,
                n: 128_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -3.6481920485874574e+146_f64,
                n: 128_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -6.601913130436128e+157_f64,
                n: 128_(),
                x: 12_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.7909969314519483e+182_f64,
                n: 128_(),
                x: 16_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 4.677328552959099e+199_f64,
                n: 128_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 1.885195379440732e+255_f64,
                n: 128_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.2605490449873036e+294_f64,
                n: 128_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_36: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_37: std::cell::LazyCell<[testcase_hermite_double_; 7]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_hermite_double_ {
                f0: -1.3224398396684072e+220_f64,
                n: 200_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.810855756159773e+224_f64,
                n: 200_(),
                x: 6_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.2034127668580903e+230_f64,
                n: 200_(),
                x: 8_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: -4.7005387186273384e+238_f64,
                n: 200_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 6.812567465019233e+247_f64,
                n: 200_(),
                x: 12_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 2.8932855639383936e+272_f64,
                n: 200_(),
                x: 16_f64,
                f: 0_f64,
            },
            testcase_hermite_double_ {
                f0: 9.875752654583018e+303_f64,
                n: 200_(),
                x: 20_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_38: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub unsafe fn test_39(data: *const [testcase_hermite_double_; 201], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 201_();
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
pub unsafe fn test_40(data: *const [testcase_hermite_double_; 9], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 9_();
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
pub unsafe fn test_41(data: *const [testcase_hermite_double_; 7], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 7_();
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
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_9)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_10)),
        )
    });
    (unsafe {
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_11)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_12)),
        )
    });
    (unsafe {
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_13)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_14)),
        )
    });
    (unsafe {
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_15)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_16)),
        )
    });
    (unsafe {
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_17)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_18)),
        )
    });
    (unsafe {
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_19)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_20)),
        )
    });
    (unsafe {
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_21)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_22)),
        )
    });
    (unsafe {
        test_39(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_23)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_24)),
        )
    });
    (unsafe {
        test_40(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_25)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_26)),
        )
    });
    (unsafe {
        test_40(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_27)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_28)),
        )
    });
    (unsafe {
        test_40(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_29)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_30)),
        )
    });
    (unsafe {
        test_40(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_31)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_32)),
        )
    });
    (unsafe {
        test_40(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_33)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_34)),
        )
    });
    (unsafe {
        test_40(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_35)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_36)),
        )
    });
    (unsafe {
        test_41(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_37)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_38)),
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
pub unsafe fn set_memory_limits_42(mut size: Option<()>) {
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
pub unsafe fn set_file_limit_43(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_44(mut mangled: *const (), mut wanted: *const ()) {
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
pub unsafe fn run_tests_wrapped_locale_45(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_46(
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
pub unsafe fn test_tm_47(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_48)).clone();
}
static mut tmp_48: std::cell::LazyCell<()> =
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
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
