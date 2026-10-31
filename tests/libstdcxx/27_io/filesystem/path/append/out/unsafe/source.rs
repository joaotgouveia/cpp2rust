extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn compare_paths_0(p1: *const Vec<libc::c_char>, p2: *const Vec<libc::c_char>) {
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if ((unimplemented!() as i32) != (unimplemented!() as i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    let mut d1: isize = unimplemented!();
    let mut d2: isize = unimplemented!();
    if ((d1) != (d2)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    if !(unimplemented!()) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
}
pub static mut test_paths_1: std::cell::LazyCell<[Vec<libc::c_char>; 18]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/".as_ptr(), libcc2rs::c_str_len(c"/".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"//".as_ptr(), libcc2rs::c_str_len(c"//".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/.".as_ptr(), libcc2rs::c_str_len(c"/.".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/./".as_ptr(), libcc2rs::c_str_len(c"/./".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/a".as_ptr(), libcc2rs::c_str_len(c"/a".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/a/".as_ptr(), libcc2rs::c_str_len(c"/a/".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"/a//".as_ptr(), libcc2rs::c_str_len(c"/a//".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(
                c"/a/b/c/d".as_ptr(),
                libcc2rs::c_str_len(c"/a/b/c/d".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"/a//b".as_ptr(),
                libcc2rs::c_str_len(c"/a//b".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(c"a".as_ptr(), libcc2rs::c_str_len(c"a".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"a/b".as_ptr(), libcc2rs::c_str_len(c"a/b".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(c"a/b/".as_ptr(), libcc2rs::c_str_len(c"a/b/".as_ptr()) + 1)
                .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/c".as_ptr(),
                libcc2rs::c_str_len(c"a/b/c".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/c.d".as_ptr(),
                libcc2rs::c_str_len(c"a/b/c.d".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/..".as_ptr(),
                libcc2rs::c_str_len(c"a/b/..".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/c.".as_ptr(),
                libcc2rs::c_str_len(c"a/b/c.".as_ptr()) + 1,
            )
            .to_vec(),
            std::slice::from_raw_parts(
                c"a/b/.c".as_ptr(),
                libcc2rs::c_str_len(c"a/b/.c".as_ptr()) + 1,
            )
            .to_vec(),
        ]
    });
pub unsafe fn root_path_2() -> Vec<libc::c_char> {
    return {
        let mut __v: Vec<libc::c_char> = c"/"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
}
pub unsafe fn nonexistent_path_3(mut file: Option<Vec<libc::c_char>>) -> Vec<libc::c_char> {
    let mut file: Vec<libc::c_char> = file.unwrap_or_else(|| unsafe {
        std::slice::from_raw_parts(
            concat!(file!(), "\0").as_ptr() as *const libc::c_char,
            libcc2rs::c_str_len(concat!(file!(), "\0").as_ptr() as *const libc::c_char) + 1,
        )
        .to_vec()
    });
    if ((file.len() - 1) > (3_usize)) && ((unimplemented!()) == (0)) {
        {
            file.pop();
            file.resize(
                ((file.len() - 1).wrapping_sub(3_usize)) as usize,
                (0 as libc::c_char),
            );
            file.push((0 as libc::c_char))
        };
    }
    let mut pos: usize = {
        let __n = file.len().saturating_sub(1);
        let __end = usize::MAX.saturating_add(1).min(__n);
        let __lookup =
            ::std::slice::from_raw_parts(c"/\\".as_ptr(), libcc2rs::c_str_len(c"/\\".as_ptr()));
        match file
            .iter()
            .take(__end)
            .rposition(|__c| __lookup.contains(__c))
        {
            Some(__idx) => __idx,
            None => usize::MAX,
        }
    };
    if ((pos) != (usize::MAX)) {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    let mut fd: i32 = unimplemented!();
    if ((fd) == (-1_i32)) {
        {
            std::process::abort();
            &(unimplemented!())
        };
    }
    libc::unlink(file.as_ptr());
    libc::close(fd);
    unimplemented!();
    return unimplemented!();
}
pub type __gnu_test_scoped_file_adopt_file_t = u32;
pub const __gnu_test_scoped_file_adopt_file_t_adopt_file: __gnu_test_scoped_file_adopt_file_t = 0;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_scoped_file {
    pub path: Vec<libc::c_char>,
}
impl __gnu_test_scoped_file {
    pub unsafe fn new_1(p: Option<*const Vec<libc::c_char>>) -> Self {
        let mut __p_default: Option<Vec<libc::c_char>> = None;
        let mut p: *const Vec<libc::c_char> = p.unwrap_or_else(|| unsafe {
            __p_default.insert((unsafe { nonexistent_path_3(None) })) as *const Vec<libc::c_char>
        });
        let mut this = Self {
            path: unimplemented!(),
        };
        ::std::fs::File::create(
            ::std::ffi::CStr::from_ptr((unimplemented!() as *const ()).cast::<libc::c_char>())
                .to_str()
                .unwrap(),
        )
        .unwrap();
        this
    }
    pub unsafe fn new_2(
        mut p: Vec<libc::c_char>,
        mut _a1: __gnu_test_scoped_file_adopt_file_t,
    ) -> Self {
        let mut this = Self {
            path: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        if !(unimplemented!()) {
            unimplemented!();
        }
    }
    pub unsafe fn move_from(_a0: *mut __gnu_test_scoped_file) -> Self {
        let mut this = Self {
            path: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_test_scoped_file,
    ) -> *mut __gnu_test_scoped_file {
        unimplemented!();
        return &mut (*(self as *mut __gnu_test_scoped_file));
    }
}
impl Default for __gnu_test_scoped_file {
    fn default() -> Self {
        unsafe { __gnu_test_scoped_file::new_1(None) }
    }
}
pub unsafe fn permissions_are_testable_5(mut print_msg: Option<bool>) -> bool {
    let mut print_msg: bool = print_msg.unwrap_or_else(|| unsafe { true });
    let mut testable: bool = false;
    if ((libc::geteuid() as ::libc::uid_t) != (0 as ::libc::uid_t)) {
        testable = true;
    }
    if (print_msg) && (!(testable)) {
        libc::puts(c"Skipping tests that depend on filesystem permissions".as_ptr());
    }
    return testable;
}
static mut counter_4: std::cell::LazyCell<u32> =
    std::cell::LazyCell::new(|| unsafe { (unimplemented!() as u32) });
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const __gnu_test_object_counter) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)).prefix_dec();
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
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__7));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__7)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__7))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__8)))
        {
            unimplemented!();
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__7)) = 0_u32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__8)) = 0_u32;
    }
    pub unsafe fn throw_on(mut count: u32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__8)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub unsafe fn count() -> u32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__9));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__9)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__9))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__10)))
        {
            unimplemented!();
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__9)) = 0_u32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__10)) = 0_u32;
    }
    pub unsafe fn throw_on(mut count: u32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__10)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> u32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_11));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_11)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_11)) = 0_u32;
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__12)).postfix_dec()
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
            if operator_lt_13(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_13(
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
            operator_eq_14(
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
pub unsafe fn operator_eq_14(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> bool {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_13(
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
pub struct __gnu_test_BoundsContainer_const_char_ {
    pub first: *const libc::c_char,
    pub last: *const libc::c_char,
}
impl __gnu_test_BoundsContainer_const_char_ {
    pub unsafe fn new(mut _first: *const libc::c_char, mut _last: *const libc::c_char) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_BoundsContainer_const_wchar_t_ {
    pub first: *const i32,
    pub last: *const i32,
}
impl __gnu_test_BoundsContainer_const_wchar_t_ {
    pub unsafe fn new(mut _first: *const i32, mut _last: *const i32) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy_deref_proxy {
    pub ptr: *const libc::c_char,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy {
    pub p: __gnu_test_input_iterator_wrapper_const_char__post_inc_proxy_deref_proxy,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_input_iterator_wrapper_const_char_ {
    pub ptr: *const libc::c_char,
    pub SharedInfo: *mut __gnu_test_BoundsContainer_const_char_,
}
impl __gnu_test_input_iterator_wrapper_const_char_ {
    pub unsafe fn new_1(
        mut _ptr: *const libc::c_char,
        mut SharedInfo_in: *mut __gnu_test_BoundsContainer_const_char_,
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
    pub unsafe fn operator_eq(
        &self,
        in_: *const __gnu_test_input_iterator_wrapper_const_char_,
    ) -> bool {
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
    pub unsafe fn operator_ne(
        &self,
        in_: *const __gnu_test_input_iterator_wrapper_const_char_,
    ) -> bool {
        return !(unsafe {
            let _in_: *const __gnu_test_input_iterator_wrapper_const_char_ = in_;
            __gnu_test_input_iterator_wrapper_const_char_::operator_eq(
                &(*(self as *const __gnu_test_input_iterator_wrapper_const_char_)),
                _in_,
            )
        });
    }
    pub unsafe fn operator_deref(&self) -> *const libc::c_char {
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
        return &(*self.ptr);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut __gnu_test_input_iterator_wrapper_const_char_ {
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
        return &mut (*(self as *mut __gnu_test_input_iterator_wrapper_const_char_));
    }
}
impl std::cmp::PartialEq for __gnu_test_input_iterator_wrapper_const_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            __gnu_test_input_iterator_wrapper_const_char_::operator_eq(
                self,
                other as *const __gnu_test_input_iterator_wrapper_const_char_,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_input_iterator_wrapper_const_char_ {}
impl UnsafeCppAdvance for __gnu_test_input_iterator_wrapper_const_char_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy_deref_proxy {
    pub ptr: *const i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy {
    pub p: __gnu_test_input_iterator_wrapper_const_wchar_t__post_inc_proxy_deref_proxy,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    pub ptr: *const i32,
    pub SharedInfo: *mut __gnu_test_BoundsContainer_const_wchar_t_,
}
impl __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    pub unsafe fn new_1(
        mut _ptr: *const i32,
        mut SharedInfo_in: *mut __gnu_test_BoundsContainer_const_wchar_t_,
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
    pub unsafe fn operator_eq(
        &self,
        in_: *const __gnu_test_input_iterator_wrapper_const_wchar_t_,
    ) -> bool {
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
    pub unsafe fn operator_ne(
        &self,
        in_: *const __gnu_test_input_iterator_wrapper_const_wchar_t_,
    ) -> bool {
        return !(unsafe {
            let _in_: *const __gnu_test_input_iterator_wrapper_const_wchar_t_ = in_;
            __gnu_test_input_iterator_wrapper_const_wchar_t_::operator_eq(
                &(*(self as *const __gnu_test_input_iterator_wrapper_const_wchar_t_)),
                _in_,
            )
        });
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
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
        return &(*self.ptr);
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut __gnu_test_input_iterator_wrapper_const_wchar_t_ {
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
        return &mut (*(self as *mut __gnu_test_input_iterator_wrapper_const_wchar_t_));
    }
}
impl std::cmp::PartialEq for __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            __gnu_test_input_iterator_wrapper_const_wchar_t_::operator_eq(
                self,
                other as *const __gnu_test_input_iterator_wrapper_const_wchar_t_,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_input_iterator_wrapper_const_wchar_t_ {}
impl UnsafeCppAdvance for __gnu_test_input_iterator_wrapper_const_wchar_t_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ {
    pub bounds: __gnu_test_BoundsContainer_const_char_,
}
impl __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ {
    pub unsafe fn new(mut _first: *const libc::c_char, mut _last: *const libc::c_char) -> Self {
        let mut this = Self {
            bounds: __gnu_test_BoundsContainer_const_char_::new({ _first }, { _last }),
        };
        this
    }
    pub unsafe fn it_4(
        &mut self,
        mut pos: *const libc::c_char,
    ) -> __gnu_test_input_iterator_wrapper_const_char_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((pos) >= (self.bounds.first)) && ((pos) <= (self.bounds.last))) {
                unimplemented!();
                std::process::abort();
            }
        }
        return __gnu_test_input_iterator_wrapper_const_char_::new_1({ pos }, {
            (&mut self.bounds as *mut __gnu_test_BoundsContainer_const_char_)
        });
    }
    pub unsafe fn begin(&mut self) -> __gnu_test_input_iterator_wrapper_const_char_ {
        return (unsafe {
            let _pos: *const libc::c_char = self.bounds.first;
            __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::it_4(
                self, _pos,
            )
        });
    }
    pub unsafe fn end(&mut self) -> __gnu_test_input_iterator_wrapper_const_char_ {
        return (unsafe {
            let _pos: *const libc::c_char = self.bounds.last;
            __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::it_4(
                self, _pos,
            )
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_ {
    pub bounds: __gnu_test_BoundsContainer_const_wchar_t_,
}
impl __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_ {
    pub unsafe fn new(mut _first: *const i32, mut _last: *const i32) -> Self {
        let mut this = Self {
            bounds: __gnu_test_BoundsContainer_const_wchar_t_::new({ _first }, { _last }),
        };
        this
    }
    pub unsafe fn it_4(
        &mut self,
        mut pos: *const i32,
    ) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((pos) >= (self.bounds.first)) && ((pos) <= (self.bounds.last))) {
                unimplemented!();
                std::process::abort();
            }
        }
        return __gnu_test_input_iterator_wrapper_const_wchar_t_::new_1({ pos }, {
            (&mut self.bounds as *mut __gnu_test_BoundsContainer_const_wchar_t_)
        });
    }
    pub unsafe fn begin(&mut self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        return (unsafe {
            let _pos: *const i32 = self.bounds.first;
            __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_::it_4(
                self, _pos,
            )
        });
    }
    pub unsafe fn end(&mut self) -> __gnu_test_input_iterator_wrapper_const_wchar_t_ {
        return (unsafe {
            let _pos: *const i32 = self.bounds.last;
            __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_::it_4(
                self, _pos,
            )
        });
    }
}
pub unsafe fn test_15(p: *const Vec<libc::c_char>, mut s: *const libc::c_char) {
    let mut expected: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    let mut oper: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    let mut func: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    let mut input_range: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ =
        __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::new({ s }, {
            s.offset((unimplemented!()) as isize)
        });
    let mut range: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    (unsafe { compare_paths_0(&oper, &expected) });
    (unsafe { compare_paths_0(&func, &expected) });
    (unsafe { compare_paths_0(&range, &expected) });
}
pub unsafe fn test_16(p: *const Vec<libc::c_char>, mut s: *const i32) {
    let mut expected: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    let mut oper: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    let mut func: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    let mut input_range : __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_ = __gnu_test_test_container_const_wchar_t____gnu_test_input_iterator_wrapper_ :: new ( { s } , { s . offset ( ( unimplemented!()  ) as isize ) } , )  ;
    let mut range: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    (unsafe { compare_paths_0(&oper, &expected) });
    (unsafe { compare_paths_0(&func, &expected) });
    (unsafe { compare_paths_0(&range, &expected) });
}
pub unsafe fn test01_17() {
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"/foo/bar"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"/foo/".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"baz"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"baz".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"baz/"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"baz".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"baz"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"/foo/bar".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"baz/"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"/foo/bar".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c""
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c""
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"rel".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"dir/"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"/file".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"dir/"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"file".as_ptr())
    });
}
pub unsafe fn test02_18() {
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"//host"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"foo".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"//host/"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"foo".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"/bar".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"c:/bar".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"c:".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"c:"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"c:foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"/bar".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"c:\\bar".as_ptr())
    });
}
pub unsafe fn test03_19() {
    'loop_: for p in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_paths_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_paths_1))
            .as_ptr()
            .offset((18_i64) as isize),
    ) {
        let p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = (*p)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        'loop_: for q in IterRange::new(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_paths_1)).as_ptr(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_paths_1))
                .as_ptr()
                .offset((18_i64) as isize),
        ) {
            let q: Vec<libc::c_char> = {
                let mut __v: Vec<libc::c_char> = (*q)
                    .path_bytes()
                    .into_iter()
                    .map(|__b| __b as libc::c_char)
                    .collect();
                __v.push(0);
                __v
            };
            (unsafe { test_15(&p, (unimplemented!() as *const ()).cast::<libc::c_char>()) });
            {}
        }
    }
}
pub unsafe fn test04_20() {
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_16(
            &mut _p,
            [47 as i32, 98 as i32, 97 as i32, 114 as i32, 0 as i32].as_ptr(),
        )
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = [102 as i32, 111 as i32, 111 as i32, 0 as i32]
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_15(&mut _p, c"/bar".as_ptr())
    });
    (unsafe {
        let mut _p: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = [102 as i32, 111 as i32, 111 as i32, 0 as i32]
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        test_16(
            &mut _p,
            [47 as i32, 98 as i32, 97 as i32, 114 as i32, 0 as i32].as_ptr(),
        )
    });
}
pub unsafe fn test05_21() {
    let mut s: () = unimplemented!();
    let mut p: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"0/1/2/3/4/5/6"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    unimplemented!();
    let mut expected: Vec<libc::c_char> = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        let mut _p2: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = expected
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        compare_paths_0(&p, &mut _p2)
    });
    let mut p2: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"0/1/2/3/4/5/"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        let mut _p2: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = expected
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        compare_paths_0(&p2, &mut _p2)
    });
    let mut p3: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = c"0/123456789/a"
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    let mut second: () = libcc2rs::next_unsafe(unimplemented!(), 1_i64);
    unimplemented!();
    expected = unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        let mut _p2: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = expected
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        compare_paths_0(&p3, &mut _p2)
    });
}
pub unsafe fn test06_22() {
    let s0: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"a/b/c".as_ptr(),
        libcc2rs::c_str_len(c"a/b/c".as_ptr()) + 1,
    )
    .to_vec();
    let mut p: Vec<libc::c_char> = {
        let mut __v: Vec<libc::c_char> = s0
            .path_bytes()
            .into_iter()
            .map(|__b| __b as libc::c_char)
            .collect();
        __v.push(0);
        __v
    };
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    let mut i: i32 = 0;
    'loop_: while ((i) < (10)) {
        unimplemented!();
        i.prefix_inc();
    }
    (unsafe { test_15(&p, s.as_ptr()) });
    let mut p2: Vec<libc::c_char> = unimplemented!();
    (unsafe { test_15(&p2, s.as_ptr()) });
}
pub unsafe fn test07_23() {
    let mut p: Vec<libc::c_char> = unimplemented!();
    let mut p0: Vec<libc::c_char> = unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    (unsafe { compare_paths_0(&p, &p0) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_17() });
    (unsafe { test02_18() });
    (unsafe { test03_19() });
    (unsafe { test04_20() });
    (unsafe { test05_21() });
    (unsafe { test06_22() });
    (unsafe { test07_23() });
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
pub unsafe fn set_memory_limits_24(mut size: Option<f32>) {
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
pub unsafe fn set_file_limit_25(mut size: u64) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_26(
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
pub unsafe fn run_tests_wrapped_locale_27(
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
pub unsafe fn run_tests_wrapped_env_28(
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
pub static mut count_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut count__7: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut throw_on__8: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut count__9: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut throw_on__10: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut _M_count_11: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
static mut next_id__12: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
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
pub unsafe fn test_tm_29(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_30));
}
static mut tmp_30: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_paths_1);
    std::cell::LazyCell::force(&*&raw const count_6);
    std::cell::LazyCell::force(&*&raw const count__7);
    std::cell::LazyCell::force(&*&raw const throw_on__8);
    std::cell::LazyCell::force(&*&raw const count__9);
    std::cell::LazyCell::force(&*&raw const throw_on__10);
    std::cell::LazyCell::force(&*&raw const _M_count_11);
    std::cell::LazyCell::force(&*&raw const next_id__12);
}
