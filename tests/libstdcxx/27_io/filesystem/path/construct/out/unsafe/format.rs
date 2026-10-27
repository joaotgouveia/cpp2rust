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
pub struct __gnu_test_BoundsContainer_const_char_ {
    pub first: *const (),
    pub last: *const (),
}
impl __gnu_test_BoundsContainer_const_char_ {
    pub unsafe fn new(mut _first: *const (), mut _last: *const ()) -> Self {
        let mut this = Self {
            first: _first,
            last: _last,
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_input_iterator_wrapper_const_char_ {
    pub ptr: *const (),
    pub SharedInfo: *mut __gnu_test_BoundsContainer_const_char_,
}
impl __gnu_test_input_iterator_wrapper_const_char_ {
    pub unsafe fn new_1(
        mut _ptr: *const (),
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
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"format.cc" .as_ptr() ) .into() , ( 250_() ) .into() , ( c"__gnu_test::input_iterator_wrapper<const char>::input_iterator_wrapper(T *, ContainerType *) [T = const char]" .as_ptr() ) .into() , ( c"ptr >= SharedInfo->first && ptr <= SharedInfo->last" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        this
    }
    pub unsafe fn operator_eq(
        &self,
        in_: *const __gnu_test_input_iterator_wrapper_const_char_,
    ) -> () {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!(self.SharedInfo).is_null()) && ((self.SharedInfo) == ((*in_).SharedInfo))) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"format.cc" .as_ptr() ) .into() , ( 262_() ) .into() , ( c"bool __gnu_test::input_iterator_wrapper<const char>::operator==(const input_iterator_wrapper<T> &) const [T = const char]" .as_ptr() ) .into() , ( c"SharedInfo && SharedInfo == in.SharedInfo" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((self.ptr) >= ((*self.SharedInfo).first))
                && (((*in_).ptr) >= ((*self.SharedInfo).first)))
            {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"format.cc" .as_ptr() ) .into() , ( 263_() ) .into() , ( c"bool __gnu_test::input_iterator_wrapper<const char>::operator==(const input_iterator_wrapper<T> &) const [T = const char]" .as_ptr() ) .into() , ( c"ptr>=SharedInfo->first && in.ptr>=SharedInfo->first" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return ((self.ptr) == ((*in_).ptr));
    }
    pub unsafe fn operator_ne(
        &self,
        in_: *const __gnu_test_input_iterator_wrapper_const_char_,
    ) -> () {
        return !(unsafe {
            let _in_: *const __gnu_test_input_iterator_wrapper_const_char_ = in_;
            __gnu_test_input_iterator_wrapper_const_char_::operator_eq(
                &(*(self as *const __gnu_test_input_iterator_wrapper_const_char_)),
                _in_,
            )
        });
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!(self.SharedInfo).is_null()) && ((self.ptr) < ((*self.SharedInfo).last))) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"format.cc" .as_ptr() ) .into() , ( 276_() ) .into() , ( c"T &__gnu_test::input_iterator_wrapper<const char>::operator*() const [T = const char]" .as_ptr() ) .into() , ( c"SharedInfo && ptr < SharedInfo->last" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((self.ptr) >= ((*self.SharedInfo).first)) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"format.cc" .as_ptr() ) .into() , ( 277_() ) .into() , ( c"T &__gnu_test::input_iterator_wrapper<const char>::operator*() const [T = const char]" .as_ptr() ) .into() , ( c"ptr >= SharedInfo->first" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return &(*self.ptr);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut __gnu_test_input_iterator_wrapper_const_char_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((!(self.SharedInfo).is_null()) && ((self.ptr) < ((*self.SharedInfo).last))) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"format.cc" .as_ptr() ) .into() , ( 290_() ) .into() , ( c"input_iterator_wrapper<T> &__gnu_test::input_iterator_wrapper<const char>::operator++() [T = const char]" .as_ptr() ) .into() , ( c"SharedInfo && ptr < SharedInfo->last" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((self.ptr) >= ((*self.SharedInfo).first)) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"format.cc" .as_ptr() ) .into() , ( 291_() ) .into() , ( c"input_iterator_wrapper<T> &__gnu_test::input_iterator_wrapper<const char>::operator++() [T = const char]" .as_ptr() ) .into() , ( c"ptr>=SharedInfo->first" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ {
    pub bounds: __gnu_test_BoundsContainer_const_char_,
}
impl __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ {
    pub unsafe fn new(mut _first: *const (), mut _last: *const ()) -> Self {
        let mut this = Self {
            bounds: __gnu_test_BoundsContainer_const_char_::new({ _first }, { _last }),
        };
        this
    }
    pub unsafe fn it_4(
        &mut self,
        mut pos: *const (),
    ) -> __gnu_test_input_iterator_wrapper_const_char_ {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((pos) >= (self.bounds.first)) && ((pos) <= (self.bounds.last))) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"format.cc" .as_ptr() ) .into() , ( 628_() ) .into() , ( c"ItType<T> __gnu_test::test_container<const char, __gnu_test::input_iterator_wrapper>::it(T *) [T = const char, ItType = __gnu_test::input_iterator_wrapper]" .as_ptr() ) .into() , ( c"pos >= bounds.first && pos <= bounds.last" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        return __gnu_test_input_iterator_wrapper_const_char_::new_1({ pos }, {
            (&mut self.bounds as *mut __gnu_test_BoundsContainer_const_char_)
        });
    }
    pub unsafe fn begin(&mut self) -> __gnu_test_input_iterator_wrapper_const_char_ {
        return (unsafe {
            let _pos: *const () = self.bounds.first;
            __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::it_4(
                self, _pos,
            )
        });
    }
    pub unsafe fn end(&mut self) -> __gnu_test_input_iterator_wrapper_const_char_ {
        return (unsafe {
            let _pos: *const () = self.bounds.last;
            __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::it_4(
                self, _pos,
            )
        });
    }
}
pub unsafe fn test01_11() {
    let mut s: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
        unsafe {
            return unimplemented!();
        }
    });
    let mut p0: () = unimplemented!();
    let mut p1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (34_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"p1 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (36_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"p2 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (38_()).into(),
                        (c"void test01()".as_ptr()).into(),
                        (c"p3 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test02_12() {
    let s: () = unimplemented!();
    let mut p0: () = unimplemented!();
    let mut p1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (48_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"p1 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (50_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"p2 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (52_()).into(),
                        (c"void test02()".as_ptr()).into(),
                        (c"p3 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test03_13() {
    let s: () = unimplemented!();
    let mut p0: () = unimplemented!();
    let mut p1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (62_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"p1 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (64_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"p2 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (66_()).into(),
                        (c"void test03()".as_ptr()).into(),
                        (c"p3 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test04_14() {
    let s: () = unimplemented!();
    let mut p0: () = unimplemented!();
    let mut p1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (77_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"p1 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (79_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"p2 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (81_()).into(),
                        (c"void test04()".as_ptr()).into(),
                        (c"p3 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test05_15() {
    let mut s: *const () = c"foo/bar".as_ptr();
    let mut p0: () = unimplemented!();
    let mut p1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"p1 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"p2 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (96_()).into(),
                        (c"void test05()".as_ptr()).into(),
                        (c"p3 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test06_16() {
    let s: [(); 8] = std::mem::transmute(*b"foo/bar\0");
    let c: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ =
        __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::new(
            { s.as_ptr() },
            {
                s.as_ptr()
                    .offset((unsafe { strlen_17(s.as_ptr()) }) as isize)
            },
        );
    let mut c0: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ = c;
    let mut p0: () = unimplemented!();
    let mut c1: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ = c;
    let mut p1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (110_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"p1 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut c2: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ = c;
    let mut p2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (113_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"p2 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut c3: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ = c;
    let mut p3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (116_()).into(),
                        (c"void test06()".as_ptr()).into(),
                        (c"p3 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test07_18() {
    let mut s: *const () = c"foo/bar".as_ptr();
    let mut loc: () = unimplemented!();
    let mut p0: () = unimplemented!();
    let mut p1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (127_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"p1 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (129_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"p2 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut p3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (131_()).into(),
                        (c"void test07()".as_ptr()).into(),
                        (c"p3 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test08_19() {
    let s: [(); 8] = std::mem::transmute(*b"foo/bar\0");
    let c: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ =
        __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_::new(
            { s.as_ptr() },
            {
                s.as_ptr()
                    .offset((unsafe { strlen_17(s.as_ptr()) }) as isize)
            },
        );
    let mut loc: () = unimplemented!();
    let mut c0: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ = c;
    let mut p0: () = unimplemented!();
    let mut c1: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ = c;
    let mut p1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (146_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"p1 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut c2: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ = c;
    let mut p2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (149_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"p2 == p0".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut c3: __gnu_test_test_container_const_char____gnu_test_input_iterator_wrapper_ = c;
    let mut p3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"format.cc".as_ptr()).into(),
                        (152_()).into(),
                        (c"void test08()".as_ptr()).into(),
                        (c"p3 == p0".as_ptr()).into(),
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
    (unsafe { test01_11() });
    (unsafe { test02_12() });
    (unsafe { test03_13() });
    (unsafe { test04_14() });
    (unsafe { test05_15() });
    (unsafe { test06_16() });
    (unsafe { test07_18() });
    (unsafe { test08_19() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_20() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_21(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_22(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_23(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_24(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_25(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_24(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_25(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_24((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_25(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_26(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_24(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_25(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_27(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_28(
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
pub unsafe fn run_tests_wrapped_locale_29(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_30(
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
        if ((self.pid_) == (unsafe { getpid_20() })) {
            (unsafe { semctl_22(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_31(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_31(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_32(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_33));
}
static mut tmp_33: std::cell::LazyCell<::libc::tm> =
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
