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
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    const _: () = assert!(
        ((::std::mem::size_of::<()>()) <= (::std::mem::size_of::<*mut ()>())),
        "sizeof(std::span<char, 0>) <= sizeof(char*)"
    );;
    const _: () = assert!(
        ((::std::mem::size_of::<()>()) <= (::std::mem::size_of::<*const ()>())),
        "sizeof(std::span<const char, 0>) <= sizeof(const char*)"
    );;
    const _: () = assert!(
        ((::std::mem::size_of::<()>()) <= (::std::mem::size_of::<*mut strawman_9>())),
        "sizeof(std::span<strawman, 0>) <= sizeof(strawman*)"
    );;
    const _: () = assert!(
        ((::std::mem::size_of::<()>()) <= (::std::mem::size_of::<*mut strawman_9>())),
        "sizeof(std::span<strawman, 1>) <= sizeof(strawman*)"
    );;
    const _: () = assert!(
        ((::std::mem::size_of::<()>()) <= (::std::mem::size_of::<naked_span_10>())),
        "sizeof(std::span<char>) <= sizeof(naked_span)"
    );;
    const _: () = assert!(
        ((::std::mem::size_of::<()>()) <= (::std::mem::size_of::<strawman_span_11>())),
        "sizeof(std::span<strawman>) <= sizeof(strawman_span)"
    );;
    let arr_data_span: () = unimplemented!();
    const _: () = assert!(true, "arr_data_span.size() == 9");;
    const _: () = assert!(true, "arr_data_span.size_bytes() == 9 * sizeof(int)");;
    const _: () = assert!(true, "*arr_data_span.begin() == 0");;
    const _: () = assert!(true, "*arr_data_span.data() == 0");;
    const _: () = assert!(true, "arr_data_span.front() == 0");;
    const _: () = assert!(true, "arr_data_span.back() == 8");;
    const _: () = assert!(true, "arr_data_span[0] == 0");;
    const _: () = assert!(true, "arr_data_span[1] == 1");;
    const _: () = assert!(true, "arr_data_span[2] == 2");;
    const _: () = assert!(true, "arr_data_span[3] == 3");;
    const _: () = assert!(true, "arr_data_span[4] == 4");;
    const _: () = assert!(true, "arr_data_span[5] == 5");;
    const _: () = assert!(true, "arr_data_span[6] == 6");;
    const _: () = assert!(true, "arr_data_span[7] == 7");;
    const _: () = assert!(true, "arr_data_span[8] == 8");;
    const _: () = assert!(true, "!arr_data_span.empty()");;
    const _: () = assert!(true, "decltype(arr_data_span)::extent == 9");;
    let data_span: () = unimplemented!();
    const _: () = assert!(true, "data_span.size() == 9");;
    const _: () = assert!(true, "data_span.size_bytes() == 9 * sizeof(int)");;
    const _: () = assert!(true, "*data_span.begin() == 0");;
    const _: () = assert!(true, "*data_span.data() == 0");;
    const _: () = assert!(true, "data_span.front() == 0");;
    const _: () = assert!(true, "data_span.back() == 8");;
    const _: () = assert!(true, "data_span[0] == 0");;
    const _: () = assert!(true, "data_span[1] == 1");;
    const _: () = assert!(true, "data_span[2] == 2");;
    const _: () = assert!(true, "data_span[3] == 3");;
    const _: () = assert!(true, "data_span[4] == 4");;
    const _: () = assert!(true, "data_span[5] == 5");;
    const _: () = assert!(true, "data_span[6] == 6");;
    const _: () = assert!(true, "data_span[7] == 7");;
    const _: () = assert!(true, "data_span[8] == 8");;
    const _: () = assert!(true, "!data_span.empty()");;
    const _: () = assert!(true, "decltype(data_span)::extent == 9");;
    let data_span_first: () = (unsafe { data_span.first() });
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_first)>, std::span<const int, 3>>"
    );;
    const _: () = assert!(true, "decltype(data_span_first)::extent == 3");;
    const _: () = assert!(true, "data_span_first.size() == 3");;
    const _: () = assert!(true, "data_span_first.front() == 0");;
    const _: () = assert!(true, "data_span_first.back() == 2");;
    let data_span_first_dyn: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_first_dyn)>, std::span<const int>>"
    );;
    const _: () = assert!(
        true,
        "decltype(data_span_first_dyn)::extent == std::dynamic_extent"
    );;
    const _: () = assert!(true, "data_span_first_dyn.size() == 4");;
    const _: () = assert!(true, "data_span_first_dyn.front() == 0");;
    const _: () = assert!(true, "data_span_first_dyn.back() == 3");;
    let data_span_last: () = (unsafe { data_span.last() });
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_last)>, std::span<const int, 5>>"
    );;
    const _: () = assert!(true, "decltype(data_span_last)::extent == 5");;
    const _: () = assert!(true, "data_span_last.size() == 5");;
    const _: () = assert!(true, "data_span_last.front() == 4");;
    const _: () = assert!(true, "data_span_last.back() == 8");;
    let data_span_last_dyn: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_last_dyn)>, std::span<const int>>"
    );;
    const _: () = assert!(
        true,
        "decltype(data_span_last_dyn)::extent == std::dynamic_extent"
    );;
    const _: () = assert!(true, "data_span_last_dyn.size() == 6");;
    const _: () = assert!(true, "data_span_last_dyn.front() == 3");;
    const _: () = assert!(true, "data_span_last_dyn.back() == 8");;
    let data_span_subspan: () = (unsafe { data_span.subspan() });
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_subspan)>, std::span<const int, 3>>"
    );;
    const _: () = assert!(true, "decltype(data_span_subspan)::extent == 3");;
    const _: () = assert!(true, "data_span_subspan.size() == 3");;
    const _: () = assert!(true, "data_span_subspan.front() == 1");;
    const _: () = assert!(true, "data_span_subspan.back() == 3");;
    let data_span_subspan_offset: () = (unsafe { data_span.subspan() });
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_subspan_offset)>, std::span<const int, 1>>"
    );;
    const _: () = assert!(true, "decltype(data_span_subspan_offset)::extent == 1");;
    const _: () = assert!(true, "data_span_subspan_offset.size() == 1");;
    const _: () = assert!(true, "data_span_subspan_offset.front() == 8");;
    const _: () = assert!(true, "data_span_subspan_offset.back() == 8");;
    let data_span_subspan_empty: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_subspan_empty)>, std::span<const int>>"
    );;
    const _: () = assert!(
        true,
        "decltype(data_span_subspan_empty)::extent == std::dynamic_extent"
    );;
    const _: () = assert!(true, "data_span_subspan_empty.size() == 0");;
    let data_span_subspan_empty_static: () = (unsafe { data_span.subspan() });
    const _: () = assert!(
        true,
        "std::is_same_v<std::remove_cv_t<decltype(data_span_subspan_empty_static)>,\n    std::span<const int, 0>>"
    );;
    const _: () = assert!(
        true,
        "decltype(data_span_subspan_empty_static)::extent == 0"
    );;
    const _: () = assert!(true, "data_span_subspan_empty.size() == 0");;
    let mut shorts: () = unimplemented!();
    let mut really_empty0: () = unimplemented!();
    let mut really_empty1: () = unimplemented!();
    let mut really_empty2: () = (unimplemented!()).is_null();
    let mut really_empty3: () = unimplemented!();
    let mut really_empty: () =
        (((really_empty0) && (really_empty1)) && (really_empty2)) && (really_empty3);
    &(really_empty);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(really_empty) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut value: () = unimplemented!();
    let mut muh_span: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (1_())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut original_bytes: *mut () = (unimplemented!() as *mut ());
    (*original_bytes.offset((0_()) as isize)) = ((1_()) as ());
    (*original_bytes.offset((1_()) as isize)) = ((2_()) as ());
    (*original_bytes.offset((2_()) as isize)) = ((3_()) as ());
    (*original_bytes.offset((3_()) as isize)) = ((4_()) as ());
    let mut muh_byte_span: () = unimplemented!();
    let mut muh_mutable_byte_span: () = unimplemented!();
    let mut muh_original_byte_span: () = unimplemented!();
    let mut definitely_reinterpret_casted0: () = unimplemented!();
    let mut definitely_reinterpret_casted1: () = unimplemented!();
    let mut definitely_reinterpret_casted: () =
        (definitely_reinterpret_casted0) && (definitely_reinterpret_casted1);
    &(definitely_reinterpret_casted);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(definitely_reinterpret_casted) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut muh_original_byte_span_ptr_size: () = unimplemented!();
    let mut definitely_equivalent: () = unimplemented!();
    &(definitely_equivalent);
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(definitely_equivalent) {
            unimplemented!();
            unimplemented!();
        }
    }
    return if ((definitely_equivalent) && (definitely_reinterpret_casted)) && (really_empty) {
        0_()
    } else {
        1_()
    };
}
#[repr(C, align(256))]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct strawman_9 {
    pub x: (),
    pub y: (),
    pub z: (),
    pub w: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct naked_span_10 {
    pub p: *mut (),
    pub n: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct strawman_span_11 {
    pub p: *mut strawman_9,
    pub n: (),
}
static mut arr_data_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut data_13: std::cell::LazyCell<[(); 9]> =
    std::cell::LazyCell::new(|| unsafe { [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_()] });
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
pub unsafe fn set_memory_limits_14(mut size: Option<()>) {
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
pub unsafe fn set_file_limit_15(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_16(mut mangled: *const (), mut wanted: *const ()) {
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
pub unsafe fn run_tests_wrapped_locale_17(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_18(
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
pub unsafe fn test_tm_19(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_20)).clone();
}
static mut tmp_20: std::cell::LazyCell<()> =
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
