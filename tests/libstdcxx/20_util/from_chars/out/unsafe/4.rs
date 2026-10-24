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
pub unsafe fn test01_10() {
    let mut s: () = unimplemented!();
    let mut d: () = unimplemented!();
    let mut res: () = unimplemented!();
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (*fmt);
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (45_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"std::isinf(d)".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ptr) == ((unimplemented!().offset((3_()) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (46_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ptr == s.data() + 3".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ec) == (unimplemented!())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (47_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ec == std::errc{}".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (51_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"std::isinf(d)".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((d) < (0_())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (52_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"d < 0".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ptr) == ((unimplemented!().offset((4_()) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (53_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ptr == s.data() + 4".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ec) == (unimplemented!())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (54_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ec == std::errc{}".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (58_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"std::isinf(d)".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ptr) == ((unimplemented!().offset((8_()) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (59_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ptr == s.data() + 8".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ec) == (unimplemented!())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (60_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ec == std::errc{}".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (64_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"std::isnan(d)".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ptr) == ((unimplemented!().offset((3_()) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (65_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ptr == s.data() + 3".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ec) == (unimplemented!())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (66_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ec == std::errc{}".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        unimplemented!();
        unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (70_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"std::isnan(d)".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ptr) == ((unimplemented!().offset((unimplemented!()) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (71_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ptr == s.data() + s.length()".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res.ec) == (unimplemented!())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"4.cc".as_ptr()).into(),
                            (72_()).into(),
                            (c"void test01()".as_ptr()).into(),
                            (c"res.ec == std::errc{}".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test02_12() {
    let mut s: () = unimplemented!();
    let mut d: () = 1_f64;
    let mut res: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (85_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"d == 0.0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((1_()) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (86_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ptr == s.data() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (87_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"d == 0.0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((1_()) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ptr == s.data() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (1_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (99_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"d == 1.0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!()).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (100_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ptr == s.data()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (101_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ec == std::errc::invalid_argument".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (106_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"d == 0.0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((1_()) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (107_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ptr == s.data() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (108_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (113_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"d == 0.0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((1_()) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (114_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ptr == s.data() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (115_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test03_13() {
    let mut s: () = unimplemented!();
    let mut d: () = 1_f64;
    let mut res: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (50_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (127_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"d == 0.5e+2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr)
            == ((unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset(-((1_()) as isize))
                .offset(-((3_()) as isize)))
            .cast_const()))
        {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (128_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ptr == s.data() + s.length() - 1 - 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (129_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0.5_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (133_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"d == 0.5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((3_()) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (134_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ptr == s.data() + 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (135_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (50_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (140_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"d == 0.5e+2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr)
            == ((unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset(-((1_()) as isize))
                .offset(-((3_()) as isize)))
            .cast_const()))
        {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (141_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ptr == s.data() + s.length() - 1 - 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (142_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (50_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (147_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"d == 0.5e+2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr)
            == ((unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset(-((1_()) as isize))
                .offset(-((3_()) as isize)))
            .cast_const()))
        {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (148_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ptr == s.data() + s.length() - 1 - 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (149_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    d = 1_f64;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0.3671875_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (154_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"d == 0x0.5Ep0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((4_()) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (155_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ptr == s.data() + 4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (156_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0.40625_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (161_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"d == 0.40625".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr)
            == ((unimplemented!()
                .offset((unimplemented!()) as isize)
                .offset(-((3_()) as isize)))
            .cast_const()))
        {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (162_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ptr == s.data() + s.length() - 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (163_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test04_14() {
    let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { 1000_() },
        { ('0' as ()) },
        None,
    );
    let mut d: () = 1_f64;
    let mut res: () = unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((unimplemented!()) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (174_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"res.ptr == s.data() + s.length()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (175_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (176_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"d == 0.0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((unimplemented!()) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (180_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"res.ptr == s.data() + s.length()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (181_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (0.5_f64)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (182_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"d == 0.5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut len: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (188_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"res.ptr == s.data() + len".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((res.ec) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (189_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"res.ec == std::errc{}".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((d) == (50_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"4.cc".as_ptr()).into(),
                        (190_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"d == 50".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn to_string_15(mut val: ()) -> () {
    let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { ((unimplemented!()) + (2_())) },
        { ('0' as ()) },
        None,
    );
    let mut iter: () = unimplemented!();
    'loop_: while ((val) != (0_())) {
        (*unimplemented!()) = ('0' as ()).wrapping_add(((val).wrapping_rem(10_())));
        val = (val).wrapping_div(10_());
    }
    return unimplemented!();
}
pub unsafe fn test05_16() {
    let mut res: () = unimplemented!();
    let mut flt: () = unimplemented!();
    let mut dbl: () = unimplemented!();
    let mut ldbl: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        let mut s: () = unimplemented!();
        let mut len: () = unimplemented!();
        unimplemented!();
        let mut s1: *const () = unimplemented!();
        let mut s1_end: *const () = s1.offset((len) as isize);
        'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut fmt: () = (*fmt);
            if ((fmt) == (unimplemented!())) && ((i) > (9_())) {
                continue 'loop_;
            }
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (233_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s1_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (234_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s1_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((flt) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (235_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"flt == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (238_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s1_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (239_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s1_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((dbl) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (240_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"dbl == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (243_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s1_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (244_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s1_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((ldbl) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (245_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"ldbl == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
        }
        if ((i) > (9_())) {
            i.prefix_inc();
            continue 'loop_;
        }
        let s2: [(); 7] = [
            ('.' as ()),
            (*s1),
            ('e' as ()),
            ('0' as ()),
            ('0' as ()),
            ('0' as ()),
            ('1' as ()),
        ];
        let mut s2_end: *const () = s2
            .as_ptr()
            .offset((::std::mem::size_of::<[(); 7]>()) as isize);
        let s3: [(); 7] = [
            (*s1),
            ('0' as ()),
            ('e' as ()),
            ('-' as ()),
            ('0' as ()),
            ('0' as ()),
            ('1' as ()),
        ];
        let mut s3_end: *const () = s3
            .as_ptr()
            .offset((::std::mem::size_of::<[(); 7]>()) as isize);
        'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut fmt: () = (*fmt);
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (263_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s2_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (264_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s2_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((flt) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (265_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"flt == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (268_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s3_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (269_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s3_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((flt) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (270_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"flt == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (273_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s2_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (274_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s2_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((dbl) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (275_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"dbl == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (278_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s3_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (279_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s3_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((dbl) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (280_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"dbl == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (283_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s2_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (284_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s2_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((ldbl) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (285_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"ldbl == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            unimplemented!();
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ec) == (unimplemented!())) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (288_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ec == std::errc{}".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((res.ptr) == (s3_end)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (289_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"res.ptr == s3_end".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((ldbl) == (i)) {
                    (unsafe {
                        __builtin_fprintf_11(
                            libcc2rs::stderr_unsafe(),
                            c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                            Box::new([
                                (c"4.cc".as_ptr()).into(),
                                (290_()).into(),
                                (c"void test05()".as_ptr()).into(),
                                (c"ldbl == i".as_ptr()).into(),
                            ]),
                        )
                    });
                    unimplemented!();
                }
            }
        }
        i.prefix_inc();
    }
}
pub unsafe fn test_max_mantissa_17() {
    {
        (unsafe {
            libc::printf(
                c"Testing %d-bit float, using %zu-bit integer\n".as_ptr() as *const libc::c_char,
                (((unimplemented!()) + (unimplemented!())) + (1_())),
                ((::std::mem::size_of::<()>() as ()).wrapping_mul(8_())),
            )
        });
        let mut res: () = unimplemented!();
        let mut flt: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            let val: () =
                ((((1_()) << (unimplemented!())) as ()).wrapping_sub(1_()) as ()).wrapping_sub(i);
            let mut s: () = unimplemented!();
            let mut len: () = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (*fmt);
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 324_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 325_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 326_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            unimplemented!();
            let orig_len: () = len;
            unimplemented!();
            len = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (*fmt);
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 337_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 338_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 339_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut s2: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 345_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 346_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 347_() ) .into() , ( c"void test_max_mantissa() [FloatT = float, UIntT = unsigned long]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_max_mantissa_18() {
    {
        (unsafe {
            libc::printf(
                c"Testing %d-bit float, using %zu-bit integer\n".as_ptr() as *const libc::c_char,
                (((unimplemented!()) + (unimplemented!())) + (1_())),
                ((::std::mem::size_of::<()>() as ()).wrapping_mul(8_())),
            )
        });
        let mut res: () = unimplemented!();
        let mut flt: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            let val: () =
                ((((1_()) << (unimplemented!())) as ()).wrapping_sub(1_()) as ()).wrapping_sub(i);
            let mut s: () = unimplemented!();
            let mut len: () = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (*fmt);
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 324_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 325_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 326_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            unimplemented!();
            let orig_len: () = len;
            unimplemented!();
            len = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (*fmt);
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 337_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 338_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 339_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut s2: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 345_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 346_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 347_() ) .into() , ( c"void test_max_mantissa() [FloatT = double, UIntT = unsigned long long]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_max_mantissa_19() {
    {
        (unsafe {
            libc::printf(
                c"Testing %d-bit float, using %zu-bit integer\n".as_ptr() as *const libc::c_char,
                (((unimplemented!()) + (unimplemented!())) + (1_())),
                ((::std::mem::size_of::<()>() as ()).wrapping_mul(8_())),
            )
        });
        let mut res: () = unimplemented!();
        let mut flt: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            let val: () =
                ((((1_()) << (unimplemented!())) as ()).wrapping_sub(1_()) as ()).wrapping_sub(i);
            let mut s: () = (unsafe { to_string_15(val) });
            let mut len: () = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (*fmt);
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 324_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 325_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 326_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            unimplemented!();
            let orig_len: () = len;
            unimplemented!();
            len = unimplemented!();
            unimplemented!();
            'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut fmt: () = (*fmt);
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 337_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 338_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 339_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut s2: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ec) == (unimplemented!())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 345_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"res.ec == std::errc{}" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((res.ptr) == ((unimplemented!().offset((len) as isize)).cast_const())) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 346_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"res.ptr == s.data() + len" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    if !((flt) == (val)) {
                        (unsafe {
                            __builtin_fprintf_11 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"4.cc" .as_ptr() ) .into() , ( 347_() ) .into() , ( c"void test_max_mantissa() [FloatT = long double, UIntT = unsigned __int128]" .as_ptr() ) .into() , ( c"flt == val" .as_ptr() ) .into() , ] ) )
                        });
                        unimplemented!();
                    }
                }
            }
            i.prefix_inc();
        }
    }
}
pub unsafe fn test06_20() {
    (unsafe { test_max_mantissa_17() });
    (unsafe { test_max_mantissa_18() });
    (unsafe { test_max_mantissa_19() });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_10() });
    (unsafe { test02_12() });
    (unsafe { test03_13() });
    (unsafe { test04_14() });
    (unsafe { test05_16() });
    (unsafe { test06_20() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_21() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_22(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_23(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_24(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_25(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_26(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_25(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_26(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_25((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_26(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_27(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_25(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_26(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_28(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_29(
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
pub unsafe fn run_tests_wrapped_locale_30(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_31(
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
        if ((self.pid_) == (unsafe { getpid_21() })) {
            (unsafe { semctl_23(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_32(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_32(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_33(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34));
}
static mut tmp_34: std::cell::LazyCell<::libc::tm> =
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
