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
const _: () = assert!(true, "! std::is_default_constructible_v<std::format_args>");
const _: () = assert!(true, "! std::is_default_constructible_v<std::wformat_args>");
pub unsafe fn equals_9(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
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
pub unsafe fn equals_20(mut fmt_arg: (), mut expected: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_21(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_22(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn equals_23(mut fmt_arg: (), mut expected: ()) -> () {
    return unimplemented!();
}
pub unsafe fn test_empty_24() {
    let mut args: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_9(unimplemented!(), unimplemented!()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut cargs: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_9(unimplemented!(), unimplemented!()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut wargs: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_10(unimplemented!(), unimplemented!()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub type E = ();
pub const E_ByGum: E = 0;
pub unsafe fn test_args_25() {
    let mut b: () = false;
    let mut i: () = 1_();
    let mut c: () = ('2' as ());
    let mut d: () = 3.4_f64;
    let mut store: () = unimplemented!();
    let mut args: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_11(unimplemented!(), false) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_12(unimplemented!(), 1_()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_13(unimplemented!(), ('2' as ())) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_14(unimplemented!(), 3.4_f64) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut l: () = 5_();
    let mut ull: () = 6_();
    let mut f: () = 7.8_f32;
    let mut cstore: () = unimplemented!();
    let mut cargs: () = unimplemented!();
    {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe { equals_15(unimplemented!(), 5_()) }) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_16(unimplemented!(), 6_()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_17(unimplemented!(), 7.8_f32) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut s: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_18(unimplemented!(), unimplemented!()) }) {
            unimplemented!();
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
    let mut wstore: () = unimplemented!();
    let mut wargs: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_19(unimplemented!(), ('9' as ())) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_19(unimplemented!(), (88 as ())) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_20(unimplemented!(), (eleven.as_mut_ptr()).cast_const()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_21(unimplemented!(), 12.13_f64) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_22(unimplemented!(), unimplemented!()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!(unimplemented!())) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut null: () = unimplemented!();
    let mut eebygum: E = E_ByGum;
    let mut another_store: () = unimplemented!();
    args = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unsafe { equals_23(unimplemented!(), unimplemented!()) }) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut is_handle: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: *const ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        },
        |mut _a0: ()| -> () {
            return unimplemented!();
        }
    );
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_member_visit_26() {}
pub unsafe fn test_visited_as_handle_27() {
    let mut v: () = unimplemented!();
    let mut store: () = unimplemented!();
    let mut args: () = unimplemented!();
    let is_handle: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: *const ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        }
    );
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_visited_as_handle_28() {
    let mut v: () = unimplemented!();
    let mut store: () = unimplemented!();
    let mut args: () = unimplemented!();
    let is_handle: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {},
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: *const ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        },
        |mut arg: ()| -> () {
            return unimplemented!();
        }
    );
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            unimplemented!();
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
    (unsafe { test_empty_24() });
    (unsafe { test_args_25() });
    (unsafe { test_member_visit_26() });
    (unsafe { test_visited_as_handle_27() });
    (unsafe { test_visited_as_handle_28() });
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
pub unsafe fn set_memory_limits_29(mut size: Option<()>) {
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
pub unsafe fn set_file_limit_30(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_31(mut mangled: *const (), mut wanted: *const ()) {
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
pub unsafe fn run_tests_wrapped_locale_32(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_33(
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
) -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_35)).clone();
}
static mut tmp_35: std::cell::LazyCell<()> =
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
