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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_ok {
    pub in_size: (),
    pub out_size: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_partial {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_unsigned_char_ {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
    pub replace_char: (),
    pub replace_pos: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_char16_t_ {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
    pub replace_char: (),
    pub replace_pos: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_wchar_t_ {
    pub in_size: (),
    pub out_size: (),
    pub expected_in_next: (),
    pub expected_out_next: (),
    pub replace_char: (),
    pub replace_pos: (),
}
pub unsafe fn array_size_10(_a0: *const [(); 11]) -> () {
    return 11_();
}
pub unsafe fn array_size_11(_a0: *const [(); 5]) -> () {
    return 5_();
}
pub unsafe fn array_size_12(_a0: *const [(); 6]) -> () {
    return 6_();
}
pub unsafe fn array_size_13(_a0: *const [(); 7]) -> () {
    return 7_();
}
pub unsafe fn array_size_14(_a0: *const [(); 4]) -> () {
    return 4_();
}
pub unsafe fn array_size_15(_a0: *const [(); 12]) -> () {
    return 12_();
}
pub unsafe fn array_size_16(_a0: *const [(); 10]) -> () {
    return 10_();
}
pub unsafe fn array_size_17(_a0: *const [(); 8]) -> () {
    return 8_();
}
pub unsafe fn array_size_18(_a0: *const [(); 6]) -> () {
    return 6_();
}
pub unsafe fn array_size_19(_a0: *const [(); 5]) -> () {
    return 5_();
}
pub unsafe fn array_size_20(_a0: *const [(); 11]) -> () {
    return 11_();
}
pub unsafe fn array_size_21(_a0: *const [(); 4]) -> () {
    return 4_();
}
pub unsafe fn array_size_22(_a0: *const [(); 6]) -> () {
    return 6_();
}
pub unsafe fn utf8_to_utf32_in_ok_23(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 59_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (in) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 60_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (exp) == 4" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 4_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_20(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 66_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_21(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 67_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 75_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 76_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 77_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 78_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_21(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 80_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 84_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 85_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_20(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 91_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 92_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = unimplemented!();
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 100_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 101_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 102_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 103_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 105_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 109_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 110_() ) .into() , ( c"void utf8_to_utf32_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf8_to_utf32_in_partial_25(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 129_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (in) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 130_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (exp) == 4" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((16_()) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_20(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 157_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_21(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 158_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 159_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 160_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_partial)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 168_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.partial" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 169_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 170_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 172_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_21(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 174_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 178_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 179_() ) .into() , ( c"void utf8_to_utf32_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf8_to_utf32_in_error_26(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 5] = [98 as (), 1096 as (), 55040 as (), 1092266 as (), 0 as ()];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 198_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (in) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 199_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (exp) == 4" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 4_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((38_()) as isize),
    ) {
        let mut t: test_offsets_error_unsigned_char_ = (*t);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_20(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 298_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_21(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 299_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 300_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 301_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_error)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 312_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.error" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 313_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 314_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 316_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_21(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 318_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 322_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 323_() ) .into() , ( c"void utf8_to_utf32_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf8_to_utf32_in_27(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_ok_23(cvt) });
    (unsafe { utf8_to_utf32_in_partial_25(cvt) });
    (unsafe { utf8_to_utf32_in_error_26(cvt) });
}
pub unsafe fn utf32_to_utf8_out_ok_28(cvt: *const ()) {
    let input: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 353_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (in) == 4" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 354_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (exp) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 10_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 360_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 361_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 369_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 370_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 371_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 372_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 374_() ) .into() , ( c"void utf32_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf8_out_partial_29(cvt: *const ()) {
    let input: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 393_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (in) == 4" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 394_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (exp) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((10_()) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 414_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 415_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 416_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 417_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_partial)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 425_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.partial" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 426_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 427_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 429_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 431_() ) .into() , ( c"void utf32_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf8_out_error_30(cvt: *const ()) {
    let input: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 450_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (in) == 4" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 451_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (exp) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_error_wchar_t_; 8] = [
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 1114112_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 1114112_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 1114112_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 1114112_(),
            replace_pos: 3_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((8_()) as isize),
    ) {
        let mut t: test_offsets_error_wchar_t_ = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 470_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 471_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 472_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 473_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_error)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 484_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.error" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 485_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 486_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 488_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 490_() ) .into() , ( c"void utf32_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf32_to_utf8_out_31(cvt: *const ()) {
    (unsafe { utf32_to_utf8_out_ok_28(cvt) });
    (unsafe { utf32_to_utf8_out_partial_29(cvt) });
    (unsafe { utf32_to_utf8_out_error_30(cvt) });
}
pub unsafe fn test_utf8_utf32_cvt_32(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_27(cvt) });
    (unsafe { utf32_to_utf8_out_31(cvt) });
}
pub unsafe fn utf8_to_utf16_in_ok_33(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 528_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (in) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 529_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (exp) == 5" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 5_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_20(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 535_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 536_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 544_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 545_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 546_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 547_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 549_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 553_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 554_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_20(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 560_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_22(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 561_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = unimplemented!();
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 569_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 570_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 571_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 572_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_22(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 574_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 578_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 579_() ) .into() , ( c"void utf8_to_utf16_in_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf8_to_utf16_in_partial_34(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 598_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (in) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 599_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (exp) == 5" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_partial; 20] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((20_()) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_20(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 631_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 632_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 633_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 634_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_partial)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 642_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.partial" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 643_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 644_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 646_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 648_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 652_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 653_() ) .into() , ( c"void utf8_to_utf16_in_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf8_to_utf16_in_error_35(cvt: *const ()) {
    let input: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [(); 6] = [
        98 as (),
        1096 as (),
        55040 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [(); 11] = [unimplemented!(); 11];
    let mut exp: [(); 6] = [unimplemented!(); 6];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 672_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (in) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 673_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (exp) == 5" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 255_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_(),
            out_size: 5_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 170_(),
            replace_pos: 0_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 170_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 170_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 170_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: ('z' as ()),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 255_(),
            replace_pos: 2_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 5_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 9_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: ('z' as ()),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 255_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 7_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: ('z' as ()),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 255_(),
            replace_pos: 8_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 160_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 172_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 176_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 188_(),
            replace_pos: 4_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 192_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 193_(),
            replace_pos: 1_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_(),
            out_size: 5_(),
            expected_in_next: 3_(),
            expected_out_next: 2_(),
            replace_char: 224_(),
            replace_pos: 3_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 240_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 245_(),
            replace_pos: 6_(),
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_(),
            out_size: 5_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 154_(),
            replace_pos: 7_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((38_()) as isize),
    ) {
        let mut t: test_offsets_error_unsigned_char_ = (*t);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_20(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 772_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 773_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 774_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 775_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_error)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 786_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.error" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 787_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 788_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 790_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 792_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 796_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 797_() ) .into() , ( c"void utf8_to_utf16_in_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf8_to_utf16_in_36(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_ok_33(cvt) });
    (unsafe { utf8_to_utf16_in_partial_34(cvt) });
    (unsafe { utf8_to_utf16_in_error_35(cvt) });
}
pub unsafe fn utf16_to_utf8_out_ok_37(cvt: *const ()) {
    let input: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 827_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (in) == 5" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 828_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (exp) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 5_(),
            out_size: 10_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_22(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 834_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 835_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 843_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 844_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 845_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 846_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 848_() ) .into() , ( c"void utf16_to_utf8_out_ok(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf16_to_utf8_out_partial_38(cvt: *const ()) {
    let input: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 867_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (in) == 5" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 868_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (exp) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_partial; 15] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 1_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 3_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((15_()) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_22(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 895_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 896_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 897_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 898_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_partial)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 906_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.partial" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 907_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 908_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 910_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 912_() ) .into() , ( c"void utf16_to_utf8_out_partial(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf16_to_utf8_out_error_39(cvt: *const ()) {
    let input: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    let expected: [(); 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [(); 6] = [unimplemented!(); 6];
    let mut exp: [(); 11] = [unimplemented!(); 11];
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 931_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<InternT>::length (in) == 5" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            (unsafe {
                __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 932_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::length (exp) == 10" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut offsets: [test_offsets_error_wchar_t_; 17] = [
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 5_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: (122 as ()),
            replace_pos: 4_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((17_()) as isize),
    ) {
        let mut t: test_offsets_error_wchar_t_ = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_22(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 967_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 968_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 969_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 970_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_error)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 981_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"res == cvt.error" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 982_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 983_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 985_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"char_traits<ExternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 987_() ) .into() , ( c"void utf16_to_utf8_out_error(const std::codecvt<InternT, ExternT, mbstate_t> &) [InternT = wchar_t, ExternT = char]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf16_to_utf8_out_40(cvt: *const ()) {
    (unsafe { utf16_to_utf8_out_ok_37(cvt) });
    (unsafe { utf16_to_utf8_out_partial_38(cvt) });
    (unsafe { utf16_to_utf8_out_error_39(cvt) });
}
pub unsafe fn test_utf8_utf16_cvt_41(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_36(cvt) });
    (unsafe { utf16_to_utf8_out_40(cvt) });
}
pub type utf16_endianess = ();
pub const utf16_endianess_utf16_big_endian: utf16_endianess = 0;
pub const utf16_endianess_utf16_little_endian: utf16_endianess = 1;
pub unsafe fn utf16_to_bytes_42(
    mut f: *const (),
    mut l: *const (),
    mut o: *mut (),
    mut e: utf16_endianess,
) -> *mut () {
    if ((e as ()) == (utf16_endianess_utf16_big_endian as ())) {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = (((*f) >> (8_())) & (255_()));
            (*o.postfix_inc()) = ((*f) & (255_()));
            f.prefix_inc();
        }
    } else {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = ((*f) & (255_()));
            (*o.postfix_inc()) = (((*f) >> (8_())) & (255_()));
            f.prefix_inc();
        }
    }
    return o;
}
pub unsafe fn utf16_to_bytes_43(
    mut f: *mut (),
    mut l: *mut (),
    mut o: *mut (),
    mut e: utf16_endianess,
) -> *mut () {
    if ((e as ()) == (utf16_endianess_utf16_big_endian as ())) {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = (((*f) >> (8_())) & (255_()));
            (*o.postfix_inc()) = ((*f) & (255_()));
            f.prefix_inc();
        }
    } else {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = ((*f) & (255_()));
            (*o.postfix_inc()) = (((*f) >> (8_())) & (255_()));
            f.prefix_inc();
        }
    }
    return o;
}
pub unsafe fn utf16_to_utf32_in_ok_44(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    let expected: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [(); 12] = [unimplemented!(); 12];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_42(_f, _l, unimplemented!(), endianess)
    });
    unimplemented!();
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 1_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 6_(),
            out_size: 3_(),
        },
        test_offsets_ok {
            in_size: 10_(),
            out_size: 4_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_15(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1546_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_21(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1547_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1555_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1556_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1557_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1558_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_21(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1560_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1564_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1565_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_15(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1571_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1572_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = unimplemented!();
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1580_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1581_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1582_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1583_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1585_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1589_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1590_() ) .into() , ( c"void utf16_to_utf32_in_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf16_to_utf32_in_partial_45(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    let expected: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [(); 12] = [unimplemented!(); 12];
    let mut exp: [(); 5] = [unimplemented!(); 5];
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_42(_f, _l, unimplemented!(), endianess)
    });
    unimplemented!();
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 2_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 1_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 1_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 2_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 1_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
        },
        test_offsets_partial {
            in_size: 6_(),
            out_size: 2_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 3_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 5_(),
            out_size: 2_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 10_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 7_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 8_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
        test_offsets_partial {
            in_size: 9_(),
            out_size: 3_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((16_()) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_15(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1635_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_21(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1636_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1637_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1638_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_partial)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1646_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"res == cvt.partial" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1647_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1648_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1650_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_21(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1652_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1656_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1657_() ) .into() , ( c"void utf16_to_utf32_in_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf16_to_utf32_in_error_46(cvt: *const (), mut endianess: utf16_endianess) {
    let mut input: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    let expected: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut exp: [(); 5] = [unimplemented!(); 5];
    unimplemented!();
    let mut offsets: [test_offsets_error_char16_t_; 17] = [
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56319_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 56320_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 57343_(),
            replace_pos: 0_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 55296_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 56320_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 1_(),
            replace_char: 57343_(),
            replace_pos: 1_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 55296_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 56319_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 4_(),
            expected_out_next: 2_(),
            replace_char: 57343_(),
            replace_pos: 2_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 56320_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 55296_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: 56319_(),
            replace_pos: 4_(),
        },
        test_offsets_error_char16_t_ {
            in_size: 10_(),
            out_size: 4_(),
            expected_in_next: 6_(),
            expected_out_next: 3_(),
            replace_char: (122 as ()),
            replace_pos: 4_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((17_()) as isize),
    ) {
        let mut t: test_offsets_error_char16_t_ = (*t);
        let mut in_: [(); 12] = [unimplemented!(); 12];
        let mut out: [(); 4] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_15(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1709_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_21(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1710_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1711_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1712_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut old_char: () = input[(t.replace_pos) as usize];
        input[(t.replace_pos) as usize] = t.replace_char;
        (unsafe {
            let _f: *mut () = unimplemented!();
            let _l: *mut () = unimplemented!();
            utf16_to_bytes_43(_f, _l, unimplemented!(), endianess)
        });
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).in_(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_error)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1724_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"res == cvt.error" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1725_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1726_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1728_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"char_traits<InternT>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_21(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1730_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        state = unsafe { std::mem::zeroed::<__mbstate_t>() };
        let mut len: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1734_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"len >= 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1735_() ) .into() , ( c"void utf16_to_utf32_in_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"static_cast<size_t> (len) == t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        input[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf32_to_utf16_out_ok_47(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    let expected: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_42(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_(),
            out_size: 0_(),
        },
        test_offsets_ok {
            in_size: 1_(),
            out_size: 2_(),
        },
        test_offsets_ok {
            in_size: 2_(),
            out_size: 4_(),
        },
        test_offsets_ok {
            in_size: 3_(),
            out_size: 6_(),
        },
        test_offsets_ok {
            in_size: 4_(),
            out_size: 10_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_()) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1761_() ) .into() , ( c"void utf32_to_utf16_out_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1762_() ) .into() , ( c"void utf32_to_utf16_out_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_ok)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1770_() ) .into() , ( c"void utf32_to_utf16_out_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"res == cvt.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1771_() ) .into() , ( c"void utf32_to_utf16_out_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"in_next == in + t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1772_() ) .into() , ( c"void utf32_to_utf16_out_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out_next == out + t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1773_() ) .into() , ( c"void utf32_to_utf16_out_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"char_traits<char>::compare (out, exp, t.out_size) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1775_() ) .into() , ( c"void utf32_to_utf16_out_ok(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out[t.out_size] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf16_out_partial_48(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    let expected: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_42(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_(),
            out_size: 0_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 1_(),
            out_size: 1_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 2_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 2_(),
            out_size: 3_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 4_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
        },
        test_offsets_partial {
            in_size: 3_(),
            out_size: 5_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 6_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 7_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 8_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
        test_offsets_partial {
            in_size: 4_(),
            out_size: 9_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((10_()) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1813_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1814_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1815_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1816_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_partial)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1824_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"res == cvt.partial" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1825_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1826_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1827_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"char_traits<char>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1829_() ) .into() , ( c"void utf32_to_utf16_out_partial(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf16_out_error_49(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [(); 5] = [98 as (), 1096 as (), 43690 as (), 1092266 as (), 0 as ()];
    let expected: [(); 6] = [
        98 as (),
        1096 as (),
        43690 as (),
        56298 as (),
        57002 as (),
        0 as (),
    ];
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [(); 5] = [unimplemented!(); 5];
    let mut exp: [(); 12] = [unimplemented!(); 12];
    unimplemented!();
    (unsafe {
        let _f: *const () = unimplemented!();
        let _l: *const () = unimplemented!();
        utf16_to_bytes_42(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_error_wchar_t_; 8] = [
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 55296_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 56319_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 56320_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 57343_(),
            replace_pos: 3_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 0_(),
            expected_out_next: 0_(),
            replace_char: 1114112_(),
            replace_pos: 0_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 1_(),
            expected_out_next: 2_(),
            replace_char: 1114112_(),
            replace_pos: 1_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 2_(),
            expected_out_next: 4_(),
            replace_char: 1114112_(),
            replace_pos: 2_(),
        },
        test_offsets_error_wchar_t_ {
            in_size: 4_(),
            out_size: 10_(),
            expected_in_next: 3_(),
            expected_out_next: 6_(),
            replace_char: 1114112_(),
            replace_pos: 3_(),
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((8_()) as isize),
    ) {
        let mut t: test_offsets_error_wchar_t_ = (*t);
        let mut out: [(); 10] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1866_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.in_size <= array_size (in)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_16(&out) })) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1867_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.out_size <= array_size (out)" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1868_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.expected_in_next <= t.in_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1869_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"t.expected_out_next <= t.out_size" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = (unsafe {
            let ___from: *const () = (in_.as_mut_ptr()).cast_const();
            let ___from_end: *const () =
                (in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const();
            let ___to: *mut () = out.as_mut_ptr();
            let ___to_end: *mut () = out.as_mut_ptr().offset((t.out_size) as isize);
            ((*cvt) as std___codecvt_abstract_base_wchar_t__char____mbstate_t_).out(
                &mut *(&raw mut state).cast::<__mbstate_t>(),
                ___from,
                ___from_end,
                &mut in_next,
                ___to,
                ___to_end,
                &mut out_next,
            )
        });
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (std_codecvt_base_result_error)) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1880_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"res == cvt.error" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1881_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"in_next == in + t.expected_in_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1882_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out_next == out + t.expected_out_next" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                (unsafe {
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1883_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"char_traits<char>::compare (out, exp, t.expected_out_next) == 0" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_16(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    (unsafe {
                        __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"codecvt_unicode_wchar_t.cc" .as_ptr() ) .into() , ( 1885_() ) .into() , ( c"void utf32_to_utf16_out_error(const std::codecvt<InternT, char, mbstate_t> &, utf16_endianess) [InternT = wchar_t]" .as_ptr() ) .into() , ( c"out[t.expected_out_next] == 0" .as_ptr() ) .into() , ] ) )
                    });
                    unimplemented!();
                }
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn test_utf16_utf32_cvt_50(cvt: *const (), mut endianess: utf16_endianess) {
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_ok_44(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_partial_45(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_error_46(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_ok_47(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_partial_48(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_error_49(_cvt, _endianess)
    });
}
pub unsafe fn test_utf8_utf32_codecvts_51() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf8_utf32_cvt_32(&cvt) });
}
pub unsafe fn test_utf8_utf16_codecvts_52() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf8_utf16_cvt_41(&cvt) });
}
pub unsafe fn test_utf8_ucs2_codecvts_53() {}
pub unsafe fn test_utf16_utf32_codecvts_54() {
    let mut cvt3: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_50(&cvt3, utf16_endianess_utf16_big_endian) });
    let mut cvt4: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_50(&cvt4, utf16_endianess_utf16_little_endian) });
}
pub unsafe fn test_utf16_ucs2_codecvts_55() {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_utf8_utf32_codecvts_51() });
    (unsafe { test_utf8_utf16_codecvts_52() });
    (unsafe { test_utf8_ucs2_codecvts_53() });
    (unsafe { test_utf16_utf32_codecvts_54() });
    (unsafe { test_utf16_ucs2_codecvts_55() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_56() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_57(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_58(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_59(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_60(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_61(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_60(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_61(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_60((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_61(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_62(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_60(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_61(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_63(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_64(
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
pub unsafe fn run_tests_wrapped_locale_65(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_24 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_66(
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
        if ((self.pid_) == (unsafe { getpid_56() })) {
            (unsafe { semctl_58(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_67(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_67(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_68(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_69));
}
static mut tmp_69: std::cell::LazyCell<::libc::tm> =
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
