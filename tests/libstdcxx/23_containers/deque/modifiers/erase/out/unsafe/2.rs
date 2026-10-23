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
pub static mut A_10: std::cell::LazyCell<[(); 21]> = std::cell::LazyCell::new(|| unsafe {
    [
        -5_(),
        -4_(),
        -3_(),
        -2_(),
        -1_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
    ]
});
pub static mut A0_11: std::cell::LazyCell<[(); 18]> = std::cell::LazyCell::new(|| unsafe {
    [
        -5_(),
        -1_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
    ]
});
pub static mut A1_12: std::cell::LazyCell<[(); 17]> = std::cell::LazyCell::new(|| unsafe {
    [
        -5_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
    ]
});
pub static mut A2_13: std::cell::LazyCell<[(); 12]> = std::cell::LazyCell::new(|| unsafe {
    [
        -5_(),
        0_(),
        1_(),
        2_(),
        8_(),
        9_(),
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
    ]
});
pub static mut A3_14: std::cell::LazyCell<[(); 8]> =
    std::cell::LazyCell::new(|| unsafe { [-5_(), 0_(), 1_(), 2_(), 8_(), 9_(), 10_(), 11_()] });
pub static mut A4_15: std::cell::LazyCell<[(); 5]> =
    std::cell::LazyCell::new(|| unsafe { [2_(), 8_(), 9_(), 10_(), 11_()] });
pub static mut A5_16: std::cell::LazyCell<[(); 4]> =
    std::cell::LazyCell::new(|| unsafe { [2_(), 8_(), 10_(), 11_()] });
pub static mut A6_17: std::cell::LazyCell<[(); 3]> =
    std::cell::LazyCell::new(|| unsafe { [2_(), 8_(), 10_()] });
