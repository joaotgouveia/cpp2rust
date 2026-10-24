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
pub unsafe fn test02_10() {
    let mut c1: () = ('z' as ());
    let mut c2: () = ('u' as ());
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((c1) != (c2)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (50_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"c1 != c2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    (unsafe { __gnu_cxx_char_traits_short_::assign_1(&mut c1, &c2) });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((c1) == ('u' as ())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (52_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"c1 == \'u\'".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    c1 = ('z' as ());
    c2 = ('u' as ());
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { __gnu_cxx_char_traits_short_::eq(&c1, &c2) })) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (57_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"!std::char_traits<char_type>::eq(c1, c2)".as_ptr()).into(),
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
            let ___c1: *const () = &c1;
            let ___c2: *const () = &c1;
            __gnu_cxx_char_traits_short_::eq(___c1, ___c2)
        }) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (58_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"std::char_traits<char_type>::eq(c1, c1)".as_ptr()).into(),
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
            let ___c1: *const () = &c2;
            let ___c2: *const () = &c2;
            __gnu_cxx_char_traits_short_::eq(___c1, ___c2)
        }) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (59_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"std::char_traits<char_type>::eq(c2, c2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    c1 = ('z' as ());
    c2 = ('u' as ());
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { __gnu_cxx_char_traits_short_::lt(&c2, &c1) }) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (64_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"std::char_traits<char_type>::lt(c2, c1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { __gnu_cxx_char_traits_short_::lt(&c1, &c2) })) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (65_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"!std::char_traits<char_type>::lt(c1, c2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe {
            let ___c1: *const () = &c1;
            let ___c2: *const () = &c1;
            __gnu_cxx_char_traits_short_::lt(___c1, ___c2)
        })) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (66_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"!std::char_traits<char_type>::lt(c1, c1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe {
            let ___c1: *const () = &c2;
            let ___c2: *const () = &c2;
            __gnu_cxx_char_traits_short_::lt(___c1, ___c2)
        })) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (67_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"!std::char_traits<char_type>::lt(c2, c2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut array1: [(); 11] = [
        ('z' as ()),
        ('u' as ()),
        ('m' as ()),
        ('a' as ()),
        (' ' as ()),
        ('b' as ()),
        ('e' as ()),
        ('a' as ()),
        ('c' as ()),
        ('h' as ()),
        0_(),
    ];
    let str_01: () = unimplemented!();
    let str_lit1: [(); 24] = [
        ('m' as ()),
        ('o' as ()),
        ('n' as ()),
        ('t' as ()),
        ('a' as ()),
        ('r' as ()),
        ('a' as ()),
        (' ' as ()),
        ('a' as ()),
        ('n' as ()),
        ('d' as ()),
        (' ' as ()),
        ('o' as ()),
        ('c' as ()),
        ('e' as ()),
        ('a' as ()),
        ('n' as ()),
        (' ' as ()),
        ('b' as ()),
        ('e' as ()),
        ('a' as ()),
        ('c' as ()),
        ('h' as ()),
        0_(),
    ];
    let array2_len: () = (((::std::mem::size_of::<[(); 24]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
        .wrapping_add(
            ((::std::mem::size_of::<[(); 11]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ()),
        ) as ())
        .wrapping_sub(1_());
    let mut array3: [(); 21] = [
        ('b' as ()),
        ('o' as ()),
        ('r' as ()),
        ('a' as ()),
        ('c' as ()),
        ('a' as ()),
        ('y' as ()),
        (',' as ()),
        (' ' as ()),
        ('p' as ()),
        ('h' as ()),
        ('i' as ()),
        ('l' as ()),
        ('i' as ()),
        ('p' as ()),
        ('p' as ()),
        ('i' as ()),
        ('n' as ()),
        ('e' as ()),
        ('s' as ()),
        0_(),
    ];
    let mut array2: [(); 34] = [unimplemented!(); 34];
    let mut len: () = (*(unsafe {
        let mut ___b: () = (::std::mem::size_of::<[(); 21]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        min_12(&array2_len, &mut ___b)
    }));
    (unsafe {
        __gnu_cxx_char_traits_short_::copy(
            array2.as_mut_ptr(),
            (array3.as_mut_ptr()).cast_const(),
            len,
        )
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        {}
    }
    c1 = array2[(0_()) as usize];
    c2 = str_lit1[(0_()) as usize];
    let mut c3: () = array2[(1_()) as usize];
    let mut c4: () = str_lit1[(1_()) as usize];
    (unsafe { __gnu_cxx_char_traits_short_::move_(array2.as_mut_ptr(), str_lit1.as_ptr(), 0_()) });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((array2[(0_()) as usize]) == (c1)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (90_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"array2[0] == c1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((str_lit1[(0_()) as usize]) == (c2)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (91_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"str_lit1[0] == c2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    (unsafe { __gnu_cxx_char_traits_short_::move_(array2.as_mut_ptr(), str_lit1.as_ptr(), 1_()) });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((array2[(0_()) as usize]) == (c2)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"array2[0] == c2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((str_lit1[(0_()) as usize]) == (c2)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"str_lit1[0] == c2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((array2[(1_()) as usize]) == (c3)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (95_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"array2[1] == c3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((str_lit1[(1_()) as usize]) == (c4)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (96_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"str_lit1[1] == c4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    (unsafe { __gnu_cxx_char_traits_short_::move_(array2.as_mut_ptr(), str_lit1.as_ptr(), 2_()) });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((array2[(0_()) as usize]) == (c2)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (98_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"array2[0] == c2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((str_lit1[(0_()) as usize]) == (c2)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (99_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"str_lit1[0] == c2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((array2[(1_()) as usize]) == (c4)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (100_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"array2[1] == c4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((str_lit1[(1_()) as usize]) == (c4)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (101_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"str_lit1[1] == c4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut pc1: *mut () = array1.as_mut_ptr().offset((1_()) as isize);
    c1 = (*pc1.offset((0_()) as isize));
    c2 = array1[(0_()) as usize];
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((c1) != (c2)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (106_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"c1 != c2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut pc2: *mut () = (unsafe {
        __gnu_cxx_char_traits_short_::move_(array1.as_mut_ptr(), (pc1).cast_const(), 0_())
    });
    c3 = (*pc1.offset((0_()) as isize));
    c4 = array1[(0_()) as usize];
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((c1) == (c3)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (110_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"c1 == c3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((c2) == (c4)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (111_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"c2 == c4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((pc2) == (array1.as_mut_ptr())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (112_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"pc2 == array1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    c1 = (*pc1.offset((0_()) as isize));
    c2 = array1[(0_()) as usize];
    let mut pc3: *mut () = pc1;
    pc2 = (unsafe {
        __gnu_cxx_char_traits_short_::move_(array1.as_mut_ptr(), (pc1).cast_const(), 10_())
    });
    c3 = (*pc1.offset((0_()) as isize));
    c4 = array1[(0_()) as usize];
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((c1) != (c3)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (120_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"c1 != c3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((c4) != (c3)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (121_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"c4 != c3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((pc2) == (array1.as_mut_ptr())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (122_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"pc2 == array1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((pc3) == (pc1)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (123_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"pc3 == pc1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    c1 = (*(unimplemented!()));
    c2 = array1[(0_()) as usize];
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((c1) != (c2)) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (126_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"c1 != c2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    len = (unsafe { __gnu_cxx_char_traits_short_::length(str_lit1.as_ptr()) });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((len)
            == (((::std::mem::size_of::<[(); 24]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())))
        {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (130_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"len == sizeof(str_lit1) / sizeof(char_type) - 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let N4: () =
        (::std::mem::size_of::<[(); 24]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut pc4: *const () = (unsafe {
        let mut ___a: () = ('a' as ());
        __gnu_cxx_char_traits_short_::find(str_lit1.as_ptr(), N4, &mut ___a)
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((pc4).is_null())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (135_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"pc4 != 0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*pc4) == ('a' as ())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (136_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"*pc4 == \'a\'".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    pc4 = (unsafe {
        let mut ___a: () = 2675_();
        __gnu_cxx_char_traits_short_::find(str_lit1.as_ptr(), N4, &mut ___a)
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((pc4).is_null()) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (139_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"pc4 == 0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    (unsafe {
        memset_13(
            (array2.as_mut_ptr() as ()),
            175_(),
            (array2_len).wrapping_mul((::std::mem::size_of::<()>() as ())),
        )
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((array2[(0_()) as usize]) != (5544_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (143_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"array2[0] != 0x15a8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    pc1 = (unsafe {
        __gnu_cxx_char_traits_short_::assign_9(array2.as_mut_ptr(), array2_len, 5544_())
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((pc1) == (array2.as_mut_ptr())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (146_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"pc1 == array2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (array2_len)) {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((array2[(i) as usize]) == (5544_())) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"1.cc".as_ptr()).into(),
                            (148_()).into(),
                            (c"void test02()".as_ptr()).into(),
                            (c"array2[i] == 0x15a8".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        i.prefix_inc();
    }
    let mut n1: () =
        (::std::mem::size_of::<[(); 24]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    pc1 =
        (unsafe { __gnu_cxx_char_traits_short_::copy(array2.as_mut_ptr(), str_lit1.as_ptr(), n1) });
    len = (unsafe { __gnu_cxx_char_traits_short_::length((array2.as_mut_ptr()).cast_const()) });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((len) == ((n1) - (1_()))) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (154_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"len == n1 - 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (len)) {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((str_lit1[(i) as usize]) == (array2[(i) as usize])) {
                (unsafe {
                    __builtin_fprintf_11(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"1.cc".as_ptr()).into(),
                            (156_()).into(),
                            (c"void test02()".as_ptr()).into(),
                            (c"str_lit1[i] == array2[i]".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        i.prefix_inc();
    }
    let mut pconst1: *const () = unimplemented!();
    let mut pconst2: *const () = str_lit1.as_ptr();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { __gnu_cxx_char_traits_short_::compare(pconst1, pconst2, 10_()) }) > (0_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (162_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"std::char_traits<char_type>::compare(pconst1, pconst2, 10) > 0"
                            .as_ptr())
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
        if !((unsafe { __gnu_cxx_char_traits_short_::compare(pconst2, pconst1, 10_()) }) < (0_())) {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (163_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"std::char_traits<char_type>::compare(pconst2, pconst1, 10) < 0"
                            .as_ptr())
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
        if !((unsafe {
            let ___s1: *const () = pconst1;
            let ___s2: *const () = pconst1;
            __gnu_cxx_char_traits_short_::compare(___s1, ___s2, 10_())
        }) == (0_()))
        {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (164_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"std::char_traits<char_type>::compare(pconst1, pconst1, 10) == 0"
                            .as_ptr())
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
        if !((unsafe {
            let ___s1: *const () = pconst2;
            let ___s2: *const () = pconst2;
            __gnu_cxx_char_traits_short_::compare(___s1, ___s2, 10_())
        }) == (0_()))
        {
            (unsafe {
                __builtin_fprintf_11(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"1.cc".as_ptr()).into(),
                        (165_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"std::char_traits<char_type>::compare(pconst2, pconst2, 10) == 0"
                            .as_ptr())
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
    (unsafe { test02_10() });
    return 0_();
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_14() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_15(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_16(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_17(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_18(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_19(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_18(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_19(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_18((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_19(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_20(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_18(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_19(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_21(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_22(
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
pub unsafe fn run_tests_wrapped_locale_23(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_24(
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
        if ((self.pid_) == (unsafe { getpid_14() })) {
            (unsafe { semctl_16(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_25(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_25(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_26(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_27));
}
static mut tmp_27: std::cell::LazyCell<::libc::tm> =
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
