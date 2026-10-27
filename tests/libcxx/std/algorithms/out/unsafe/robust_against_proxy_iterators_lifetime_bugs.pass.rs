extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(lhs: *const LifetimeIterator, rhs: *const LifetimeIterator) -> () {
    unimplemented!();
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_ne_1(lhs: *const LifetimeIterator, rhs: *const LifetimeIterator) -> () {
    unimplemented!();
    return (((*lhs).ptr_) != ((*rhs).ptr_));
}
pub unsafe fn operator_lt_2(lhs: *const LifetimeIterator, rhs: *const LifetimeIterator) -> () {
    unimplemented!();
    return (((*lhs).ptr_) < ((*rhs).ptr_));
}
pub unsafe fn operator_gt_3(lhs: *const LifetimeIterator, rhs: *const LifetimeIterator) -> () {
    unimplemented!();
    return (((*lhs).ptr_) > ((*rhs).ptr_));
}
pub unsafe fn operator_le_4(lhs: *const LifetimeIterator, rhs: *const LifetimeIterator) -> () {
    unimplemented!();
    return (((*lhs).ptr_) <= ((*rhs).ptr_));
}
pub unsafe fn operator_ge_5(lhs: *const LifetimeIterator, rhs: *const LifetimeIterator) -> () {
    unimplemented!();
    return (((*lhs).ptr_) >= ((*rhs).ptr_));
}
pub unsafe fn operator_add_6(lhs: *const LifetimeIterator, mut n: ()) -> LifetimeIterator {
    unimplemented!();
    return LifetimeIterator::new_1({ (*lhs).ptr_.offset((n) as isize) });
}
pub unsafe fn operator_add_7(mut n: (), lhs: *const LifetimeIterator) -> LifetimeIterator {
    unimplemented!();
    return LifetimeIterator::new_1({ (*lhs).ptr_.offset((n) as isize) });
}
pub unsafe fn operator_sub_8(lhs: *const LifetimeIterator, mut n: ()) -> LifetimeIterator {
    unimplemented!();
    return LifetimeIterator::new_1({ (*lhs).ptr_.offset(-((n) as isize)) });
}
pub unsafe fn operator_sub_9(mut lhs: LifetimeIterator, mut rhs: LifetimeIterator) -> () {
    unimplemented!();
    return ((lhs.ptr_ as usize - rhs.ptr_ as usize)
        / ::std::mem::size_of::<LifetimeIterator_Value>()) as ();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
struct LifetimeIterator_LifetimeCache {
    cache_: (),
}
impl LifetimeIterator_LifetimeCache {
    pub unsafe fn contains(&self, mut ptr: ()) -> () {
        return unimplemented!();
    }
    pub unsafe fn insert(&mut self, mut ptr: ()) {
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn erase(&mut self, mut ptr: ()) {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn operator_lt_11(
    lhs: *const LifetimeIterator_Value,
    rhs: *const LifetimeIterator_Value,
) -> () {
    unimplemented!();
    unimplemented!();
    return (((*lhs).i_) < ((*rhs).i_));
}
pub unsafe fn operator_eq_12(
    lhs: *const LifetimeIterator_Value,
    rhs: *const LifetimeIterator_Value,
) -> () {
    unimplemented!();
    unimplemented!();
    return (((*lhs).i_) == ((*rhs).i_));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, DestructorUnsafe)]
pub struct LifetimeIterator_Value {
    pub i_: (),
    pub moved_from_: (),
}
impl LifetimeIterator_Value {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            i_: unimplemented!(),
            moved_from_: false,
        };
        (unsafe {
            LifetimeIterator_LifetimeCache::insert(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                (&raw mut this as ()),
            )
        });
        this
    }
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self {
            i_: i,
            moved_from_: false,
        };
        (unsafe {
            LifetimeIterator_LifetimeCache::insert(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                (&raw mut this as ()),
            )
        });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe {
            LifetimeIterator_LifetimeCache::erase(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                ((self as *mut LifetimeIterator_Value) as ()),
            )
        });
    }
    pub unsafe fn copy_from(rhs: *const LifetimeIterator_Value) -> Self {
        let mut this = Self {
            i_: (*rhs).i_,
            moved_from_: false,
        };
        unimplemented!();
        unimplemented!();
        (unsafe {
            LifetimeIterator_LifetimeCache::insert(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                (&raw mut this as ()),
            )
        });
        this
    }
    pub unsafe fn move_from(rhs: *mut LifetimeIterator_Value) -> Self {
        let mut this = Self {
            i_: (*rhs).i_,
            moved_from_: false,
        };
        unimplemented!();
        unimplemented!();
        (*rhs).moved_from_ = true;
        (unsafe {
            LifetimeIterator_LifetimeCache::insert(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                (&raw mut this as ()),
            )
        });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        rhs: *const LifetimeIterator_Value,
    ) -> *mut LifetimeIterator_Value {
        unimplemented!();
        unimplemented!();
        self.i_ = (*rhs).i_;
        self.moved_from_ = false;
        return &mut (*(self as *mut LifetimeIterator_Value));
    }
    pub unsafe fn move_assign(
        &mut self,
        rhs: *mut LifetimeIterator_Value,
    ) -> *mut LifetimeIterator_Value {
        unimplemented!();
        unimplemented!();
        (*rhs).moved_from_ = true;
        self.i_ = (*rhs).i_;
        self.moved_from_ = false;
        return &mut (*(self as *mut LifetimeIterator_Value));
    }
}
impl std::cmp::Ord for LifetimeIterator_Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_11(
                self as *const LifetimeIterator_Value,
                other as *const LifetimeIterator_Value,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_11(
                other as *const LifetimeIterator_Value,
                self as *const LifetimeIterator_Value,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LifetimeIterator_Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LifetimeIterator_Value {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const LifetimeIterator_Value,
                other as *const LifetimeIterator_Value,
            )
        }
    }
}
impl std::cmp::Eq for LifetimeIterator_Value {}
impl Clone for LifetimeIterator_Value {
    fn clone(&self) -> Self {
        unsafe { LifetimeIterator_Value::copy_from(self as *const LifetimeIterator_Value) }
    }
}
impl Default for LifetimeIterator_Value {
    fn default() -> Self {
        unsafe { LifetimeIterator_Value::new_1() }
    }
}
pub unsafe fn operator_lt_13(
    lhs: *const LifetimeIterator_Reference,
    rhs: *const LifetimeIterator_Reference,
) -> () {
    unimplemented!();
    unimplemented!();
    return (unsafe {
        let _lhs: *const LifetimeIterator_Value = &(*(*lhs).v_);
        let _rhs: *const LifetimeIterator_Value = &(*(*rhs).v_);
        operator_lt_11(_lhs, _rhs)
    });
}
pub unsafe fn operator_eq_14(
    lhs: *const LifetimeIterator_Reference,
    rhs: *const LifetimeIterator_Reference,
) -> () {
    unimplemented!();
    unimplemented!();
    return (unsafe {
        let _lhs: *const LifetimeIterator_Value = &(*(*lhs).v_);
        let _rhs: *const LifetimeIterator_Value = &(*(*rhs).v_);
        operator_eq_12(_lhs, _rhs)
    });
}
pub unsafe fn swap_15(mut lhs: LifetimeIterator_Reference, mut rhs: LifetimeIterator_Reference) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, DestructorUnsafe)]
pub struct LifetimeIterator_Reference {
    pub v_: *mut LifetimeIterator_Value,
    pub moved_from_: (),
}
impl LifetimeIterator_Reference {
    pub unsafe fn new(v: *mut LifetimeIterator_Value) -> Self {
        let mut this = Self {
            v_: (v),
            moved_from_: false,
        };
        (unsafe {
            LifetimeIterator_LifetimeCache::insert(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                (&raw mut this as ()),
            )
        });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe {
            LifetimeIterator_LifetimeCache::erase(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                ((self as *mut LifetimeIterator_Reference) as ()),
            )
        });
    }
    pub unsafe fn copy_from(rhs: *const LifetimeIterator_Reference) -> Self {
        let mut this = Self {
            v_: (*rhs).v_,
            moved_from_: false,
        };
        unimplemented!();
        unimplemented!();
        (unsafe {
            LifetimeIterator_LifetimeCache::insert(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                (&raw mut this as ()),
            )
        });
        this
    }
    pub unsafe fn move_from(rhs: *mut LifetimeIterator_Reference) -> Self {
        let mut this = Self {
            v_: (*rhs).v_,
            moved_from_: false,
        };
        unimplemented!();
        unimplemented!();
        (*rhs).moved_from_ = true;
        (unsafe {
            LifetimeIterator_LifetimeCache::insert(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut lifetime_cache_10)),
                (&raw mut this as ()),
            )
        });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        rhs: *const LifetimeIterator_Reference,
    ) -> *mut LifetimeIterator_Reference {
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _rhs: *const LifetimeIterator_Value = &(*(*rhs).v_);
            LifetimeIterator_Value::copy_assign(&mut (*self.v_), _rhs)
        });
        self.moved_from_ = false;
        return &mut (*(self as *mut LifetimeIterator_Reference));
    }
    pub unsafe fn move_assign(
        &mut self,
        rhs: *mut LifetimeIterator_Reference,
    ) -> *mut LifetimeIterator_Reference {
        unimplemented!();
        unimplemented!();
        (*rhs).moved_from_ = true;
        (unsafe {
            let _rhs: *const LifetimeIterator_Value = &(*(*rhs).v_);
            LifetimeIterator_Value::copy_assign(&mut (*self.v_), _rhs)
        });
        self.moved_from_ = false;
        return &mut (*(self as *mut LifetimeIterator_Reference));
    }
    pub unsafe fn to_LifetimeIterator_Value(&self) -> LifetimeIterator_Value {
        unimplemented!();
        unimplemented!();
        return LifetimeIterator_Value::copy_from({ &(*self.v_) });
    }
    pub unsafe fn operator_assign_8(
        &mut self,
        mut v: LifetimeIterator_Value,
    ) -> *mut LifetimeIterator_Reference {
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _rhs: *const LifetimeIterator_Value = &v;
            LifetimeIterator_Value::copy_assign(&mut (*self.v_), _rhs)
        });
        self.moved_from_ = false;
        return &mut (*(self as *mut LifetimeIterator_Reference));
    }
}
impl std::cmp::Ord for LifetimeIterator_Reference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_13(
                self as *const LifetimeIterator_Reference,
                other as *const LifetimeIterator_Reference,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_13(
                other as *const LifetimeIterator_Reference,
                self as *const LifetimeIterator_Reference,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LifetimeIterator_Reference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LifetimeIterator_Reference {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
                self as *const LifetimeIterator_Reference,
                other as *const LifetimeIterator_Reference,
            )
        }
    }
}
impl std::cmp::Eq for LifetimeIterator_Reference {}
impl Clone for LifetimeIterator_Reference {
    fn clone(&self) -> Self {
        unsafe { LifetimeIterator_Reference::copy_from(self as *const LifetimeIterator_Reference) }
    }
}
impl Default for LifetimeIterator_Reference {
    fn default() -> Self {
        unsafe {
            LifetimeIterator_Reference {
                v_: std::ptr::null_mut(),
                moved_from_: false,
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct LifetimeIterator {
    pub ptr_: *mut LifetimeIterator_Value,
    pub moved_from_: (),
}
impl LifetimeIterator {
    pub unsafe fn new_1(mut ptr: *mut LifetimeIterator_Value) -> Self {
        let mut this = Self {
            ptr_: ptr,
            moved_from_: false,
        };
        this
    }
    pub unsafe fn copy_from(rhs: *const LifetimeIterator) -> Self {
        let mut this = Self {
            ptr_: (*rhs).ptr_,
            moved_from_: false,
        };
        unimplemented!();
        this
    }
    pub unsafe fn copy_assign(&mut self, rhs: *const LifetimeIterator) -> *mut LifetimeIterator {
        unimplemented!();
        self.ptr_ = (*rhs).ptr_;
        self.moved_from_ = false;
        return &mut (*(self as *mut LifetimeIterator));
    }
    pub unsafe fn move_from(rhs: *mut LifetimeIterator) -> Self {
        let mut this = Self {
            ptr_: (*rhs).ptr_,
            moved_from_: false,
        };
        unimplemented!();
        (*rhs).moved_from_ = true;
        (*rhs).ptr_ = std::ptr::null_mut();
        this
    }
    pub unsafe fn move_assign(&mut self, rhs: *mut LifetimeIterator) -> *mut LifetimeIterator {
        unimplemented!();
        (*rhs).moved_from_ = true;
        self.moved_from_ = false;
        self.ptr_ = (*rhs).ptr_;
        (*rhs).ptr_ = std::ptr::null_mut();
        return &mut (*(self as *mut LifetimeIterator));
    }
    pub unsafe fn operator_deref(&self) -> LifetimeIterator_Reference {
        unimplemented!();
        return LifetimeIterator_Reference::new({ &mut (*self.ptr_) });
    }
    pub unsafe fn operator_inc_8(&mut self) -> *mut LifetimeIterator {
        unimplemented!();
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut LifetimeIterator));
    }
    pub unsafe fn operator_post_inc_9(&mut self, mut _a0: ()) -> LifetimeIterator {
        unimplemented!();
        let mut tmp: LifetimeIterator =
            LifetimeIterator::copy_from({ &(*(self as *mut LifetimeIterator)) });
        (unsafe { LifetimeIterator::operator_inc_8(&mut (*(self as *mut LifetimeIterator))) });
        return LifetimeIterator::move_from({ &mut tmp });
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut LifetimeIterator {
        unimplemented!();
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut LifetimeIterator));
    }
    pub unsafe fn operator_post_dec_11(&mut self, mut _a0: ()) -> LifetimeIterator {
        unimplemented!();
        let mut tmp: LifetimeIterator =
            LifetimeIterator::copy_from({ &(*(self as *mut LifetimeIterator)) });
        (unsafe { LifetimeIterator::operator_dec_10(&mut (*(self as *mut LifetimeIterator))) });
        return LifetimeIterator::move_from({ &mut tmp });
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut LifetimeIterator {
        unimplemented!();
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut LifetimeIterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut LifetimeIterator {
        unimplemented!();
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut LifetimeIterator));
    }
    pub unsafe fn operator_index(&self, mut i: ()) -> LifetimeIterator_Reference {
        unimplemented!();
        return LifetimeIterator_Reference::new({ &mut (*(self.ptr_.offset((i) as isize))) });
    }
}
impl std::cmp::Ord for LifetimeIterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_2(
                self as *const LifetimeIterator,
                other as *const LifetimeIterator,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_2(
                other as *const LifetimeIterator,
                self as *const LifetimeIterator,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LifetimeIterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LifetimeIterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const LifetimeIterator,
                other as *const LifetimeIterator,
            )
        }
    }
}
impl std::cmp::Eq for LifetimeIterator {}
impl Clone for LifetimeIterator {
    fn clone(&self) -> Self {
        unsafe { LifetimeIterator::copy_from(self as *const LifetimeIterator) }
    }
}
impl Default for LifetimeIterator {
    fn default() -> Self {
        unsafe {
            LifetimeIterator {
                ptr_: std::ptr::null_mut(),
                moved_from_: false,
            }
        }
    }
}
pub static mut lifetime_cache_10: std::cell::LazyCell<LifetimeIterator_LifetimeCache> =
    std::cell::LazyCell::new(|| unsafe { <LifetimeIterator_LifetimeCache>::default() });
