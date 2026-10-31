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
    _M_size: i32,
    _M_tests: [Option<unsafe fn()>; 15],
}
impl __gnu_test_func_callback {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            _M_size: 0,
            _M_tests: [None; 15],
        };
        this
    }
    pub unsafe fn size(&self) -> i32 {
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
    pub unsafe fn count() -> u32 {
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)) = 0_u32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = 0_u32;
    }
    pub unsafe fn throw_on(mut count: u32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub unsafe fn count() -> u32 {
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)) = 0_u32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)) = 0_u32;
    }
    pub unsafe fn throw_on(mut count: u32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> u32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)) = 0_u32;
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_copy_tracker {
    id_: i32,
    throw_on_copy_: bool,
}
impl __gnu_test_copy_tracker {
    pub unsafe fn new(mut id: Option<i32>, mut throw_on_copy: Option<bool>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__6)).postfix_dec()
        });
        let mut throw_on_copy: bool = throw_on_copy.unwrap_or_else(|| unsafe { false });
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
                    (unsafe { __gnu_test_copy_constructor::count() }).wrapping_add(1_u32),
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
                    (unsafe { __gnu_test_assignment_operator::count() }).wrapping_add(1_u32),
                )
            });
        }
        (unsafe { __gnu_test_assignment_operator::mark_call() });
        return &mut (*(self as *mut __gnu_test_copy_tracker));
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { __gnu_test_destructor::mark_call() });
    }
    pub unsafe fn id(&self) -> i32 {
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
) -> bool {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_7(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> bool {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        < (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_semaphore {
    sem_set_: i32,
    pid_: (),
}
impl __gnu_test_semaphore {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_ok {
    pub in_size: usize,
    pub out_size: usize,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_partial {
    pub in_size: usize,
    pub out_size: usize,
    pub expected_in_next: usize,
    pub expected_out_next: usize,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_unsigned_char_ {
    pub in_size: usize,
    pub out_size: usize,
    pub expected_in_next: usize,
    pub expected_out_next: usize,
    pub replace_char: u8,
    pub replace_pos: usize,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_char16_t_ {
    pub in_size: usize,
    pub out_size: usize,
    pub expected_in_next: usize,
    pub expected_out_next: usize,
    pub replace_char: u16,
    pub replace_pos: usize,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_offsets_error_char32_t_ {
    pub in_size: usize,
    pub out_size: usize,
    pub expected_in_next: usize,
    pub expected_out_next: usize,
    pub replace_char: u32,
    pub replace_pos: usize,
}
pub unsafe fn array_size_9(_a0: *const [u8; 11]) -> usize {
    return (11_u64 as usize);
}
pub unsafe fn array_size_10(_a0: *const [u32; 5]) -> usize {
    return (5_u64 as usize);
}
pub unsafe fn array_size_11(_a0: *const [u16; 6]) -> usize {
    return (6_u64 as usize);
}
pub unsafe fn array_size_12(_a0: *const [u8; 7]) -> usize {
    return (7_u64 as usize);
}
pub unsafe fn array_size_13(_a0: *const [u16; 4]) -> usize {
    return (4_u64 as usize);
}
pub unsafe fn array_size_14(_a0: *const [libc::c_char; 12]) -> usize {
    return (12_u64 as usize);
}
pub unsafe fn array_size_15(_a0: *const [libc::c_char; 10]) -> usize {
    return (10_u64 as usize);
}
pub unsafe fn array_size_16(_a0: *const [libc::c_char; 8]) -> usize {
    return (8_u64 as usize);
}
pub unsafe fn array_size_17(_a0: *const [libc::c_char; 6]) -> usize {
    return (6_u64 as usize);
}
pub unsafe fn array_size_18(_a0: *const [libc::c_char; 11]) -> usize {
    return (11_u64 as usize);
}
pub unsafe fn array_size_19(_a0: *const [u32; 4]) -> usize {
    return (4_u64 as usize);
}
pub unsafe fn array_size_20(_a0: *const [u16; 5]) -> usize {
    return (5_u64 as usize);
}
pub unsafe fn array_size_21(_a0: *const [u32; 6]) -> usize {
    return (6_u64 as usize);
}
pub unsafe fn array_size_22(_a0: *const [libc::c_char; 7]) -> usize {
    return (7_u64 as usize);
}
pub unsafe fn array_size_23(_a0: *const [u16; 3]) -> usize {
    return (3_u64 as usize);
}
pub unsafe fn utf8_to_utf32_in_ok_24(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u32; 5] = [0_u32; 5];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 10_usize,
            out_size: 4_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u32; 4] = [0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u32; 5] = [0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_10(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_10(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf8_to_utf32_in_partial_25(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u32; 5] = [0_u32; 5];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((16_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [u32; 4] = [0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf8_to_utf32_in_error_26(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [u32; 5] = [
        98 as u32,
        1096 as u32,
        55040 as u32,
        1092266 as u32,
        0 as u32,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u32; 5] = [0_u32; 5];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 255_u8,
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 255_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 170_u8,
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 170_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 170_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 170_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 2_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 255_u8,
            replace_pos: 2_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 5_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 5_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 9_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 9_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 160_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 172_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 176_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 188_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 192_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 193_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 4_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 224_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 240_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 245_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 154_u8,
            replace_pos: 7_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((38_i64) as isize),
    ) {
        let mut t: test_offsets_error_unsigned_char_ = (*t);
        let mut out: [u32; 4] = [0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: libc::c_char = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = (t.replace_char as libc::c_char);
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn utf8_to_utf32_in_27(cvt: *const ()) {
    (unsafe { utf8_to_utf32_in_ok_24(cvt) });
    (unsafe { utf8_to_utf32_in_partial_25(cvt) });
    (unsafe { utf8_to_utf32_in_error_26(cvt) });
}
pub unsafe fn utf32_to_utf8_out_ok_28(cvt: *const ()) {
    let input: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u32; 5] = [0_u32; 5];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
        test_offsets_ok {
            in_size: 4_usize,
            out_size: 10_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_10(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf8_out_partial_29(cvt: *const ()) {
    let input: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u32; 5] = [0_u32; 5];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((10_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_10(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf8_out_error_30(cvt: *const ()) {
    let input: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u32; 5] = [0_u32; 5];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (4_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_error_char32_t_; 8] = [
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u32,
            replace_pos: 0_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_u32,
            replace_pos: 1_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_u32,
            replace_pos: 2_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_u32,
            replace_pos: 3_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 1114112_u32,
            replace_pos: 0_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 1114112_u32,
            replace_pos: 1_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 1114112_u32,
            replace_pos: 2_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 1114112_u32,
            replace_pos: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut t: test_offsets_error_char32_t_ = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_10(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u32 = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = t.replace_char;
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        in_[(t.replace_pos)] = old_char;
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
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u16; 6] = [0_u16; 6];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 10_usize,
            out_size: 5_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u16; 5] = [0_u16, 0_u16, 0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u16; 6] = [0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_11(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_11(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf8_to_utf16_in_ok_34(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u32; 6] = [0_u32; 6];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as u32);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 10_usize,
            out_size: 5_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u32; 5] = [0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_10(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_10(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u32; 6] = [0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_21(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_21(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf8_to_utf16_in_partial_35(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u16; 6] = [0_u16; 6];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_partial; 20] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((20_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [u16; 5] = [0_u16, 0_u16, 0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf8_to_utf16_in_partial_36(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u32; 6] = [0_u32; 6];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as u32);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_partial; 20] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((20_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [u32; 5] = [0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_10(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_10(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf8_to_utf16_in_error_37(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        55040 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u16; 6] = [0_u16; 6];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_usize,
            out_size: 5_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 255_u8,
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 255_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_usize,
            out_size: 5_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 170_u8,
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 170_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 170_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 170_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 2_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 255_u8,
            replace_pos: 2_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 5_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 5_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 9_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 9_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 160_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 172_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 176_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 188_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 192_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 193_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 224_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 240_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 245_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 154_u8,
            replace_pos: 7_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((38_i64) as isize),
    ) {
        let mut t: test_offsets_error_unsigned_char_ = (*t);
        let mut out: [u16; 5] = [0_u16, 0_u16, 0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: libc::c_char = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = (t.replace_char as libc::c_char);
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn utf8_to_utf16_in_error_38(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        55040 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u32; 6] = [0_u32; 6];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as u32);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_error_unsigned_char_; 38] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_usize,
            out_size: 5_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 255_u8,
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 255_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_usize,
            out_size: 5_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 170_u8,
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 170_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 170_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 170_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 2_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 255_u8,
            replace_pos: 2_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 5_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 5_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 9_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 9_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 160_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 172_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 176_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 188_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 192_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 193_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 224_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 240_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 245_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 154_u8,
            replace_pos: 7_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((38_i64) as isize),
    ) {
        let mut t: test_offsets_error_unsigned_char_ = (*t);
        let mut out: [u32; 5] = [0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_10(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: libc::c_char = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = (t.replace_char as libc::c_char);
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_10(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn utf8_to_utf16_in_39(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_ok_33(cvt) });
    (unsafe { utf8_to_utf16_in_partial_35(cvt) });
    (unsafe { utf8_to_utf16_in_error_37(cvt) });
}
pub unsafe fn utf8_to_utf16_in_40(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_ok_34(cvt) });
    (unsafe { utf8_to_utf16_in_partial_36(cvt) });
    (unsafe { utf8_to_utf16_in_error_38(cvt) });
}
pub unsafe fn utf16_to_utf8_out_ok_41(cvt: *const ()) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u16; 6] = [0_u16; 6];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
        test_offsets_ok {
            in_size: 5_usize,
            out_size: 10_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_11(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn utf16_to_utf8_out_ok_42(cvt: *const ()) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u32; 6] = [0_u32; 6];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as u32);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
        test_offsets_ok {
            in_size: 5_usize,
            out_size: 10_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_21(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn utf16_to_utf8_out_partial_43(cvt: *const ()) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u16; 6] = [0_u16; 6];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_partial; 15] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((15_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_11(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn utf16_to_utf8_out_partial_44(cvt: *const ()) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u32; 6] = [0_u32; 6];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as u32);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_partial; 15] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((15_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_21(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn utf16_to_utf8_out_error_45(cvt: *const ()) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u16; 6] = [0_u16; 6];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_error_char16_t_; 17] = [
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56319_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56320_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 57343_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 55296_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56320_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 57343_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 55296_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56319_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 57343_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56320_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 55296_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56319_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (122 as u16),
            replace_pos: 4_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((17_i64) as isize),
    ) {
        let mut t: test_offsets_error_char16_t_ = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_11(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u16 = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = t.replace_char;
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn utf16_to_utf8_out_error_46(cvt: *const ()) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u32; 6] = [0_u32; 6];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as u32);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_error_char32_t_; 17] = [
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u32,
            replace_pos: 0_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56319_u32,
            replace_pos: 0_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56320_u32,
            replace_pos: 0_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 57343_u32,
            replace_pos: 0_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 55296_u32,
            replace_pos: 1_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_u32,
            replace_pos: 1_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56320_u32,
            replace_pos: 1_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 57343_u32,
            replace_pos: 1_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 55296_u32,
            replace_pos: 2_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56319_u32,
            replace_pos: 2_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_u32,
            replace_pos: 2_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 57343_u32,
            replace_pos: 2_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56320_u32,
            replace_pos: 3_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_u32,
            replace_pos: 3_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 55296_u32,
            replace_pos: 4_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56319_u32,
            replace_pos: 4_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: ((122 as u16) as u32),
            replace_pos: 4_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((17_i64) as isize),
    ) {
        let mut t: test_offsets_error_char32_t_ = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_21(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u32 = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = t.replace_char;
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn utf16_to_utf8_out_47(cvt: *const ()) {
    (unsafe { utf16_to_utf8_out_ok_41(cvt) });
    (unsafe { utf16_to_utf8_out_partial_43(cvt) });
    (unsafe { utf16_to_utf8_out_error_45(cvt) });
}
pub unsafe fn utf16_to_utf8_out_48(cvt: *const ()) {
    (unsafe { utf16_to_utf8_out_ok_42(cvt) });
    (unsafe { utf16_to_utf8_out_partial_44(cvt) });
    (unsafe { utf16_to_utf8_out_error_46(cvt) });
}
pub unsafe fn test_utf8_utf16_cvt_49(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_39(cvt) });
    (unsafe { utf16_to_utf8_out_47(cvt) });
}
pub unsafe fn test_utf8_utf16_cvt_50(cvt: *const ()) {
    (unsafe { utf8_to_utf16_in_40(cvt) });
    (unsafe { utf16_to_utf8_out_48(cvt) });
}
pub unsafe fn utf8_to_ucs2_in_ok_51(cvt: *const ()) {
    let input: [u8; 7] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\0");
    let expected: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    const _: () = assert!(true, "array_size (input) == 7");;
    const _: () = assert!(true, "array_size (expected) == 4");;
    let mut in_: [libc::c_char; 7] = [(0 as libc::c_char); 7];
    let mut exp: [u16; 4] = [0_u16; 4];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (6_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (3_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_ok; 4] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u16; 3] = [0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_22(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_23(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_23(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u16; 4] = [0_u16, 0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_22(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_13(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_13(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf8_to_ucs2_in_partial_52(cvt: *const ()) {
    let input: [u8; 7] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\0");
    let expected: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    const _: () = assert!(true, "array_size (input) == 7");;
    const _: () = assert!(true, "array_size (expected) == 4");;
    let mut in_: [libc::c_char; 7] = [(0 as libc::c_char); 7];
    let mut exp: [u16; 4] = [0_u16; 4];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (6_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (3_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_partial; 9] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((9_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [u16; 3] = [0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_22(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_23(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_23(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf8_to_ucs2_in_error_53(cvt: *const ()) {
    let input: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xed\x9c\x80\xf4\x8a\xaa\xaa\0");
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        55040 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 11");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    let mut exp: [u16; 6] = [0_u16; 6];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_error_unsigned_char_; 46] = [
        test_offsets_error_unsigned_char_ {
            in_size: 1_usize,
            out_size: 5_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 255_u8,
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 255_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 1_usize,
            out_size: 5_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 170_u8,
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 170_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 170_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 170_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 2_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 255_u8,
            replace_pos: 2_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 5_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 5_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 9_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 9_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 5_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 255_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('z' as libc::c_char) as u8),
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 255_u8,
            replace_pos: 8_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 160_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 172_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 176_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 188_u8,
            replace_pos: 4_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 192_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 193_u8,
            replace_pos: 1_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 6_usize,
            out_size: 5_usize,
            expected_in_next: 3_usize,
            expected_out_next: 2_usize,
            replace_char: 224_u8,
            replace_pos: 3_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 240_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 245_u8,
            replace_pos: 6_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 154_u8,
            replace_pos: 7_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('b' as libc::c_char) as u8),
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('b' as libc::c_char) as u8),
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 7_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('b' as libc::c_char) as u8),
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('b' as libc::c_char) as u8),
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('b' as libc::c_char) as u8),
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 7_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('b' as libc::c_char) as u8),
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('b' as libc::c_char) as u8),
            replace_pos: 0_usize,
        },
        test_offsets_error_unsigned_char_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (('b' as libc::c_char) as u8),
            replace_pos: 0_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((46_i64) as isize),
    ) {
        let mut t: test_offsets_error_unsigned_char_ = (*t);
        let mut out: [u16; 5] = [0_u16, 0_u16, 0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_18(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: libc::c_char = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = (t.replace_char as libc::c_char);
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn utf8_to_ucs2_in_54(cvt: *const ()) {
    (unsafe { utf8_to_ucs2_in_ok_51(cvt) });
    (unsafe { utf8_to_ucs2_in_partial_52(cvt) });
    (unsafe { utf8_to_ucs2_in_error_53(cvt) });
}
pub unsafe fn ucs2_to_utf8_out_ok_55(cvt: *const ()) {
    let input: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    let expected: [u8; 7] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 4");;
    const _: () = assert!(true, "array_size (expected) == 7");;
    let mut in_: [u16; 4] = [0_u16; 4];
    let mut exp: [libc::c_char; 7] = [(0 as libc::c_char); 7];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (3_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (6_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_ok; 4] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [libc::c_char; 6] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_13(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_17(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_17(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn ucs2_to_utf8_out_partial_56(cvt: *const ()) {
    let input: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    let expected: [u8; 7] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 4");;
    const _: () = assert!(true, "array_size (expected) == 7");;
    let mut in_: [u16; 4] = [0_u16; 4];
    let mut exp: [libc::c_char; 7] = [(0 as libc::c_char); 7];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (3_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (6_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_partial; 6] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 1_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((6_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [libc::c_char; 6] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_13(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_17(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_17(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn ucs2_to_utf8_out_error_57(cvt: *const ()) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u8; 11] = std::mem::transmute(*b"b\xd1\x88\xea\xaa\xaa\xf4\x8a\xaa\xaa\0");
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 11");;
    let mut in_: [u16; 6] = [0_u16; 6];
    let mut exp: [libc::c_char; 11] = [(0 as libc::c_char); 11];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = ((*__first.iter_deref()) as libc::c_char);
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (5_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (10_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut offsets: [test_offsets_error_char16_t_; 25] = [
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56319_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56320_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 57343_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 55296_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 56320_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 1_usize,
            expected_out_next: 1_usize,
            replace_char: 57343_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 55296_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56319_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 2_usize,
            expected_out_next: 3_usize,
            replace_char: 57343_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56320_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 55296_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56319_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (122 as u16),
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 4_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((25_i64) as isize),
    ) {
        let mut t: test_offsets_error_char16_t_ = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_11(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u16 = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = t.replace_char;
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn ucs2_to_utf8_out_58(cvt: *const ()) {
    (unsafe { ucs2_to_utf8_out_ok_55(cvt) });
    (unsafe { ucs2_to_utf8_out_partial_56(cvt) });
    (unsafe { ucs2_to_utf8_out_error_57(cvt) });
}
pub unsafe fn test_utf8_ucs2_cvt_59(cvt: *const ()) {
    (unsafe { utf8_to_ucs2_in_54(cvt) });
    (unsafe { ucs2_to_utf8_out_58(cvt) });
}
pub type utf16_endianess = u32;
pub const utf16_endianess_utf16_big_endian: utf16_endianess = 0;
pub const utf16_endianess_utf16_little_endian: utf16_endianess = 1;
pub unsafe fn utf16_to_bytes_60(
    mut f: *const u16,
    mut l: *const u16,
    mut o: *mut libc::c_char,
    mut e: utf16_endianess,
) -> *mut libc::c_char {
    if ((e as i32) == (utf16_endianess_utf16_big_endian as i32)) {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = (((((*f) as i32) >> (8)) & (255)) as libc::c_char);
            (*o.postfix_inc()) = ((((*f) as i32) & (255)) as libc::c_char);
            f.prefix_inc();
        }
    } else {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = ((((*f) as i32) & (255)) as libc::c_char);
            (*o.postfix_inc()) = (((((*f) as i32) >> (8)) & (255)) as libc::c_char);
            f.prefix_inc();
        }
    }
    return o;
}
pub unsafe fn utf16_to_bytes_61(
    mut f: *mut u16,
    mut l: *mut u16,
    mut o: *mut libc::c_char,
    mut e: utf16_endianess,
) -> *mut libc::c_char {
    if ((e as i32) == (utf16_endianess_utf16_big_endian as i32)) {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = (((((*f) as i32) >> (8)) & (255)) as libc::c_char);
            (*o.postfix_inc()) = ((((*f) as i32) & (255)) as libc::c_char);
            f.prefix_inc();
        }
    } else {
        'loop_: while ((f) != (l)) {
            (*o.postfix_inc()) = ((((*f) as i32) & (255)) as libc::c_char);
            (*o.postfix_inc()) = (((((*f) as i32) >> (8)) & (255)) as libc::c_char);
            f.prefix_inc();
        }
    }
    return o;
}
pub unsafe fn utf16_to_utf32_in_ok_62(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [libc::c_char; 12] = [(0 as libc::c_char); 12];
    let mut exp: [u32; 5] = [0_u32; 5];
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = input.as_ptr().add(input.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 4_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
        test_offsets_ok {
            in_size: 10_usize,
            out_size: 4_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u32; 4] = [0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u32; 5] = [0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_10(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_10(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf16_to_utf32_in_partial_63(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut in_: [libc::c_char; 12] = [(0 as libc::c_char); 12];
    let mut exp: [u32; 5] = [0_u32; 5];
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = input.as_ptr().add(input.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut offsets: [test_offsets_partial; 16] = [
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 1_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 1_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 2_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 10_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 7_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 8_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
        test_offsets_partial {
            in_size: 9_usize,
            out_size: 3_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((16_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [u32; 4] = [0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf16_to_utf32_in_error_64(cvt: *const (), mut endianess: utf16_endianess) {
    let mut input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 5");;
    let mut exp: [u32; 5] = [0_u32; 5];
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut offsets: [test_offsets_error_char16_t_; 17] = [
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56319_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56320_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 57343_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 55296_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 56320_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 57343_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 55296_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 56319_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 56320_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 57343_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 57343_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 55296_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 56319_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (122 as u16),
            replace_pos: 4_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((17_i64) as isize),
    ) {
        let mut t: test_offsets_error_char16_t_ = (*t);
        let mut in_: [libc::c_char; 12] = [(0 as libc::c_char); 12];
        let mut out: [u32; 4] = [0_u32, 0_u32, 0_u32, 0_u32];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_19(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u16 = input[(t.replace_pos)];
        input[(t.replace_pos)] = t.replace_char;
        (unsafe {
            let _f: *mut u16 = unimplemented!();
            let _l: *mut u16 = (input.as_ptr().add(input.len()) as *mut u16);
            utf16_to_bytes_61(_f, _l, unimplemented!(), endianess)
        });
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u32 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_19(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)]) == (0_u32)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
        input[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn utf32_to_utf16_out_ok_65(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [u32; 5] = [0_u32; 5];
    let mut exp: [libc::c_char; 12] = [(0 as libc::c_char); 12];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = expected.as_ptr().add(expected.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_ok; 5] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 4_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
        test_offsets_ok {
            in_size: 4_usize,
            out_size: 10_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((5_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_10(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf16_out_partial_66(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [u32; 5] = [0_u32; 5];
    let mut exp: [libc::c_char; 12] = [(0 as libc::c_char); 12];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = expected.as_ptr().add(expected.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_partial; 10] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 1_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 3_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 6_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 7_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((10_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_10(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn utf32_to_utf16_out_error_67(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u32; 5] = [
        98 as u32,
        1096 as u32,
        43690 as u32,
        1092266 as u32,
        0 as u32,
    ];
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 5");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [u32; 5] = [0_u32; 5];
    let mut exp: [libc::c_char; 12] = [(0 as libc::c_char); 12];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = expected.as_ptr().add(expected.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_error_char32_t_; 8] = [
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u32,
            replace_pos: 0_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
            replace_char: 56319_u32,
            replace_pos: 1_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
            replace_char: 56320_u32,
            replace_pos: 2_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_u32,
            replace_pos: 3_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 1114112_u32,
            replace_pos: 0_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
            replace_char: 1114112_u32,
            replace_pos: 1_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
            replace_char: 1114112_u32,
            replace_pos: 2_usize,
        },
        test_offsets_error_char32_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 1114112_u32,
            replace_pos: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut t: test_offsets_error_char32_t_ = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_10(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u32 = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = t.replace_char;
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u32 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn test_utf16_utf32_cvt_68(cvt: *const (), mut endianess: utf16_endianess) {
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_ok_62(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_partial_63(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_utf32_in_error_64(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_ok_65(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_partial_66(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf32_to_utf16_out_error_67(_cvt, _endianess)
    });
}
pub unsafe fn utf16_to_ucs2_in_ok_69(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    let expected: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    const _: () = assert!(true, "array_size (input) == 4");;
    const _: () = assert!(true, "array_size (expected) == 4");;
    let mut in_: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    let mut exp: [u16; 4] = [0_u16; 4];
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = input.as_ptr().add(input.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut offsets: [test_offsets_ok; 4] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 1_usize,
        },
        test_offsets_ok {
            in_size: 4_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 6_usize,
            out_size: 3_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u16; 3] = [0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_16(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_23(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_23(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [u16; 4] = [0_u16, 0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_16(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_13(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_13(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf16_to_ucs2_in_partial_70(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    let expected: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    const _: () = assert!(true, "array_size (input) == 4");;
    const _: () = assert!(true, "array_size (expected) == 4");;
    let mut in_: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    let mut exp: [u16; 4] = [0_u16; 4];
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = input.as_ptr().add(input.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut offsets: [test_offsets_partial; 9] = [
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 1_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 4_usize,
            out_size: 1_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 2_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 1_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
        },
        test_offsets_partial {
            in_size: 6_usize,
            out_size: 2_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 3_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 5_usize,
            out_size: 2_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((9_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [u16; 3] = [0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_16(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_23(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_23(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
    }
}
pub unsafe fn utf16_to_ucs2_in_error_71(cvt: *const (), mut endianess: utf16_endianess) {
    let mut input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut exp: [u16; 6] = [0_u16; 6];
    {
        let mut __first = unimplemented!();
        let __last = expected.as_ptr().add(expected.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    let mut offsets: [test_offsets_error_char16_t_; 23] = [
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56319_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56320_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 57343_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 55296_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 56319_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 56320_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 2_usize,
            expected_out_next: 1_usize,
            replace_char: 57343_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 55296_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 56319_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 56320_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 6_usize,
            out_size: 3_usize,
            expected_in_next: 4_usize,
            expected_out_next: 2_usize,
            replace_char: 57343_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 56320_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 57343_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 55296_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: 56319_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (122 as u16),
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 8_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 9_usize,
            out_size: 5_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 10_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 8_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 9_usize,
            out_size: 4_usize,
            expected_in_next: 6_usize,
            expected_out_next: 3_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((23_i64) as isize),
    ) {
        let mut t: test_offsets_error_char16_t_ = (*t);
        let mut in_: [libc::c_char; 12] = [(0 as libc::c_char); 12];
        let mut out: [u16; 5] = [0_u16, 0_u16, 0_u16, 0_u16, 0_u16];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_14(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_20(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u16 = input[(t.replace_pos)];
        input[(t.replace_pos)] = t.replace_char;
        (unsafe {
            let _f: *mut u16 = unimplemented!();
            let _l: *mut u16 = (input.as_ptr().add(input.len()) as *mut u16);
            utf16_to_bytes_61(_f, _l, unimplemented!(), endianess)
        });
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const libc::c_char = std::ptr::null();
        let mut out_next: *mut u16 = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_20(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        state = unsafe { std::mem::zeroed() };
        let mut len: i32 = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len) >= (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((len as usize) == (t.expected_in_next)) {
                unimplemented!();
                std::process::abort();
            }
        }
        input[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn ucs2_to_utf16_out_ok_72(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    let expected: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    const _: () = assert!(true, "array_size (input) == 4");;
    const _: () = assert!(true, "array_size (expected) == 4");;
    let mut in_: [u16; 4] = [0_u16; 4];
    let mut exp: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = expected.as_ptr().add(expected.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_ok; 4] = [
        test_offsets_ok {
            in_size: 0_usize,
            out_size: 0_usize,
        },
        test_offsets_ok {
            in_size: 1_usize,
            out_size: 2_usize,
        },
        test_offsets_ok {
            in_size: 2_usize,
            out_size: 4_usize,
        },
        test_offsets_ok {
            in_size: 3_usize,
            out_size: 6_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let mut t: test_offsets_ok = (*t);
        let mut out: [libc::c_char; 6] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_13(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_17(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next) == ((in_.as_mut_ptr().offset((t.in_size) as isize)).cast_const())) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.out_size) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.out_size) < (unsafe { array_size_17(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.out_size)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn ucs2_to_utf16_out_partial_73(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    let expected: [u16; 4] = [98 as u16, 1096 as u16, 43690 as u16, 0 as u16];
    const _: () = assert!(true, "array_size (input) == 4");;
    const _: () = assert!(true, "array_size (expected) == 4");;
    let mut in_: [u16; 4] = [0_u16; 4];
    let mut exp: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = expected.as_ptr().add(expected.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_partial; 6] = [
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 0_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 1_usize,
            out_size: 1_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 2_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 2_usize,
            out_size: 3_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 4_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
        },
        test_offsets_partial {
            in_size: 3_usize,
            out_size: 5_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((6_i64) as isize),
    ) {
        let mut t: test_offsets_partial = (*t);
        let mut out: [libc::c_char; 6] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_13(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_17(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_17(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
    }
}
pub unsafe fn ucs2_to_utf16_out_error_74(cvt: *const (), mut endianess: utf16_endianess) {
    let input: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    let expected: [u16; 6] = [
        98 as u16,
        1096 as u16,
        43690 as u16,
        56298 as u16,
        57002 as u16,
        0 as u16,
    ];
    const _: () = assert!(true, "array_size (input) == 6");;
    const _: () = assert!(true, "array_size (expected) == 6");;
    let mut in_: [u16; 6] = [0_u16; 6];
    let mut exp: [libc::c_char; 12] = [(0 as libc::c_char); 12];
    {
        let mut __first = unimplemented!();
        let __last = input.as_ptr().add(input.len());
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    (unsafe {
        let _f: *const u16 = unimplemented!();
        let _l: *const u16 = expected.as_ptr().add(expected.len());
        utf16_to_bytes_60(_f, _l, unimplemented!(), endianess)
    });
    let mut offsets: [test_offsets_error_char16_t_; 23] = [
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 55296_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56319_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 56320_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 0_usize,
            expected_out_next: 0_usize,
            replace_char: 57343_u16,
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
            replace_char: 55296_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
            replace_char: 56319_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
            replace_char: 56320_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 1_usize,
            expected_out_next: 2_usize,
            replace_char: 57343_u16,
            replace_pos: 1_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
            replace_char: 55296_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
            replace_char: 56319_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
            replace_char: 56320_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 3_usize,
            out_size: 6_usize,
            expected_in_next: 2_usize,
            expected_out_next: 4_usize,
            replace_char: 57343_u16,
            replace_pos: 2_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56320_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 57343_u16,
            replace_pos: 3_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 55296_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: 56319_u16,
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (122 as u16),
            replace_pos: 4_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 5_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 4_usize,
            out_size: 10_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 4_usize,
            out_size: 8_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
        test_offsets_error_char16_t_ {
            in_size: 4_usize,
            out_size: 9_usize,
            expected_in_next: 3_usize,
            expected_out_next: 6_usize,
            replace_char: (98 as u16),
            replace_pos: 0_usize,
        },
    ];
    'loop_: for t in IterRange::new(
        offsets.as_mut_ptr(),
        offsets.as_mut_ptr().offset((23_i64) as isize),
    ) {
        let mut t: test_offsets_error_char16_t_ = (*t);
        let mut out: [libc::c_char; 10] = [
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
            (0 as libc::c_char),
        ];
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.in_size) <= (unsafe { array_size_11(&in_) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.out_size) <= (unsafe { array_size_15(&out) })) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_in_next) <= (t.in_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((t.expected_out_next) <= (t.out_size)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut old_char: u16 = in_[(t.replace_pos)];
        in_[(t.replace_pos)] = t.replace_char;
        let mut state: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut in_next: *const u16 = std::ptr::null();
        let mut out_next: *mut libc::c_char = std::ptr::null_mut();
        let mut res: () = unimplemented!();
        res = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((res as i32) == (unimplemented!() as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((in_next)
                == ((in_.as_mut_ptr().offset((t.expected_in_next) as isize)).cast_const()))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((out_next) == (out.as_mut_ptr().offset((t.expected_out_next) as isize))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unimplemented!()) == (0)) {
                unimplemented!();
                std::process::abort();
            }
        }
        if ((t.expected_out_next) < (unsafe { array_size_15(&out) })) {
            let mut __do_while = true;
            'loop_: while __do_while || (false) {
                __do_while = false;
                if !((out[(t.expected_out_next)] as i32) == (0)) {
                    unimplemented!();
                    std::process::abort();
                }
            }
        }
        in_[(t.replace_pos)] = old_char;
    }
}
pub unsafe fn test_utf16_ucs2_cvt_75(cvt: *const (), mut endianess: utf16_endianess) {
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_ucs2_in_ok_69(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_ucs2_in_partial_70(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        utf16_to_ucs2_in_error_71(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        ucs2_to_utf16_out_ok_72(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        ucs2_to_utf16_out_partial_73(_cvt, _endianess)
    });
    (unsafe {
        let _cvt: *const () = cvt;
        let _endianess: utf16_endianess = endianess;
        ucs2_to_utf16_out_error_74(_cvt, _endianess)
    });
}
pub unsafe fn test_utf8_utf32_codecvts_76() {
    let loc_c: *const () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let cvt: *const () = unimplemented!();
    (unsafe { test_utf8_utf32_cvt_32(cvt) });
    let mut cvt2: () = unimplemented!();
    (unsafe { test_utf8_utf32_cvt_32(&cvt2) });
}
pub unsafe fn test_utf8_utf16_codecvts_77() {
    let loc_c: *const () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let cvt: *const () = unimplemented!();
    (unsafe { test_utf8_utf16_cvt_49(cvt) });
    let mut cvt2: () = unimplemented!();
    (unsafe { test_utf8_utf16_cvt_49(&cvt2) });
    let mut cvt3: () = unimplemented!();
    (unsafe { test_utf8_utf16_cvt_50(&cvt3) });
}
pub unsafe fn test_utf8_ucs2_codecvts_78() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf8_ucs2_cvt_59(&cvt) });
}
pub unsafe fn test_utf16_utf32_codecvts_79() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_68(&cvt, utf16_endianess_utf16_big_endian) });
    let mut cvt2: () = unimplemented!();
    (unsafe { test_utf16_utf32_cvt_68(&cvt2, utf16_endianess_utf16_little_endian) });
}
pub unsafe fn test_utf16_ucs2_codecvts_80() {
    let mut cvt: () = unimplemented!();
    (unsafe { test_utf16_ucs2_cvt_75(&cvt, utf16_endianess_utf16_big_endian) });
    let mut cvt2: () = unimplemented!();
    (unsafe { test_utf16_ucs2_cvt_75(&cvt2, utf16_endianess_utf16_little_endian) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_utf8_utf32_codecvts_76() });
    (unsafe { test_utf8_utf16_codecvts_77() });
    (unsafe { test_utf8_ucs2_codecvts_78() });
    (unsafe { test_utf16_utf32_codecvts_79() });
    (unsafe { test_utf16_ucs2_codecvts_80() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: 0_i32,
            pid_: unimplemented!(),
        };
        this.pid_ = unimplemented!();
        this.sem_set_ = unimplemented!();
        if ((this.sem_set_) == (-1_i32)) {
            unimplemented!();
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0;
        if ((unimplemented!()) == (-1_i32)) {
            unimplemented!();
        }
        this
    }
}
pub unsafe fn set_memory_limits_81(mut size: Option<f32>) {
    let mut size: f32 = size.unwrap_or_else(|| unsafe { (16_f64 as f32) });
    let mut r: () = unimplemented!();
    let mut limit: () = (((size) * (1048576_f32)) as ());
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
pub unsafe fn set_file_limit_82(mut size: u64) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_83(
    mut mangled: *const libc::c_char,
    mut wanted: *const libc::c_char,
) {
    let mut status: i32 = 0;
    let mut s: *const libc::c_char = std::ptr::null();
    let mut demangled: *mut libc::c_char = unimplemented!();
    if !(demangled).is_null() {
        s = (demangled).cast_const();
    } else {
        switch!(match status {
            __v if __v == 0 => {
                s = c"error code = 0: success".as_ptr();
                break;
            }
            __v if __v == -1_i32 => {
                s = c"error code = -1: memory allocation failure".as_ptr();
                break;
            }
            __v if __v == -2_i32 => {
                s = c"error code = -2: invalid mangled name".as_ptr();
                break;
            }
            __v if __v == -3_i32 => {
                s = c"error code = -3: invalid arguments".as_ptr();
                break;
            }
            _ => {
                s = c"error code unknown - who knows what happened".as_ptr();
            }
        });
    }
    let mut w: Vec<libc::c_char> =
        std::slice::from_raw_parts(wanted, libcc2rs::c_str_len(wanted) + 1).to_vec();
    if !(w == std::slice::from_raw_parts(s, libcc2rs::c_str_len(s) + 1).to_vec()) {
        unimplemented!();
    }
    libcc2rs::free_unsafe((demangled as *mut ::libc::c_void));
}
pub unsafe fn run_tests_wrapped_locale_84(
    mut name: *const libc::c_char,
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut res: *const libc::c_char = (libc::setlocale(6, name)).cast_const();
    if !(res).is_null() {
        let mut preLC_ALL: Vec<libc::c_char> =
            std::slice::from_raw_parts(res, libcc2rs::c_str_len(res) + 1).to_vec();
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: i32 = 0;
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        let mut postLC_ALL: Vec<libc::c_char> = std::slice::from_raw_parts(
            (libc::setlocale(6, std::ptr::null())).cast_const(),
            libcc2rs::c_str_len((libc::setlocale(6, std::ptr::null())).cast_const()) + 1,
        )
        .to_vec();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                std::process::abort();
            }
        }
    } else {
        let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"LC_ALL for ".as_ptr(),
            libcc2rs::c_str_len(c"LC_ALL for ".as_ptr()) + 1,
        )
        .to_vec();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn run_tests_wrapped_env_85(
    mut name: *const libc::c_char,
    mut env: *const libc::c_char,
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut oldENV: *const libc::c_char = (libc::getenv(env)).cast_const();
    if !(libc::setenv(env, name, 1) != 0) {
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: i32 = 0;
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        libc::setenv(
            env,
            if !(oldENV).is_null() {
                oldENV
            } else {
                c"".as_ptr()
            },
            1,
        );
    } else {
        let mut s: Vec<libc::c_char> =
            std::slice::from_raw_parts(env, libcc2rs::c_str_len(env) + 1).to_vec();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut count__1: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut throw_on__2: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut count__3: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut throw_on__4: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut _M_count_5: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut next_id__6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union __gnu_test_semun {
    pub val: i32,
    pub buf: *mut (),
    pub array: *mut u16,
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
        val.val = 0;
        if ((self.pid_) == (unimplemented!())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_i32)) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_i32)) {
            unimplemented!();
        }
    }
}
pub unsafe fn test_tm_86(
    mut sec: i32,
    mut min: i32,
    mut hour: i32,
    mut mday: i32,
    mut mon: i32,
    mut year: i32,
    mut wday: i32,
    mut yday: i32,
    mut isdst: i32,
) -> ::libc::tm {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_87));
}
static mut tmp_87: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
