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
            unimplemented!();
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
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__3))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)))
        {
            unimplemented!();
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
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)) = 0_();
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__6)).postfix_dec()
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
            if operator_lt_7(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(
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
            operator_eq_8(
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
pub unsafe fn operator_eq_8(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_7(
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
    pid_: (),
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
pub unsafe fn array_size_9(_a0: *const [(); 11]) -> () {
    return 11_();
}
pub unsafe fn array_size_10(_a0: *const [(); 5]) -> () {
    return 5_();
}
pub unsafe fn array_size_11(_a0: *const [(); 6]) -> () {
    return 6_();
}
pub unsafe fn array_size_12(_a0: *const [(); 7]) -> () {
    return 7_();
}
pub unsafe fn array_size_13(_a0: *const [(); 4]) -> () {
    return 4_();
}
pub unsafe fn array_size_14(_a0: *const [(); 12]) -> () {
    return 12_();
}
pub unsafe fn array_size_15(_a0: *const [(); 10]) -> () {
    return 10_();
}
pub unsafe fn array_size_16(_a0: *const [(); 8]) -> () {
    return 8_();
}
pub unsafe fn array_size_17(_a0: *const [(); 6]) -> () {
    return 6_();
}
pub unsafe fn array_size_18(_a0: *const [(); 5]) -> () {
    return 5_();
}
pub unsafe fn array_size_19(_a0: *const [(); 11]) -> () {
    return 11_();
}
pub unsafe fn array_size_20(_a0: *const [(); 4]) -> () {
    return 4_();
}
pub unsafe fn array_size_21(_a0: *const [(); 6]) -> () {
    return 6_();
}
pub unsafe fn utf8_to_utf32_in_ok_22(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_18(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_18(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf8_to_utf32_in_partial_23(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf8_to_utf32_in_error_24(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                unimplemented!();
                unimplemented!();
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf8_to_utf32_in_25(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_ok_22(cvt) });
    (unsafe { utf8_to_utf32_in_partial_23(cvt) });
    (unsafe { utf8_to_utf32_in_error_24(cvt) });
}
pub unsafe fn utf32_to_utf8_out_ok_26(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf8_out_partial_27(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf8_out_error_28(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf32_to_utf8_out_29(cvt: *const ()) {
    (unsafe { utf32_to_utf8_out_ok_26(cvt) });
    (unsafe { utf32_to_utf8_out_partial_27(cvt) });
    (unsafe { utf32_to_utf8_out_error_28(cvt) });
}
pub unsafe fn test_utf8_utf32_cvt_30(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_25(cvt) });
    (unsafe { utf32_to_utf8_out_29(cvt) });
}
pub unsafe fn utf8_to_utf16_in_ok_31(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_18(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_18(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_21(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_21(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf8_to_utf16_in_partial_32(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_18(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_18(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf8_to_utf16_in_error_33(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_19(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_18(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_18(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                unimplemented!();
                unimplemented!();
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf8_to_utf16_in_34(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_ok_31(cvt) });
    (unsafe { utf8_to_utf16_in_partial_32(cvt) });
    (unsafe { utf8_to_utf16_in_error_33(cvt) });
}
pub unsafe fn utf16_to_utf8_out_ok_35(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_21(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf16_to_utf8_out_partial_36(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_21(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf16_to_utf8_out_error_37(cvt: *const ()) {
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
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_())) {
            unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_21(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf16_to_utf8_out_38(cvt: *const ()) {
    (unsafe { utf16_to_utf8_out_ok_35(cvt) });
    (unsafe { utf16_to_utf8_out_partial_36(cvt) });
    (unsafe { utf16_to_utf8_out_error_37(cvt) });
}
pub unsafe fn test_utf8_utf16_cvt_39(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_34(cvt) });
    (unsafe { utf16_to_utf8_out_38(cvt) });
}
pub type utf16_endianess = ();
pub const utf16_endianess_utf16_big_endian: utf16_endianess = 0;
pub const utf16_endianess_utf16_little_endian: utf16_endianess = 1;
pub unsafe fn utf16_to_bytes_40(
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
pub unsafe fn utf16_to_bytes_41(
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
pub unsafe fn utf16_to_utf32_in_ok_42(cvt: *const (), mut endianess: utf16_endianess) {
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
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
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
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                unimplemented!();
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
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_18(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_18(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf16_to_utf32_in_partial_43(cvt: *const (), mut endianess: utf16_endianess) {
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
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
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
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn utf16_to_utf32_in_error_44(cvt: *const (), mut endianess: utf16_endianess) {
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
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut old_char: () = input[(t.replace_pos) as usize];
        input[(t.replace_pos) as usize] = t.replace_char;
        (unsafe {
            let _f: *mut () = unimplemented!();
            let _l: *mut () = unimplemented!();
            utf16_to_bytes_41(_f, _l, unimplemented!(), endianess)
        });
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) == (t.expected_in_next)) {
                unimplemented!();
                unimplemented!();
            }
        }
        input[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn utf32_to_utf16_out_ok_45(cvt: *const (), mut endianess: utf16_endianess) {
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
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
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
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.out_size) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf16_out_partial_46(cvt: *const (), mut endianess: utf16_endianess) {
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
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
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
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf16_out_error_47(cvt: *const (), mut endianess: utf16_endianess) {
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
        utf16_to_bytes_40(_f, _l, unimplemented!(), endianess)
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
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut old_char: () = in_[(t.replace_pos) as usize];
        in_[(t.replace_pos) as usize] = t.replace_char;
        let mut state: () = unimplemented!();
        let mut in_next: *const () = std::ptr::null();
        let mut out_next: *mut () = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res) == (unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next) as usize]) == (0_())) {
                    unimplemented!();
                    unimplemented!();
                }
            }
        }
        in_[(t.replace_pos) as usize] = old_char;
    }
}
pub unsafe fn test_utf16_utf32_cvt_48(cvt: *const (), mut endianess: utf16_endianess) {
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_ok_42(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_partial_43(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_error_44(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_ok_45(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_partial_46(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_error_47(_cvt, _endianess)
    });
}
pub unsafe fn test_utf8_utf32_codecvts_49() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf8_utf32_cvt_30(&cvt) });
}
pub unsafe fn test_utf8_utf16_codecvts_50() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf8_utf16_cvt_39(&cvt) });
}
pub unsafe fn test_utf8_ucs2_codecvts_51() {}
pub unsafe fn test_utf16_utf32_codecvts_52() {
    let mut cvt3: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_48(&cvt3, utf16_endianess_utf16_big_endian) });
    let mut cvt4: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_48(&cvt4, utf16_endianess_utf16_little_endian) });
}
pub unsafe fn test_utf16_ucs2_codecvts_53() {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_utf8_utf32_codecvts_49() });
    (unsafe { test_utf8_utf16_codecvts_50() });
    (unsafe { test_utf8_ucs2_codecvts_51() });
    (unsafe { test_utf16_utf32_codecvts_52() });
    (unsafe { test_utf16_ucs2_codecvts_53() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: unimplemented!(),
        };
        this.pid_ = unimplemented!();
        this.sem_set_ = unimplemented!();
        if ((this.sem_set_) == (-1_())) {
            unimplemented!();
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
        this
    }
}
pub unsafe fn set_memory_limits_54(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) * (1048576_()));
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn set_file_limit_55(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_56(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = unimplemented!();
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
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_57(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn run_tests_wrapped_env_58(
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
        unimplemented!();
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut _M_count_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut next_id__6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union __gnu_test_semun {
    pub val: (),
    pub buf: *mut (),
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
        if ((self.pid_) == (unimplemented!())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
    }
}
pub unsafe fn test_tm_59(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_60)).clone();
}
static mut tmp_60: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
