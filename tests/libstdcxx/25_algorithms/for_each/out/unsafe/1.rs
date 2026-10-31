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
pub struct __gnu_test_BoundsContainer_int_ {
    pub first: *mut i32,
    pub last: *mut i32,
}
impl __gnu_test_BoundsContainer_int_ {
    pub unsafe fn new(mut _first: *mut i32, mut _last: *mut i32) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_input_iterator_wrapper_int_ {
    pub ptr: *mut i32,
    pub SharedInfo: *mut __gnu_test_BoundsContainer_int_,
}
impl __gnu_test_input_iterator_wrapper_int_ {
    pub unsafe fn new_1(
        mut _ptr: *mut i32,
        mut SharedInfo_in: *mut __gnu_test_BoundsContainer_int_,
    ) -> Self {
        let mut this = Self {
            ptr: _ptr,
            SharedInfo: SharedInfo_in,
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((this.ptr) >= ((*this.SharedInfo).first))
                && ((this.ptr) <= ((*this.SharedInfo).last)))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        this
    }
    pub unsafe fn operator_eq(&self, in_: *const __gnu_test_input_iterator_wrapper_int_) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!(self.SharedInfo).is_null()) && ((self.SharedInfo) == ((*in_).SharedInfo))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((self.ptr) >= ((*self.SharedInfo).first))
                && (((*in_).ptr) >= ((*self.SharedInfo).first)))
            {
                unimplemented!();
                std::process::abort();
            }
        }
        return ((self.ptr) == ((*in_).ptr));
    }
    pub unsafe fn operator_ne(&self, in_: *const __gnu_test_input_iterator_wrapper_int_) -> bool {
        return !(unsafe {
            let _in_: *const __gnu_test_input_iterator_wrapper_int_ = in_;
            __gnu_test_input_iterator_wrapper_int_::operator_eq(
                &(*(self as *const __gnu_test_input_iterator_wrapper_int_)),
                _in_,
            )
        });
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!(self.SharedInfo).is_null()) && ((self.ptr) < ((*self.SharedInfo).last))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((self.ptr) >= ((*self.SharedInfo).first)) {
                unimplemented!();
                std::process::abort();
            }
        }
        return &mut (*self.ptr);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut __gnu_test_input_iterator_wrapper_int_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!(self.SharedInfo).is_null()) && ((self.ptr) < ((*self.SharedInfo).last))) {
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((self.ptr) >= ((*self.SharedInfo).first)) {
                unimplemented!();
                std::process::abort();
            }
        }
        self.ptr.postfix_inc();
        (*self.SharedInfo).first = self.ptr;
        return &mut (*(self as *mut __gnu_test_input_iterator_wrapper_int_));
    }
}
impl std::cmp::PartialEq for __gnu_test_input_iterator_wrapper_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            __gnu_test_input_iterator_wrapper_int_::operator_eq(
                self,
                other as *const __gnu_test_input_iterator_wrapper_int_,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_input_iterator_wrapper_int_ {}
impl UnsafeCppAdvance for __gnu_test_input_iterator_wrapper_int_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_ {
    pub bounds: __gnu_test_BoundsContainer_int_,
}
impl __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_ {
    pub unsafe fn new(mut _first: *mut i32, mut _last: *mut i32) -> Self {
        let mut this = Self {
            bounds: __gnu_test_BoundsContainer_int_::new({ _first }, { _last }),
        };
        this
    }
    pub unsafe fn it_4(&mut self, mut pos: *mut i32) -> __gnu_test_input_iterator_wrapper_int_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((pos) >= (self.bounds.first)) && ((pos) <= (self.bounds.last))) {
                unimplemented!();
                std::process::abort();
            }
        }
        return __gnu_test_input_iterator_wrapper_int_::new_1({ pos }, {
            (&mut self.bounds as *mut __gnu_test_BoundsContainer_int_)
        });
    }
    pub unsafe fn begin(&mut self) -> __gnu_test_input_iterator_wrapper_int_ {
        return (unsafe {
            let _pos: *mut i32 = self.bounds.first;
            __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_::it_4(self, _pos)
        });
    }
    pub unsafe fn end(&mut self) -> __gnu_test_input_iterator_wrapper_int_ {
        return (unsafe {
            let _pos: *mut i32 = self.bounds.last;
            __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_::it_4(self, _pos)
        });
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct Function {
    tot: i32,
}
impl Function {
    pub unsafe fn new() -> Self {
        let mut this = Self { tot: 0 };
        this
    }
    pub unsafe fn move_from(f: *mut Function) -> Self {
        let mut this = Self { tot: (*f).tot };
        (*f).tot = 0;
        this
    }
    pub unsafe fn operator_call(&mut self, mut num: i32) {
        self.tot += num;
    }
    pub unsafe fn get(&mut self) -> i32 {
        return self.tot;
    }
}
impl Default for Function {
    fn default() -> Self {
        unsafe { Function::new() }
    }
}
pub unsafe fn test01_9() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut con: __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_ =
        __gnu_test_test_container_int____gnu_test_input_iterator_wrapper_::new(
            { array.as_mut_ptr() },
            { array.as_mut_ptr().offset((5) as isize) },
        );
    let mut f: Function = Function::new();
    let mut f_res: Function = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unsafe { Function::get(&mut f_res) }) == (15)) {
            unimplemented!();
            std::process::abort();
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_9() });
    return 0;
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
pub unsafe fn set_memory_limits_10(mut size: Option<f32>) {
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
pub unsafe fn set_file_limit_11(mut size: u64) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_12(
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
pub unsafe fn run_tests_wrapped_locale_13(
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
pub unsafe fn run_tests_wrapped_env_14(
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
pub unsafe fn test_tm_15(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_16));
}
static mut tmp_16: std::cell::LazyCell<::libc::tm> =
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