pub unsafe fn operator_eq_16(lhs: *const ConstexprIterator, rhs: *const ConstexprIterator) -> () {
    unimplemented!();
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_ne_17(lhs: *const ConstexprIterator, rhs: *const ConstexprIterator) -> () {
    unimplemented!();
    return (((*lhs).ptr_) != ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_18(lhs: *const ConstexprIterator, rhs: *const ConstexprIterator) -> () {
    unimplemented!();
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
pub unsafe fn operator_add_19(lhs: *const ConstexprIterator, mut n: ()) -> ConstexprIterator {
    unimplemented!();
    return ConstexprIterator::new_1({ (*lhs).ptr_.offset((n) as isize) });
}
pub unsafe fn operator_add_20(mut n: (), lhs: *const ConstexprIterator) -> ConstexprIterator {
    unimplemented!();
    return ConstexprIterator::new_1({ (*lhs).ptr_.offset((n) as isize) });
}
pub unsafe fn operator_sub_21(lhs: *const ConstexprIterator, mut n: ()) -> ConstexprIterator {
    unimplemented!();
    return ConstexprIterator::new_1({ (*lhs).ptr_.offset(-((n) as isize)) });
}
pub unsafe fn operator_sub_22(mut lhs: ConstexprIterator, mut rhs: ConstexprIterator) -> () {
    unimplemented!();
    return ((lhs.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ();
}
pub unsafe fn operator_lt_23(
    lhs: *const ConstexprIterator_Reference,
    rhs: *const ConstexprIterator_Reference,
) -> () {
    unimplemented!();
    return ((*(*lhs).v_) < (*(*rhs).v_));
}
pub unsafe fn operator_eq_24(
    lhs: *const ConstexprIterator_Reference,
    rhs: *const ConstexprIterator_Reference,
) -> () {
    unimplemented!();
    return ((*(*lhs).v_) == (*(*rhs).v_));
}
pub unsafe fn swap_25(mut lhs: ConstexprIterator_Reference, mut rhs: ConstexprIterator_Reference) {
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct ConstexprIterator_Reference {
    pub v_: *mut (),
    pub moved_from_: (),
}
impl ConstexprIterator_Reference {
    pub unsafe fn new(v: *mut ()) -> Self {
        let mut this = Self {
            v_: (v),
            moved_from_: false,
        };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        rhs: *const ConstexprIterator_Reference,
    ) -> *mut ConstexprIterator_Reference {
        unimplemented!();
        self.v_ = (*rhs).v_;
        self.moved_from_ = false;
        return &mut (*(self as *mut ConstexprIterator_Reference));
    }
    pub unsafe fn move_from(rhs: *mut ConstexprIterator_Reference) -> Self {
        let mut this = Self {
            v_: (*rhs).v_,
            moved_from_: false,
        };
        unimplemented!();
        (*rhs).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        rhs: *mut ConstexprIterator_Reference,
    ) -> *mut ConstexprIterator_Reference {
        unimplemented!();
        (*rhs).moved_from_ = true;
        self.moved_from_ = false;
        self.v_ = (*rhs).v_;
        return &mut (*(self as *mut ConstexprIterator_Reference));
    }
    pub unsafe fn to_(&self) -> () {
        unimplemented!();
        return (*self.v_);
    }
    pub unsafe fn operator_assign_7(&mut self, mut v: ()) -> *mut ConstexprIterator_Reference {
        (*self.v_) = v;
        self.moved_from_ = false;
        return &mut (*(self as *mut ConstexprIterator_Reference));
    }
}
impl std::cmp::Ord for ConstexprIterator_Reference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_23(
                self as *const ConstexprIterator_Reference,
                other as *const ConstexprIterator_Reference,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_23(
                other as *const ConstexprIterator_Reference,
                self as *const ConstexprIterator_Reference,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for ConstexprIterator_Reference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ConstexprIterator_Reference {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_24(
                self as *const ConstexprIterator_Reference,
                other as *const ConstexprIterator_Reference,
            )
        }
    }
}
impl std::cmp::Eq for ConstexprIterator_Reference {}
impl Default for ConstexprIterator_Reference {
    fn default() -> Self {
        unsafe {
            ConstexprIterator_Reference {
                v_: std::ptr::null_mut(),
                moved_from_: false,
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct ConstexprIterator {
    pub ptr_: *mut (),
    pub moved_from_: (),
}
impl ConstexprIterator {
    pub unsafe fn new_1(mut ptr: *mut ()) -> Self {
        let mut this = Self {
            ptr_: ptr,
            moved_from_: false,
        };
        this
    }
    pub unsafe fn copy_from(rhs: *const ConstexprIterator) -> Self {
        let mut this = Self {
            ptr_: (*rhs).ptr_,
            moved_from_: false,
        };
        unimplemented!();
        this
    }
    pub unsafe fn copy_assign(&mut self, rhs: *const ConstexprIterator) -> *mut ConstexprIterator {
        unimplemented!();
        self.ptr_ = (*rhs).ptr_;
        self.moved_from_ = false;
        return &mut (*(self as *mut ConstexprIterator));
    }
    pub unsafe fn move_from(rhs: *mut ConstexprIterator) -> Self {
        let mut this = Self {
            ptr_: (*rhs).ptr_,
            moved_from_: false,
        };
        unimplemented!();
        (*rhs).moved_from_ = true;
        (*rhs).ptr_ = std::ptr::null_mut();
        this
    }
    pub unsafe fn move_assign(&mut self, rhs: *mut ConstexprIterator) -> *mut ConstexprIterator {
        unimplemented!();
        (*rhs).moved_from_ = true;
        self.moved_from_ = false;
        self.ptr_ = (*rhs).ptr_;
        (*rhs).ptr_ = std::ptr::null_mut();
        return &mut (*(self as *mut ConstexprIterator));
    }
    pub unsafe fn operator_deref(&self) -> ConstexprIterator_Reference {
        unimplemented!();
        return ConstexprIterator_Reference::new({ &mut (*self.ptr_) });
    }
    pub unsafe fn operator_inc_8(&mut self) -> *mut ConstexprIterator {
        unimplemented!();
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut ConstexprIterator));
    }
    pub unsafe fn operator_post_inc_9(&mut self, mut _a0: ()) -> ConstexprIterator {
        unimplemented!();
        let mut tmp: ConstexprIterator =
            ConstexprIterator::copy_from({ &(*(self as *mut ConstexprIterator)) });
        (unsafe { ConstexprIterator::operator_inc_8(&mut (*(self as *mut ConstexprIterator))) });
        return ConstexprIterator::move_from({ &mut tmp });
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut ConstexprIterator {
        unimplemented!();
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut ConstexprIterator));
    }
    pub unsafe fn operator_post_dec_11(&mut self, mut _a0: ()) -> ConstexprIterator {
        unimplemented!();
        let mut tmp: ConstexprIterator =
            ConstexprIterator::copy_from({ &(*(self as *mut ConstexprIterator)) });
        (unsafe { ConstexprIterator::operator_dec_10(&mut (*(self as *mut ConstexprIterator))) });
        return ConstexprIterator::move_from({ &mut tmp });
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut ConstexprIterator {
        unimplemented!();
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut ConstexprIterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut ConstexprIterator {
        unimplemented!();
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut ConstexprIterator));
    }
    pub unsafe fn operator_index(&self, mut i: ()) -> ConstexprIterator_Reference {
        return ConstexprIterator_Reference::new({ &mut (*(self.ptr_.offset((i) as isize))) });
    }
}
impl std::cmp::Ord for ConstexprIterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_18(
                self as *const ConstexprIterator,
                other as *const ConstexprIterator,
            )
        }
    }
}
impl std::cmp::PartialOrd for ConstexprIterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ConstexprIterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_16(
                self as *const ConstexprIterator,
                other as *const ConstexprIterator,
            )
        }
    }
}
impl std::cmp::Eq for ConstexprIterator {}
impl Clone for ConstexprIterator {
    fn clone(&self) -> Self {
        unsafe { ConstexprIterator::copy_from(self as *const ConstexprIterator) }
    }
}
impl Default for ConstexprIterator {
    fn default() -> Self {
        unsafe {
            ConstexprIterator {
                ptr_: std::ptr::null_mut(),
                moved_from_: false,
            }
        }
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn new_1(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
            ],
        };
        const _: () = assert!(((0_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 0_();
        this
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn new_2(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
            ],
        };
        const _: () = assert!(((1_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 1_();
        this
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn new_3(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
            ],
        };
        const _: () = assert!(((2_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 2_();
        this
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn new_4(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
            ],
        };
        const _: () = assert!(((3_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 3_();
        this
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn new_5(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
            ],
        };
        const _: () = assert!(((9_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 9_();
        this
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn new_6(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
            ],
        };
        const _: () = assert!(((8_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 8_();
        this
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn new_7(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
                LifetimeIterator_Value::new_1(),
            ],
        };
        const _: () = assert!(((11_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 11_();
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct Input_LifetimeIterator_Value__32_ {
    size_: (),
    values_: [LifetimeIterator_Value; 32],
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn begin(&mut self) -> *mut LifetimeIterator_Value {
        return self.values_.as_mut_ptr();
    }
    pub unsafe fn end(&mut self) -> *mut LifetimeIterator_Value {
        return self.values_.as_mut_ptr().offset((self.size_) as isize);
    }
    pub unsafe fn size(&self) -> () {
        return self.size_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const Input_LifetimeIterator_Value__32_,
    ) -> *mut Input_LifetimeIterator_Value__32_ {
        self.size_ = (*_a0).size_;
        let mut __i0: () = 0_();
        'loop_: while ((__i0) != (32_())) {
            (unsafe {
                let _rhs: *const LifetimeIterator_Value = &(*_a0).values_[(__i0) as usize];
                LifetimeIterator_Value::copy_assign(&mut self.values_[(__i0) as usize], _rhs)
            });
            __i0.prefix_inc();
        }
        return &mut (*(self as *mut Input_LifetimeIterator_Value__32_));
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub unsafe fn destructor(&mut self) {
        for __e in self.values_.iter_mut() {
            LifetimeIterator_Value::destructor(__e);
        }
    }
}
impl Default for Input_LifetimeIterator_Value__32_ {
    fn default() -> Self {
        unsafe {
            Input_LifetimeIterator_Value__32_ {
                size_: 0_(),
                values_: [
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                    LifetimeIterator_Value::new_1(),
                ],
            }
        }
    }
}
impl Input_int__32_ {
    pub unsafe fn new_1(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ],
        };
        const _: () = assert!(((0_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 0_();
        this
    }
}
impl Input_int__32_ {
    pub unsafe fn new_2(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ],
        };
        const _: () = assert!(((1_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 1_();
        this
    }
}
impl Input_int__32_ {
    pub unsafe fn new_3(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ],
        };
        const _: () = assert!(((2_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 2_();
        this
    }
}
impl Input_int__32_ {
    pub unsafe fn new_4(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ],
        };
        const _: () = assert!(((3_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 3_();
        this
    }
}
impl Input_int__32_ {
    pub unsafe fn new_5(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ],
        };
        const _: () = assert!(((9_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 9_();
        this
    }
}
impl Input_int__32_ {
    pub unsafe fn new_6(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ],
        };
        const _: () = assert!(((8_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 8_();
        this
    }
}
impl Input_int__32_ {
    pub unsafe fn new_7(mut from: ()) -> Self {
        let mut this = Self {
            size_: 0_(),
            values_: [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ],
        };
        const _: () = assert!(((11_()) <= (32_())), "N <= StorageSize");;
        unimplemented!();
        this.size_ = 11_();
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Input_int__32_ {
    size_: (),
    values_: [(); 32],
}
impl Input_int__32_ {
    pub unsafe fn begin(&mut self) -> *mut () {
        return self.values_.as_mut_ptr();
    }
    pub unsafe fn end(&mut self) -> *mut () {
        return self.values_.as_mut_ptr().offset((self.size_) as isize);
    }
    pub unsafe fn size(&self) -> () {
        return self.size_;
    }
}
impl Default for Input_int__32_ {
    fn default() -> Self {
        unsafe {
            Input_int__32_ {
                size_: 0_(),
                values_: [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
pub unsafe fn get_simple_in_26() -> () {
    let mut result: () = unimplemented!();
    return result;
}
pub unsafe fn get_simple_in_27() -> () {
    let mut result: () = unimplemented!();
    return result;
}
pub unsafe fn get_sorted_in_28() -> () {
    let mut result: () = unimplemented!();
    return result;
}
pub unsafe fn get_sorted_in_29() -> () {
    let mut result: () = unimplemented!();
    return result;
}
pub unsafe fn get_sort_test_in_30() -> () {
    let mut result: () = unimplemented!();
    return result;
}
pub unsafe fn get_sort_test_in_31() -> () {
    let mut result: () = unimplemented!();
    return result;
}
pub unsafe fn test_32(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_33(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_34(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_35(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_36(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_37(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_38(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_39(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_40(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_41(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_42(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_43(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_44(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_45(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_46(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_47(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_48(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_49(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_50(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_51(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_52(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_53(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_54(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_55(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_56(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_57(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_58(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_59(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_60(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_61(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_62(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_63(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_64(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_65(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_66(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_67(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_68(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_69(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_70(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_71(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_72(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_73(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _e: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::end(&mut (*in_)) })
            });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_74(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_75(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_76(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_77(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_78(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_79(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_80(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_81(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_82(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_83(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_84(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_85(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_86(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_87(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_88(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_89(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_90(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_91(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_92(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_93(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_94(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_95(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_96(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_97(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_98(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_99(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_100(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_101(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_102(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_103(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_104(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_105(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_106(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_107(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_108(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_109(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_110(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_111(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_112(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_113(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_114(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_115(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::end(&mut (*in_)) }) });
            func.call(_b, _e)
        });
    }
}
pub unsafe fn test_n_116(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, ())>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _n: () = (unsafe { Input_LifetimeIterator_Value__32_::size(&(*in_)) });
            func.call(_b, _n)
        });
    }
}
pub unsafe fn test_n_117(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, ())>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _n: () = (unsafe { Input_LifetimeIterator_Value__32_::size(&(*in_)) });
            func.call(_b, _n)
        });
    }
}
pub unsafe fn test_n_118(mut inputs: (), mut func: FnPtr<fn(LifetimeIterator, ())>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_LifetimeIterator_Value__32_ = &mut (*in_);
        (unsafe {
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                (unsafe { Input_LifetimeIterator_Value__32_::begin(&mut (*in_)) })
            });
            let _n: () = (unsafe { Input_LifetimeIterator_Value__32_::size(&(*in_)) });
            func.call(_b, _n)
        });
    }
}
pub unsafe fn test_n_119(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ())>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _n: () = (unsafe { Input_int__32_::size(&(*in_)) });
            func.call(_b, _n)
        });
    }
}
pub unsafe fn test_n_120(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ())>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _n: () = (unsafe { Input_int__32_::size(&(*in_)) });
            func.call(_b, _n)
        });
    }
}
pub unsafe fn test_n_121(mut inputs: (), mut func: FnPtr<fn(ConstexprIterator, ())>) {
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: *mut Input_int__32_ = &mut (*in_);
        (unsafe {
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ (unsafe { Input_int__32_::begin(&mut (*in_)) }) });
            let _n: () = (unsafe { Input_int__32_::size(&(*in_)) });
            func.call(_b, _n)
        });
    }
}
pub unsafe fn to_int_122(mut x: ()) -> () {
    return x;
}
pub unsafe fn to_int_123(mut x: LifetimeIterator_Value) -> () {
    return x.i_;
}
pub unsafe fn rand_gen_124() -> () {
    return unimplemented!();
}
pub unsafe fn test_125() -> () {
    let mut is_neg: FnPtr<fn(*const LifetimeIterator_Value) -> ()> =
        FnPtr::<fn(*const LifetimeIterator_Value) -> ()>::new(
            |val: *const LifetimeIterator_Value| -> () {
                unsafe {
                    return ((unsafe { to_int_123(LifetimeIterator_Value::copy_from({ val })) })
                        < (0_()));
                }
            },
        );
    let mut gen_: FnPtr<fn() -> LifetimeIterator_Value> =
        FnPtr::<fn() -> LifetimeIterator_Value>::new(|| -> LifetimeIterator_Value {
            unsafe {
                return LifetimeIterator_Value::new_2({ 42_() });
            }
        });
    let mut identity: FnPtr<fn(LifetimeIterator_Value) -> LifetimeIterator_Value> =
        FnPtr::<fn(LifetimeIterator_Value) -> LifetimeIterator_Value>::new(
            |mut val: LifetimeIterator_Value| -> LifetimeIterator_Value {
                unsafe {
                    return LifetimeIterator_Value::move_from({ &mut val });
                }
            },
        );
    let N: () = 32_();
    let mut output: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut x: LifetimeIterator_Value = LifetimeIterator_Value::new_2({ 1_() });
    let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, LifetimeIterator_Value::destructor);
    let mut y: LifetimeIterator_Value = LifetimeIterator_Value::new_2({ 3_() });
    let _dtor_y = ScopedDestructorUnsafe::new(&raw mut y, LifetimeIterator_Value::destructor);
    let mut simple_in: () = (unsafe { get_simple_in_26() });
    let mut sorted_in: () = (unsafe { get_sorted_in_28() });
    let mut sort_test_in: () = (unsafe { get_sort_test_in_30() });
    (unsafe {
        test_32(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_33(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_34(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_35(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_36(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_37(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_38(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_39(
            sorted_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut LifetimeIterator_Value = &mut x;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_40(
            sorted_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut LifetimeIterator_Value = &mut x;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_41(
            sorted_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut LifetimeIterator_Value = &mut x;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_42(
            sorted_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut LifetimeIterator_Value = &mut x;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_43(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_44(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_45(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_46(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut LifetimeIterator_Value = &mut x;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_47(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_48(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_n_116(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut n: ()| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_49(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_n_117(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: LifetimeIterator, mut n: ()| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_50(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_51(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_52(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_53(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_54(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let identity: *mut FnPtr<fn(LifetimeIterator_Value) -> LifetimeIterator_Value> =
                        &mut identity;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_55(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let gen_: *mut FnPtr<fn() -> LifetimeIterator_Value> = &mut gen_;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_n_118(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let gen_: *mut FnPtr<fn() -> LifetimeIterator_Value> = &mut gen_;
                },
                |mut b: LifetimeIterator, mut n: ()| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_56(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let x: *mut LifetimeIterator_Value = &mut x;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_57(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_58(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut LifetimeIterator_Value = &mut x;
                    let y: *mut LifetimeIterator_Value = &mut y;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_59(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                    let y: *mut LifetimeIterator_Value = &mut y;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_60(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let x: *mut LifetimeIterator_Value = &mut x;
                    let y: *mut LifetimeIterator_Value = &mut y;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_61(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                    let y: *mut LifetimeIterator_Value = &mut y;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_62(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_63(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut LifetimeIterator_Value = &mut x;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_64(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_65(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    {
        (unsafe {
            test_66(
                simple_in.clone(),
                FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                    |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    (unsafe {
        test_67(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> = &mut is_neg;
                },
                |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    {
        (unsafe {
            test_68(
                simple_in.clone(),
                lambda_unsafe!(
                    {
                        let is_neg: *mut FnPtr<fn(*const LifetimeIterator_Value) -> ()> =
                            &mut is_neg;
                    },
                    |mut b: LifetimeIterator, mut e: LifetimeIterator| {
                        &(unimplemented!());
                    }
                ),
            )
        });
    }
    {
        (unsafe {
            test_69(
                sort_test_in.clone(),
                FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                    |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    {
        (unsafe {
            test_70(
                sort_test_in.clone(),
                FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                    |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    (unsafe {
        test_71(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_72(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_73(
            simple_in.clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |mut b: LifetimeIterator, mut e: LifetimeIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    return true;
}
pub unsafe fn test_126() -> () {
    let mut is_neg: FnPtr<fn(*const ()) -> ()> =
        FnPtr::<fn(*const ()) -> ()>::new(|val: *const ()| -> () {
            unsafe {
                return ((unsafe { to_int_122((*val)) }) < (0_()));
            }
        });
    let mut gen_: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
        unsafe {
            return 42_();
        }
    });
    let mut identity: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut val: ()| -> () {
        unsafe {
            return val;
        }
    });
    let N: () = 32_();
    let mut output: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut x: () = 1_();
    let mut y: () = 3_();
    let mut simple_in: () = (unsafe { get_simple_in_27() });
    let mut sorted_in: () = (unsafe { get_sorted_in_29() });
    let mut sort_test_in: () = (unsafe { get_sort_test_in_31() });
    (unsafe {
        test_74(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_75(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_76(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_77(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_78(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_79(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_80(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_81(
            sorted_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut () = &mut x;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_82(
            sorted_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut () = &mut x;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_83(
            sorted_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut () = &mut x;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_84(
            sorted_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut () = &mut x;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_85(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_86(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_87(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_88(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut () = &mut x;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_89(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_90(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_n_119(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut n: ()| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_91(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_n_120(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: ConstexprIterator, mut n: ()| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_92(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_93(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_94(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_95(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_96(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let identity: *mut FnPtr<fn(()) -> ()> = &mut identity;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_97(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let gen_: *mut FnPtr<fn() -> ()> = &mut gen_;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_n_121(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let gen_: *mut FnPtr<fn() -> ()> = &mut gen_;
                },
                |mut b: ConstexprIterator, mut n: ()| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_98(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let x: *mut () = &mut x;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_99(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_100(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut () = &mut x;
                    let y: *mut () = &mut y;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_101(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                    let y: *mut () = &mut y;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_102(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let x: *mut () = &mut x;
                    let y: *mut () = &mut y;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_103(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                    let y: *mut () = &mut y;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_104(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let out: *mut () = &mut out;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_105(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let x: *mut () = &mut x;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_106(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    (unsafe {
        test_107(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    {
        (unsafe {
            test_108(
                simple_in.clone(),
                FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                    |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    (unsafe {
        test_109(
            simple_in.clone(),
            lambda_unsafe!(
                {
                    let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                },
                |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                    &(unimplemented!());
                }
            ),
        )
    });
    {
        (unsafe {
            test_110(
                simple_in.clone(),
                lambda_unsafe!(
                    {
                        let is_neg: *mut FnPtr<fn(*const ()) -> ()> = &mut is_neg;
                    },
                    |mut b: ConstexprIterator, mut e: ConstexprIterator| {
                        &(unimplemented!());
                    }
                ),
            )
        });
    }
    {
        (unsafe {
            test_111(
                sort_test_in.clone(),
                FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                    |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    {
        (unsafe {
            test_112(
                sort_test_in.clone(),
                FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                    |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    (unsafe {
        test_113(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_114(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    (unsafe {
        test_115(
            simple_in.clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |mut b: ConstexprIterator, mut e: ConstexprIterator| unsafe {
                    &(unimplemented!());
                },
            ),
        )
    });
    return true;
}
pub unsafe fn test_all_127() {
    (unsafe { test_125() });
    const _: () = assert!(true, "test<ConstexprIterator>()");;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_all_127() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const lifetime_cache_10);
}
