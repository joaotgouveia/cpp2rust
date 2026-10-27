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
    let mut nested_element: () = unimplemented!();
    let mut element: () = unimplemented!();
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
                        (c"77288.cc".as_ptr()).into(),
                        (32_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"nested_element".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct service_result_int_ {}
impl service_result_int_ {
    pub unsafe fn get_result() -> () {
        let mut sr: () = unimplemented!();
        return sr;
    }
    pub unsafe fn get_result_with_cond(mut cond: ()) -> () {
        if cond {
            return unimplemented!();
        }
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct service_result_std_optional_int__ {}
impl service_result_std_optional_int__ {
    pub unsafe fn get_result() -> () {
        let mut sr: () = unimplemented!();
        return sr;
    }
    pub unsafe fn get_result_with_cond(mut cond: ()) -> () {
        if cond {
            return unimplemented!();
        }
        return unimplemented!();
    }
}
pub unsafe fn test02_12() {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (54_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"service_result<int>::get_result()".as_ptr()).into(),
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
                        (c"77288.cc".as_ptr()).into(),
                        (55_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"service_result<optional<int>>::get_result()".as_ptr()).into(),
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
                        (c"77288.cc".as_ptr()).into(),
                        (56_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"service_result<int>::get_result_with_cond(true)".as_ptr()).into(),
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
                        (c"77288.cc".as_ptr()).into(),
                        (57_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"service_result<optional<int>>::get_result_with_cond(true)".as_ptr())
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
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (58_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"!service_result<int>::get_result_with_cond(false)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (59_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"!service_result<optional<int>>::get_result_with_cond(false)".as_ptr())
                            .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Widget {}
impl Widget {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test03_13() {
    let mut w: () = unimplemented!();
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
                        (c"77288.cc".as_ptr()).into(),
                        (73_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"w".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Widget>&,\n\t\toptional<short>>"
    );;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (77_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"!w".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Widget>&,\n\t\toptional<optional<short>>>"
    );;
    let mut w2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (82_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"w2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (84_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"w3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (86_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"w4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<short>&&, optional<Widget>>"
    );;
    let mut w6: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (90_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"!w6".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w7: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"!w7".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w8: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"!w8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<optional<short>>&&,\n\t\toptional<Widget>>"
    );;
    let mut w10: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (98_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"w10".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w11: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (100_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"!w11".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w12: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (102_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"!w12".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w13: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (104_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"!w13".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w14: () = unimplemented!();
    w14 = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (107_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"!w14".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Widget2 {}
impl Widget2 {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn operator_assign_3(&mut self, mut _a0: ()) -> *mut Widget2 {
        return &mut (*(self as *mut Widget2));
    }
    pub unsafe fn operator_assign_4(&mut self, mut _a0: ()) -> *mut Widget2 {
        return &mut (*(self as *mut Widget2));
    }
}
pub unsafe fn test04_14() {
    let mut w: () = unimplemented!();
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
                        (c"77288.cc".as_ptr()).into(),
                        (122_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"w".as_ptr()).into(),
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
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (124_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"w".as_ptr()).into(),
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
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (126_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"!w".as_ptr()).into(),
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
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (128_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"!w".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    let mut w2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (131_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"!w2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (133_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"!w3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (135_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"!w4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut w5: () = unimplemented!();
    w5 = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (138_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"!w5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Thingy {}
impl Thingy {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Widget) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test05_15() {
    let mut ot: () = unimplemented!();
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy>&,\n\t\toptional<int>>"
    );;
    const _: () = assert!(
        true,
        "std::is_assignable_v<optional<Thingy>&,\n\t\toptional<short>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy>&,\n\t\toptional<optional<int>>>"
    );;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (158_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (160_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"ot2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<int>&&,\n\t\toptional<Thingy>>"
    );;
    let mut ot3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (164_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (166_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (169_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<optional<int>>&&,\n\t\toptional<Thingy>>"
    );;
    let mut ot7: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (174_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot7".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot8: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (176_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_constructible_v<optional<Thingy>,\n\t\toptional<optional<short>>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<optional<short>>,\n\t\toptional<Thingy>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy>&,\n\t\toptional<optional<short>>>"
    );;
    let mut ot9: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (184_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot9".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot10: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (186_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot10".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot11: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (188_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot11".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot12: () = unimplemented!();
    ot12 = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (191_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"!ot12".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RvalueConstructible {}
impl RvalueConstructible {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(_a0: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test06_16() {
    let mut oi: () = unimplemented!();
    let mut ori: () = unimplemented!();
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<RvalueConstructible>&,\n\t\toptional<int>&>"
    );;
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
                        (c"77288.cc".as_ptr()).into(),
                        (207_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"ori".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ooi: () = unimplemented!();
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<RvalueConstructible>&,\n\t\toptional<optional<int>>&>"
    );;
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (213_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"!ori".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_constructible_v<optional<RvalueConstructible>,\n\t\toptional<int>&>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<int>&,\n\t\toptional<RvalueConstructible>>"
    );;
    let mut ori2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (221_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"ori2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ori3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (223_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"ori3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_constructible_v<optional<RvalueConstructible>,\n\t\toptional<optional<int>>&>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<optional<int>>&,\n\t\toptional<RvalueConstructible>>"
    );;
    let mut ori6: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (230_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"!ori6".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ori7: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (232_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"!ori7".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ori8: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (234_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"!ori8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ori9: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (236_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"!ori9".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ori10: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (238_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"!ori10".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ori11: () = unimplemented!();
    ori11 = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (241_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"!ori11".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Thingy2 {}
impl Thingy2 {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Widget) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test07_17() {
    let mut ot: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (254_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"ot".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<int>,\n\t\toptional<Thingy2>>"
    );;
    let mut ot2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (258_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"ot2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<short>,\n\t\toptional<Thingy2>>"
    );;
    let mut ot3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (262_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<optional<int>>,\n\t\toptional<Thingy2>>"
    );;
    let mut ot4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (266_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<optional<short>>,\n\t\toptional<Thingy2>>"
    );;
    let mut ot5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (271_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot6: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (273_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot6".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy2>&,\n\t\toptional<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy2>&,\n\t\toptional<short>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy2>&,\n\t\toptional<optional<int>>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy2>&,\n\t\toptional<optional<short>>>"
    );;
    let mut ot7: () = unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (285_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot7".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot8: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (287_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot9: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (289_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot9".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot10: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (291_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot10".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot11: () = unimplemented!();
    ot11 = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (294_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"!ot11".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
impl Thingy3 {
    pub unsafe fn new_1(args: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
impl Thingy3 {
    pub unsafe fn new_2(args: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Thingy3 {}
impl Thingy3 {
    pub unsafe fn new_3(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_4(mut _a0: Widget) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test08_18() {
    let mut ot: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (310_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"ot".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<int>,\n\t\toptional<Thingy3>>"
    );;
    let mut ot2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (314_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"ot2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<short>,\n\t\toptional<Thingy3>>"
    );;
    let mut ot3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (318_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<optional<int>>,\n\t\toptional<Thingy3>>"
    );;
    let mut ot4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (322_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_convertible_v<optional<optional<short>>,\n\t\toptional<Thingy3>>"
    );;
    let mut ot5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (327_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot6: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (329_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot6".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy3>&,\n\t\toptional<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy3>&,\n\t\toptional<short>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy3>&,\n\t\toptional<optional<int>>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_assignable_v<optional<Thingy3>&,\n\t\toptional<optional<short>>>"
    );;
    let mut ot7: () = unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (341_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot7".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot8: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (343_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot9: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (345_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot9".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot10: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (347_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot10".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot11: () = unimplemented!();
    ot11 = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (350_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"!ot11".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test09_19() {
    let mut a: () = unimplemented!();
    let mut oa2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (357_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"oa2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (42_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (358_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"std::any_cast<int>(*oa2) == 42".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut oa3: () = oa2.clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (360_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"oa3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (42_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (361_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"std::any_cast<int>(*oa3) == 42".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut oa4: () = oa2.clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (363_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"oa4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (42_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (364_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"std::any_cast<int>(*oa4) == 42".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut oa5: () = oa2.clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (366_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"oa5".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (42_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (367_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"std::any_cast<int>(*oa5) == 42".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut oa6: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (369_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"!oa6".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut oa7: () = oa6.clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (371_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"!oa7".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut oa8: () = oa6.clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (373_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"!oa8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut oa9: () = oa6.clone();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (375_()).into(),
                        (c"void test09()".as_ptr()).into(),
                        (c"!oa9".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test10_20() {
    let mut oi: () = unimplemented!();
    oi = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (false)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (383_()).into(),
                        (c"void test10()".as_ptr()).into(),
                        (c"oi.has_value() == false".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut ot: () = unimplemented!();
    ot = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (false)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"77288.cc".as_ptr()).into(),
                        (386_()).into(),
                        (c"void test10()".as_ptr()).into(),
                        (c"ot.has_value() == false".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut oi2: () = unimplemented!();
    let mut si: () = 6_();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X_21 {}
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
    (unsafe { test05_15() });
    (unsafe { test06_16() });
    (unsafe { test07_17() });
    (unsafe { test08_18() });
    (unsafe { test09_19() });
    (unsafe { test10_20() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_22() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_23(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_24(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_25(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_26(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_27(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_26(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_27(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_26((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_27(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_28(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_26(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_27(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_29(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_30(
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
pub unsafe fn run_tests_wrapped_locale_31(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_32(
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
        if ((self.pid_) == (unsafe { getpid_22() })) {
            (unsafe { semctl_24(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_33(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_33(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_34(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35));
}
static mut tmp_35: std::cell::LazyCell<::libc::tm> =
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
