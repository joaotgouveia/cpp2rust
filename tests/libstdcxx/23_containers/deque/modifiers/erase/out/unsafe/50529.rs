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
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct __gnu_test_rvalstruct {
    pub val: (),
    pub valid: (),
}
impl __gnu_test_rvalstruct {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            val: 0_(),
            valid: true,
        };
        this
    }
    pub unsafe fn new_2(mut inval: ()) -> Self {
        let mut this = Self {
            val: inval,
            valid: true,
        };
        this
    }
    pub unsafe fn operator_assign_3(&mut self, mut newval: ()) -> *mut __gnu_test_rvalstruct {
        self.val = newval;
        self.valid = true;
        return &mut (*(self as *mut __gnu_test_rvalstruct));
    }
    pub unsafe fn move_from(in_: *mut __gnu_test_rvalstruct) -> Self {
        let mut this = Self {
            val: unimplemented!(),
            valid: unimplemented!(),
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid) == (true)) {
                (unsafe {
                    __builtin_fprintf_10(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"50529.cc".as_ptr()).into(),
                            (56_()).into(),
                            (c"__gnu_test::rvalstruct::rvalstruct(rvalstruct &&)".as_ptr()).into(),
                            (c"in.valid == true".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        this.val = (*in_).val;
        (*in_).valid = false;
        this.valid = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        in_: *mut __gnu_test_rvalstruct,
    ) -> *mut __gnu_test_rvalstruct {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((self as *mut __gnu_test_rvalstruct) != (in_)) {
                (unsafe {
                    __builtin_fprintf_10(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"50529.cc".as_ptr()).into(),
                            (68_()).into(),
                            (c"rvalstruct &__gnu_test::rvalstruct::operator=(rvalstruct &&)"
                                .as_ptr())
                            .into(),
                            (c"this != &in".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid) == (true)) {
                (unsafe {
                    __builtin_fprintf_10(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"50529.cc".as_ptr()).into(),
                            (69_()).into(),
                            (c"rvalstruct &__gnu_test::rvalstruct::operator=(rvalstruct &&)"
                                .as_ptr())
                            .into(),
                            (c"in.valid == true".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        self.val = (*in_).val;
        (*in_).valid = false;
        self.valid = true;
        return &mut (*(self as *mut __gnu_test_rvalstruct));
    }
}
impl std::cmp::Ord for __gnu_test_rvalstruct {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_11(
                self as *const __gnu_test_rvalstruct,
                other as *const __gnu_test_rvalstruct,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_11(
                other as *const __gnu_test_rvalstruct,
                self as *const __gnu_test_rvalstruct,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_rvalstruct {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_rvalstruct {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const __gnu_test_rvalstruct,
                other as *const __gnu_test_rvalstruct,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_rvalstruct {}
impl Default for __gnu_test_rvalstruct {
    fn default() -> Self {
        unsafe { __gnu_test_rvalstruct::new_1() }
    }
}
pub unsafe fn operator_eq_12(
    lhs: *const __gnu_test_rvalstruct,
    rhs: *const __gnu_test_rvalstruct,
) -> () {
    return (((*lhs).val) == ((*rhs).val));
}
pub unsafe fn operator_lt_11(
    lhs: *const __gnu_test_rvalstruct,
    rhs: *const __gnu_test_rvalstruct,
) -> () {
    return (((*lhs).val) < ((*rhs).val));
}
pub unsafe fn swap_13(lhs: *mut __gnu_test_rvalstruct, rhs: *mut __gnu_test_rvalstruct) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*lhs).valid) && ((*rhs).valid)) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"50529.cc".as_ptr()).into(),
                        (88_()).into(),
                        (c"void __gnu_test::swap(rvalstruct &, rvalstruct &)".as_ptr()).into(),
                        (c"lhs.valid && rhs.valid".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut temp: () = (*lhs).val;
    (*lhs).val = (*rhs).val;
    (*rhs).val = temp;
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_copycounter {
    pub val: (),
    pub valid: (),
}
impl __gnu_test_copycounter {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            val: 0_(),
            valid: true,
        };
        this
    }
    pub unsafe fn new_2(mut inval: ()) -> Self {
        let mut this = Self {
            val: inval,
            valid: true,
        };
        this
    }
    pub unsafe fn copy_from(in_: *const __gnu_test_copycounter) -> Self {
        let mut this = Self {
            val: (*in_).val,
            valid: true,
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid) == (true)) {
                (unsafe {
                    __builtin_fprintf_10(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"50529.cc".as_ptr()).into(),
                            (112_()).into(),
                            (c"__gnu_test::copycounter::copycounter(const copycounter &)".as_ptr())
                                .into(),
                            (c"in.valid == true".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_14)).prefix_inc();
        this
    }
    pub unsafe fn move_from(in_: *mut __gnu_test_copycounter) -> Self {
        let mut this = Self {
            val: unimplemented!(),
            valid: unimplemented!(),
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid) == (true)) {
                (unsafe {
                    __builtin_fprintf_10(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"50529.cc".as_ptr()).into(),
                            (118_()).into(),
                            (c"__gnu_test::copycounter::copycounter(copycounter &&)".as_ptr())
                                .into(),
                            (c"in.valid == true".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        this.val = (*in_).val;
        (*in_).valid = false;
        this.valid = true;
        this
    }
    pub unsafe fn operator_assign_5(&mut self, mut newval: ()) -> *mut __gnu_test_copycounter {
        self.val = newval;
        self.valid = true;
        return &mut (*(self as *mut __gnu_test_copycounter));
    }
    pub unsafe fn copy_assign(&mut self, in_: *const __gnu_test_copycounter) -> () {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid) == (true)) {
                (unsafe {
                    __builtin_fprintf_10(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"50529.cc".as_ptr()).into(),
                            (135_()).into(),
                            (c"bool __gnu_test::copycounter::operator=(const copycounter &)"
                                .as_ptr())
                            .into(),
                            (c"in.valid == true".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_14)).prefix_inc();
        self.val = (*in_).val;
        self.valid = true;
        return true;
    }
    pub unsafe fn move_assign(
        &mut self,
        in_: *mut __gnu_test_copycounter,
    ) -> *mut __gnu_test_copycounter {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid) == (true)) {
                (unsafe {
                    __builtin_fprintf_10(
                        libcc2rs::stderr_unsafe(),
                        c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                        Box::new([
                            (c"50529.cc".as_ptr()).into(),
                            (145_()).into(),
                            (c"copycounter &__gnu_test::copycounter::operator=(copycounter &&)"
                                .as_ptr())
                            .into(),
                            (c"in.valid == true".as_ptr()).into(),
                        ]),
                    )
                });
                unimplemented!();
            }
        }
        self.val = (*in_).val;
        (*in_).valid = false;
        self.valid = true;
        return &mut (*(self as *mut __gnu_test_copycounter));
    }
    pub unsafe fn destructor(&mut self) {
        self.valid = false;
    }
}
impl std::cmp::Ord for __gnu_test_copycounter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_15(
                self as *const __gnu_test_copycounter,
                other as *const __gnu_test_copycounter,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_15(
                other as *const __gnu_test_copycounter,
                self as *const __gnu_test_copycounter,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_copycounter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_copycounter {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_16(
                self as *const __gnu_test_copycounter,
                other as *const __gnu_test_copycounter,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_copycounter {}
impl Clone for __gnu_test_copycounter {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_copycounter::copy_from(self as *const __gnu_test_copycounter) }
    }
}
impl Default for __gnu_test_copycounter {
    fn default() -> Self {
        unsafe { __gnu_test_copycounter::new_1() }
    }
}
pub static mut copycount_14: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn operator_eq_16(
    lhs: *const __gnu_test_copycounter,
    rhs: *const __gnu_test_copycounter,
) -> () {
    return (((*lhs).val) == ((*rhs).val));
}
pub unsafe fn operator_lt_15(
    lhs: *const __gnu_test_copycounter,
    rhs: *const __gnu_test_copycounter,
) -> () {
    return (((*lhs).val) < ((*rhs).val));
}
pub unsafe fn swap_17(lhs: *mut __gnu_test_copycounter, rhs: *mut __gnu_test_copycounter) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*lhs).valid) && ((*rhs).valid)) {
            (unsafe {
                __builtin_fprintf_10(
                    libcc2rs::stderr_unsafe(),
                    c"%s:%d: %s: Assertion \'%s\' failed.\n".as_ptr(),
                    Box::new([
                        (c"50529.cc".as_ptr()).into(),
                        (169_()).into(),
                        (c"void __gnu_test::swap(copycounter &, copycounter &)".as_ptr()).into(),
                        (c"lhs.valid && rhs.valid".as_ptr()).into(),
                    ]),
                )
            });
            unimplemented!();
        }
    }
    let mut temp: () = (*lhs).val;
    (*lhs).val = (*rhs).val;
    (*rhs).val = temp;
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct __gnu_test_rvalstruct_compare_by_value {
    pub val: (),
    pub ok: (),
}
impl __gnu_test_rvalstruct_compare_by_value {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { val: v, ok: true };
        this
    }
    pub unsafe fn copy_from(rh: *const __gnu_test_rvalstruct_compare_by_value) -> Self {
        let mut this = Self {
            val: (*rh).val,
            ok: (*rh).ok,
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((*rh).ok) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"50529.cc" .as_ptr() ) .into() , ( 187_() ) .into() , ( c"__gnu_test::rvalstruct_compare_by_value::rvalstruct_compare_by_value(const rvalstruct_compare_by_value &)" .as_ptr() ) .into() , ( c"rh.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        rh: *const __gnu_test_rvalstruct_compare_by_value,
    ) -> *mut __gnu_test_rvalstruct_compare_by_value {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((*rh).ok) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"50529.cc" .as_ptr() ) .into() , ( 193_() ) .into() , ( c"rvalstruct_compare_by_value &__gnu_test::rvalstruct_compare_by_value::operator=(const rvalstruct_compare_by_value &)" .as_ptr() ) .into() , ( c"rh.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        self.val = (*rh).val;
        self.ok = (*rh).ok;
        return &mut (*(self as *mut __gnu_test_rvalstruct_compare_by_value));
    }
    pub unsafe fn move_from(rh: *mut __gnu_test_rvalstruct_compare_by_value) -> Self {
        let mut this = Self {
            val: (*rh).val,
            ok: (*rh).ok,
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((*rh).ok) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"50529.cc" .as_ptr() ) .into() , ( 202_() ) .into() , ( c"__gnu_test::rvalstruct_compare_by_value::rvalstruct_compare_by_value(rvalstruct_compare_by_value &&)" .as_ptr() ) .into() , ( c"rh.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        (*rh).ok = false;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        rh: *mut __gnu_test_rvalstruct_compare_by_value,
    ) -> *mut __gnu_test_rvalstruct_compare_by_value {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((*rh).ok) {
                (unsafe {
                    __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"50529.cc" .as_ptr() ) .into() , ( 209_() ) .into() , ( c"rvalstruct_compare_by_value &__gnu_test::rvalstruct_compare_by_value::operator=(rvalstruct_compare_by_value &&)" .as_ptr() ) .into() , ( c"rh.ok" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
        self.val = (*rh).val;
        self.ok = (*rh).ok;
        (*rh).ok = false;
        return &mut (*(self as *mut __gnu_test_rvalstruct_compare_by_value));
    }
}
impl std::cmp::Ord for __gnu_test_rvalstruct_compare_by_value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_18(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_18(other.clone(), self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_rvalstruct_compare_by_value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_rvalstruct_compare_by_value {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_18(self.clone(), other.clone()))
                && !(operator_lt_18(other.clone(), self.clone()))
        }
    }
}
impl std::cmp::Eq for __gnu_test_rvalstruct_compare_by_value {}
impl Clone for __gnu_test_rvalstruct_compare_by_value {
    fn clone(&self) -> Self {
        unsafe {
            __gnu_test_rvalstruct_compare_by_value::copy_from(
                self as *const __gnu_test_rvalstruct_compare_by_value,
            )
        }
    }
}
pub unsafe fn operator_lt_18(
    mut lh: __gnu_test_rvalstruct_compare_by_value,
    mut rh: __gnu_test_rvalstruct_compare_by_value,
) -> () {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(rh.ok) {
            (unsafe {
                __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"50529.cc" .as_ptr() ) .into() , ( 221_() ) .into() , ( c"bool __gnu_test::operator<(rvalstruct_compare_by_value, rvalstruct_compare_by_value)" .as_ptr() ) .into() , ( c"rh.ok" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(lh.ok) {
            (unsafe {
                __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"50529.cc" .as_ptr() ) .into() , ( 222_() ) .into() , ( c"bool __gnu_test::operator<(rvalstruct_compare_by_value, rvalstruct_compare_by_value)" .as_ptr() ) .into() , ( c"lh.ok" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    return ((lh.val) < (rh.val));
}
pub unsafe fn order_19(
    mut lh: __gnu_test_rvalstruct_compare_by_value,
    mut rh: __gnu_test_rvalstruct_compare_by_value,
) -> () {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(rh.ok) {
            (unsafe {
                __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"50529.cc" .as_ptr() ) .into() , ( 230_() ) .into() , ( c"bool __gnu_test::order(rvalstruct_compare_by_value, rvalstruct_compare_by_value)" .as_ptr() ) .into() , ( c"rh.ok" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(lh.ok) {
            (unsafe {
                __builtin_fprintf_10 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"50529.cc" .as_ptr() ) .into() , ( 231_() ) .into() , ( c"bool __gnu_test::order(rvalstruct_compare_by_value, rvalstruct_compare_by_value)" .as_ptr() ) .into() , ( c"lh.ok" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
    return ((lh.val) < (rh.val));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct __gnu_test_throwing_move_constructor {}
impl __gnu_test_throwing_move_constructor {
    pub unsafe fn move_from(_a0: *mut __gnu_test_throwing_move_constructor) -> Self {
        let mut this = Self {};
        {
            unimplemented!();
            &(1_())
        };
        this
    }
}
pub unsafe fn test01_20() {
    let mut d: () = unimplemented!();
    let mut it: () = unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_20() });
    return 0_();
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_21() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_22(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_23(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_24(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_25(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_26(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_25(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_26(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_25((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_26(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_27(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_25(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_26(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_28(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_29(
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
pub unsafe fn run_tests_wrapped_locale_30(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_31(
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
        if ((self.pid_) == (unsafe { getpid_21() })) {
            (unsafe { semctl_23(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_32(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_32(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_33(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_34));
}
static mut tmp_34: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copycount_14);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
