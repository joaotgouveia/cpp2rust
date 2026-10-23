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
const _: () = assert!(
    true,
    "std::is_same<decltype(std::acosf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::asinf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::atanf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::atan2f(0.0, 0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::ceilf(0.0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::cosf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::coshf(0.0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::expf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::fabsf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::floorf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::fmodf(0.0, 0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::frexpf(0.0, nullptr)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::ldexpf(0.0, 0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::logf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::log10f(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::modff(0.0, nullptr)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::powf(0.0, 0.0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::sinf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sinhf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sqrtf(0.0)), float>::value"
);
const _: () = assert!(true, "std::is_same<decltype(std::tanf(0.0)), float>::value");
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tanhf(0.0)), float>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::acosl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::asinl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::atanl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::atan2l(0.0, 0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::ceill(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::cosl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::coshl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::expl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::fabsl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::floorl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::fmodl(0.0, 0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::frexpl(0.0, nullptr)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::ldexpl(0.0, 0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::logl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::log10l(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::modfl(0.0, nullptr)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::powl(0.0, 0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sinl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sinhl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::sqrtl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tanl(0.0)), long double>::value"
);
const _: () = assert!(
    true,
    "std::is_same<decltype(std::tanhl(0.0)), long double>::value"
);
pub unsafe fn test_float_global_10() {
    let mut x: () = 1_f32;
    let mut e1: () = 0_();
    let mut e2: () = 0_();
    let mut i1: () = 0_f32;
    let mut i2: () = 0_f32;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { acosf_11(x) }) == (unsafe { acosf_11(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (83_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::acosf(x) == std::acosf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { asinf_13(x) }) == (unsafe { asinf_13(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (84_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::asinf(x) == std::asinf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { atanf_14(x) }) == (unsafe { atanf_14(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (85_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::atanf(x) == std::atanf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe {
            let ___y: () = x;
            let ___x: () = x;
            atan2f_15(___y, ___x)
        }) == (unsafe {
            let ___y: () = x;
            let ___x: () = x;
            atan2f_15(___y, ___x)
        })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (86_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::atan2f(x, x) == std::atan2f(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { ceilf_16(x) }) == (unsafe { ceilf_16(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (87_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::ceilf(x) == std::ceilf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { cosf_17(x) }) == (unsafe { cosf_17(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (88_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::cosf(x) == std::cosf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { coshf_18(x) }) == (unsafe { coshf_18(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (89_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::coshf(x) == std::coshf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { expf_19(x) }) == (unsafe { expf_19(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (90_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::expf(x) == std::expf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { fabsf_20(x) }) == (unsafe { fabsf_20(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (91_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::fabsf(x) == std::fabsf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { floorf_21(x) }) == (unsafe { floorf_21(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::floorf(x) == std::floorf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe {
            let ___x: () = x;
            let ___y: () = x;
            fmodf_22(___x, ___y)
        }) == (unsafe {
            let ___x: () = x;
            let ___y: () = x;
            fmodf_22(___x, ___y)
        })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::fmodf(x, x) == std::fmodf(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { frexpf_23(x, (&mut e1 as *mut ())) })
            == (unsafe { frexpf_23(x, (&mut e2 as *mut ())) }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::frexpf(x, &e1) == std::frexpf(x, &e2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((e1) == (e2)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (95_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"e1 == e2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { ldexpf_24(x, 1_()) }) == (unsafe { ldexpf_24(x, 1_()) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (96_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::ldexpf(x, 1) == std::ldexpf(x, 1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { logf_25(x) }) == (unsafe { logf_25(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (97_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::logf(x) == std::logf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { log10f_26(x) }) == (unsafe { log10f_26(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (98_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::log10f(x) == std::log10f(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { modff_27(x, (&mut i1 as *mut ())) })
            == (unsafe { modff_27(x, (&mut i2 as *mut ())) }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (99_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::modff(x, &i1) == std::modff(x, &i2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((i1) == (i2)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (100_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"i1 == i2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe {
            let ___x: () = x;
            let ___y: () = x;
            powf_28(___x, ___y)
        }) == (unsafe {
            let ___x: () = x;
            let ___y: () = x;
            powf_28(___x, ___y)
        })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (101_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::powf(x, x) == std::powf(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { sinf_29(x) }) == (unsafe { sinf_29(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (102_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::sinf(x) == std::sinf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { sinhf_30(x) }) == (unsafe { sinhf_30(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (103_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::sinhf(x) == std::sinhf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { sqrtf_31(x) }) == (unsafe { sqrtf_31(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (104_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::sqrtf(x) == std::sqrtf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { tanf_32(x) }) == (unsafe { tanf_32(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (105_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::tanf(x) == std::tanf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { tanhf_33(x) }) == (unsafe { tanhf_33(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (106_()).into(),
                        (c"void test_float_global()".as_ptr()).into(),
                        (c"::tanhf(x) == std::tanhf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_float_overload_34() {
    let mut x: () = 1_f32;
    let mut e1: () = 0_();
    let mut e2: () = 0_();
    let mut i1: () = 0_f32;
    let mut i2: () = 0_f32;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { acosf_11(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (119_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::acos(x) == std::acosf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { asinf_13(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (120_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::asin(x) == std::asinf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { atanf_14(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (121_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::atan(x) == std::atanf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!())
            == (unsafe {
                let ___y: () = x;
                let ___x: () = x;
                atan2f_15(___y, ___x)
            }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (122_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::atan2(x, x) == std::atan2f(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { ceilf_16(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (123_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::ceil(x) == std::ceilf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { cosf_17(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (124_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::cos(x) == std::cosf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { coshf_18(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (125_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::cosh(x) == std::coshf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { expf_19(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (126_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::exp(x) == std::expf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { fabsf_20(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (127_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::fabs(x) == std::fabsf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { floorf_21(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (128_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::floor(x) == std::floorf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!())
            == (unsafe {
                let ___x: () = x;
                let ___y: () = x;
                fmodf_22(___x, ___y)
            }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (129_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::fmod(x, x) == std::fmodf(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { frexpf_23(x, (&mut e2 as *mut ())) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (130_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::frexp(x, &e1) == std::frexpf(x, &e2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((e1) == (e2)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (131_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"e1 == e2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { ldexpf_24(x, 1_()) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (132_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::ldexp(x, 1) == std::ldexpf(x, 1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { logf_25(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (133_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::log(x) == std::logf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { log10f_26(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (134_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::log10(x) == std::log10f(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { modff_27(x, (&mut i2 as *mut ())) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (135_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::modf(x, &i1) == std::modff(x, &i2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((i1) == (i2)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (136_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"i1 == i2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!())
            == (unsafe {
                let ___x: () = x;
                let ___y: () = x;
                powf_28(___x, ___y)
            }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (137_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::pow(x, x) == std::powf(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { sinf_29(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (138_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::sin(x) == std::sinf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { sinhf_30(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (139_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::sinh(x) == std::sinhf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { sqrtf_31(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (140_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::sqrt(x) == std::sqrtf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { tanf_32(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (141_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::tan(x) == std::tanf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { tanhf_33(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (142_()).into(),
                        (c"void test_float_overload()".as_ptr()).into(),
                        (c"std::tanh(x) == std::tanhf(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_long_double_global_35() {
    let mut x: () = 1_f64;
    let mut e1: () = 0_();
    let mut e2: () = 0_();
    let mut i1: () = 0_f64;
    let mut i2: () = 0_f64;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { acosl_36(x) }) == (unsafe { acosl_36(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (155_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::acosl(x) == std::acosl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { asinl_37(x) }) == (unsafe { asinl_37(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (156_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::asinl(x) == std::asinl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { atanl_38(x) }) == (unsafe { atanl_38(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (157_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::atanl(x) == std::atanl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe {
            let ___y: () = x;
            let ___x: () = x;
            atan2l_39(___y, ___x)
        }) == (unsafe {
            let ___y: () = x;
            let ___x: () = x;
            atan2l_39(___y, ___x)
        })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (158_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::atan2l(x, x) == std::atan2l(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { ceill_40(x) }) == (unsafe { ceill_40(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (159_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::ceill(x) == std::ceill(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { cosl_41(x) }) == (unsafe { cosl_41(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (160_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::cosl(x) == std::cosl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { coshl_42(x) }) == (unsafe { coshl_42(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (161_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::coshl(x) == std::coshl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { expl_43(x) }) == (unsafe { expl_43(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (162_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::expl(x) == std::expl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { fabsl_44(x) }) == (unsafe { fabsl_44(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (163_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::fabsl(x) == std::fabsl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { floorl_45(x) }) == (unsafe { floorl_45(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (164_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::floorl(x) == std::floorl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe {
            let ___x: () = x;
            let ___y: () = x;
            fmodl_46(___x, ___y)
        }) == (unsafe {
            let ___x: () = x;
            let ___y: () = x;
            fmodl_46(___x, ___y)
        })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (165_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::fmodl(x, x) == std::fmodl(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { frexpl_47(x, (&mut e1 as *mut ())) })
            == (unsafe { frexpl_47(x, (&mut e2 as *mut ())) }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (166_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::frexpl(x, &e1) == std::frexpl(x, &e2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((e1) == (e2)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (167_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"e1 == e2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { ldexpl_48(x, 1_()) }) == (unsafe { ldexpl_48(x, 1_()) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (168_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::ldexpl(x, 1) == std::ldexpl(x, 1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { logl_49(x) }) == (unsafe { logl_49(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (169_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::logl(x) == std::logl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { log10l_50(x) }) == (unsafe { log10l_50(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (170_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::log10l(x) == std::log10l(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { modfl_51(x, (&mut i1 as *mut ())) })
            == (unsafe { modfl_51(x, (&mut i2 as *mut ())) }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (171_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::modfl(x, &i1) == std::modfl(x, &i2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((i1) == (i2)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (172_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"i1 == i2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe {
            let ___x: () = x;
            let ___y: () = x;
            powl_52(___x, ___y)
        }) == (unsafe {
            let ___x: () = x;
            let ___y: () = x;
            powl_52(___x, ___y)
        })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (173_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::powl(x, x) == std::powl(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { sinl_53(x) }) == (unsafe { sinl_53(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (174_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::sinl(x) == std::sinl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { sinhl_54(x) }) == (unsafe { sinhl_54(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (175_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::sinhl(x) == std::sinhl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { sqrtl_55(x) }) == (unsafe { sqrtl_55(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (176_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::sqrtl(x) == std::sqrtl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { tanl_56(x) }) == (unsafe { tanl_56(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (177_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::tanl(x) == std::tanl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { tanhl_57(x) }) == (unsafe { tanhl_57(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (178_()).into(),
                        (c"void test_long_double_global()".as_ptr()).into(),
                        (c"::tanhl(x) == std::tanhl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_long_double_overload_58() {
    let mut x: () = 1_f64;
    let mut e1: () = 0_();
    let mut e2: () = 0_();
    let mut i1: () = 0_f64;
    let mut i2: () = 0_f64;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { acosl_36(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (191_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::acos(x) == std::acosl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { asinl_37(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (192_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::asin(x) == std::asinl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { atanl_38(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (193_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::atan(x) == std::atanl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!())
            == (unsafe {
                let ___y: () = x;
                let ___x: () = x;
                atan2l_39(___y, ___x)
            }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (194_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::atan2(x, x) == std::atan2l(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { ceill_40(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (195_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::ceil(x) == std::ceill(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { cosl_41(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (196_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::cos(x) == std::cosl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { coshl_42(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (197_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::cosh(x) == std::coshl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { expl_43(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (198_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::exp(x) == std::expl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { fabsl_44(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (199_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::fabs(x) == std::fabsl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { floorl_45(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (200_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::floor(x) == std::floorl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!())
            == (unsafe {
                let ___x: () = x;
                let ___y: () = x;
                fmodl_46(___x, ___y)
            }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (201_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::fmod(x, x) == std::fmodl(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { frexpl_47(x, (&mut e2 as *mut ())) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (202_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::frexp(x, &e1) == std::frexpl(x, &e2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((e1) == (e2)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (203_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"e1 == e2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { ldexpl_48(x, 1_()) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (204_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::ldexp(x, 1) == std::ldexpl(x, 1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { logl_49(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (205_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::log(x) == std::logl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { log10l_50(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (206_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::log10(x) == std::log10l(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { modfl_51(x, (&mut i2 as *mut ())) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (207_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::modf(x, &i1) == std::modfl(x, &i2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((i1) == (i2)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (208_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"i1 == i2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!())
            == (unsafe {
                let ___x: () = x;
                let ___y: () = x;
                powl_52(___x, ___y)
            }))
        {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (209_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::pow(x, x) == std::powl(x, x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { sinl_53(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (210_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::sin(x) == std::sinl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { sinhl_54(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (211_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::sinh(x) == std::sinhl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { sqrtl_55(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (212_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::sqrt(x) == std::sqrtl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { tanl_56(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (213_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::tan(x) == std::tanl(x)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (unsafe { tanhl_57(x) })) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"equivalent_functions.cc".as_ptr()).into(),
                        (214_()).into(),
                        (c"void test_long_double_overload()".as_ptr()).into(),
                        (c"std::tanh(x) == std::tanhl(x)".as_ptr()).into(),
                    ]),
                )
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
    (unsafe { test_float_global_10() });
    (unsafe { test_float_overload_34() });
    (unsafe { test_long_double_global_35() });
    (unsafe { test_long_double_overload_58() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_59() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_60(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_61(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_62(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_63(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_64(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_63(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_64(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_63((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_64(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_65(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_63(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_64(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_66(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_67(
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
pub unsafe fn run_tests_wrapped_locale_68(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_12 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_69(
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
        if ((self.pid_) == (unsafe { getpid_59() })) {
            (unsafe { semctl_61(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_70(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_70(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_71(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_72));
}
static mut tmp_72: std::cell::LazyCell<::libc::tm> =
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