pub static mut N_18: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[(); 21]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()))
});
pub static mut N0_19: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[(); 18]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()))
});
pub static mut N1_20: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[(); 17]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()))
});
pub static mut N2_21: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()))
});
pub static mut N3_22: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[(); 8]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()))
});
pub static mut N4_23: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()))
});
pub static mut N5_24: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[(); 4]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()))
});
pub static mut N6_25: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[(); 3]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct My_class_1_ {
    dummy: [(); 1],
    data: (),
}
impl My_class_1_ {
    pub unsafe fn new(mut num: ()) -> Self {
        let mut this = Self {
            dummy: [unimplemented!(); 1],
            data: num,
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.data;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct My_class_8_ {
    dummy: [(); 8],
    data: (),
}
impl My_class_8_ {
    pub unsafe fn new(mut num: ()) -> Self {
        let mut this = Self {
            dummy: [unimplemented!(); 8],
            data: num,
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.data;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct My_class_32_ {
    dummy: [(); 32],
    data: (),
}
impl My_class_32_ {
    pub unsafe fn new(mut num: ()) -> Self {
        let mut this = Self {
            dummy: [unimplemented!(); 32],
            data: num,
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.data;
    }
}
pub unsafe fn test01_26() {
    let mut v: () = std_deque_My_class_1___std_allocator_My_class_1___::new_1(
        { (*std::cell::LazyCell::force_mut(&mut *&raw mut A_10)).as_ptr() },
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut A_10))
                .as_ptr()
                .offset((*std::cell::LazyCell::force_mut(&mut *&raw mut N_18)) as isize)
        },
        None,
    );
    let mut it0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (67_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"it0 == v.begin() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N0_19))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (68_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"v.size() == N0".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (69_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A0)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (72_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"it1 == v.begin() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N1_20))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (73_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"v.size() == N1".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (74_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (77_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"it2 == v.begin() + 4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N2_21))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (78_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"v.size() == N2".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (79_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (82_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"it3 == v.begin() + 8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N3_22))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (83_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"v.size() == N3".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (84_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A3)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (87_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"it4 == v.begin()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N4_23))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (88_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"v.size() == N4".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (89_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A4)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"it5 == v.begin() + 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N5_24))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"v.size() == N5".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A5)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it6: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (97_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"it6 == v.begin() + 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N6_25))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (98_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"v.size() == N6".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (99_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A6)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it7: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (102_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"it7 == v.begin()".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (103_()).into(),
                        (c"void test01() [T = My_class<1>]".as_ptr()).into(),
                        (c"v.empty()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test01_28() {
    let mut v: () = std_deque_My_class_8___std_allocator_My_class_8___::new_1(
        { (*std::cell::LazyCell::force_mut(&mut *&raw mut A_10)).as_ptr() },
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut A_10))
                .as_ptr()
                .offset((*std::cell::LazyCell::force_mut(&mut *&raw mut N_18)) as isize)
        },
        None,
    );
    let mut it0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (67_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"it0 == v.begin() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N0_19))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (68_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"v.size() == N0".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (69_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A0)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (72_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"it1 == v.begin() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N1_20))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (73_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"v.size() == N1".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (74_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (77_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"it2 == v.begin() + 4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N2_21))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (78_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"v.size() == N2".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (79_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (82_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"it3 == v.begin() + 8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N3_22))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (83_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"v.size() == N3".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (84_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A3)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (87_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"it4 == v.begin()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N4_23))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (88_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"v.size() == N4".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (89_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A4)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"it5 == v.begin() + 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N5_24))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"v.size() == N5".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A5)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it6: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (97_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"it6 == v.begin() + 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N6_25))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (98_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"v.size() == N6".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (99_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A6)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it7: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (102_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"it7 == v.begin()".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (103_()).into(),
                        (c"void test01() [T = My_class<8>]".as_ptr()).into(),
                        (c"v.empty()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
}
pub unsafe fn test01_29() {
    let mut v: () = std_deque_My_class_32___std_allocator_My_class_32___::new_1(
        { (*std::cell::LazyCell::force_mut(&mut *&raw mut A_10)).as_ptr() },
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut A_10))
                .as_ptr()
                .offset((*std::cell::LazyCell::force_mut(&mut *&raw mut N_18)) as isize)
        },
        None,
    );
    let mut it0: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (67_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"it0 == v.begin() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N0_19))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (68_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"v.size() == N0".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (69_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A0)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it1: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (72_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"it1 == v.begin() + 1".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N1_20))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (73_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"v.size() == N1".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (74_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A1)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it2: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (77_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"it2 == v.begin() + 4".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N2_21))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (78_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"v.size() == N2".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (79_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A2)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it3: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (82_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"it3 == v.begin() + 8".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N3_22))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (83_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"v.size() == N3".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (84_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A3)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it4: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (87_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"it4 == v.begin()".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N4_23))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (88_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"v.size() == N4".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (89_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A4)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it5: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (92_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"it5 == v.begin() + 2".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N5_24))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (93_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"v.size() == N5".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (94_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A5)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it6: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (97_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"it6 == v.begin() + 3".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((unimplemented!()) == (*std::cell::LazyCell::force_mut(&mut *&raw mut N6_25))) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (98_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"v.size() == N6".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (99_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"std::equal(v.begin(), v.end(), A6)".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut it7: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(unimplemented!()) {
            (unsafe {
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (102_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"it7 == v.begin()".as_ptr()).into(),
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
                __builtin_fprintf_27(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"2.cc".as_ptr()).into(),
                        (103_()).into(),
                        (c"void test01() [T = My_class<32>]".as_ptr()).into(),
                        (c"v.empty()".as_ptr()).into(),
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
    (unsafe { test01_26() });
    (unsafe { test01_28() });
    (unsafe { test01_29() });
    return 0_();
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_30() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_31(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_32(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_33(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_34(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_35(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_34(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_35(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_34((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_35(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_36(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_34(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_35(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_37(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_38(
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
    let mut w: () =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1({ wanted }, None);
    if unimplemented!() {
        (unsafe { __throw_runtime_error_3(s) });
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_39(mut name: *const (), l: *const __gnu_test_func_callback) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut res: *const () = (unimplemented!()).cast_const();
    if !(res).is_null() {
        let mut preLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { res },
                None,
            );
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (unimplemented!()).cast_const() },
                None,
            );
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_27 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"LC_ALL for ".as_ptr() },
            None,
        );
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub unsafe fn run_tests_wrapped_env_40(
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
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { env },
            None,
        );
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
        if ((self.pid_) == (unsafe { getpid_30() })) {
            (unsafe { semctl_32(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_41(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_41(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_42(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_43));
}
static mut tmp_43: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const A_10);
    std::cell::LazyCell::force(&*&raw const A0_11);
    std::cell::LazyCell::force(&*&raw const A1_12);
    std::cell::LazyCell::force(&*&raw const A2_13);
    std::cell::LazyCell::force(&*&raw const A3_14);
    std::cell::LazyCell::force(&*&raw const A4_15);
    std::cell::LazyCell::force(&*&raw const A5_16);
    std::cell::LazyCell::force(&*&raw const A6_17);
    std::cell::LazyCell::force(&*&raw const N_18);
    std::cell::LazyCell::force(&*&raw const N0_19);
    std::cell::LazyCell::force(&*&raw const N1_20);
    std::cell::LazyCell::force(&*&raw const N2_21);
    std::cell::LazyCell::force(&*&raw const N3_22);
    std::cell::LazyCell::force(&*&raw const N4_23);
    std::cell::LazyCell::force(&*&raw const N5_24);
    std::cell::LazyCell::force(&*&raw const N6_25);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
