extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_from_chars_0(
    mut expected: (),
    mut s: (),
    mut base: Option<()>,
    mut term: Option<()>,
) -> () {
    let mut base: () = base.unwrap_or_else(|| unsafe { 0_() });
    let mut term: () = term.unwrap_or_else(|| unsafe { ('\0' as ()) });
    let mut begin: *const () = (unimplemented!()).cast_const();
    let mut end: *const () = (unimplemented!().offset((unimplemented!()) as isize)).cast_const();
    let mut val: () = unimplemented!();
    let mut r: () = if ((base) == (0_())) {
        (unsafe { from_chars_1(begin, end, &mut val, None) })
    } else {
        (unsafe { from_chars_1(begin, end, &mut val, Some(base)) })
    };
    return (((r.ec) == (unimplemented!())) && (((r.ptr) == (end)) || ((*r.ptr) == (term))))
        && ((val) == (expected));
}
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_2)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const __gnu_test_object_counter) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_2)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_2)).prefix_dec();
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
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__3))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)))
        {
            (unsafe { __throw_runtime_error_5(c"copy_constructor::mark_call".as_ptr()) });
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
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__6));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__6)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__6))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__7)))
        {
            (unsafe { __throw_runtime_error_5(c"assignment_operator::mark_call".as_ptr()) });
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__6)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__7)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__7)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_8));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_8)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_8)) = 0_();
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__9)).postfix_dec()
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
            if operator_lt_10(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_10(
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
            operator_eq_11(
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
pub unsafe fn operator_eq_11(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_10(
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
pub unsafe fn test01_12() {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            check_from_chars_0(
                123_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"123".as_ptr() },
                    None,
                ),
                None,
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (45_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"check_from_chars(123, \"123\")".as_ptr()).into(),
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
            check_from_chars_0(
                -123_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"-123".as_ptr() },
                    None,
                ),
                None,
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (46_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"check_from_chars(-123, \"-123\")".as_ptr()).into(),
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
            check_from_chars_0(
                123_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"123a".as_ptr() },
                    None,
                ),
                Some(10_()),
                Some(('a' as ())),
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (47_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"check_from_chars(123, \"123a\", 10, \'a\')".as_ptr()).into(),
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
            check_from_chars_0(
                123_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"0000000000000000000000000000123".as_ptr() },
                    None,
                ),
                None,
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (48_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"check_from_chars(123, \"0000000000000000000000000000123\")".as_ptr())
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
            check_from_chars_0(
                123_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"0000000000000000000000000000123a".as_ptr() },
                    None,
                ),
                Some(10_()),
                Some(('a' as ())),
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (49_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"check_from_chars(123, \"0000000000000000000000000000123a\", 10, \'a\')"
                            .as_ptr())
                        .into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test02_14() {
    let mut base: () = 2_();
    'loop_: while ((base) < (34_())) {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                check_from_chars_0(
                    0_(),
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { c"0x1".as_ptr() },
                        None,
                    ),
                    Some(base),
                    Some(('x' as ())),
                )
            }) {
                (unsafe {
                    __builtin_fprintf_13(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"1.cc".as_ptr()).into(),
                            (58_()).into(),
                            (c"void test02()".as_ptr()).into(),
                            (c"check_from_chars(0, \"0x1\", base, \'x\')".as_ptr()).into(),
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
                check_from_chars_0(
                    0_(),
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { c"0X1".as_ptr() },
                        None,
                    ),
                    Some(base),
                    Some(('X' as ())),
                )
            }) {
                (unsafe {
                    __builtin_fprintf_13(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"1.cc".as_ptr()).into(),
                            (59_()).into(),
                            (c"void test02()".as_ptr()).into(),
                            (c"check_from_chars(0, \"0X1\", base, \'X\')".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        base.prefix_inc();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe {
            check_from_chars_0(
                1123_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"0x1".as_ptr() },
                    None,
                ),
                Some(34_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (62_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1123, \"0x1\", 34)".as_ptr()).into(),
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
            check_from_chars_0(
                1123_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"0X1".as_ptr() },
                    None,
                ),
                Some(34_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (63_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1123, \"0X1\", 34)".as_ptr()).into(),
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
            check_from_chars_0(
                1156_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"0x1".as_ptr() },
                    None,
                ),
                Some(35_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (64_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1156, \"0x1\", 35)".as_ptr()).into(),
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
            check_from_chars_0(
                1156_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"0X1".as_ptr() },
                    None,
                ),
                Some(35_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (65_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1156, \"0X1\", 35)".as_ptr()).into(),
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
            check_from_chars_0(
                1189_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"0x1".as_ptr() },
                    None,
                ),
                Some(36_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (66_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1189, \"0x1\", 36)".as_ptr()).into(),
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
            check_from_chars_0(
                1189_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"0X1".as_ptr() },
                    None,
                ),
                Some(36_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (67_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1189, \"0X1\", 36)".as_ptr()).into(),
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
            check_from_chars_0(
                1155_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"xx".as_ptr() },
                    None,
                ),
                Some(34_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (69_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1155, \"xx\", 34)".as_ptr()).into(),
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
            check_from_chars_0(
                1155_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"XX".as_ptr() },
                    None,
                ),
                Some(34_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (70_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1155, \"XX\", 34)".as_ptr()).into(),
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
            check_from_chars_0(
                1155_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"Xx".as_ptr() },
                    None,
                ),
                Some(34_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (71_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1155, \"Xx\", 34)".as_ptr()).into(),
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
            check_from_chars_0(
                1224_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"yy".as_ptr() },
                    None,
                ),
                Some(35_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (72_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1224, \"yy\", 35)".as_ptr()).into(),
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
            check_from_chars_0(
                1224_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"YY".as_ptr() },
                    None,
                ),
                Some(35_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (73_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1224, \"YY\", 35)".as_ptr()).into(),
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
            check_from_chars_0(
                1224_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"yY".as_ptr() },
                    None,
                ),
                Some(35_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (74_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1224, \"yY\", 35)".as_ptr()).into(),
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
            check_from_chars_0(
                1295_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"zz".as_ptr() },
                    None,
                ),
                Some(36_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (75_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1295, \"zz\", 36)".as_ptr()).into(),
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
            check_from_chars_0(
                1295_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ZZ".as_ptr() },
                    None,
                ),
                Some(36_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (76_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1295, \"ZZ\", 36)".as_ptr()).into(),
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
            check_from_chars_0(
                1295_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"Zz".as_ptr() },
                    None,
                ),
                Some(36_()),
                None,
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (77_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1295, \"Zz\", 36)".as_ptr()).into(),
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
            check_from_chars_0(
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"01234".as_ptr() },
                    None,
                ),
                Some(2_()),
                Some(('2' as ())),
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (80_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1, \"01234\", 2, \'2\')".as_ptr()).into(),
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
            check_from_chars_0(
                27_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234".as_ptr() },
                    None,
                ),
                Some(4_()),
                Some(('4' as ())),
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (81_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(27, \"1234\", 4, \'4\')".as_ptr()).into(),
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
            check_from_chars_0(
                1155_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"xxy".as_ptr() },
                    None,
                ),
                Some(34_()),
                Some(('y' as ())),
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (82_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1155, \"xxy\", 34, \'y\')".as_ptr()).into(),
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
            check_from_chars_0(
                1224_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"yyz".as_ptr() },
                    None,
                ),
                Some(35_()),
                Some(('z' as ())),
            )
        }) {
            (unsafe {
                __builtin_fprintf_13(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (83_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"check_from_chars(1224, \"yyz\", 35, \'z\')".as_ptr()).into(),
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
    (unsafe { test01_12() });
    (unsafe { test02_14() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_15() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_16(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_5(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_17(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_5(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_18(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_19(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_20(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_19(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_20(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_19((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_20(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_21(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_19(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_20(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_22(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_23(
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
        (unsafe { __throw_runtime_error_5(s) });
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_24(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_13 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
        (unsafe { __throw_runtime_error_5(unimplemented!()) });
    }
}
pub unsafe fn run_tests_wrapped_env_25(
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
        (unsafe { __throw_runtime_error_5(unimplemented!()) });
    }
}
pub static mut count_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut _M_count_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut next_id__9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
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
        if ((self.pid_) == (unsafe { getpid_15() })) {
            (unsafe { semctl_17(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_26(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_5(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_26(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_5(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_27(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_28));
}
static mut tmp_28: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const count__6);
    std::cell::LazyCell::force(&*&raw const throw_on__7);
    std::cell::LazyCell::force(&*&raw const _M_count_8);
    std::cell::LazyCell::force(&*&raw const next_id__9);
}
