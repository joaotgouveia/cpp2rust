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
pub struct __gnu_test_tracker_allocator_counter {}
impl __gnu_test_tracker_allocator_counter {
    pub unsafe fn allocate(mut blocksize: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut allocationCount__10)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut allocationCount__10))
                .wrapping_add(blocksize);
    }
    pub unsafe fn construct() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructCount__12)).prefix_inc();
    }
    pub unsafe fn destroy() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destructCount__13)).prefix_inc();
    }
    pub unsafe fn deallocate(mut blocksize: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocationCount__11)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocationCount__11))
                .wrapping_add(blocksize);
    }
    pub unsafe fn get_allocation_count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut allocationCount__10));
    }
    pub unsafe fn get_deallocation_count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocationCount__11));
    }
    pub unsafe fn get_construct_count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut constructCount__12));
    }
    pub unsafe fn get_destruct_count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut destructCount__13));
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut allocationCount__10)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocationCount__11)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructCount__12)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destructCount__13)) = 0_();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_uneq_allocator_base {}
impl __gnu_test_uneq_allocator_base {
    pub unsafe fn get_map() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_map_14));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_SimpleAllocator_char_ {}
impl __gnu_test_SimpleAllocator_char_ {}
pub unsafe fn operator_eq_15(
    mut _a0: __gnu_test_NullablePointer_void_,
    mut _a1: __gnu_test_NullablePointer_void_,
) -> () {
    return true;
}
pub unsafe fn operator_ne_16(
    mut _a0: __gnu_test_NullablePointer_void_,
    mut _a1: __gnu_test_NullablePointer_void_,
) -> () {
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_NullablePointer_void_ {}
impl __gnu_test_NullablePointer_void_ {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn to_(&self) -> () {
        return false;
    }
}
impl std::cmp::PartialEq for __gnu_test_NullablePointer_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for __gnu_test_NullablePointer_void_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_NullablePointer_void_ptr_ {
    value: (),
}
impl __gnu_test_NullablePointer_void_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_NullablePointer_const_void_ptr_ {
    value: (),
}
impl __gnu_test_NullablePointer_const_void_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_PointerBase_void_void_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_PointerBase_void_const_void_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct __gnu_test_default_resource_mgr {
    pub prev: *mut (),
}
impl __gnu_test_default_resource_mgr {
    pub unsafe fn new(mut r: *mut ()) -> Self {
        let mut this = Self {
            prev: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
    }
}
static mut alloc_map_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test_recommended_practice_17() {
    let mut s: () = unimplemented!();
    const _: () = assert!(true, "stream_extractable<parse_manip, char>");;
    const _: () = assert!(!(false), "not stream_extractable<parse_manip, wchar_t>");;
    const _: () = assert!(true, "stream_extractable<wparse_manip, wchar_t>");;
    const _: () = assert!(!(false), "not stream_extractable<wparse_manip, char>");;
    const _: () = assert!(true, "not std::is_move_constructible_v<parse_manip>");;
    const _: () = assert!(true, "not std::is_move_assignable_v<parse_manip>");;
    const _: () = assert!(!(false), "not stream_extractable<parse_manip&, char>");;
    const _: () = assert!(!(false), "not stream_extractable<const parse_manip&, char>");;
    const _: () = assert!(!(false), "not stream_extractable<wparse_manip&, wchar_t>");;
    const _: () = assert!(
        !(false),
        "not stream_extractable<const wparse_manip&, wchar_t>"
    );;
}
pub static mut f_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N_A {}
pub unsafe fn from_stream_19(_a0: *mut (), mut fmt: *const (), _a2: *mut N_A) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((fmt) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (45_()).into(),
                        (c"void N::from_stream(std::istream &, const char *, A &)".as_ptr()).into(),
                        (c"fmt == f.c_str()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N_B {}
pub unsafe fn from_stream_21(_a0: *mut (), mut fmt: *const (), _a2: *mut N_B, mut abbrev: *mut ()) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((fmt) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (57_()).into(),
                        (c"void N::from_stream(std::istream &, const char *, B &, std::string *)"
                            .as_ptr())
                        .into(),
                        (c"fmt == f.c_str()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((abbrev).is_null())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (58_()).into(),
                        (c"void N::from_stream(std::istream &, const char *, B &, std::string *)"
                            .as_ptr())
                        .into(),
                        (c"abbrev != nullptr".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N_C {}
pub unsafe fn from_stream_22(
    _a0: *mut (),
    mut fmt: *const (),
    _a2: *mut N_C,
    mut abbrev: *mut (),
    mut offset: *mut (),
) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((fmt) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"parse.cc" .as_ptr() ) .into() , ( 70_() ) .into() , ( c"void N::from_stream(std::istream &, const char *, C &, std::string *, std::chrono::minutes *)" .as_ptr() ) .into() , ( c"fmt == f.c_str()" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((abbrev).is_null()) {
            (unsafe {
                __builtin_fprintf_20 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"parse.cc" .as_ptr() ) .into() , ( 71_() ) .into() , ( c"void N::from_stream(std::istream &, const char *, C &, std::string *, std::chrono::minutes *)" .as_ptr() ) .into() , ( c"abbrev == nullptr" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((offset).is_null())) {
            (unsafe {
                __builtin_fprintf_20 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"parse.cc" .as_ptr() ) .into() , ( 72_() ) .into() , ( c"void N::from_stream(std::istream &, const char *, C &, std::string *, std::chrono::minutes *)" .as_ptr() ) .into() , ( c"offset != nullptr" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N_D {}
pub unsafe fn from_stream_23(
    _a0: *mut (),
    mut fmt: *const (),
    _a2: *mut N_D,
    mut abbrev: *mut (),
    mut offset: *mut (),
) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((fmt) == (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"parse.cc" .as_ptr() ) .into() , ( 81_() ) .into() , ( c"void N::from_stream(std::istream &, const char *, D &, std::string *, std::chrono::minutes *)" .as_ptr() ) .into() , ( c"fmt == f.c_str()" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((abbrev).is_null())) {
            (unsafe {
                __builtin_fprintf_20 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"parse.cc" .as_ptr() ) .into() , ( 82_() ) .into() , ( c"void N::from_stream(std::istream &, const char *, D &, std::string *, std::chrono::minutes *)" .as_ptr() ) .into() , ( c"abbrev != nullptr" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(!((offset).is_null())) {
            (unsafe {
                __builtin_fprintf_20 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"parse.cc" .as_ptr() ) .into() , ( 83_() ) .into() , ( c"void N::from_stream(std::istream &, const char *, D &, std::string *, std::chrono::minutes *)" .as_ptr() ) .into() , ( c"offset != nullptr" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N_E {}
pub unsafe fn from_stream_24(
    _a0: *mut (),
    mut _a1: *const (),
    _a2: *mut N_E,
    mut _a3: Option<*mut ()>,
    mut _a4: Option<*mut ()>,
) {
    let mut _a3: *mut () = _a3.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    let mut _a4: *mut () = _a4.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
}
pub unsafe fn test_adl_25() {
    let mut abbrev: () = unimplemented!();
    let mut offset: () = unimplemented!();
    let mut a: N_A = <N_A>::default();
    &(unimplemented!());
    let mut b: N_B = <N_B>::default();
    &(unsafe {
        parse_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut f_18)),
            &mut b,
            &mut abbrev,
        )
    });
    let mut c: N_C = <N_C>::default();
    &(unsafe {
        parse_27(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut f_18)),
            &mut c,
            &mut offset,
        )
    });
    const _: () = assert!(true, "parsable<const char*, N::C, string, minutes>");;
    let mut d: N_D = <N_D>::default();
    &(unsafe {
        parse_28(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut f_18)),
            &mut d,
            &mut abbrev,
            &mut offset,
        )
    });
    const _: () = assert!(true, "parsable<const char*, N::D, minutes>");;
    const _: () = assert!(true, "parsable<const wchar_t*, N::E, wstring>");;
    const _: () = assert!(true, "parsable<const wchar_t*, N::E, wstring>");;
    const _: () = assert!(true, "parsable<const wchar_t*, N::E, minutes>");;
    const _: () = assert!(true, "parsable<const wchar_t*, N::E, wstring, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::A, std::string>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::A, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::A, string, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::B>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::B, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::B, string, minutes>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::C>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::C, string>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::D>");;
    const _: () = assert!(!(false), "not parsable<const char*, N::D, string>");;
    const _: () = assert!(!(false), "not parsable<string, std::chrono::year, wstring>");;
    const _: () = assert!(
        !(false),
        "not parsable<string, std::chrono::year, wstring, minutes>"
    );;
    const _: () = assert!(true, "parsable<String, std::chrono::year>");;
    const _: () = assert!(true, "parsable<String, std::chrono::year, String>");;
    const _: () = assert!(true, "parsable<String, std::chrono::year, minutes>");;
    const _: () = assert!(true, "parsable<String, std::chrono::year, String, minutes>");;
    const _: () = assert!(true, "parsable<const char*, std::chrono::year, String>");;
    const _: () = assert!(
        true,
        "parsable<const char*, std::chrono::year, String, minutes>"
    );;
    const _: () = assert!(!(false), "not parsable<string, std::chrono::year, String>");;
    const _: () = assert!(
        !(false),
        "not parsable<string, std::chrono::year, String, minutes>"
    );;
    const _: () = assert!(!(false), "not parsable<String, std::chrono::year, string>");;
    const _: () = assert!(
        !(false),
        "not parsable<String, std::chrono::year, string, minutes>"
    );;
}
pub unsafe fn test_whitespace_29() {
    let mut min: () = unimplemented!();
    let mut is: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (169_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (170_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"min == 1min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (173_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"is.eof() && !is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (174_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"min == 1min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (178_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (182_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (183_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"min == 1min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (186_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (187_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"min == 1min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (190_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"is.eof() && !is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (191_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"min == 1min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (195_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"is.eof() && !is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (196_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"min == 1min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (200_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"is.eof() && is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (201_()).into(),
                        (c"void test_whitespace()".as_ptr()).into(),
                        (c"min == 1min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_errors_30() {
    let mut min: () = unimplemented!();
    let mut y: () = unimplemented!();
    let mut is: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (214_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"!is.eof() && is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (215_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"min == 999min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (220_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"!is.eof() && is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (221_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"min == 999min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (226_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"!is.eof() && is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (227_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"min == 999min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (232_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"!is.eof() && is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (233_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"min == 999min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (238_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (239_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"min == (12h + 15min)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (245_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (246_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"y == -1y".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (251_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"is.eof() && ! is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (252_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"y == 2010y".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (258_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (259_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"min == -1min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (264_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (265_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"y == 2010y".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (270_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (271_()).into(),
                        (c"void test_errors()".as_ptr()).into(),
                        (c"y == 2010y".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_modifiers_31() {
    let mut min: () = unimplemented!();
    let mut is: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (283_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (284_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"min == 60min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (288_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (289_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"min == 62min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (!(unimplemented!()))) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (293_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"is.eof() && !is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (294_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"min == 2min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (299_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (300_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"min == 6min".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((!(unimplemented!())) && (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (305_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"!is.eof() && is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) && (unimplemented!())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (310_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"is.eof() && is.fail()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (316_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (317_()).into(),
                        (c"void test_modifiers()".as_ptr()).into(),
                        (c"s == 12s".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test_wchar_32() {
    let mut is: () = unimplemented!();
    let mut ms: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (329_()).into(),
                        (c"void test_wchar()".as_ptr()).into(),
                        (c"is.good()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (125_())) {
            (unsafe {
                __builtin_fprintf_20(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"parse.cc".as_ptr()).into(),
                        (330_()).into(),
                        (c"void test_wchar()".as_ptr()).into(),
                        (c"ms.count() == 125".as_ptr()).into(),
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
    (unsafe { test_recommended_practice_17() });
    (unsafe { test_adl_25() });
    (unsafe { test_whitespace_29() });
    (unsafe { test_errors_30() });
    (unsafe { test_modifiers_31() });
    (unsafe { test_wchar_32() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_33() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_34(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_35(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_36(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_37(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_38(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_37(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_38(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_37((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_38(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_39(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_37(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_38(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_40(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_41(
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
pub unsafe fn run_tests_wrapped_locale_42(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_20 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_43(
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
        if ((self.pid_) == (unsafe { getpid_33() })) {
            (unsafe { semctl_35(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_44(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_44(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_45(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_46));
}
static mut tmp_46: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
static mut allocationCount__10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
static mut deallocationCount__11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
static mut constructCount__12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
static mut destructCount__13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn check_construct_destroy_47(
    mut tag: *const (),
    mut expected_c: (),
    mut expected_d: (),
) -> () {
    let mut ret: () = true;
    if ((unsafe { __gnu_test_tracker_allocator_counter::get_construct_count() }) != (expected_c))
        || ((unsafe { __gnu_test_tracker_allocator_counter::get_destruct_count() }) != (expected_d))
    {
        unimplemented!();
        ret = false;
    }
    return ret;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const f_18);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
    std::cell::LazyCell::force(&*&raw const allocationCount__10);
    std::cell::LazyCell::force(&*&raw const deallocationCount__11);
    std::cell::LazyCell::force(&*&raw const constructCount__12);
    std::cell::LazyCell::force(&*&raw const destructCount__13);
}
