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
pub struct testcase_assoc_legendre_double_ {
    pub f0: (),
    pub l: (),
    pub m: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_9: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_11: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -1_f64,
                l: 1_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.9_f64,
                l: 1_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.8_f64,
                l: 1_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.7_f64,
                l: 1_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.6_f64,
                l: 1_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.5_f64,
                l: 1_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.3999999999999999_f64,
                l: 1_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.29999999999999993_f64,
                l: 1_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.19999999999999996_f64,
                l: 1_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.09999999999999998_f64,
                l: 1_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.10000000000000009_f64,
                l: 1_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.20000000000000018_f64,
                l: 1_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.30000000000000004_f64,
                l: 1_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.40000000000000013_f64,
                l: 1_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5_f64,
                l: 1_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.6000000000000001_f64,
                l: 1_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7000000000000002_f64,
                l: 1_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8_f64,
                l: 1_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9000000000000001_f64,
                l: 1_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 1_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_13: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.4358898943540673_f64,
                l: 1_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5999999999999999_f64,
                l: 1_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7141428428542851_f64,
                l: 1_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8_f64,
                l: 1_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8660254037844386_f64,
                l: 1_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.916515138991168_f64,
                l: 1_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9539392014169455_f64,
                l: 1_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9797958971132712_f64,
                l: 1_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.99498743710662_f64,
                l: 1_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 1_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.99498743710662_f64,
                l: 1_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9797958971132712_f64,
                l: 1_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9539392014169458_f64,
                l: 1_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9165151389911679_f64,
                l: 1_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8660254037844386_f64,
                l: 1_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7999999999999999_f64,
                l: 1_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7141428428542848_f64,
                l: 1_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5999999999999999_f64,
                l: 1_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.4358898943540671_f64,
                l: 1_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_15: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 2_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7150000000000001_f64,
                l: 2_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.4600000000000002_f64,
                l: 2_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.23499999999999988_f64,
                l: 2_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.039999999999999925_f64,
                l: 2_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.125_f64,
                l: 2_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2600000000000001_f64,
                l: 2_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.36500000000000005_f64,
                l: 2_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.44000000000000006_f64,
                l: 2_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.485_f64,
                l: 2_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.5_f64,
                l: 2_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.485_f64,
                l: 2_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.4399999999999999_f64,
                l: 2_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.365_f64,
                l: 2_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.25999999999999984_f64,
                l: 2_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.125_f64,
                l: 2_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.04000000000000015_f64,
                l: 2_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.23500000000000032_f64,
                l: 2_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.4600000000000002_f64,
                l: 2_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7150000000000003_f64,
                l: 2_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 2_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_17: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 2_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.1769027147559816_f64,
                l: 2_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.44_f64,
                l: 2_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.4996999699939983_f64,
                l: 2_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.44_f64,
                l: 2_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.299038105676658_f64,
                l: 2_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.0998181667894014_f64,
                l: 2_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.8585452812752508_f64,
                l: 2_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.5878775382679626_f64,
                l: 2_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2984962311319859_f64,
                l: 2_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.29849623113198626_f64,
                l: 2_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5878775382679633_f64,
                l: 2_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8585452812752513_f64,
                l: 2_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0998181667894018_f64,
                l: 2_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.299038105676658_f64,
                l: 2_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.4400000000000002_f64,
                l: 2_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.4996999699939983_f64,
                l: 2_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.44_f64,
                l: 2_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.1769027147559812_f64,
                l: 2_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_19: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5699999999999998_f64,
                l: 2_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0799999999999996_f64,
                l: 2_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.5300000000000005_f64,
                l: 2_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.9200000000000004_f64,
                l: 2_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.25_f64,
                l: 2_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.52_f64,
                l: 2_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.7299999999999995_f64,
                l: 2_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.88_f64,
                l: 2_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.97_f64,
                l: 2_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3_f64,
                l: 2_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.97_f64,
                l: 2_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.88_f64,
                l: 2_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.7300000000000004_f64,
                l: 2_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.519999999999999_f64,
                l: 2_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.25_f64,
                l: 2_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.9199999999999997_f64,
                l: 2_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.529999999999999_f64,
                l: 2_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0799999999999996_f64,
                l: 2_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5699999999999993_f64,
                l: 2_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_21: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -1_f64,
                l: 5_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.04114125000000009_f64,
                l: 5_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.39951999999999993_f64,
                l: 5_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.3651987499999999_f64,
                l: 5_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.15263999999999994_f64,
                l: 5_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.08984375_f64,
                l: 5_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2706400000000001_f64,
                l: 5_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.34538624999999995_f64,
                l: 5_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.30751999999999996_f64,
                l: 5_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.17882874999999995_f64,
                l: 5_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.17882875000000015_f64,
                l: 5_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.3075200000000001_f64,
                l: 5_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.34538625_f64,
                l: 5_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.2706399999999999_f64,
                l: 5_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.08984375_f64,
                l: 5_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.15264000000000016_f64,
                l: 5_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.36519875000000024_f64,
                l: 5_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.39951999999999993_f64,
                l: 5_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.04114124999999915_f64,
                l: 5_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 5_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_23: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 5_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.809936960835098_f64,
                l: 5_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7218000000000009_f64,
                l: 5_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.0951826834447254_f64,
                l: 5_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.9775999999999998_f64,
                l: 5_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.9282596881137892_f64,
                l: 5_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.2070504380513671_f64,
                l: 5_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.160798376638843_f64,
                l: 5_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8700587566365854_f64,
                l: 5_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.6083350053680323_f64,
                l: 5_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.875_f64,
                l: 5_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.6083350053680314_f64,
                l: 5_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8700587566365833_f64,
                l: 5_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.16079837663884422_f64,
                l: 5_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.2070504380513694_f64,
                l: 5_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.9282596881137892_f64,
                l: 5_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.9775999999999998_f64,
                l: 5_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.0951826834447216_f64,
                l: 5_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7218000000000009_f64,
                l: 5_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.8099369608351_f64,
                l: 5_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 5_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_25: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -12.837825_f64,
                l: 5_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -13.910400000000001_f64,
                l: 5_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.808974999999997_f64,
                l: 5_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.6128_f64,
                l: 5_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.921875_f64,
                l: 5_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.1728_f64,
                l: 5_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 10.462724999999997_f64,
                l: 5_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.870399999999998_f64,
                l: 5_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.041574999999999_f64,
                l: 5_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 5_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.041575000000004_f64,
                l: 5_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.870400000000005_f64,
                l: 5_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -10.462725_f64,
                l: 5_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9.172799999999997_f64,
                l: 5_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.921875_f64,
                l: 5_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.6128000000000047_f64,
                l: 5_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.80897500000001_f64,
                l: 5_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 13.910400000000001_f64,
                l: 5_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12.83782499999999_f64,
                l: 5_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_26: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_27: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 14.870165800941818_f64,
                l: 5_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 73.48319999999993_f64,
                l: 5_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 175.53238298794764_f64,
                l: 5_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 309.65760000000006_f64,
                l: 5_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 460.3466286991656_f64,
                l: 5_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 611.1249625581988_f64,
                l: 5_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 746.509414795237_f64,
                l: 5_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 853.3160043467132_f64,
                l: 5_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 921.5518918172473_f64,
                l: 5_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 945_f64,
                l: 5_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 921.5518918172473_f64,
                l: 5_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 853.3160043467132_f64,
                l: 5_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 746.5094147952376_f64,
                l: 5_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 611.1249625581984_f64,
                l: 5_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 460.3466286991656_f64,
                l: 5_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 309.6575999999999_f64,
                l: 5_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 175.53238298794724_f64,
                l: 5_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 73.48319999999993_f64,
                l: 5_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 14.87016580094179_f64,
                l: 5_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_28: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_29: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 10_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2631456178558596_f64,
                l: 10_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.30052979560000004_f64,
                l: 10_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.08580579553164033_f64,
                l: 10_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2436627456_f64,
                l: 10_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.18822860717773438_f64,
                l: 10_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.09683906440000026_f64,
                l: 10_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.2514763495160156_f64,
                l: 10_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.12907202559999983_f64,
                l: 10_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.12212499738710943_f64,
                l: 10_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.24609375_f64,
                l: 10_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.12212499738710922_f64,
                l: 10_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.12907202560000042_f64,
                l: 10_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.2514763495160156_f64,
                l: 10_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.09683906439999963_f64,
                l: 10_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.18822860717773438_f64,
                l: 10_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.24366274559999984_f64,
                l: 10_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.08580579553164128_f64,
                l: 10_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.30052979560000004_f64,
                l: 10_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2631456178558601_f64,
                l: 10_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 10_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_30: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data012_31: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.043874878147904_f64,
                l: 10_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8761426080000025_f64,
                l: 10_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.9685359952934527_f64,
                l: 10_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.2511825919999997_f64,
                l: 10_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.006687739436126_f64,
                l: 10_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.4822196173476647_f64,
                l: 10_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.12309508907433593_f64,
                l: 10_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.2468221751958413_f64,
                l: 10_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.2472659777983512_f64,
                l: 10_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.2472659777983535_f64,
                l: 10_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.2468221751958377_f64,
                l: 10_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.1230950890743391_f64,
                l: 10_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.482219617347667_f64,
                l: 10_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.006687739436126_f64,
                l: 10_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.2511825920000037_f64,
                l: 10_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.9685359952934505_f64,
                l: 10_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.8761426080000025_f64,
                l: 10_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.043874878147898_f64,
                l: 10_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 10_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_32: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data013_33: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 16.376387762496137_f64,
                l: 10_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -35.394657804000005_f64,
                l: 10_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.619142942378865_f64,
                l: 10_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 28.679675904000014_f64,
                l: 10_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 18.388023376464844_f64,
                l: 10_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -12.818955996000021_f64,
                l: 10_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -27.739821675972646_f64,
                l: 10_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -13.280661503999987_f64,
                l: 10_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 13.885467170308601_f64,
                l: 10_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 27.0703125_f64,
                l: 10_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 13.885467170308573_f64,
                l: 10_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -13.28066150400005_f64,
                l: 10_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -27.739821675972664_f64,
                l: 10_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -12.81895599599996_f64,
                l: 10_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 18.388023376464844_f64,
                l: 10_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 28.679675904_f64,
                l: 10_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.6191429423789856_f64,
                l: 10_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -35.394657804000005_f64,
                l: 10_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 16.3763877624962_f64,
                l: 10_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_34: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_35: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 10_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -21343.61851816468_f64,
                l: 10_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -40457.01640780798_f64,
                l: 10_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -20321.279317331315_f64,
                l: 10_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 14410.820616192004_f64,
                l: 10_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 30086.169706116176_f64,
                l: 10_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 17177.549337582834_f64,
                l: 10_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9272.511949541255_f64,
                l: 10_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -26591.511184414714_f64,
                l: 10_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -21961.95123850421_f64,
                l: 10_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 21961.95123850423_f64,
                l: 10_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 26591.511184414703_f64,
                l: 10_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9272.511949541236_f64,
                l: 10_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -17177.549337582877_f64,
                l: 10_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -30086.169706116176_f64,
                l: 10_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -14410.820616191972_f64,
                l: 10_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 20321.27931733139_f64,
                l: 10_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 40457.01640780798_f64,
                l: 10_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 21343.618518164636_f64,
                l: 10_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 10_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_36: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_37: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 162117.4007878422_f64,
                l: 10_(),
                m: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3958896.3481267113_f64,
                l: 10_(),
                m: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 22589806.343887307_f64,
                l: 10_(),
                m: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 70300999.12163332_f64,
                l: 10_(),
                m: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 155370278.540039_f64,
                l: 10_(),
                m: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 273815518.2015052_f64,
                l: 10_(),
                m: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 408571989.1315885_f64,
                l: 10_(),
                m: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 533848212.07990247_f64,
                l: 10_(),
                m: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 622640835.7052308_f64,
                l: 10_(),
                m: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 654729075_f64,
                l: 10_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 622640835.7052308_f64,
                l: 10_(),
                m: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 533848212.07990247_f64,
                l: 10_(),
                m: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 408571989.1315892_f64,
                l: 10_(),
                m: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 273815518.2015048_f64,
                l: 10_(),
                m: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 155370278.540039_f64,
                l: 10_(),
                m: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 70300999.12163322_f64,
                l: 10_(),
                m: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 22589806.343887202_f64,
                l: 10_(),
                m: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3958896.3481267113_f64,
                l: 10_(),
                m: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 162117.4007878416_f64,
                l: 10_(),
                m: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_38: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data016_39: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 20_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.14930823530984835_f64,
                l: 20_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.22420460541741347_f64,
                l: 20_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.20457394463834172_f64,
                l: 20_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.1591675291009811_f64,
                l: 20_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.04835838106737356_f64,
                l: 20_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.10159261558628112_f64,
                l: 20_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.18028715947998047_f64,
                l: 20_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0980421943445948_f64,
                l: 20_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.08207713094452766_f64,
                l: 20_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.17619705200195312_f64,
                l: 20_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.08207713094452802_f64,
                l: 20_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.09804219434459409_f64,
                l: 20_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.18028715947998042_f64,
                l: 20_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.10159261558628192_f64,
                l: 20_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.04835838106737356_f64,
                l: 20_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.15916752910098075_f64,
                l: 20_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.20457394463834136_f64,
                l: 20_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.22420460541741347_f64,
                l: 20_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.14930823530984758_f64,
                l: 20_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 20_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_40: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_41: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.38383348182205_f64,
                l: 20_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.6313829614634084_f64,
                l: 20_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.722748714133914_f64,
                l: 20_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.320352874382491_f64,
                l: 20_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.7399919228791405_f64,
                l: 20_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.1692202279270085_f64,
                l: 20_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.1580446883534503_f64,
                l: 20_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.036618239327117_f64,
                l: 20_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.211552381558021_f64,
                l: 20_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.211552381558017_f64,
                l: 20_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.036618239327126_f64,
                l: 20_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.15804468835344135_f64,
                l: 20_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.169220227926997_f64,
                l: 20_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.7399919228791405_f64,
                l: 20_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.3203528743824995_f64,
                l: 20_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7227487141339379_f64,
                l: 20_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.6313829614634084_f64,
                l: 20_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.383833481822069_f64,
                l: 20_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler017_42: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data018_43: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 80.8124255873101_f64,
                l: 20_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -95.84962217254937_f64,
                l: 20_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 87.33792763032551_f64,
                l: 20_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -70.33089153398583_f64,
                l: 20_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 24.62909073517949_f64,
                l: 20_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 39.90257633891225_f64,
                l: 20_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -75.62120147139655_f64,
                l: 20_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 42.417415829726494_f64,
                l: 20_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 33.82684867887129_f64,
                l: 20_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -74.00276184082031_f64,
                l: 20_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 33.826848678871464_f64,
                l: 20_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 42.41741582972619_f64,
                l: 20_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -75.6212014713966_f64,
                l: 20_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 39.90257633891255_f64,
                l: 20_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 24.62909073517949_f64,
                l: 20_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -70.33089153398572_f64,
                l: 20_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 87.33792763032545_f64,
                l: 20_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -95.84962217254937_f64,
                l: 20_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 80.81242558730975_f64,
                l: 20_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler018_44: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data019_45: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 315702.3271513422_f64,
                l: 20_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -503060.9148485259_f64,
                l: 20_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 298127.28360361955_f64,
                l: 20_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 114444.61447464029_f64,
                l: 20_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -543428.4091459285_f64,
                l: 20_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 613842.0772818548_f64,
                l: 20_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -143765.4241127106_f64,
                l: 20_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -472600.45321372285_f64,
                l: 20_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 563861.7677149649_f64,
                l: 20_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -563861.7677149646_f64,
                l: 20_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 472600.45321372483_f64,
                l: 20_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 143765.42411270936_f64,
                l: 20_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -613842.0772818535_f64,
                l: 20_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 543428.4091459285_f64,
                l: 20_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -114444.61447464178_f64,
                l: 20_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -298127.2836036152_f64,
                l: 20_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 503060.9148485259_f64,
                l: 20_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -315702.3271513362_f64,
                l: 20_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler019_46: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data020_47: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 990017476694.9908_f64,
                l: 20_(),
                m: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2392757933281.05_f64,
                l: 20_(),
                m: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1548364524949.5808_f64,
                l: 20_(),
                m: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -424471915195.0563_f64,
                l: 20_(),
                m: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1744502295946.2073_f64,
                l: 20_(),
                m: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -899973487310.5553_f64,
                l: 20_(),
                m: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1092420454297.7119_f64,
                l: 20_(),
                m: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1466609267659.8816_f64,
                l: 20_(),
                m: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 356041756390.71674_f64,
                l: 20_(),
                m: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1612052956674.3164_f64,
                l: 20_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 356041756390.71985_f64,
                l: 20_(),
                m: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1466609267659.8796_f64,
                l: 20_(),
                m: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1092420454297.7161_f64,
                l: 20_(),
                m: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -899973487310.5481_f64,
                l: 20_(),
                m: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1744502295946.2073_f64,
                l: 20_(),
                m: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -424471915195.05896_f64,
                l: 20_(),
                m: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1548364524949.573_f64,
                l: 20_(),
                m: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2392757933281.05_f64,
                l: 20_(),
                m: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 990017476694.9883_f64,
                l: 20_(),
                m: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler020_48: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data021_49: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 19609049712023808_f64,
                l: 20_(),
                m: 20_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 11693527616833220608_f64,
                l: 20_(),
                m: 20_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 380734558806206906368_f64,
                l: 20_(),
                m: 20_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3687400224900792713216_f64,
                l: 20_(),
                m: 20_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.8010806978179592e+22_f64,
                l: 20_(),
                m: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.593883258401247e+22_f64,
                l: 20_(),
                m: 20_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.2454734132297759e+23_f64,
                l: 20_(),
                m: 20_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.1263407800797497e+23_f64,
                l: 20_(),
                m: 20_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.8924941146976873e+23_f64,
                l: 20_(),
                m: 20_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.1983098677287775e+23_f64,
                l: 20_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.8924941146976873e+23_f64,
                l: 20_(),
                m: 20_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.1263407800797497e+23_f64,
                l: 20_(),
                m: 20_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.245473413229781e+23_f64,
                l: 20_(),
                m: 20_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.5938832584012366e+22_f64,
                l: 20_(),
                m: 20_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.8010806978179592e+22_f64,
                l: 20_(),
                m: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3687400224900780654592_f64,
                l: 20_(),
                m: 20_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 380734558806203432960_f64,
                l: 20_(),
                m: 20_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 11693527616833220608_f64,
                l: 20_(),
                m: 20_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 19609049712023672_f64,
                l: 20_(),
                m: 20_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler021_50: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data022_51: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 50_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.1700376599438367_f64,
                l: 50_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.13879737345093113_f64,
                l: 50_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.014572731645892852_f64,
                l: 50_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.058860798844002096_f64,
                l: 50_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.03105909923960981_f64,
                l: 50_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.041569033381824674_f64,
                l: 50_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.1091105157471479_f64,
                l: 50_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.0834322722041975_f64,
                l: 50_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0382058126613136_f64,
                l: 50_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.11227517265921705_f64,
                l: 50_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.038205812661314155_f64,
                l: 50_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.08343227220419656_f64,
                l: 50_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.10911051574714797_f64,
                l: 50_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.04156903338182601_f64,
                l: 50_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.03105909923960981_f64,
                l: 50_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.05886079884400143_f64,
                l: 50_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.014572731645890737_f64,
                l: 50_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.13879737345093113_f64,
                l: 50_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.1700376599438368_f64,
                l: 50_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 50_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler022_52: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data023_53: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 50_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.1342414998444949_f64,
                l: 50_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.201121967241302_f64,
                l: 50_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -6.6622414993232_f64,
                l: 50_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.577284693691925_f64,
                l: 50_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.878714881560761_f64,
                l: 50_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.547345945863408_f64,
                l: 50_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.8444956647620248_f64,
                l: 50_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.8722014306642127_f64,
                l: 50_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.348875132228563_f64,
                l: 50_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.348875132228552_f64,
                l: 50_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.872201430664262_f64,
                l: 50_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.844495664761993_f64,
                l: 50_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.547345945863381_f64,
                l: 50_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.878714881560761_f64,
                l: 50_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.577284693691945_f64,
                l: 50_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6.662241499323218_f64,
                l: 50_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.201121967241302_f64,
                l: 50_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.13424149984438935_f64,
                l: 50_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 50_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler023_54: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data024_55: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 433.0416848371351_f64,
                l: 50_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -348.06364372056424_f64,
                l: 50_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 50.221071418108444_f64,
                l: 50_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 158.46096409274352_f64,
                l: 50_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 85.98885829972146_f64,
                l: 50_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -101.15891460879088_f64,
                l: 50_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -277.07168105316526_f64,
                l: 50_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -214.333113735104_f64,
                l: 50_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 96.34965793095166_f64,
                l: 50_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 286.30169028100346_f64,
                l: 50_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 96.34965793095324_f64,
                l: 50_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -214.33311373510165_f64,
                l: 50_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -277.0716810531662_f64,
                l: 50_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -101.15891460879435_f64,
                l: 50_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 85.98885829972146_f64,
                l: 50_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 158.46096409274153_f64,
                l: 50_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 50.22107141810314_f64,
                l: 50_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -348.06364372056424_f64,
                l: 50_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 433.04168483713596_f64,
                l: 50_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler024_56: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data025_57: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 27340473.95213283_f64,
                l: 50_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -27753716.768532373_f64,
                l: 50_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -40808153.91349363_f64,
                l: 50_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -32071189.03579009_f64,
                l: 50_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -36265736.218529105_f64,
                l: 50_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -37089596.70020493_f64,
                l: 50_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -14562029.629244687_f64,
                l: 50_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 23686895.21751719_f64,
                l: 50_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 34878992.965676_f64,
                l: 50_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -34878992.96567596_f64,
                l: 50_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -23686895.21751753_f64,
                l: 50_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 14562029.629244499_f64,
                l: 50_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 37089596.70020478_f64,
                l: 50_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 36265736.218529105_f64,
                l: 50_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 32071189.035790242_f64,
                l: 50_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 40808153.913493834_f64,
                l: 50_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 27753716.768532373_f64,
                l: 50_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -27340473.952132136_f64,
                l: 50_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 50_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler025_58: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data026_59: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 50_(),
                m: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8994661710093155_f64,
                l: 50_(),
                m: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 932311375306569.6_f64,
                l: 50_(),
                m: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12153535011507012_f64,
                l: 50_(),
                m: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12176690755542240_f64,
                l: 50_(),
                m: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9180035388465754_f64,
                l: 50_(),
                m: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 889201701866984_f64,
                l: 50_(),
                m: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9451384032851544_f64,
                l: 50_(),
                m: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9926439446673564_f64,
                l: 50_(),
                m: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2794368162749970.5_f64,
                l: 50_(),
                m: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 11452238249246346_f64,
                l: 50_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2794368162750031_f64,
                l: 50_(),
                m: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9926439446673506_f64,
                l: 50_(),
                m: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9451384032851604_f64,
                l: 50_(),
                m: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 889201701866835.2_f64,
                l: 50_(),
                m: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9180035388465754_f64,
                l: 50_(),
                m: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12176690755542214_f64,
                l: 50_(),
                m: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12153535011506908_f64,
                l: 50_(),
                m: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 932311375306569.6_f64,
                l: 50_(),
                m: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8994661710093013_f64,
                l: 50_(),
                m: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler026_60: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data027_61: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.66309251586455e+33_f64,
                l: 50_(),
                m: 20_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0622676657892052e+33_f64,
                l: 50_(),
                m: 20_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.602252116471711e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.0860128756808466e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.016986081427446e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.2324325279774e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.0054067236247267e+31_f64,
                l: 50_(),
                m: 20_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.930926605643431e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.415135829089898e+31_f64,
                l: 50_(),
                m: 20_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.873593569733221e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.415135829089492e+31_f64,
                l: 50_(),
                m: 20_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.930926605643445e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.005406723624373e+31_f64,
                l: 50_(),
                m: 20_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.23243252797739e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.016986081427446e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.0860128756807846e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.602252116471629e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0622676657892052e+33_f64,
                l: 50_(),
                m: 20_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.663092515864554e+33_f64,
                l: 50_(),
                m: 20_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler027_62: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data028_63: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 50_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.536699497443134e+60_f64,
                l: 50_(),
                m: 50_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.2028834403101213e+67_f64,
                l: 50_(),
                m: 50_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.332549655956665e+71_f64,
                l: 50_(),
                m: 50_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.889809643178197e+73_f64,
                l: 50_(),
                m: 50_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.0509760257037188e+75_f64,
                l: 50_(),
                m: 50_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.4866724533443283e+76_f64,
                l: 50_(),
                m: 50_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.5790740224149893e+77_f64,
                l: 50_(),
                m: 50_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.822223793168099e+77_f64,
                l: 50_(),
                m: 50_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.1198682190366617e+78_f64,
                l: 50_(),
                m: 50_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.7253921397507295e+78_f64,
                l: 50_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.1198682190366617e+78_f64,
                l: 50_(),
                m: 50_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.822223793168099e+77_f64,
                l: 50_(),
                m: 50_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.5790740224150207e+77_f64,
                l: 50_(),
                m: 50_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.486672453344312e+76_f64,
                l: 50_(),
                m: 50_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.0509760257037188e+75_f64,
                l: 50_(),
                m: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.8898096431781724e+73_f64,
                l: 50_(),
                m: 50_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.3325496559566344e+71_f64,
                l: 50_(),
                m: 50_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.2028834403101213e+67_f64,
                l: 50_(),
                m: 50_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.5366994974430855e+60_f64,
                l: 50_(),
                m: 50_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler028_64: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data029_65: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 100_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.10226582055871893_f64,
                l: 100_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.05086116791358423_f64,
                l: 100_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.07713250719977864_f64,
                l: 100_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.02374702390513314_f64,
                l: 100_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0605180259618612_f64,
                l: 100_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.07225820212568503_f64,
                l: 100_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.057127392202801046_f64,
                l: 100_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.014681835355659706_f64,
                l: 100_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0638950984347502_f64,
                l: 100_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.07958923738717873_f64,
                l: 100_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.06389509843474976_f64,
                l: 100_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.014681835355657875_f64,
                l: 100_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.057127392202801566_f64,
                l: 100_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.07225820212568408_f64,
                l: 100_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0605180259618612_f64,
                l: 100_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.023747023905134217_f64,
                l: 100_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0771325071997805_f64,
                l: 100_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.05086116791358423_f64,
                l: 100_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.10226582055872063_f64,
                l: 100_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 100_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler029_66: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data030_67: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -6.5200167187780345_f64,
                l: 100_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9.006517000702749_f64,
                l: 100_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.469090854118098_f64,
                l: 100_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.627543917043079_f64,
                l: 100_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6.090903166344845_f64,
                l: 100_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.1160338699560395_f64,
                l: 100_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.849104301075863_f64,
                l: 100_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.943513872308983_f64,
                l: 100_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.7996285823989355_f64,
                l: 100_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.79962858239901_f64,
                l: 100_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.9435138723090155_f64,
                l: 100_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.849104301075801_f64,
                l: 100_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.116033869956216_f64,
                l: 100_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -6.090903166344845_f64,
                l: 100_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.627543917043047_f64,
                l: 100_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.469090854117869_f64,
                l: 100_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.006517000702749_f64,
                l: 100_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6.520016718777779_f64,
                l: 100_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler030_68: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data031_69: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1005.9604880761002_f64,
                l: 100_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -489.6804172586595_f64,
                l: 100_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 768.3167601166992_f64,
                l: 100_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 226.90362556627937_f64,
                l: 100_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 604.1988930494034_f64,
                l: 100_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 733.4006103783852_f64,
                l: 100_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -573.3077448399563_f64,
                l: 100_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -151.5294630508088_f64,
                l: 100_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 646.3052558358798_f64,
                l: 100_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -803.8512976105052_f64,
                l: 100_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 646.3052558358744_f64,
                l: 100_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -151.52946305079013_f64,
                l: 100_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -573.3077448399639_f64,
                l: 100_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 733.4006103783755_f64,
                l: 100_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 604.1988930494034_f64,
                l: 100_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 226.90362556629168_f64,
                l: 100_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 768.3167601167177_f64,
                l: 100_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -489.6804172586595_f64,
                l: 100_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1005.9604880761161_f64,
                l: 100_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler031_70: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data032_71: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -900551126.0965381_f64,
                l: 100_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -988567431.5575614_f64,
                l: 100_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 645646451.9034462_f64,
                l: 100_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 897114585.2992069_f64,
                l: 100_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 661710744.4248385_f64,
                l: 100_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -380163158.51424754_f64,
                l: 100_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -617391071.3663319_f64,
                l: 100_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 805288801.8550911_f64,
                l: 100_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -481041740.1672865_f64,
                l: 100_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 481041740.1672939_f64,
                l: 100_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -805288801.8550951_f64,
                l: 100_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 617391071.3663257_f64,
                l: 100_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 380163158.51426536_f64,
                l: 100_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -661710744.4248385_f64,
                l: 100_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -897114585.2992048_f64,
                l: 100_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -645646451.9034228_f64,
                l: 100_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 988567431.5575614_f64,
                l: 100_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 900551126.0965164_f64,
                l: 100_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler032_72: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data033_73: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 256433959576586016_f64,
                l: 100_(),
                m: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1577867354567348480_f64,
                l: 100_(),
                m: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4435504848749680128_f64,
                l: 100_(),
                m: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -959361116591242880_f64,
                l: 100_(),
                m: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4238712302196343808_f64,
                l: 100_(),
                m: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8237083461842676736_f64,
                l: 100_(),
                m: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4908935838805197824_f64,
                l: 100_(),
                m: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2346881035809127424_f64,
                l: 100_(),
                m: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6862785522503456768_f64,
                l: 100_(),
                m: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8249459718167037952_f64,
                l: 100_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6862785522503405568_f64,
                l: 100_(),
                m: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2346881035808951808_f64,
                l: 100_(),
                m: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4908935838805294080_f64,
                l: 100_(),
                m: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8237083461842611200_f64,
                l: 100_(),
                m: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4238712302196343808_f64,
                l: 100_(),
                m: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -959361116591126400_f64,
                l: 100_(),
                m: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4435504848749966848_f64,
                l: 100_(),
                m: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1577867354567348480_f64,
                l: 100_(),
                m: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 256433959576300576_f64,
                l: 100_(),
                m: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler033_74: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data034_75: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.160434487878013e+37_f64,
                l: 100_(),
                m: 20_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.396389511696223e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.902223685311014e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.268000557412101e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.0750497039999552e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.612058604384356e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.147449689190092e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.396625130744424e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.050326645170259e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.772743983615958e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.050326645170221e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.396625130744278e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.1474496891901797e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.612058604384418e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.0750497039999552e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.26800055741204e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.902223685310842e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.396389511696223e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.160434487875185e+37_f64,
                l: 100_(),
                m: 20_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler034_76: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data035_77: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 50_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.323127851689372e+96_f64,
                l: 100_(),
                m: 50_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.1029797977454281e+98_f64,
                l: 100_(),
                m: 50_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.8089333903465606e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.9364045925669405e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.225262033972712e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.143138509374073e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.3520602067479935e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.7791149588121382e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.011933855018042e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.1712145031578381e+98_f64,
                l: 100_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.01193385501812e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.7791149588123644e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.3520602067479344e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.143138509373882e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.225262033972712e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.936404592566802e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.8089333903469005e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.1029797977454281e+98_f64,
                l: 100_(),
                m: 50_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.323127851689294e+96_f64,
                l: 100_(),
                m: 50_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler035_78: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data036_79: std::cell::LazyCell<[testcase_assoc_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 100_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.7751792255758316e+150_f64,
                l: 100_(),
                m: 100_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.3552236041585515e+164_f64,
                l: 100_(),
                m: 100_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.5936546850595123e+172_f64,
                l: 100_(),
                m: 100_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.3579510590289176e+177_f64,
                l: 100_(),
                m: 100_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.7752749682889513e+180_f64,
                l: 100_(),
                m: 100_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0910627330458913e+183_f64,
                l: 100_(),
                m: 100_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.9697347526821064e+184_f64,
                l: 100_(),
                m: 100_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.658587914752671e+185_f64,
                l: 100_(),
                m: 100_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.033157190805701e+186_f64,
                l: 100_(),
                m: 100_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6.666308670072954e+186_f64,
                l: 100_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.033157190805701e+186_f64,
                l: 100_(),
                m: 100_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.658587914752671e+185_f64,
                l: 100_(),
                m: 100_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.969734752682248e+184_f64,
                l: 100_(),
                m: 100_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0910627330458797e+183_f64,
                l: 100_(),
                m: 100_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.7752749682889513e+180_f64,
                l: 100_(),
                m: 100_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.3579510590289e+177_f64,
                l: 100_(),
                m: 100_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.5936546850594382e+172_f64,
                l: 100_(),
                m: 100_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.3552236041585515e+164_f64,
                l: 100_(),
                m: 100_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.775179225575613e+150_f64,
                l: 100_(),
                m: 100_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 100_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler036_80: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub unsafe fn test_81(data: *const [testcase_assoc_legendre_double_; 21], mut toler: ()) {
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
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_9)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_10)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_11)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_12)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_13)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_14)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_15)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_16)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_17)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_18)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_19)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_20)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_21)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_22)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_23)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_24)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_25)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_26)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_27)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_28)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_29)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_30)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_31)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_32)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_33)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_34)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_35)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_36)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_37)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_38)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_39)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_40)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_41)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_42)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_43)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_44)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_45)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_46)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data020_47)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler020_48)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data021_49)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler021_50)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data022_51)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler022_52)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data023_53)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler023_54)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data024_55)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler024_56)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data025_57)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler025_58)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data026_59)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler026_60)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data027_61)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler027_62)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data028_63)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler028_64)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data029_65)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler029_66)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data030_67)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler030_68)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data031_69)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler031_70)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data032_71)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler032_72)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data033_73)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler033_74)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data034_75)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler034_76)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data035_77)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler035_78)),
        )
    });
    (unsafe {
        test_81(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data036_79)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler036_80)),
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
pub unsafe fn set_memory_limits_82(mut size: Option<()>) {
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
pub unsafe fn set_file_limit_83(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_84(mut mangled: *const (), mut wanted: *const ()) {
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
pub unsafe fn run_tests_wrapped_locale_85(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_86(
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
pub unsafe fn test_tm_87(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_88)).clone();
}
static mut tmp_88: std::cell::LazyCell<()> =
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
    std::cell::LazyCell::force(&*&raw const data020_47);
    std::cell::LazyCell::force(&*&raw const toler020_48);
    std::cell::LazyCell::force(&*&raw const data021_49);
    std::cell::LazyCell::force(&*&raw const toler021_50);
    std::cell::LazyCell::force(&*&raw const data022_51);
    std::cell::LazyCell::force(&*&raw const toler022_52);
    std::cell::LazyCell::force(&*&raw const data023_53);
    std::cell::LazyCell::force(&*&raw const toler023_54);
    std::cell::LazyCell::force(&*&raw const data024_55);
    std::cell::LazyCell::force(&*&raw const toler024_56);
    std::cell::LazyCell::force(&*&raw const data025_57);
    std::cell::LazyCell::force(&*&raw const toler025_58);
    std::cell::LazyCell::force(&*&raw const data026_59);
    std::cell::LazyCell::force(&*&raw const toler026_60);
    std::cell::LazyCell::force(&*&raw const data027_61);
    std::cell::LazyCell::force(&*&raw const toler027_62);
    std::cell::LazyCell::force(&*&raw const data028_63);
    std::cell::LazyCell::force(&*&raw const toler028_64);
    std::cell::LazyCell::force(&*&raw const data029_65);
    std::cell::LazyCell::force(&*&raw const toler029_66);
    std::cell::LazyCell::force(&*&raw const data030_67);
    std::cell::LazyCell::force(&*&raw const toler030_68);
    std::cell::LazyCell::force(&*&raw const data031_69);
    std::cell::LazyCell::force(&*&raw const toler031_70);
    std::cell::LazyCell::force(&*&raw const data032_71);
    std::cell::LazyCell::force(&*&raw const toler032_72);
    std::cell::LazyCell::force(&*&raw const data033_73);
    std::cell::LazyCell::force(&*&raw const toler033_74);
    std::cell::LazyCell::force(&*&raw const data034_75);
    std::cell::LazyCell::force(&*&raw const toler034_76);
    std::cell::LazyCell::force(&*&raw const data035_77);
    std::cell::LazyCell::force(&*&raw const toler035_78);
    std::cell::LazyCell::force(&*&raw const data036_79);
    std::cell::LazyCell::force(&*&raw const toler036_80);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
