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
pub unsafe fn eq_10(x: *const (), y: *const ()) -> () {
    let mut nan_reals: () = (unimplemented!()) && (unimplemented!());
    let mut nan_imags: () = (unimplemented!()) && (unimplemented!());
    let mut sign_reals: () = ((unsafe { copysign_11(1_(), unimplemented!()) })
        == (unsafe { copysign_11(1_(), unimplemented!()) }));
    let mut sign_imags: () = ((unsafe { copysign_11(1_(), unimplemented!()) })
        == (unsafe { copysign_11(1_(), unimplemented!()) }));
    return ((((unimplemented!()) == (unimplemented!())) && (sign_reals)) || (nan_reals))
        && ((((unimplemented!()) == (unimplemented!())) && (sign_imags)) || (nan_imags));
}
pub unsafe fn eq_12(x: *const (), y: *const ()) -> () {
    let mut nan_reals: () = (unimplemented!()) && (unimplemented!());
    let mut nan_imags: () = (unimplemented!()) && (unimplemented!());
    let mut sign_reals: () = ((unimplemented!()) == (unimplemented!()));
    let mut sign_imags: () = ((unimplemented!()) == (unimplemented!()));
    return ((((unimplemented!()) == (unimplemented!())) && (sign_reals)) || (nan_reals))
        && ((((unimplemented!()) == (unimplemented!())) && (sign_imags)) || (nan_imags));
}
pub unsafe fn eq_13(x: *const (), y: *const ()) -> () {
    let mut nan_reals: () = (unimplemented!()) && (unimplemented!());
    let mut nan_imags: () = (unimplemented!()) && (unimplemented!());
    let mut sign_reals: () = ((unimplemented!()) == (unimplemented!()));
    let mut sign_imags: () = ((unimplemented!()) == (unimplemented!()));
    return ((((unimplemented!()) == (unimplemented!())) && (sign_reals)) || (nan_reals))
        && ((((unimplemented!()) == (unimplemented!())) && (sign_imags)) || (nan_imags));
}
pub unsafe fn test01_14() {
    let qnan: () = unimplemented!();
    let pinf: () = unimplemented!();
    let ninf: () = -pinf;
    let mut c00: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c00;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (63_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c00) , c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (64_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c00) , -c00 )".as_ptr()).into(),
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
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c00;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (66_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c00) , c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (67_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c00) , -c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c01: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c01;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (70_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c01) , c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (71_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c01) , -c01 )".as_ptr()).into(),
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
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c01;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (73_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c01) , c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (74_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c01) , -c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c10: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c10;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (77_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c10) , c10 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (78_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c10) , -c10 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c12: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c12;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (81_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c12) , c12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (82_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c12) , -c12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0q: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c0q;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (85_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c0q) , c0q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (86_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c0q) , -c0q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1q: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c1q;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (89_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c1q) , c1q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (90_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c1q) , -c1q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cq0;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cq0) , cq0 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cq0) , -cq0 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cq1;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (97_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cq1) , cq1 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (98_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cq1) , -cq1 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cqq;
            eq_10(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (101_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cqq) , cqq )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (102_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cqq) , -cqq )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0p: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (105_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c0p) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (106_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c0p) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1p: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (109_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c1p) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (110_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c1p) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (113_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cqp) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (114_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cqp) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (117_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cpp) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (118_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cpp) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0n: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (121_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c0n) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (122_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c0n) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1n: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (125_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(c1n) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (126_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-c1n) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (129_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cqn) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (130_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cqn) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (133_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cpn) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (134_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cpn) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (137_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cnn) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (138_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cnn) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (141_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cp0) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (142_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cp0) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (145_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cp1) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (146_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cp1) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (149_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cpq) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (150_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cpq) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (153_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cn0) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (154_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cn0) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (157_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cn1) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (158_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cn1) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (161_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cnq) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (162_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cnq) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (165_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(cnp) , std::complex<double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_10(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (166_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"eq( std::proj(-cnp) , std::complex<double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test02_16() {
    let qnan: () = unimplemented!();
    let pinf: () = unimplemented!();
    let ninf: () = -pinf;
    let mut c00: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c00;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (177_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c00) , c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (178_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c00) , -c00 )".as_ptr()).into(),
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
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c00;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (180_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c00) , c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (181_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c00) , -c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c01: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c01;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (184_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c01) , c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (185_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c01) , -c01 )".as_ptr()).into(),
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
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c01;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (187_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c01) , c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (188_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c01) , -c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c10: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c10;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (191_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c10) , c10 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (192_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c10) , -c10 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c12: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c12;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (195_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c12) , c12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (196_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c12) , -c12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0q: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c0q;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (199_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c0q) , c0q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (200_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c0q) , -c0q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1q: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c1q;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (203_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c1q) , c1q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (204_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c1q) , -c1q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cq0;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (207_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cq0) , cq0 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (208_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cq0) , -cq0 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cq1;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (211_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cq1) , cq1 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (212_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cq1) , -cq1 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cqq;
            eq_12(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (215_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cqq) , cqq )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (216_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cqq) , -cqq )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0p: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (219_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c0p) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (220_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c0p) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1p: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (223_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c1p) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (224_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c1p) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (227_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cqp) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (228_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cqp) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (231_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cpp) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (232_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cpp) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0n: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (235_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c0n) , std::complex<float>(pinf, -0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (236_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c0n) , std::complex<float>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1n: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (239_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(c1n) , std::complex<float>(pinf, -0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (240_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-c1n) , std::complex<float>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (243_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cqn) , std::complex<float>(pinf, -0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (244_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cqn) , std::complex<float>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (247_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cpn) , std::complex<float>(pinf, -0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (248_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cpn) , std::complex<float>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (251_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cnn) , std::complex<float>(pinf, -0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (252_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cnn) , std::complex<float>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (255_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cp0) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (256_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cp0) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (259_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cp1) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (260_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cp1) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (263_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cpq) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (264_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cpq) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (267_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cn0) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (268_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cn0) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (271_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cn1) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (272_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cn1) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (275_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cnq) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (276_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cnq) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (279_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(cnp) , std::complex<float>(pinf, +0.0) )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_12(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (280_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"eq( std::proj(-cnp) , std::complex<float>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test03_17() {
    let qnan: () = unimplemented!();
    let pinf: () = unimplemented!();
    let ninf: () = -pinf;
    let mut c00: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c00;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (291_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c00) , c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (292_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c00) , -c00 )".as_ptr()).into(),
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
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c00;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (294_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c00) , c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (295_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c00) , -c00 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c01: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c01;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (298_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c01) , c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (299_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c01) , -c01 )".as_ptr()).into(),
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
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c01;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (301_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c01) , c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (302_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c01) , -c01 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c10: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c10;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (305_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c10) , c10 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (306_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c10) , -c10 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c12: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c12;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (309_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c12) , c12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (310_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c12) , -c12 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0q: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c0q;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (313_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c0q) , c0q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (314_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c0q) , -c0q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1q: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &c1q;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (317_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c1q) , c1q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (318_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c1q) , -c1q )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cq0;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (321_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cq0) , cq0 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (322_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cq0) , -cq0 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cq1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cq1;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (325_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cq1) , cq1 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (326_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cq1) , -cq1 )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let _y: *const () = &cqq;
            eq_13(&mut _x, _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (329_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cqq) , cqq )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (330_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cqq) , -cqq )".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0p: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (333_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c0p) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (334_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c0p) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1p: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (337_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c1p) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (338_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c1p) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (341_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cqp) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (342_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cqp) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (345_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cpp) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (346_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cpp) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c0n: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (349_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c0n) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (350_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c0n) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let c1n: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (353_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(c1n) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (354_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-c1n) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cqn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (357_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cqn) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (358_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cqn) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (361_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cpn) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (362_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cpn) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnn: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (365_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cnn) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (366_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cnn) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (369_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cp0) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (370_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cp0) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cp1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (373_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cp1) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (374_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cp1) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cpq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (377_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cpq) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (378_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cpq) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (381_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cn0) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (382_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cn0) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cn1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (385_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cn1) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (386_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cn1) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnq: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (389_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cnq) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (390_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cnq) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let cnp: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (393_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(cnp) , std::complex<long double>(pinf, +0.0) )".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            let mut _x: () = unimplemented!();
            let mut _y: () = unimplemented!();
            eq_13(&mut _x, &mut _y)
        }) {
            (unsafe {
                __builtin_fprintf_15(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"proj.cc".as_ptr()).into(),
                        (394_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"eq( std::proj(-cnp) , std::complex<long double>(pinf, -0.0) )".as_ptr())
                            .into(),
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
    (unsafe { test01_14() });
    (unsafe { test02_16() });
    (unsafe { test03_17() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_18() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_19(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_20(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_21(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_22(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_23(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_22(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_23(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_22((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_23(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_24(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_22(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_23(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_25(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_26(
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
pub unsafe fn run_tests_wrapped_locale_27(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_28(
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
        if ((self.pid_) == (unsafe { getpid_18() })) {
            (unsafe { semctl_20(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_29(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_29(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_30(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_31));
}
static mut tmp_31: std::cell::LazyCell<::libc::tm> =
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
