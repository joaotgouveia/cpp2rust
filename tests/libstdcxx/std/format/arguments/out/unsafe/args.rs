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
const _: () = assert!(true, "! std::is_default_constructible_v<std::format_args>");
const _: () = assert!(true, "! std::is_default_constructible_v<std::wformat_args>");
pub unsafe fn equals_10(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_11(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_12(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_13(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_14(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_15(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_16(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_17(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_18(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_19(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_20(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_21(mut fmt_arg: (), mut expected: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_22(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_23(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_24(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn test_empty_25() {
    let mut args: () = {
        let mut __tmp_0 : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_char___char__  = ( unsafe { make_format_args_26 ( ) } )  ;
        std_basic_format_args_std_basic_format_context_std___format__Sink_iter_char___char__::new_1(
            { &mut __tmp_0 },
        )
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { unimplemented!()() })) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (26_()).into(),
                        (c"void test_empty()".as_ptr()).into(),
                        (c"!args.get(0)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { unimplemented!()() })) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (27_()).into(),
                        (c"void test_empty()".as_ptr()).into(),
                        (c"!args.get(1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { unimplemented!()() })) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (28_()).into(),
                        (c"void test_empty()".as_ptr()).into(),
                        (c"!args.get((std::size_t)-1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_10(unimplemented!(), unimplemented!()) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (29_()).into(),
                        (c"void test_empty()".as_ptr()).into(),
                        (c"equals(args.get(0), std::monostate{})".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut cargs: () = {
        let mut __tmp_1 : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_char___char__  = ( unsafe { make_format_args_26 ( ) } )  ;
        std_basic_format_args_std_basic_format_context_std___format__Sink_iter_char___char__::new_1(
            { &mut __tmp_1 },
        )
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { unimplemented!()() })) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (32_()).into(),
                        (c"void test_empty()".as_ptr()).into(),
                        (c"!cargs.get(0)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_10(unimplemented!(), unimplemented!()) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (33_()).into(),
                        (c"void test_empty()".as_ptr()).into(),
                        (c"equals(cargs.get(0), std::monostate{})".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut wargs: () = {
        let mut __tmp_2 : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_wchar_t___wchar_t__  = unimplemented!()  ;
        std_basic_format_args_std_basic_format_context_std___format__Sink_iter_wchar_t___wchar_t__ :: new_1 ( { & mut __tmp_2 } , )
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { unimplemented!()() })) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (36_()).into(),
                        (c"void test_empty()".as_ptr()).into(),
                        (c"!wargs.get(0)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_11(unimplemented!(), unimplemented!()) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (37_()).into(),
                        (c"void test_empty()".as_ptr()).into(),
                        (c"equals(wargs.get(0), std::monostate{})".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub type E = ();
pub const E_ByGum: E = 0;
pub unsafe fn test_args_28() {
    let mut b: () = false;
    let mut i: () = 1_();
    let mut c: () = ('2' as ());
    let mut d: () = 3.4_f64;
    let mut store : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_char___char___bool__int__char__double_ = ( unsafe { make_format_args_29 ( & mut b , & mut i , & mut c , & mut d , ) } )  ;
    let mut args: () =
        std_basic_format_args_std_basic_format_context_std___format__Sink_iter_char___char__::new_2(
            { &store },
        );
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_12(unimplemented!(), false) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (65_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(args.get(0), false)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_13(unimplemented!(), 1_()) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (66_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(args.get(1), 1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_14(unimplemented!(), ('2' as ())) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (67_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(args.get(2), \'2\')".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_15(unimplemented!(), 3.4_f64) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (68_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(args.get(3), 3.4)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { unimplemented!()() })) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (69_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"!args.get(4)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut l: () = 5_();
    let mut ull: () = 6_();
    let mut f: () = 7.8_f32;
    let mut cstore : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_char___char___long_long__unsigned_long_long__float_ = ( unsafe { make_format_args_30 ( & mut l , & mut ull , & mut f , ) } )  ;
    let mut cargs: () =
        std_basic_format_args_std_basic_format_context_std___format__Sink_iter_char___char__::new_3(
            { &cstore },
        );
    {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe { equals_16(unimplemented!(), 5_()) }) {
                (unsafe {
                    __builtin_fprintf_27(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"args.cc".as_ptr()).into(),
                            (80_()).into(),
                            (c"void test_args()".as_ptr()).into(),
                            (c"equals(cargs.get(0), 5LL)".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_17(unimplemented!(), 6_()) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (81_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(cargs.get(1), 6ULL)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_18(unimplemented!(), 7.8_f32) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (82_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(cargs.get(2), 7.8f)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { unimplemented!()() })) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (83_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"!cargs.get(3)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut s: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_19(unimplemented!(), unimplemented!()) }) {
            (unsafe {
                __builtin_fprintf_27 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"args.cc" .as_ptr() ) .into() , ( 86_() ) .into() , ( c"void test_args()" .as_ptr() ) .into() , ( c"equals(std::format_args(std::make_format_args(s)).get(0), std::string_view(\"tenfour\"))" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut nine: () = ('9' as ());
    let mut ten: () = (88 as ());
    let mut eleven: [(); 7] = [
        101 as (), 108 as (), 101 as (), 118 as (), 101 as (), 110 as (), 0 as (),
    ];
    let mut twelve13: () = 12.13_f64;
    let mut tenfour: () = unimplemented!();
    let mut wstore : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_wchar_t___wchar_t___wchar_t__wchar_t__const_wchar_t_ptr__long_double__std_basic_string_view_wchar_t__std_char_traits_wchar_t___ = unimplemented!()  ;
    let mut wargs : () = std_basic_format_args_std_basic_format_context_std___format__Sink_iter_wchar_t___wchar_t__ :: new_2 ( { & wstore } , )  ;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_20(unimplemented!(), ('9' as ())) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (97_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(wargs.get(0), static_cast<wchar_t>(\'9\'))".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_20(unimplemented!(), (88 as ())) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (98_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(wargs.get(1), L\'X\')".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_21(unimplemented!(), (eleven.as_mut_ptr()).cast_const()) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (99_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(wargs.get(2), static_cast<const wchar_t*>(eleven))".as_ptr())
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
        if !(unsafe { equals_22(unimplemented!(), 12.13_f64) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (100_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(wargs.get(3), 12.13L)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_23(unimplemented!(), (unsafe { tenfour() })) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (101_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(wargs.get(4), std::wstring_view(tenfour))".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unsafe { unimplemented!()() })) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (102_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"!wargs.get(5)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut null: () = unimplemented!();
    let mut eebygum: E = E_ByGum;
    let mut another_store : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_char___char___const_void_ptr__std_basic_format_arg_std_basic_format_context_std___format__Sink_iter_char___char___handle_ = ( unsafe { make_format_args_31 ( & mut null , & mut eebygum , ) } )  ;
    args =
        std_basic_format_args_std_basic_format_context_std___format__Sink_iter_char___char__::new_4(
            { &another_store },
        );
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_24(unimplemented!(), unimplemented!()) }) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (108_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"equals(args.get(0), static_cast<const void*>(nullptr))".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut is_handle : FnPtr<Generic> = lambda_unsafe! ( Generic , { } , | mut _a0 : std_basic_format_arg_std_basic_format_context_std___format__Sink_iter_char___char___handle , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_32))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_33))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_34))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_35))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_36))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_37))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_38))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_39))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_40))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_41))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_42))  ;
 } , | mut _a0 : *const () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_43))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_44))  ;
 } , | mut _a0 : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_45))  ;
 } ) ;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (111_()).into(),
                        (c"void test_args()".as_ptr()).into(),
                        (c"std::visit_format_arg(is_handle, args.get(1))".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_member_visit_46() {}
pub unsafe fn test_visited_as_handle_47() {
    let mut v: () = unimplemented!();
    let mut store : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_char___char_____int128_ = ( unsafe { make_format_args_48 ( & mut v , ) } )  ;
    let mut args: () =
        std_basic_format_args_std_basic_format_context_std___format__Sink_iter_char___char__::new_5(
            { &store },
        );
    let is_handle : FnPtr<Generic> = lambda_unsafe! ( Generic , { } , | mut arg : std_basic_format_arg_std_basic_format_context_std___format__Sink_iter_char___char___handle , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_32))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_33))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_34))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_35))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_36))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_37))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_38))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_39))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_40))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_41))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_42))  ;
 } , | mut arg : *const () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_43))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_44))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_45))  ;
 } ) ;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (161_()).into(),
                        (c"void test_visited_as_handle() [T = __int128]".as_ptr()).into(),
                        (c"std::visit_format_arg(is_handle, args.get(0))".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_visited_as_handle_49() {
    let mut v: () = unimplemented!();
    let mut store : std___format__Arg_store_std_basic_format_context_std___format__Sink_iter_char___char___unsigned___int128_ = ( unsafe { make_format_args_50 ( & mut v , ) } )  ;
    let mut args: () =
        std_basic_format_args_std_basic_format_context_std___format__Sink_iter_char___char__::new_6(
            { &store },
        );
    let is_handle : FnPtr<Generic> = lambda_unsafe! ( Generic , { } , | mut arg : std_basic_format_arg_std_basic_format_context_std___format__Sink_iter_char___char___handle , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_32))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_33))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_34))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_35))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_36))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_37))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_38))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_39))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_40))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_41))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_42))  ;
 } , | mut arg : *const () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_43))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_44))  ;
 } , | mut arg : () , | -> () { return (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_45))  ;
 } ) ;
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"args.cc".as_ptr()).into(),
                        (161_()).into(),
                        (c"void test_visited_as_handle() [T = unsigned __int128]".as_ptr()).into(),
                        (c"std::visit_format_arg(is_handle, args.get(0))".as_ptr()).into(),
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
    (unsafe { test_empty_25() });
    (unsafe { test_args_28() });
    (unsafe { test_member_visit_46() });
    (unsafe { test_visited_as_handle_47() });
    (unsafe { test_visited_as_handle_49() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_51() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_52(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_53(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_54(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_55(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_56(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_55(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_56(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_55((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_56(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_57(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_55(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_56(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_58(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_59(
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
pub unsafe fn run_tests_wrapped_locale_60(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_27 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_61(
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
        if ((self.pid_) == (unsafe { getpid_51() })) {
            (unsafe { semctl_53(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_62(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_62(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_63(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_64));
}
static mut tmp_64: std::cell::LazyCell<::libc::tm> =
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
