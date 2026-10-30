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
                unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(((*in_).valid) == (true)) {
                unimplemented!();
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
        unsafe { __gnu_test_rvalstruct::new_1() }
    }
}
pub unsafe fn operator_eq_10(
    lhs: *const __gnu_test_rvalstruct,
    rhs: *const __gnu_test_rvalstruct,
) -> () {
    return (((*lhs).val) == ((*rhs).val));
}
pub unsafe fn operator_lt_9(
    lhs: *const __gnu_test_rvalstruct,
    rhs: *const __gnu_test_rvalstruct,
) -> () {
    return (((*lhs).val) < ((*rhs).val));
}
pub unsafe fn swap_11(lhs: *mut __gnu_test_rvalstruct, rhs: *mut __gnu_test_rvalstruct) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*lhs).valid) && ((*rhs).valid)) {
            unimplemented!();
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
                unimplemented!();
                unimplemented!();
            }
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)).prefix_inc();
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
                unimplemented!();
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
                unimplemented!();
                unimplemented!();
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
            if !(((*in_).valid) == (true)) {
                unimplemented!();
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
        unsafe { __gnu_test_copycounter::new_1() }
    }
}
pub static mut copycount_12: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn operator_eq_14(
    lhs: *const __gnu_test_copycounter,
    rhs: *const __gnu_test_copycounter,
) -> () {
    return (((*lhs).val) == ((*rhs).val));
}
pub unsafe fn operator_lt_13(
    lhs: *const __gnu_test_copycounter,
    rhs: *const __gnu_test_copycounter,
) -> () {
    return (((*lhs).val) < ((*rhs).val));
}
pub unsafe fn swap_15(lhs: *mut __gnu_test_copycounter, rhs: *mut __gnu_test_copycounter) {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(((*lhs).valid) && ((*rhs).valid)) {
            unimplemented!();
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
                unimplemented!();
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
                unimplemented!();
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
                unimplemented!();
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
                unimplemented!();
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
) -> () {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(rh.ok) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(lh.ok) {
            unimplemented!();
            unimplemented!();
        }
    }
    return ((lh.val) < (rh.val));
}
pub unsafe fn order_17(
    mut lh: __gnu_test_rvalstruct_compare_by_value,
    mut rh: __gnu_test_rvalstruct_compare_by_value,
) -> () {
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(rh.ok) {
            unimplemented!();
            unimplemented!();
        }
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(lh.ok) {
            unimplemented!();
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
pub unsafe fn test01_18() {
    let mut a: () = unimplemented!();
    let mut c: __gnu_test_copycounter = __gnu_test_copycounter::new_2({ 1_() });
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, __gnu_test_copycounter::destructor);
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (1000_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == (1000_())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test02_19() {
    let mut a: () = unimplemented!();
    let mut c: __gnu_test_copycounter = __gnu_test_copycounter::new_2({ 1_() });
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, __gnu_test_copycounter::destructor);
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (1000_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == (1000_())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test03_20() {
    let mut a: () = unimplemented!();
    let mut c: __gnu_test_copycounter = __gnu_test_copycounter::new_2({ 1_() });
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, __gnu_test_copycounter::destructor);
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) = 0_();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (500_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == (502_())) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test04_21() {
    let mut c: __gnu_test_copycounter = __gnu_test_copycounter::new_2({ 1_() });
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, __gnu_test_copycounter::destructor);
    let mut a: () = unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) = 0_();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == (20_())) {
            unimplemented!();
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == (70_())) {
            unimplemented!();
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == ((170_()) + (1_())))
        {
            unimplemented!();
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == ((1170_()) + (2_())))
        {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test05_22() {
    let mut c: __gnu_test_copycounter = __gnu_test_copycounter::new_2({ 1_() });
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, __gnu_test_copycounter::destructor);
    let mut a: () = unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) = 0_();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == (20_())) {
            unimplemented!();
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == (70_())) {
            unimplemented!();
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == ((170_()) + (1_())))
        {
            unimplemented!();
            unimplemented!();
        }
    }
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((*std::cell::LazyCell::force_mut(&mut *&raw mut copycount_12)) == ((370_()) + (2_())))
        {
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
    (unsafe { test01_18() });
    (unsafe { test02_19() });
    (unsafe { test03_20() });
    (unsafe { test04_21() });
    (unsafe { test05_22() });
    return 0_();
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
pub unsafe fn set_memory_limits_23(mut size: Option<()>) {
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
pub unsafe fn set_file_limit_24(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_25(mut mangled: *const (), mut wanted: *const ()) {
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
pub unsafe fn run_tests_wrapped_locale_26(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_27(
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
pub unsafe fn test_tm_28(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).clone();
}
static mut tmp_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copycount_12);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
