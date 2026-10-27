extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(lhs: Ptr<LifetimeIterator>, rhs: Ptr<LifetimeIterator>) -> () {
    unimplemented!();
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_ne_1(lhs: Ptr<LifetimeIterator>, rhs: Ptr<LifetimeIterator>) -> () {
    unimplemented!();
    return ({ lhs.with(|__s| __s.ptr_.clone()) } != { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_lt_2(lhs: Ptr<LifetimeIterator>, rhs: Ptr<LifetimeIterator>) -> () {
    unimplemented!();
    return ({ lhs.with(|__s| __s.ptr_.clone()) } < { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_gt_3(lhs: Ptr<LifetimeIterator>, rhs: Ptr<LifetimeIterator>) -> () {
    unimplemented!();
    return ({ lhs.with(|__s| __s.ptr_.clone()) } > { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_le_4(lhs: Ptr<LifetimeIterator>, rhs: Ptr<LifetimeIterator>) -> () {
    unimplemented!();
    return ({ lhs.with(|__s| __s.ptr_.clone()) } <= { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_ge_5(lhs: Ptr<LifetimeIterator>, rhs: Ptr<LifetimeIterator>) -> () {
    unimplemented!();
    return ({ lhs.with(|__s| __s.ptr_.clone()) } >= { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_add_6(lhs: Ptr<LifetimeIterator>, mut n: ()) -> LifetimeIterator {
    unimplemented!();
    return LifetimeIterator::new_1({ lhs.with(|__s| __s.ptr_.clone()).offset((n) as isize) });
}
pub fn operator_add_7(mut n: (), lhs: Ptr<LifetimeIterator>) -> LifetimeIterator {
    unimplemented!();
    return LifetimeIterator::new_1({ lhs.with(|__s| __s.ptr_.clone()).offset((n) as isize) });
}
pub fn operator_sub_8(lhs: Ptr<LifetimeIterator>, mut n: ()) -> LifetimeIterator {
    unimplemented!();
    return LifetimeIterator::new_1({ lhs.with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) });
}
pub fn operator_sub_9(mut lhs: LifetimeIterator, mut rhs: LifetimeIterator) -> () {
    unimplemented!();
    return ((lhs.ptr_).clone() - (rhs.ptr_).clone()) as ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
struct LifetimeIterator_LifetimeCache {
    #[offset(0)]
    #[byte_size(24)]
    cache_: (),
}
pub fn operator_lt_11(lhs: Ptr<LifetimeIterator_Value>, rhs: Ptr<LifetimeIterator_Value>) -> () {
    unimplemented!();
    unimplemented!();
    return ({ lhs.with(|__s| __s.i_) } < { rhs.with(|__s| __s.i_) });
}
pub fn operator_eq_12(lhs: Ptr<LifetimeIterator_Value>, rhs: Ptr<LifetimeIterator_Value>) -> () {
    unimplemented!();
    unimplemented!();
    return ({ lhs.with(|__s| __s.i_) } == { rhs.with(|__s| __s.i_) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor)]
#[byte_size(8)]
pub struct LifetimeIterator_Value {
    #[offset(0)]
    pub i_: (),
    #[offset(4)]
    pub moved_from_: (),
}
impl LifetimeIterator_Value {
    pub fn new_1() -> Self {
        let __this: Value<LifetimeIterator_Value> = Rc::new(RefCell::new(Self {
            i_: unimplemented!(),
            moved_from_: false,
        }));
        let this: Ptr<LifetimeIterator_Value> = __this.as_pointer();
        ({
            LifetimeIterator_LifetimeCacheImpl::insert(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (this).to_any(),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(mut i: ()) -> Self {
        let __this: Value<LifetimeIterator_Value> = Rc::new(RefCell::new(Self {
            i_: i,
            moved_from_: false,
        }));
        let this: Ptr<LifetimeIterator_Value> = __this.as_pointer();
        ({
            LifetimeIterator_LifetimeCacheImpl::insert(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (this).to_any(),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(rhs: Ptr<LifetimeIterator_Value>) -> Self {
        let __this: Value<LifetimeIterator_Value> = Rc::new(RefCell::new(Self {
            i_: rhs.with(|__s| __s.i_),
            moved_from_: false,
        }));
        let this: Ptr<LifetimeIterator_Value> = __this.as_pointer();
        unimplemented!();
        unimplemented!();
        ({
            LifetimeIterator_LifetimeCacheImpl::insert(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (this).to_any(),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(rhs: Ptr<LifetimeIterator_Value>) -> Self {
        let __this: Value<LifetimeIterator_Value> = Rc::new(RefCell::new(Self {
            i_: rhs.with(|__s| __s.i_),
            moved_from_: false,
        }));
        let this: Ptr<LifetimeIterator_Value> = __this.as_pointer();
        unimplemented!();
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        ({
            LifetimeIterator_LifetimeCacheImpl::insert(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (this).to_any(),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for LifetimeIterator_Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_11(
                Rc::new(RefCell::new(LifetimeIterator_Value {
                    i_: self.i_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator_Value {
                    i_: other.i_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_11(
                Rc::new(RefCell::new(LifetimeIterator_Value {
                    i_: other.i_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator_Value {
                    i_: self.i_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
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
        {
            operator_eq_12(
                Rc::new(RefCell::new(LifetimeIterator_Value {
                    i_: self.i_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator_Value {
                    i_: other.i_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for LifetimeIterator_Value {}
impl Clone for LifetimeIterator_Value {
    fn clone(&self) -> Self {
        let __src: Value<LifetimeIterator_Value> = Rc::new(RefCell::new(LifetimeIterator_Value {
            i_: self.i_.clone(),
            moved_from_: self.moved_from_.clone(),
        }));
        LifetimeIterator_Value::copy_from(__src.as_pointer())
    }
}
impl Default for LifetimeIterator_Value {
    fn default() -> Self {
        { LifetimeIterator_Value::new_1() }
    }
}
pub fn operator_lt_13(
    lhs: Ptr<LifetimeIterator_Reference>,
    rhs: Ptr<LifetimeIterator_Reference>,
) -> () {
    unimplemented!();
    unimplemented!();
    return ({
        let _lhs: Ptr<LifetimeIterator_Value> = lhs.with(|__s| __s.v_.clone());
        let _rhs: Ptr<LifetimeIterator_Value> = rhs.with(|__s| __s.v_.clone());
        operator_lt_11(_lhs, _rhs)
    });
}
pub fn operator_eq_14(
    lhs: Ptr<LifetimeIterator_Reference>,
    rhs: Ptr<LifetimeIterator_Reference>,
) -> () {
    unimplemented!();
    unimplemented!();
    return ({
        let _lhs: Ptr<LifetimeIterator_Value> = lhs.with(|__s| __s.v_.clone());
        let _rhs: Ptr<LifetimeIterator_Value> = rhs.with(|__s| __s.v_.clone());
        operator_eq_12(_lhs, _rhs)
    });
}
pub fn swap_15(lhs: LifetimeIterator_Reference, rhs: LifetimeIterator_Reference) {
    let lhs: Value<LifetimeIterator_Reference> = Rc::new(RefCell::new(lhs));
    let rhs: Value<LifetimeIterator_Reference> = Rc::new(RefCell::new(rhs));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor)]
#[byte_size(16)]
pub struct LifetimeIterator_Reference {
    #[offset(0)]
    #[byte_size(8)]
    pub v_: Ptr<LifetimeIterator_Value>,
    #[offset(8)]
    pub moved_from_: (),
}
impl LifetimeIterator_Reference {
    pub fn new(v: Ptr<LifetimeIterator_Value>) -> Self {
        let __this: Value<LifetimeIterator_Reference> = Rc::new(RefCell::new(Self {
            v_: (v).clone(),
            moved_from_: false,
        }));
        let this: Ptr<LifetimeIterator_Reference> = __this.as_pointer();
        ({
            LifetimeIterator_LifetimeCacheImpl::insert(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (this).to_any(),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(rhs: Ptr<LifetimeIterator_Reference>) -> Self {
        let __this: Value<LifetimeIterator_Reference> = Rc::new(RefCell::new(Self {
            v_: rhs.with(|__s| __s.v_.clone()),
            moved_from_: false,
        }));
        let this: Ptr<LifetimeIterator_Reference> = __this.as_pointer();
        unimplemented!();
        unimplemented!();
        ({
            LifetimeIterator_LifetimeCacheImpl::insert(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (this).to_any(),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(rhs: Ptr<LifetimeIterator_Reference>) -> Self {
        let __this: Value<LifetimeIterator_Reference> = Rc::new(RefCell::new(Self {
            v_: rhs.with(|__s| __s.v_.clone()),
            moved_from_: false,
        }));
        let this: Ptr<LifetimeIterator_Reference> = __this.as_pointer();
        unimplemented!();
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        ({
            LifetimeIterator_LifetimeCacheImpl::insert(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (this).to_any(),
            )
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for LifetimeIterator_Reference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_13(
                Rc::new(RefCell::new(LifetimeIterator_Reference {
                    v_: self.v_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator_Reference {
                    v_: other.v_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_13(
                Rc::new(RefCell::new(LifetimeIterator_Reference {
                    v_: other.v_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator_Reference {
                    v_: self.v_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
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
        {
            operator_eq_14(
                Rc::new(RefCell::new(LifetimeIterator_Reference {
                    v_: self.v_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator_Reference {
                    v_: other.v_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for LifetimeIterator_Reference {}
impl Clone for LifetimeIterator_Reference {
    fn clone(&self) -> Self {
        let __src: Value<LifetimeIterator_Reference> =
            Rc::new(RefCell::new(LifetimeIterator_Reference {
                v_: self.v_.clone(),
                moved_from_: self.moved_from_.clone(),
            }));
        LifetimeIterator_Reference::copy_from(__src.as_pointer())
    }
}
impl Default for LifetimeIterator_Reference {
    fn default() -> Self {
        {
            LifetimeIterator_Reference {
                v_: Ptr::<LifetimeIterator_Value>::null(),
                moved_from_: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(16)]
pub struct LifetimeIterator {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr_: Ptr<LifetimeIterator_Value>,
    #[offset(8)]
    pub moved_from_: (),
}
impl LifetimeIterator {
    pub fn new_1(mut ptr: Ptr<LifetimeIterator_Value>) -> Self {
        Self {
            ptr_: (ptr).clone(),
            moved_from_: false,
        }
    }
    pub fn copy_from(rhs: Ptr<LifetimeIterator>) -> Self {
        let __this: LifetimeIterator = Self {
            ptr_: rhs.with(|__s| __s.ptr_.clone()),
            moved_from_: false,
        };
        unimplemented!();
        __this
    }
    pub fn move_from(rhs: Ptr<LifetimeIterator>) -> Self {
        let __this: LifetimeIterator = Self {
            ptr_: rhs.with(|__s| __s.ptr_.clone()),
            moved_from_: false,
        };
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        field!(rhs, ptr_).write(Ptr::<LifetimeIterator_Value>::null());
        __this
    }
}
impl std::cmp::Ord for LifetimeIterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_2(
                Rc::new(RefCell::new(LifetimeIterator {
                    ptr_: self.ptr_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator {
                    ptr_: other.ptr_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_2(
                Rc::new(RefCell::new(LifetimeIterator {
                    ptr_: other.ptr_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator {
                    ptr_: self.ptr_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
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
        {
            operator_eq_0(
                Rc::new(RefCell::new(LifetimeIterator {
                    ptr_: self.ptr_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LifetimeIterator {
                    ptr_: other.ptr_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for LifetimeIterator {}
impl Clone for LifetimeIterator {
    fn clone(&self) -> Self {
        let __src: Value<LifetimeIterator> = Rc::new(RefCell::new(LifetimeIterator {
            ptr_: self.ptr_.clone(),
            moved_from_: self.moved_from_.clone(),
        }));
        LifetimeIterator::copy_from(__src.as_pointer())
    }
}
impl Default for LifetimeIterator {
    fn default() -> Self {
        {
            LifetimeIterator {
                ptr_: Ptr::<LifetimeIterator_Value>::null(),
                moved_from_: false,
            }
        }
    }
}
thread_local!(
    pub static lifetime_cache_10: Value<LifetimeIterator_LifetimeCache> =
        Rc::new(RefCell::new(<LifetimeIterator_LifetimeCache>::default()));
);
pub fn operator_eq_16(lhs: Ptr<ConstexprIterator>, rhs: Ptr<ConstexprIterator>) -> () {
    unimplemented!();
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_ne_17(lhs: Ptr<ConstexprIterator>, rhs: Ptr<ConstexprIterator>) -> () {
    unimplemented!();
    return ({ lhs.with(|__s| __s.ptr_.clone()) } != { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_18(lhs: Ptr<ConstexprIterator>, rhs: Ptr<ConstexprIterator>) -> () {
    unimplemented!();
    return std::cmp::Ord::cmp(
        &(lhs.with(|__s| __s.ptr_.clone())),
        &(rhs.with(|__s| __s.ptr_.clone())),
    );
}
pub fn operator_add_19(lhs: Ptr<ConstexprIterator>, mut n: ()) -> ConstexprIterator {
    unimplemented!();
    return ConstexprIterator::new_1({ lhs.with(|__s| __s.ptr_.clone()).offset((n) as isize) });
}
pub fn operator_add_20(mut n: (), lhs: Ptr<ConstexprIterator>) -> ConstexprIterator {
    unimplemented!();
    return ConstexprIterator::new_1({ lhs.with(|__s| __s.ptr_.clone()).offset((n) as isize) });
}
pub fn operator_sub_21(lhs: Ptr<ConstexprIterator>, mut n: ()) -> ConstexprIterator {
    unimplemented!();
    return ConstexprIterator::new_1({ lhs.with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) });
}
pub fn operator_sub_22(mut lhs: ConstexprIterator, mut rhs: ConstexprIterator) -> () {
    unimplemented!();
    return ((lhs.ptr_).clone() - (rhs.ptr_).clone()) as ();
}
pub fn operator_lt_23(
    lhs: Ptr<ConstexprIterator_Reference>,
    rhs: Ptr<ConstexprIterator_Reference>,
) -> () {
    unimplemented!();
    return ({ (lhs.with(|__s| __s.v_.clone()).read()) } < {
        (rhs.with(|__s| __s.v_.clone()).read())
    });
}
pub fn operator_eq_24(
    lhs: Ptr<ConstexprIterator_Reference>,
    rhs: Ptr<ConstexprIterator_Reference>,
) -> () {
    unimplemented!();
    return ({ (lhs.with(|__s| __s.v_.clone()).read()) } == {
        (rhs.with(|__s| __s.v_.clone()).read())
    });
}
pub fn swap_25(mut lhs: ConstexprIterator_Reference, mut rhs: ConstexprIterator_Reference) {
    unimplemented!();
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(16)]
pub struct ConstexprIterator_Reference {
    #[offset(0)]
    #[byte_size(8)]
    pub v_: Ptr<()>,
    #[offset(8)]
    pub moved_from_: (),
}
impl ConstexprIterator_Reference {
    pub fn new(v: Ptr<()>) -> Self {
        Self {
            v_: (v).clone(),
            moved_from_: false,
        }
    }
    pub fn move_from(rhs: Ptr<ConstexprIterator_Reference>) -> Self {
        let __this: ConstexprIterator_Reference = Self {
            v_: rhs.with(|__s| __s.v_.clone()),
            moved_from_: false,
        };
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        __this
    }
}
impl std::cmp::Ord for ConstexprIterator_Reference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_23(
                Rc::new(RefCell::new(ConstexprIterator_Reference {
                    v_: self.v_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ConstexprIterator_Reference {
                    v_: other.v_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_23(
                Rc::new(RefCell::new(ConstexprIterator_Reference {
                    v_: other.v_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ConstexprIterator_Reference {
                    v_: self.v_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
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
        {
            operator_eq_24(
                Rc::new(RefCell::new(ConstexprIterator_Reference {
                    v_: self.v_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ConstexprIterator_Reference {
                    v_: other.v_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ConstexprIterator_Reference {}
impl Default for ConstexprIterator_Reference {
    fn default() -> Self {
        {
            ConstexprIterator_Reference {
                v_: Ptr::<()>::null(),
                moved_from_: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(16)]
pub struct ConstexprIterator {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr_: Ptr<()>,
    #[offset(8)]
    pub moved_from_: (),
}
impl ConstexprIterator {
    pub fn new_1(mut ptr: Ptr<()>) -> Self {
        Self {
            ptr_: (ptr).clone(),
            moved_from_: false,
        }
    }
    pub fn copy_from(rhs: Ptr<ConstexprIterator>) -> Self {
        let __this: ConstexprIterator = Self {
            ptr_: rhs.with(|__s| __s.ptr_.clone()),
            moved_from_: false,
        };
        unimplemented!();
        __this
    }
    pub fn move_from(rhs: Ptr<ConstexprIterator>) -> Self {
        let __this: ConstexprIterator = Self {
            ptr_: rhs.with(|__s| __s.ptr_.clone()),
            moved_from_: false,
        };
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        field!(rhs, ptr_).write(Ptr::<()>::null());
        __this
    }
}
impl std::cmp::Ord for ConstexprIterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_18(
                Rc::new(RefCell::new(ConstexprIterator {
                    ptr_: self.ptr_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ConstexprIterator {
                    ptr_: other.ptr_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
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
        {
            operator_eq_16(
                Rc::new(RefCell::new(ConstexprIterator {
                    ptr_: self.ptr_.clone(),
                    moved_from_: self.moved_from_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ConstexprIterator {
                    ptr_: other.ptr_.clone(),
                    moved_from_: other.moved_from_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ConstexprIterator {}
impl Clone for ConstexprIterator {
    fn clone(&self) -> Self {
        let __src: Value<ConstexprIterator> = Rc::new(RefCell::new(ConstexprIterator {
            ptr_: self.ptr_.clone(),
            moved_from_: self.moved_from_.clone(),
        }));
        ConstexprIterator::copy_from(__src.as_pointer())
    }
}
impl Default for ConstexprIterator {
    fn default() -> Self {
        {
            ConstexprIterator {
                ptr_: Ptr::<()>::null(),
                moved_from_: false,
            }
        }
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub fn new_1(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_LifetimeIterator_Value__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_LifetimeIterator_Value__32_> = __this.as_pointer();
        const _: () = assert!((0_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(0_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub fn new_2(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_LifetimeIterator_Value__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_LifetimeIterator_Value__32_> = __this.as_pointer();
        const _: () = assert!((1_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub fn new_3(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_LifetimeIterator_Value__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_LifetimeIterator_Value__32_> = __this.as_pointer();
        const _: () = assert!((2_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(2_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub fn new_4(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_LifetimeIterator_Value__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_LifetimeIterator_Value__32_> = __this.as_pointer();
        const _: () = assert!((3_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(3_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub fn new_5(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_LifetimeIterator_Value__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_LifetimeIterator_Value__32_> = __this.as_pointer();
        const _: () = assert!((9_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(9_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub fn new_6(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_LifetimeIterator_Value__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_LifetimeIterator_Value__32_> = __this.as_pointer();
        const _: () = assert!((8_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(8_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_LifetimeIterator_Value__32_ {
    pub fn new_7(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_LifetimeIterator_Value__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_LifetimeIterator_Value__32_> = __this.as_pointer();
        const _: () = assert!((11_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(11_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(264)]
pub struct Input_LifetimeIterator_Value__32_ {
    #[offset(0)]
    size_: (),
    #[offset(8)]
    #[byte_size(256)]
    values_: Box<[LifetimeIterator_Value]>,
}
impl Default for Input_LifetimeIterator_Value__32_ {
    fn default() -> Self {
        {
            Input_LifetimeIterator_Value__32_ {
                size_: 0_(),
                values_: Box::new([
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
                ]),
            }
        }
    }
}
impl Input_int__32_ {
    pub fn new_1(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_int__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_int__32_> = __this.as_pointer();
        const _: () = assert!((0_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(0_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_int__32_ {
    pub fn new_2(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_int__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_int__32_> = __this.as_pointer();
        const _: () = assert!((1_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(1_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_int__32_ {
    pub fn new_3(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_int__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_int__32_> = __this.as_pointer();
        const _: () = assert!((2_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(2_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_int__32_ {
    pub fn new_4(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_int__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_int__32_> = __this.as_pointer();
        const _: () = assert!((3_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(3_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_int__32_ {
    pub fn new_5(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_int__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_int__32_> = __this.as_pointer();
        const _: () = assert!((9_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(9_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_int__32_ {
    pub fn new_6(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_int__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_int__32_> = __this.as_pointer();
        const _: () = assert!((8_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(8_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Input_int__32_ {
    pub fn new_7(from: ()) -> Self {
        let from: Value<()> = Rc::new(RefCell::new(from));
        let __this: Value<Input_int__32_> = Rc::new(RefCell::new(Self {
            size_: 0_(),
            values_: Box::new([
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
            ]),
        }));
        let this: Ptr<Input_int__32_> = __this.as_pointer();
        const _: () = assert!((11_() <= 32_()), "N <= StorageSize");;
        unimplemented!();
        field!(this, size_).write(11_());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(136)]
pub struct Input_int__32_ {
    #[offset(0)]
    size_: (),
    #[offset(8)]
    #[byte_size(128)]
    values_: Box<[()]>,
}
impl Default for Input_int__32_ {
    fn default() -> Self {
        {
            Input_int__32_ {
                size_: 0_(),
                values_: Box::new([
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
                ]),
            }
        }
    }
}
pub fn get_simple_in_26() -> () {
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return (*result.borrow_mut()).clone();
}
pub fn get_simple_in_27() -> () {
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return (*result.borrow_mut()).clone();
}
pub fn get_sorted_in_28() -> () {
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return (*result.borrow_mut()).clone();
}
pub fn get_sorted_in_29() -> () {
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return (*result.borrow_mut()).clone();
}
pub fn get_sort_test_in_30() -> () {
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return (*result.borrow_mut()).clone();
}
pub fn get_sort_test_in_31() -> () {
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return (*result.borrow_mut()).clone();
}
pub fn test_32(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_33(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_34(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_35(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_36(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_37(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_38(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_39(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_40(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_41(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_42(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_43(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_44(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_45(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_46(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_47(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_48(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_49(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_50(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_51(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_52(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_53(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_54(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_55(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_56(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_57(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_58(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_59(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_60(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_61(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_62(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_63(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_64(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_65(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_66(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_67(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_68(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_69(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_70(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_71(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_72(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_73(inputs: (), func: FnPtr<fn(LifetimeIterator, LifetimeIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, LifetimeIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _e: LifetimeIterator =
                LifetimeIterator::new_1({ ({ Input_LifetimeIterator_Value__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_74(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_75(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_76(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_77(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_78(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_79(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_80(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_81(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_82(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_83(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_84(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_85(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_86(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_87(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_88(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_89(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_90(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_91(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_92(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_93(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_94(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_95(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_96(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_97(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_98(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_99(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_100(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_101(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_102(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_103(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_104(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_105(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_106(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_107(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_108(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_109(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_110(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_111(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_112(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_113(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_114(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_115(inputs: (), func: FnPtr<fn(ConstexprIterator, ConstexprIterator)>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ConstexprIterator)>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _e: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::end(&in_) }) });
            (*func.borrow()).call(_b, _e)
        });
    }
}
pub fn test_n_116(inputs: (), func: FnPtr<fn(LifetimeIterator, ())>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, ())>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _n: () = ({ Input_LifetimeIterator_Value__32_Impl::size(&in_) });
            (*func.borrow()).call(_b, _n)
        });
    }
}
pub fn test_n_117(inputs: (), func: FnPtr<fn(LifetimeIterator, ())>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, ())>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _n: () = ({ Input_LifetimeIterator_Value__32_Impl::size(&in_) });
            (*func.borrow()).call(_b, _n)
        });
    }
}
pub fn test_n_118(inputs: (), func: FnPtr<fn(LifetimeIterator, ())>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(LifetimeIterator, ())>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_LifetimeIterator_Value__32_> = (in_).clone();
        ({
            let _b: LifetimeIterator = LifetimeIterator::new_1({
                ({ Input_LifetimeIterator_Value__32_Impl::begin(&in_) })
            });
            let _n: () = ({ Input_LifetimeIterator_Value__32_Impl::size(&in_) });
            (*func.borrow()).call(_b, _n)
        });
    }
}
pub fn test_n_119(inputs: (), func: FnPtr<fn(ConstexprIterator, ())>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ())>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _n: () = ({ Input_int__32_Impl::size(&in_) });
            (*func.borrow()).call(_b, _n)
        });
    }
}
pub fn test_n_120(inputs: (), func: FnPtr<fn(ConstexprIterator, ())>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ())>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _n: () = ({ Input_int__32_Impl::size(&in_) });
            (*func.borrow()).call(_b, _n)
        });
    }
}
pub fn test_n_121(inputs: (), func: FnPtr<fn(ConstexprIterator, ())>) {
    let inputs: Value<()> = Rc::new(RefCell::new(inputs));
    let func: Value<FnPtr<fn(ConstexprIterator, ())>> = Rc::new(RefCell::new(func));
    'loop_: for in_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let in_: Ptr<Input_int__32_> = (in_).clone();
        ({
            let _b: ConstexprIterator =
                ConstexprIterator::new_1({ ({ Input_int__32_Impl::begin(&in_) }) });
            let _n: () = ({ Input_int__32_Impl::size(&in_) });
            (*func.borrow()).call(_b, _n)
        });
    }
}
pub fn to_int_122(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn to_int_123(x: LifetimeIterator_Value) -> () {
    let x: Value<LifetimeIterator_Value> = Rc::new(RefCell::new(x));
    return { (*x.borrow()).i_ };
}
pub fn rand_gen_124() -> () {
    return unimplemented!();
}
pub fn test_125() -> () {
    let is_neg: Value<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn(Ptr<LifetimeIterator_Value>) -> ()>::new(
            |val: Ptr<LifetimeIterator_Value>| -> () {
                {
                    return (({
                        to_int_123(LifetimeIterator_Value::copy_from({ (val).clone() }))
                    }) < 0_());
                }
            },
        ),
    ));
    let gen_: Value<FnPtr<fn() -> LifetimeIterator_Value>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> LifetimeIterator_Value>::new(
            || -> LifetimeIterator_Value {
                {
                    return LifetimeIterator_Value::new_2({ 42_() });
                }
            },
        )));
    let identity: Value<FnPtr<fn(LifetimeIterator_Value) -> LifetimeIterator_Value>> =
        Rc::new(RefCell::new(FnPtr::<
            fn(LifetimeIterator_Value) -> LifetimeIterator_Value,
        >::new(
            |val: LifetimeIterator_Value| -> LifetimeIterator_Value {
                {
                    let val: Value<LifetimeIterator_Value> = Rc::new(RefCell::new(val));
                    return LifetimeIterator_Value::move_from({ val.as_pointer() });
                }
            },
        )));
    let mut N: () = 32_();
    let output: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let x: Value<LifetimeIterator_Value> =
        Rc::new(RefCell::new(LifetimeIterator_Value::new_2({ 1_() })));
    let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
    let y: Value<LifetimeIterator_Value> =
        Rc::new(RefCell::new(LifetimeIterator_Value::new_2({ 3_() })));
    let _dtor_y = ScopedDestructor::new(&y, |__p| __p.destructor());
    let simple_in: Value<()> = Rc::new(RefCell::new(({ get_simple_in_26() })));
    let sorted_in: Value<()> = Rc::new(RefCell::new(({ get_sorted_in_28() })));
    let sort_test_in: Value<()> = Rc::new(RefCell::new(({ get_sort_test_in_30() })));
    ({
        test_32(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_33(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_34(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_35(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_36(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_37(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_38(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_39(
            (*sorted_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_40(
            (*sorted_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_41(
            (*sorted_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_42(
            (*sorted_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_43(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_44(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_45(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_46(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_47(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_48(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_n_116(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, mut n: ()| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_49(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_n_117(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: LifetimeIterator, mut n: ()| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_50(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_51(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_52(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_53(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_54(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let identity: Ptr<FnPtr<fn(LifetimeIterator_Value) -> LifetimeIterator_Value>> =
                        identity.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_55(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let gen_: Ptr<FnPtr<fn() -> LifetimeIterator_Value>> = gen_.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_n_118(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let gen_: Ptr<FnPtr<fn() -> LifetimeIterator_Value>> = gen_.as_pointer();
                },
                |b: LifetimeIterator, mut n: ()| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_56(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_57(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_58(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                    let y: Ptr<LifetimeIterator_Value> = y.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_59(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                    let y: Ptr<LifetimeIterator_Value> = y.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_60(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                    let y: Ptr<LifetimeIterator_Value> = y.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_61(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                    let y: Ptr<LifetimeIterator_Value> = y.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_62(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_63(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<LifetimeIterator_Value> = x.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_64(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_65(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    {
        ({
            test_66(
                (*simple_in.borrow()).clone(),
                FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                    |b: LifetimeIterator, e: LifetimeIterator| {
                        let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                        let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    ({
        test_67(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                        is_neg.as_pointer();
                },
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    {
        ({
            test_68(
                (*simple_in.borrow()).clone(),
                lambda!(
                    {
                        let is_neg: Ptr<FnPtr<fn(Ptr<LifetimeIterator_Value>) -> ()>> =
                            is_neg.as_pointer();
                    },
                    |b: LifetimeIterator, e: LifetimeIterator| {
                        let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                        let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                        &(unimplemented!());
                    }
                ),
            )
        });
    }
    {
        ({
            test_69(
                (*sort_test_in.borrow()).clone(),
                FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                    |b: LifetimeIterator, e: LifetimeIterator| {
                        let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                        let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    {
        ({
            test_70(
                (*sort_test_in.borrow()).clone(),
                FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                    |b: LifetimeIterator, e: LifetimeIterator| {
                        let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                        let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    ({
        test_71(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_72(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_73(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(LifetimeIterator, LifetimeIterator)>::new(
                |b: LifetimeIterator, e: LifetimeIterator| {
                    let b: Value<LifetimeIterator> = Rc::new(RefCell::new(b));
                    let e: Value<LifetimeIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    return true;
}
pub fn test_126() -> () {
    let is_neg: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn(Ptr<()>) -> ()>::new(|val: Ptr<()>| -> () {
            {
                return (({ to_int_122((val.read())) }) < 0_());
            }
        }),
    ));
    let gen_: Value<FnPtr<fn() -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
            {
                return 42_();
            }
        })));
    let identity: Value<FnPtr<fn(()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(|val: ()| -> () {
            {
                let val: Value<()> = Rc::new(RefCell::new(val));
                return (*val.borrow());
            }
        })));
    let mut N: () = 32_();
    let output: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let x: Value<()> = Rc::new(RefCell::new(1_()));
    let y: Value<()> = Rc::new(RefCell::new(3_()));
    let simple_in: Value<()> = Rc::new(RefCell::new(({ get_simple_in_27() })));
    let sorted_in: Value<()> = Rc::new(RefCell::new(({ get_sorted_in_29() })));
    let sort_test_in: Value<()> = Rc::new(RefCell::new(({ get_sort_test_in_31() })));
    ({
        test_74(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_75(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_76(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_77(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_78(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_79(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_80(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_81(
            (*sorted_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<()> = x.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_82(
            (*sorted_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<()> = x.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_83(
            (*sorted_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<()> = x.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_84(
            (*sorted_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<()> = x.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_85(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_86(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_87(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_88(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<()> = x.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_89(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_90(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_n_119(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, mut n: ()| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_91(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_n_120(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: ConstexprIterator, mut n: ()| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_92(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_93(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_94(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_95(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_96(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let identity: Ptr<FnPtr<fn(()) -> ()>> = identity.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_97(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let gen_: Ptr<FnPtr<fn() -> ()>> = gen_.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_n_121(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let gen_: Ptr<FnPtr<fn() -> ()>> = gen_.as_pointer();
                },
                |b: ConstexprIterator, mut n: ()| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_98(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let x: Ptr<()> = x.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_99(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_100(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<()> = x.as_pointer();
                    let y: Ptr<()> = y.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_101(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                    let y: Ptr<()> = y.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_102(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let x: Ptr<()> = x.as_pointer();
                    let y: Ptr<()> = y.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_103(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                    let y: Ptr<()> = y.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_104(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let out: Ptr<()> = out.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_105(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let x: Ptr<()> = x.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_106(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    ({
        test_107(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    {
        ({
            test_108(
                (*simple_in.borrow()).clone(),
                FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                    |b: ConstexprIterator, e: ConstexprIterator| {
                        let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                        let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    ({
        test_109(
            (*simple_in.borrow()).clone(),
            lambda!(
                {
                    let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                },
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                }
            ),
        )
    });
    {
        ({
            test_110(
                (*simple_in.borrow()).clone(),
                lambda!(
                    {
                        let is_neg: Ptr<FnPtr<fn(Ptr<()>) -> ()>> = is_neg.as_pointer();
                    },
                    |b: ConstexprIterator, e: ConstexprIterator| {
                        let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                        let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                        &(unimplemented!());
                    }
                ),
            )
        });
    }
    {
        ({
            test_111(
                (*sort_test_in.borrow()).clone(),
                FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                    |b: ConstexprIterator, e: ConstexprIterator| {
                        let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                        let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    {
        ({
            test_112(
                (*sort_test_in.borrow()).clone(),
                FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                    |b: ConstexprIterator, e: ConstexprIterator| {
                        let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                        let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                        &(unimplemented!());
                    },
                ),
            )
        });
    }
    ({
        test_113(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_114(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    ({
        test_115(
            (*simple_in.borrow()).clone(),
            FnPtr::<fn(ConstexprIterator, ConstexprIterator)>::new(
                |b: ConstexprIterator, e: ConstexprIterator| {
                    let b: Value<ConstexprIterator> = Rc::new(RefCell::new(b));
                    let e: Value<ConstexprIterator> = Rc::new(RefCell::new(e));
                    &(unimplemented!());
                },
            ),
        )
    });
    return true;
}
pub fn test_all_127() {
    ({ test_125() });
    const _: () = assert!(true, "test<ConstexprIterator>()");;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_all_127() });
    return 0_();
}
pub trait ConstexprIteratorImpl {
    fn copy_assign(&self, rhs: Ptr<ConstexprIterator>) -> Ptr<ConstexprIterator>;
    fn move_assign(&self, rhs: Ptr<ConstexprIterator>) -> Ptr<ConstexprIterator>;
    fn operator_deref(&self) -> ConstexprIterator_Reference;
    fn operator_inc_8(&self) -> Ptr<ConstexprIterator>;
    fn operator_post_inc_9(&self, _a0: ()) -> ConstexprIterator;
    fn operator_dec_10(&self) -> Ptr<ConstexprIterator>;
    fn operator_post_dec_11(&self, _a0: ()) -> ConstexprIterator;
    fn operator_add_assign(&self, n: ()) -> Ptr<ConstexprIterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<ConstexprIterator>;
    fn operator_index(&self, i: ()) -> ConstexprIterator_Reference;
}
impl ConstexprIteratorImpl for Ptr<ConstexprIterator> {
    fn copy_assign(&self, rhs: Ptr<ConstexprIterator>) -> Ptr<ConstexprIterator> {
        unimplemented!();
        field!((*self), ptr_).write({ rhs.with(|__s| __s.ptr_.clone()) });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
    fn move_assign(&self, rhs: Ptr<ConstexprIterator>) -> Ptr<ConstexprIterator> {
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        field!((*self), ptr_).write({ rhs.with(|__s| __s.ptr_.clone()) });
        field!(rhs, ptr_).write(Ptr::<()>::null());
        return (*self).clone();
    }
    fn operator_deref(&self) -> ConstexprIterator_Reference {
        unimplemented!();
        return ConstexprIterator_Reference::new({ (*self).with(|__s| __s.ptr_.clone()) });
    }
    fn operator_inc_8(&self) -> Ptr<ConstexprIterator> {
        unimplemented!();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> ConstexprIterator {
        unimplemented!();
        let tmp: Value<ConstexprIterator> = Rc::new(RefCell::new(ConstexprIterator::copy_from({
            (*self).clone()
        })));
        ({ ConstexprIteratorImpl::operator_inc_8(&(*self)) });
        return ConstexprIterator::move_from({ tmp.as_pointer() });
    }
    fn operator_dec_10(&self) -> Ptr<ConstexprIterator> {
        unimplemented!();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_11(&self, mut _a0: ()) -> ConstexprIterator {
        unimplemented!();
        let tmp: Value<ConstexprIterator> = Rc::new(RefCell::new(ConstexprIterator::copy_from({
            (*self).clone()
        })));
        ({ ConstexprIteratorImpl::operator_dec_10(&(*self)) });
        return ConstexprIterator::move_from({ tmp.as_pointer() });
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<ConstexprIterator> {
        unimplemented!();
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<ConstexprIterator> {
        unimplemented!();
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_index(&self, mut i: ()) -> ConstexprIterator_Reference {
        return ConstexprIterator_Reference::new({
            ((*self).with(|__s| __s.ptr_.clone()).offset((i) as isize))
        });
    }
}
pub trait ConstexprIterator_ReferenceImpl {
    fn copy_assign(
        &self,
        rhs: Ptr<ConstexprIterator_Reference>,
    ) -> Ptr<ConstexprIterator_Reference>;
    fn move_assign(
        &self,
        rhs: Ptr<ConstexprIterator_Reference>,
    ) -> Ptr<ConstexprIterator_Reference>;
    fn to_(&self) -> ();
    fn operator_assign_7(&self, v: ()) -> Ptr<ConstexprIterator_Reference>;
}
impl ConstexprIterator_ReferenceImpl for Ptr<ConstexprIterator_Reference> {
    fn copy_assign(
        &self,
        rhs: Ptr<ConstexprIterator_Reference>,
    ) -> Ptr<ConstexprIterator_Reference> {
        unimplemented!();
        field!((*self), v_).write({ rhs.with(|__s| __s.v_.clone()) });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
    fn move_assign(
        &self,
        rhs: Ptr<ConstexprIterator_Reference>,
    ) -> Ptr<ConstexprIterator_Reference> {
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        field!((*self), v_).write({ rhs.with(|__s| __s.v_.clone()) });
        return (*self).clone();
    }
    fn to_(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.v_.clone()).read());
    }
    fn operator_assign_7(&self, mut v: ()) -> Ptr<ConstexprIterator_Reference> {
        (*self).with(|__s| __s.v_.clone()).write({ v });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub trait Input_LifetimeIterator_Value__32_Impl {
    fn begin(&self) -> Ptr<LifetimeIterator_Value>;
    fn end(&self) -> Ptr<LifetimeIterator_Value>;
    fn size(&self) -> ();
    fn copy_assign(
        &self,
        _a0: Ptr<Input_LifetimeIterator_Value__32_>,
    ) -> Ptr<Input_LifetimeIterator_Value__32_>;
    fn destructor(&self);
}
impl Input_LifetimeIterator_Value__32_Impl for Ptr<Input_LifetimeIterator_Value__32_> {
    fn begin(&self) -> Ptr<LifetimeIterator_Value> {
        return (array_field_ptr!((*self), values_) as Ptr<LifetimeIterator_Value>);
    }
    fn end(&self) -> Ptr<LifetimeIterator_Value> {
        return (array_field_ptr!((*self), values_) as Ptr<LifetimeIterator_Value>)
            .offset(((*self).with(|__s| __s.size_)) as isize);
    }
    fn size(&self) -> () {
        return (*self).with(|__s| __s.size_);
    }
    fn copy_assign(
        &self,
        _a0: Ptr<Input_LifetimeIterator_Value__32_>,
    ) -> Ptr<Input_LifetimeIterator_Value__32_> {
        field!((*self), size_).write({ _a0.with(|__s| __s.size_) });
        let mut __i0: () = 0_();
        'loop_: while (__i0 != 32_()) {
            ({
                let _rhs: Ptr<LifetimeIterator_Value> = (array_field_ptr!(_a0, values_)
                    as Ptr<LifetimeIterator_Value>)
                    .offset((__i0) as isize);
                LifetimeIterator_ValueImpl::copy_assign(
                    &(array_field_ptr!((*self), values_) as Ptr<LifetimeIterator_Value>)
                        .offset((__i0) as isize),
                    _rhs,
                )
            });
            __i0.prefix_inc();
        }
        return (*self).clone();
    }
    fn destructor(&self) {
        {
            let __p = array_field_ptr!(self, values_);
            for __i in 0..__p.len() {
                LifetimeIterator_ValueImpl::destructor(&__p.offset(__i as isize));
            }
        }
    }
}
pub trait Input_int__32_Impl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
    fn size(&self) -> ();
}
impl Input_int__32_Impl for Ptr<Input_int__32_> {
    fn begin(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), values_) as Ptr<()>);
    }
    fn end(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), values_) as Ptr<()>)
            .offset(((*self).with(|__s| __s.size_)) as isize);
    }
    fn size(&self) -> () {
        return (*self).with(|__s| __s.size_);
    }
}
pub trait LifetimeIteratorImpl {
    fn copy_assign(&self, rhs: Ptr<LifetimeIterator>) -> Ptr<LifetimeIterator>;
    fn move_assign(&self, rhs: Ptr<LifetimeIterator>) -> Ptr<LifetimeIterator>;
    fn operator_deref(&self) -> LifetimeIterator_Reference;
    fn operator_inc_8(&self) -> Ptr<LifetimeIterator>;
    fn operator_post_inc_9(&self, _a0: ()) -> LifetimeIterator;
    fn operator_dec_10(&self) -> Ptr<LifetimeIterator>;
    fn operator_post_dec_11(&self, _a0: ()) -> LifetimeIterator;
    fn operator_add_assign(&self, n: ()) -> Ptr<LifetimeIterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<LifetimeIterator>;
    fn operator_index(&self, i: ()) -> LifetimeIterator_Reference;
}
impl LifetimeIteratorImpl for Ptr<LifetimeIterator> {
    fn copy_assign(&self, rhs: Ptr<LifetimeIterator>) -> Ptr<LifetimeIterator> {
        unimplemented!();
        field!((*self), ptr_).write({ rhs.with(|__s| __s.ptr_.clone()) });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
    fn move_assign(&self, rhs: Ptr<LifetimeIterator>) -> Ptr<LifetimeIterator> {
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        field!((*self), ptr_).write({ rhs.with(|__s| __s.ptr_.clone()) });
        field!(rhs, ptr_).write(Ptr::<LifetimeIterator_Value>::null());
        return (*self).clone();
    }
    fn operator_deref(&self) -> LifetimeIterator_Reference {
        unimplemented!();
        return LifetimeIterator_Reference::new({ (*self).with(|__s| __s.ptr_.clone()) });
    }
    fn operator_inc_8(&self) -> Ptr<LifetimeIterator> {
        unimplemented!();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> LifetimeIterator {
        unimplemented!();
        let tmp: Value<LifetimeIterator> = Rc::new(RefCell::new(LifetimeIterator::copy_from({
            (*self).clone()
        })));
        ({ LifetimeIteratorImpl::operator_inc_8(&(*self)) });
        return LifetimeIterator::move_from({ tmp.as_pointer() });
    }
    fn operator_dec_10(&self) -> Ptr<LifetimeIterator> {
        unimplemented!();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_11(&self, mut _a0: ()) -> LifetimeIterator {
        unimplemented!();
        let tmp: Value<LifetimeIterator> = Rc::new(RefCell::new(LifetimeIterator::copy_from({
            (*self).clone()
        })));
        ({ LifetimeIteratorImpl::operator_dec_10(&(*self)) });
        return LifetimeIterator::move_from({ tmp.as_pointer() });
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<LifetimeIterator> {
        unimplemented!();
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<LifetimeIterator> {
        unimplemented!();
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_index(&self, mut i: ()) -> LifetimeIterator_Reference {
        unimplemented!();
        return LifetimeIterator_Reference::new({
            ((*self).with(|__s| __s.ptr_.clone()).offset((i) as isize))
        });
    }
}
pub trait LifetimeIterator_LifetimeCacheImpl {
    fn contains(&self, ptr: ()) -> ();
    fn insert(&self, ptr: ());
    fn erase(&self, ptr: ());
}
impl LifetimeIterator_LifetimeCacheImpl for Ptr<LifetimeIterator_LifetimeCache> {
    fn contains(&self, mut ptr: ()) -> () {
        return unimplemented!();
    }
    fn insert(&self, mut ptr: ()) {
        unimplemented!();
        unimplemented!();
    }
    fn erase(&self, mut ptr: ()) {
        unimplemented!();
        unimplemented!();
    }
}
pub trait LifetimeIterator_ReferenceImpl {
    fn destructor(&self);
    fn copy_assign(&self, rhs: Ptr<LifetimeIterator_Reference>) -> Ptr<LifetimeIterator_Reference>;
    fn move_assign(&self, rhs: Ptr<LifetimeIterator_Reference>) -> Ptr<LifetimeIterator_Reference>;
    fn to_LifetimeIterator_Value(&self) -> LifetimeIterator_Value;
    fn operator_assign_8(&self, v: LifetimeIterator_Value) -> Ptr<LifetimeIterator_Reference>;
}
impl LifetimeIterator_ReferenceImpl for Ptr<LifetimeIterator_Reference> {
    fn destructor(&self) {
        ({
            LifetimeIterator_LifetimeCacheImpl::erase(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (*self).to_any(),
            )
        });
    }
    fn copy_assign(&self, rhs: Ptr<LifetimeIterator_Reference>) -> Ptr<LifetimeIterator_Reference> {
        unimplemented!();
        unimplemented!();
        ({
            let _rhs: Ptr<LifetimeIterator_Value> = rhs.with(|__s| __s.v_.clone());
            LifetimeIterator_ValueImpl::copy_assign(&(*self).with(|__s| __s.v_.clone()), _rhs)
        });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
    fn move_assign(&self, rhs: Ptr<LifetimeIterator_Reference>) -> Ptr<LifetimeIterator_Reference> {
        unimplemented!();
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        ({
            let _rhs: Ptr<LifetimeIterator_Value> = rhs.with(|__s| __s.v_.clone());
            LifetimeIterator_ValueImpl::copy_assign(&(*self).with(|__s| __s.v_.clone()), _rhs)
        });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
    fn to_LifetimeIterator_Value(&self) -> LifetimeIterator_Value {
        unimplemented!();
        unimplemented!();
        return LifetimeIterator_Value::copy_from({ (*self).with(|__s| __s.v_.clone()) });
    }
    fn operator_assign_8(&self, v: LifetimeIterator_Value) -> Ptr<LifetimeIterator_Reference> {
        let v: Value<LifetimeIterator_Value> = Rc::new(RefCell::new(v));
        unimplemented!();
        unimplemented!();
        ({
            let _rhs: Ptr<LifetimeIterator_Value> = v.as_pointer();
            LifetimeIterator_ValueImpl::copy_assign(&(*self).with(|__s| __s.v_.clone()), _rhs)
        });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub trait LifetimeIterator_ValueImpl {
    fn destructor(&self);
    fn copy_assign(&self, rhs: Ptr<LifetimeIterator_Value>) -> Ptr<LifetimeIterator_Value>;
    fn move_assign(&self, rhs: Ptr<LifetimeIterator_Value>) -> Ptr<LifetimeIterator_Value>;
}
impl LifetimeIterator_ValueImpl for Ptr<LifetimeIterator_Value> {
    fn destructor(&self) {
        ({
            LifetimeIterator_LifetimeCacheImpl::erase(
                &lifetime_cache_10.with(|v| v.as_pointer()),
                (*self).to_any(),
            )
        });
    }
    fn copy_assign(&self, rhs: Ptr<LifetimeIterator_Value>) -> Ptr<LifetimeIterator_Value> {
        unimplemented!();
        unimplemented!();
        field!((*self), i_).write({ rhs.with(|__s| __s.i_) });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
    fn move_assign(&self, rhs: Ptr<LifetimeIterator_Value>) -> Ptr<LifetimeIterator_Value> {
        unimplemented!();
        unimplemented!();
        field!(rhs, moved_from_).write(true);
        field!((*self), i_).write({ rhs.with(|__s| __s.i_) });
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = lifetime_cache_10.with(|_| ());
}
