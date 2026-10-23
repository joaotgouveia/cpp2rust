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
pub unsafe fn eq_10(mut d: (), mut e: ()) -> () {
    return ((unimplemented!()) < (0.01_f64));
}
pub unsafe fn test01_11() {
    let mut v: () = unimplemented!();
    (*unimplemented!()) = -0.5_f64;
    (*unimplemented!()) = 0.25_f64;
    let mut v_abs: () = {
        let mut __tmp_0: std__Expr_std___detail__UnClos_std__Abs__std__ValArray__double___double_ =
            unimplemented!();
        std_valarray_double_::new_1({ &mut __tmp_0 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (0.5_f64)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (35_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"v_abs[0] == 0.5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*unimplemented!()) == (0.25_f64)) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (36_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"v_abs[1] == 0.25".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_acos: () = {
        let mut __tmp_1: std__Expr_std___detail__UnClos_std__Acos__std__ValArray__double___double_ =
            unimplemented!();
        std_valarray_double_::new_2({ &mut __tmp_1 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 2.09_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (39_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_acos[0], 2.09 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 1.31_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (40_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_acos[1], 1.31 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_asin: () = {
        let mut __tmp_2: std__Expr_std___detail__UnClos_std__Asin__std__ValArray__double___double_ =
            unimplemented!();
        std_valarray_double_::new_3({ &mut __tmp_2 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), -0.52_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (43_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_asin[0], -0.52 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.25_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (44_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_asin[1], 0.25 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_atan: () = {
        let mut __tmp_3: std__Expr_std___detail__UnClos_std__Atan__std__ValArray__double___double_ =
            unimplemented!();
        std_valarray_double_::new_4({ &mut __tmp_3 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), -0.46_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (47_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_atan[0], -0.46 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.24_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (48_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_atan[1], 0.24 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v2: () = unimplemented!();
    (*unimplemented!()) = 4_();
    (*unimplemented!()) = 3_();
    let mut v_atan2: () = {
        let mut __tmp_4 : std__Expr_std___detail__BinClos_std__Atan2__std__ValArray__std__ValArray__double__double___double_  = unimplemented!()  ;
        std_valarray_double_::new_5({ &mut __tmp_4 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), -0.12_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (54_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_atan2[0], -0.12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.08_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (55_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_atan2[1], 0.08 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    v_atan2 = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), -0.12_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (58_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_atan2[0], -0.12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.06_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (59_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_atan2[1], 0.06 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    v_atan2 = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 1.69_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (62_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_atan2[0], 1.69 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 1.5_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (63_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_atan2[1], 1.50 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_cos: () = {
        let mut __tmp_5: std__Expr_std___detail__UnClos_std__Cos__std__ValArray__double___double_ =
            unimplemented!();
        std_valarray_double_::new_6({ &mut __tmp_5 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.87_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (66_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_cos[0], 0.87 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.96_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (67_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_cos[1], 0.96 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_cosh: () = {
        let mut __tmp_6: std__Expr_std___detail__UnClos_std__Cosh__std__ValArray__double___double_ =
            unimplemented!();
        std_valarray_double_::new_7({ &mut __tmp_6 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 1.12_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (70_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_cosh[0], 1.12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 1.03_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (71_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_cosh[1], 1.03 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_exp: () = {
        let mut __tmp_7: std__Expr_std___detail__UnClos_std__Exp__std__ValArray__double___double_ =
            unimplemented!();
        std_valarray_double_::new_8({ &mut __tmp_7 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.6_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (74_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_exp[0], 0.60 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 1.28_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (75_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_exp[1], 1.28 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_log: () = {
        let mut __tmp_8: std__Expr_std___detail__UnClos_std__Log__std__ValArray__double___double_ =
            unimplemented!();
        std_valarray_double_::new_9({ &mut __tmp_8 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), -1.38_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (78_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_log[1], -1.38 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_log10: () = {
        let mut __tmp_9 : std__Expr_std___detail__UnClos_std__Log10__std__ValArray__double___double_  = unimplemented!()  ;
        std_valarray_double_::new_10({ &mut __tmp_9 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), -0.6_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (81_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_log10[1], -0.60 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut v_pow: () = {
        let mut __tmp_10 : std__Expr_std___detail__BinClos_std__Pow__std__ValArray__std__ValArray__double__double___double_  = unimplemented!()  ;
        std_valarray_double_::new_11({ &mut __tmp_10 })
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.06_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (84_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_pow[0], 0.06 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.01_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (85_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_pow[1], 0.01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    v_pow = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), -0.12_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (88_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_pow[0], -0.12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.01_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (89_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_pow[1], 0.01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    v_pow = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 0.5_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_pow[0], 0.5 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { eq_10((*unimplemented!()), 1.41_f64) }) {
            (unsafe {
                __builtin_fprintf_12(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"transcend.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( v_pow[1], 1.41 )".as_ptr()).into(),
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
    (unsafe { test01_11() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_13() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_14(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_15(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_16(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_17(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_18(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_17(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_18(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_17((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_18(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_19(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_17(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_18(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_20(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_21(
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
pub unsafe fn run_tests_wrapped_locale_22(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_12 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_23(
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
        if ((self.pid_) == (unsafe { getpid_13() })) {
            (unsafe { semctl_15(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_24(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_24(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_25(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_26));
}
static mut tmp_26: std::cell::LazyCell<::libc::tm> =
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
