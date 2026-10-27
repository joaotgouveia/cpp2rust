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
pub unsafe fn test_default_10() -> () {
    let mut e: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (10_()).into(),
                        (c"bool test_default()".as_ptr()).into(),
                        (c"e.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (0_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (11_()).into(),
                        (c"bool test_default()".as_ptr()).into(),
                        (c"*e == 0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (14_()).into(),
                        (c"bool test_default()".as_ptr()).into(),
                        (c"ev.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        {}
    }
    return true;
}
pub unsafe fn test_val_12() -> () {
    let mut e1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (24_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"e1.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (25_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"*e1 == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (28_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"e2.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (29_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"*e2 == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (38_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"e3.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*unimplemented!()).n) == (3_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (39_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"e3->n == 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (42_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"e4.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*unimplemented!()).n) == (4_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (43_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"e4->n == 4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (46_()).into(),
                        (c"bool test_val()".as_ptr()).into(),
                        (c"ev.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        {}
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X_13 {
    pub n: (),
}
impl X_13 {
    pub unsafe fn new(mut l: (), mut _a1: ()) -> Self {
        let mut this = Self {
            n: unimplemented!(),
        };
        this
    }
}
pub unsafe fn test_err_14() -> () {
    let mut e1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (56_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! e1.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (57_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"e1.error() == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let u2: () = unimplemented!();
    let mut e2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (61_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! e2.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (62_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"e2.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (65_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! e3.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (3_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (66_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"e3.error() == 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (76_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! e4.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*unimplemented!()).n) == (4_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (77_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"e4.error().n == 4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (80_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! e5.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*unimplemented!()).n) == (5_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (81_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"e5.error().n == 5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (84_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! ev1.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (85_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"ev1.error() == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (88_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! ev2.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (89_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"ev2.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! ev3.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (3_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"ev3.error() == 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (96_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! ev4.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*unimplemented!()).n) == (4_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (97_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"ev4.error().n == 4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (100_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"! ev5.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*unimplemented!()).n) == (5_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (101_()).into(),
                        (c"bool test_err()".as_ptr()).into(),
                        (c"ev5.error().n == 5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X_15 {
    pub n: (),
}
impl X_15 {
    pub unsafe fn new_1(mut i: (), mut j: ()) -> Self {
        let mut this = Self { n: ((i) + (j)) };
        this
    }
    pub unsafe fn new_2(mut l: (), mut _a1: ()) -> Self {
        let mut this = Self {
            n: unimplemented!(),
        };
        this
    }
}
pub unsafe fn test_copy_16() -> () {
    let mut e1: () = unimplemented!();
    let mut e2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (111_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"e2.value() == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (113_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"e2.value() == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (114_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"e3.value() == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (116_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"e4.value() == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut e5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (118_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"e4.value() == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (1_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (119_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"e5.value() == 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut u1: () = unimplemented!();
    let mut u2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (123_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"! u2.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (124_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"u2.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut u3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (126_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"! u3.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (127_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"u3.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut u4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (129_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"! u4.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (130_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"u4.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut u5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (132_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"! u5.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (133_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"u5.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev1: () = unimplemented!();
    let mut ev2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (137_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"ev2.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (139_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"ev2.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (140_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"ev3.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (142_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"ev4.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ev5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (144_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"ev4.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (145_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"ev5.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut uv1: () = unimplemented!();
    let mut uv2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (149_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"! uv2.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (150_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"uv2.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut uv3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (152_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"! uv3.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (153_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"uv3.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut uv4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (155_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"! uv4.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (156_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"uv4.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut uv5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (158_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"! uv5.has_value()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (2_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"cons.cc".as_ptr()).into(),
                        (159_()).into(),
                        (c"bool test_copy()".as_ptr()).into(),
                        (c"uv5.error() == 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    return true;
}
pub unsafe fn test_pr105153_17() -> () {
    let mut e: () = unimplemented!();
    const _: () = assert!(
        true,
        "! std::is_constructible_v<std::expected<void, int>,\n\t\t\t\t\t   std::expected<int, int>>"
    );;
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct E_18 {}
impl E_18 {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_default_10() });
    const _: () = assert!(true, "test_default()");;
    (unsafe { test_val_12() });
    const _: () = assert!(true, "test_val()");;
    (unsafe { test_err_14() });
    const _: () = assert!(true, "test_err()");;
    (unsafe { test_copy_16() });
    const _: () = assert!(true, "test_copy()");;
    (unsafe { test_pr105153_17() });
    const _: () = assert!(true, "test_pr105153()");;
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_19() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_20(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_21(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_22(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_23(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_24(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_23(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_24(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_23((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_24(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_25(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_23(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_24(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_26(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_27(
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
    if !(unimplemented!()) {
        (unsafe { __throw_runtime_error_3(s) });
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_28(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_29(
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
        if ((self.pid_) == (unsafe { getpid_19() })) {
            (unsafe { semctl_21(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_30(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_30(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
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
) -> ::libc::tm {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32));
}
static mut tmp_32: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
