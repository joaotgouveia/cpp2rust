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
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct __gnu_test_rvalstruct {
    pub val: i32,
    pub valid: bool,
}
impl __gnu_test_rvalstruct {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {
            val: 0,
            valid: true,
        };
        this
    }
    pub unsafe fn new_1(mut inval: i32) -> Self {
        let mut this = Self {
            val: inval,
            valid: true,
        };
        this
    }
    pub unsafe fn operator_assign_3(&mut self, mut newval: i32) -> *mut __gnu_test_rvalstruct {
        self.val = newval;
        self.valid = true;
        return &mut (*(self as *mut __gnu_test_rvalstruct));
    }
    pub unsafe fn move_from(in_: *mut __gnu_test_rvalstruct) -> Self {
        let mut this = Self {
            val: 0_i32,
            valid: false,
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
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
                unimplemented!();
                std::process::abort();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
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
            if operator_lt_9(
                self as *const __gnu_test_rvalstruct,
                other as *const __gnu_test_rvalstruct,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(
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
            operator_eq_10(
                self as *const __gnu_test_rvalstruct,
                other as *const __gnu_test_rvalstruct,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_rvalstruct {}
impl Default for __gnu_test_rvalstruct {
    fn default() -> Self {
        unsafe { __gnu_test_rvalstruct::new_2() }
    }
}
pub unsafe fn operator_eq_10(
    lhs: *const __gnu_test_rvalstruct,
    rhs: *const __gnu_test_rvalstruct,
) -> bool {
    return (((*lhs).val) == ((*rhs).val));
}
pub unsafe fn operator_lt_9(
    lhs: *const __gnu_test_rvalstruct,
    rhs: *const __gnu_test_rvalstruct,
) -> bool {
    return (((*lhs).val) < ((*rhs).val));
}
pub unsafe fn swap_11(lhs: *mut __gnu_test_rvalstruct, rhs: *mut __gnu_test_rvalstruct) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*lhs).valid) && ((*rhs).valid)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut temp: i32 = (*lhs).val;
    (*lhs).val = (*rhs).val;
    (*rhs).val = temp;
}
#[repr(C)]
#[derive(
    VaArg,
    FnPtrArg,
    MoveCtorUnsafe,
    CppFromUnsafe,
    CopyAssignUnsafe,
    MoveAssignUnsafe,
    DestructorUnsafe,
)]
#[cpp_from(i32, new_1, value)]
pub struct __gnu_test_copycounter {
    pub val: i32,
    pub valid: bool,
}
impl __gnu_test_copycounter {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {
            val: 0,
            valid: true,
        };
        this
    }
    pub unsafe fn new_1(mut inval: i32) -> Self {
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
            if !(((*in_).valid as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)).prefix_inc();
        this
    }
    pub unsafe fn move_from(in_: *mut __gnu_test_copycounter) -> Self {
        let mut this = Self {
            val: 0_i32,
            valid: false,
        };
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        this.val = (*in_).val;
        (*in_).valid = false;
        this.valid = true;
        this
    }
    pub unsafe fn operator_assign_5(&mut self, mut newval: i32) -> *mut __gnu_test_copycounter {
        self.val = newval;
        self.valid = true;
        return &mut (*(self as *mut __gnu_test_copycounter));
    }
    pub unsafe fn copy_assign(&mut self, in_: *const __gnu_test_copycounter) -> bool {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
            }
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)).prefix_inc();
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
            if !(((*in_).valid as i32) == (true as i32)) {
                unimplemented!();
                std::process::abort();
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
            if operator_lt_13(
                self as *const __gnu_test_copycounter,
                other as *const __gnu_test_copycounter,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_13(
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
            operator_eq_14(
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
        unsafe { __gnu_test_copycounter::new_2() }
    }
}
pub static mut copycount_12: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn operator_eq_14(
    lhs: *const __gnu_test_copycounter,
    rhs: *const __gnu_test_copycounter,
) -> bool {
    return (((*lhs).val) == ((*rhs).val));
}
pub unsafe fn operator_lt_13(
    lhs: *const __gnu_test_copycounter,
    rhs: *const __gnu_test_copycounter,
) -> bool {
    return (((*lhs).val) < ((*rhs).val));
}
pub unsafe fn swap_15(lhs: *mut __gnu_test_copycounter, rhs: *mut __gnu_test_copycounter) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*lhs).valid) && ((*rhs).valid)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut temp: i32 = (*lhs).val;
    (*lhs).val = (*rhs).val;
    (*rhs).val = temp;
}
#[repr(C)]
#[derive(
    VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default,
)]
#[cpp_from(i32, new, value)]
pub struct __gnu_test_rvalstruct_compare_by_value {
    pub val: i32,
    pub ok: bool,
}
impl __gnu_test_rvalstruct_compare_by_value {
    pub unsafe fn new(mut v: i32) -> Self {
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
                unimplemented!();
                std::process::abort();
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
                unimplemented!();
                std::process::abort();
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
                unimplemented!();
                std::process::abort();
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
                unimplemented!();
                std::process::abort();
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
            if operator_lt_16(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_16(other.clone(), self.clone()) {
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
            !(operator_lt_16(self.clone(), other.clone()))
                && !(operator_lt_16(other.clone(), self.clone()))
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
pub unsafe fn operator_lt_16(
    mut lh: __gnu_test_rvalstruct_compare_by_value,
    mut rh: __gnu_test_rvalstruct_compare_by_value,
) -> bool {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(rh.ok) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(lh.ok) {
            unimplemented!();
            std::process::abort();
        }
    }
    return ((lh.val) < (rh.val));
}
pub unsafe fn order_17(
    mut lh: __gnu_test_rvalstruct_compare_by_value,
    mut rh: __gnu_test_rvalstruct_compare_by_value,
) -> bool {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(rh.ok) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(lh.ok) {
            unimplemented!();
            std::process::abort();
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
            std::process::abort();
            &(1)
        };
        this
    }
}
#[repr(C)]
#[derive(
    VaArg,
    FnPtrArg,
    MoveCtorUnsafe,
    CppFromUnsafe,
    CopyAssignUnsafe,
    MoveAssignUnsafe,
    DestructorUnsafe,
)]
#[cpp_from(i32, new_1, value)]
pub struct __gnu_test_counter_type {
    pub val: i32,
}
impl __gnu_test_counter_type {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { val: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_count_18)).prefix_inc();
        this
    }
    pub unsafe fn new_1(mut inval: i32) -> Self {
        let mut this = Self { val: inval };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut specialize_count_19)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(in_: *const __gnu_test_counter_type) -> Self {
        let mut this = Self { val: (*in_).val };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_count_20)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destructor_count_25)).prefix_inc();
    }
    pub unsafe fn copy_assign(
        &mut self,
        in_: *const __gnu_test_counter_type,
    ) -> *mut __gnu_test_counter_type {
        self.val = (*in_).val;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_assign_count_21)).prefix_inc();
        return &mut (*(self as *mut __gnu_test_counter_type));
    }
    pub unsafe fn move_from(in_: *mut __gnu_test_counter_type) -> Self {
        let mut this = Self { val: 0_i32 };
        this.val = (*in_).val;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_count_23)).prefix_inc();
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        rhs: *mut __gnu_test_counter_type,
    ) -> *mut __gnu_test_counter_type {
        self.val = (*rhs).val;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assign_count_24)).prefix_inc();
        return &mut (*(self as *mut __gnu_test_counter_type));
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut default_count_18)) = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut specialize_count_19)) = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_count_20)) = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_assign_count_21)) = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut less_compare_count_22)) = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_count_23)) = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assign_count_24)) = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destructor_count_25)) = 0;
    }
    pub unsafe fn operator_eq(&self, rhs: *const __gnu_test_counter_type) -> bool {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn operator_lt(&self, rhs: *const __gnu_test_counter_type) -> bool {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut less_compare_count_22)).prefix_inc();
        return ((self.val) < ((*rhs).val));
    }
}
impl std::cmp::Ord for __gnu_test_counter_type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if __gnu_test_counter_type::operator_lt(self, other as *const __gnu_test_counter_type) {
                std::cmp::Ordering::Less
            } else if __gnu_test_counter_type::operator_lt(
                other,
                self as *const __gnu_test_counter_type,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_counter_type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_counter_type {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            __gnu_test_counter_type::operator_eq(self, other as *const __gnu_test_counter_type)
        }
    }
}
impl std::cmp::Eq for __gnu_test_counter_type {}
impl Clone for __gnu_test_counter_type {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_counter_type::copy_from(self as *const __gnu_test_counter_type) }
    }
}
impl Default for __gnu_test_counter_type {
    fn default() -> Self {
        unsafe { __gnu_test_counter_type::new_2() }
    }
}
pub static mut default_count_18: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut specialize_count_19: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copy_count_20: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copy_assign_count_21: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut less_compare_count_22: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut move_count_23: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut move_assign_count_24: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut destructor_count_25: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_counter_type_hasher {}
impl __gnu_test_counter_type_hasher {
    pub unsafe fn operator_call(&self, c: *const __gnu_test_counter_type) -> usize {
        return ((*c).val as usize);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Mapped {}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct DefaultConstructibleType {
    pub val: i32,
}
impl DefaultConstructibleType {
    pub unsafe fn new() -> Self {
        let mut this = Self { val: 123 };
        this
    }
    pub unsafe fn operator_assign_4(&mut self, mut x: i32) -> *mut DefaultConstructibleType {
        self.val = x;
        return &mut (*(self as *mut DefaultConstructibleType));
    }
}
impl Default for DefaultConstructibleType {
    fn default() -> Self {
        unsafe { DefaultConstructibleType::new() }
    }
}
pub unsafe fn test01_26() {
    let mut m1: UnsafeMap<i32, (i32, Mapped)> = UnsafeMap::new();
    (*m1.subscript_move(&mut 0, |__k| cpp_move_unsafe!(__k))) = <Mapped>::default();
    let mut m2: UnsafeMap<i32, (i32, __gnu_test_rvalstruct)> = UnsafeMap::new();
    (unsafe {
        let mut _in_: __gnu_test_rvalstruct = __gnu_test_rvalstruct::new_1({ 13 });
        __gnu_test_rvalstruct::move_assign(
            &mut (*m2.subscript_move(&mut 0, |__k| cpp_move_unsafe!(__k))),
            &mut _in_,
        )
    });
    let mut m3: UnsafeMap<i32, (i32, DefaultConstructibleType)> = UnsafeMap::new();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*m3.subscript_move(&mut 0, |__k| cpp_move_unsafe!(__k))).val) == (123)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((m3.len()) == (1_usize)) {
            unimplemented!();
            std::process::abort();
        }
    }
    (unsafe {
        DefaultConstructibleType::operator_assign_4(
            &mut (*m3.subscript_move(&mut 0, |__k| cpp_move_unsafe!(__k))),
            2,
        )
    });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*m3.subscript_move(&mut 0, |__k| cpp_move_unsafe!(__k))).val) == (2)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut m4: UnsafeMap<__gnu_test_counter_type, (__gnu_test_counter_type, i32)> =
        UnsafeMap::new();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*m4.subscript_move(&mut __gnu_test_counter_type::new_1({ 1 }), |__k| {
            cpp_move_unsafe!(__k)
        })) == (0))
        {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut specialize_count_19)) == (1)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_count_20)) == (0)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut move_count_23)) == (1)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut k: __gnu_test_counter_type = __gnu_test_counter_type::new_1({ 2 });
    let _dtor_k = ScopedDestructorUnsafe::new(&raw mut k, __gnu_test_counter_type::destructor);
    (unsafe { __gnu_test_counter_type::reset() });
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*m4.subscript(&k)) == (0)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_count_20)) == (1)) {
            unimplemented!();
            std::process::abort();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut move_count_23)) == (0)) {
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
    (unsafe { test01_26() });
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
pub unsafe fn set_memory_limits_27(mut size: Option<f32>) {
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
pub unsafe fn set_file_limit_28(mut size: u64) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_29(
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
pub unsafe fn run_tests_wrapped_locale_30(
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
pub unsafe fn run_tests_wrapped_env_31(
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
pub unsafe fn test_tm_32(
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
    std::cell::LazyCell::force(&*&raw const copycount_12);
    std::cell::LazyCell::force(&*&raw const default_count_18);
    std::cell::LazyCell::force(&*&raw const specialize_count_19);
    std::cell::LazyCell::force(&*&raw const copy_count_20);
    std::cell::LazyCell::force(&*&raw const copy_assign_count_21);
    std::cell::LazyCell::force(&*&raw const less_compare_count_22);
    std::cell::LazyCell::force(&*&raw const move_count_23);
    std::cell::LazyCell::force(&*&raw const move_assign_count_24);
    std::cell::LazyCell::force(&*&raw const destructor_count_25);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
