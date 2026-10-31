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
pub struct Cmp {}
impl Cmp {
    pub unsafe fn operator_call_1(&self, mut i: i32, mut l: i64) -> bool {
        return ((i as i64) < (l));
    }
    pub unsafe fn operator_call_2(&self, mut l: i64, mut i: i32) -> bool {
        return ((l) < (i as i64));
    }
    pub unsafe fn operator_call_3(&self, mut i: i32, mut j: i32) -> bool {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)).prefix_inc();
        return ((i) < (j));
    }
}
impl Callable2<i32, i64, bool> for Cmp {
    fn call(&self, a1: i32, a2: i64) -> bool {
        unsafe { Cmp::operator_call_1(self, a1, a2) }
    }
}
impl Callable2<i64, i32, bool> for Cmp {
    fn call(&self, a1: i64, a2: i32) -> bool {
        unsafe { Cmp::operator_call_2(self, a1, a2) }
    }
}
impl Callable2<i32, i32, bool> for Cmp {
    fn call(&self, a1: i32, a2: i32) -> bool {
        unsafe { Cmp::operator_call_3(self, a1, a2) }
    }
}
pub static mut count_9: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut x_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut cx_11: std::cell::LazyCell<*const ()> =
    std::cell::LazyCell::new(|| unsafe { &(*std::cell::LazyCell::force_mut(&mut *&raw mut x_10)) });
pub unsafe fn test01_12() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) = 0;
    let mut it: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((it != unimplemented!()) && ((*it.iter_deref()) == (1))) {
            unimplemented!();
            std::process::abort();
        }
    }
    it = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(it == unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut cit: UnsafeBidirectionalIterator<i32> = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((cit != unimplemented!()) && ((*cit.iter_deref()) == (3))) {
            unimplemented!();
            std::process::abort();
        }
    }
    cit = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(cit == unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (0)) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(
        true,
        "std::is_same<decltype(it), test_type::iterator>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(cit), test_type::const_iterator>::value"
    );;
}
pub unsafe fn test02_13() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) = 0;
    let mut n: usize = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((n) == (1_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    n = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((n) == (0_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut cn: usize = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((cn) == (2_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    cn = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((cn) == (0_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (0)) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub unsafe fn test03_14() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) = 0;
    let mut it: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((it != unimplemented!()) && ((*it.iter_deref()) == (1))) {
            unimplemented!();
            std::process::abort();
        }
    }
    it = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((it != unimplemented!()) && ((*it.iter_deref()) == (3))) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut cit: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((cit != unimplemented!()) && ((*cit.iter_deref()) == (1))) {
            unimplemented!();
            std::process::abort();
        }
    }
    cit = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((cit != unimplemented!()) && ((*cit.iter_deref()) == (3))) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (0)) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(
        true,
        "std::is_same<decltype(it), test_type::iterator>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(cit), test_type::const_iterator>::value"
    );;
}
pub unsafe fn test04_15() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) = 0;
    let mut it: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((it != unimplemented!()) && ((*it.iter_deref()) == (3))) {
            unimplemented!();
            std::process::abort();
        }
    }
    it = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(it == unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut cit: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((cit != unimplemented!()) && ((*cit.iter_deref()) == (3))) {
            unimplemented!();
            std::process::abort();
        }
    }
    cit = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(cit == unimplemented!()) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (0)) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(
        true,
        "std::is_same<decltype(it), test_type::iterator>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(cit), test_type::const_iterator>::value"
    );;
}
pub unsafe fn test05_16() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) = 0;
    let mut it: (
        UnsafeBidirectionalIterator<i32>,
        UnsafeBidirectionalIterator<i32>,
    ) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((it.0 != it.1) && ((*it.0.iter_deref()) == (1))) {
            unimplemented!();
            std::process::abort();
        }
    }
    it = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((it.0 == it.1) && (it.0 != unimplemented!())) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut cit: (
        UnsafeBidirectionalIterator<i32>,
        UnsafeBidirectionalIterator<i32>,
    ) = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((cit.0 != cit.1) && ((*cit.0.iter_deref()) == (3))) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (2_isize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    cit = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((cit.0 == cit.1) && (cit.0 != unimplemented!())) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)) == (0)) {
            unimplemented!();
            std::process::abort();
        }
    }
    const _: () = assert!(true, "std::is_same<decltype(it), pair>::value");;
    const _: () = assert!(true, "std::is_same<decltype(cit), cpair>::value");;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_12() });
    (unsafe { test02_13() });
    (unsafe { test03_14() });
    (unsafe { test04_15() });
    (unsafe { test05_16() });
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
pub unsafe fn set_memory_limits_17(mut size: Option<f32>) {
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
pub unsafe fn set_file_limit_18(mut size: u64) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_19(
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
pub unsafe fn run_tests_wrapped_locale_20(
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
pub unsafe fn run_tests_wrapped_env_21(
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
pub unsafe fn test_tm_22(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_23));
}
static mut tmp_23: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_9);
    std::cell::LazyCell::force(&*&raw const x_10);
    std::cell::LazyCell::force(&*&raw const cx_11);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
