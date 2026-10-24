extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: (),
    pub throw_after: (),
    pub count: (),
    pub alloc_count: (),
    pub allocated_size: (),
    pub construct_count: (),
    pub destroy_count: (),
    pub copied: (),
    pub moved: (),
    pub converted: (),
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        unimplemented!();
        self.count = 0_();
        self.time_to_throw = 0_();
        self.alloc_count = 0_();
        self.allocated_size = 0_();
        self.construct_count = 0_();
        self.destroy_count = 0_();
        self.throw_after = 2147483647_();
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0_();
        self.moved = 0_();
        self.converted = 0_();
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
pub static mut destructed_value_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_2(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_2(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0_(),
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: (),
        mut id: (),
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const test_allocator_void_,
    ) -> *mut test_allocator_void_ {
        self.data_ = (*_a0).data_;
        self.id_ = (*_a0).id_;
        self.stats_ = (*_a0).stats_;
        return &mut (*(self as *mut test_allocator_void_));
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const test_allocator_void_,
                other as *const test_allocator_void_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_void_::copy_from(self as *const test_allocator_void_) }
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        unsafe {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MutableCopy {
    pub val: (),
    pub alloc_constructed: (),
}
impl MutableCopy {
    pub unsafe fn new_1(mut _val: ()) -> Self {
        let mut this = Self {
            val: _val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, o: *mut MutableCopy) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: true,
        };
        this
    }
}
impl Default for MutableCopy {
    fn default() -> Self {
        unsafe {
            MutableCopy {
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConstCopy {
    pub val: (),
    pub alloc_constructed: (),
}
impl ConstCopy {
    pub unsafe fn new_1(mut _val: ()) -> Self {
        let mut this = Self {
            val: _val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, o: *const ConstCopy) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: true,
        };
        this
    }
}
impl Default for ConstCopy {
    fn default() -> Self {
        unsafe {
            ConstCopy {
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct MutableMove {
    pub val: (),
    pub alloc_constructed: (),
}
impl MutableMove {
    pub unsafe fn new_1(mut _val: ()) -> Self {
        let mut this = Self {
            val: _val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut MutableMove) -> Self {
        let mut this = Self {
            val: (*_a0).val,
            alloc_constructed: (*_a0).alloc_constructed,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, o: *mut MutableMove) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: true,
        };
        this
    }
}
impl Default for MutableMove {
    fn default() -> Self {
        unsafe {
            MutableMove {
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct ConstMove {
    pub val: (),
    pub alloc_constructed: (),
}
impl ConstMove {
    pub unsafe fn new_1(mut _val: ()) -> Self {
        let mut this = Self {
            val: _val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn move_from(o: *const ConstMove) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, o: *const ConstMove) -> Self {
        let mut this = Self {
            val: (*o).val,
            alloc_constructed: true,
        };
        this
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        unsafe {
            ConstMove {
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_ConstMove_ {
    pub unsafe fn new_1(_v: *const ConstMove) -> Self {
        let mut this = Self {
            v: ConstMove::move_from({ unimplemented!() }),
            alloc_constructed: false,
        };
        this
    }
}
impl ConvertibleFrom_ConstMove_ {
    pub unsafe fn new_2(
        mut _a0: (),
        _a1: *const test_allocator_int_,
        _u: *const ConstMove,
    ) -> Self {
        let mut this = ConvertibleFrom_ConstMove_::new_1({ _u });
        this.alloc_constructed = true;
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ConvertibleFrom_ConstMove_ {
    pub v: ConstMove,
    pub alloc_constructed: (),
}
impl Default for ConvertibleFrom_ConstMove_ {
    fn default() -> Self {
        unsafe {
            ConvertibleFrom_ConstMove_ {
                v: <ConstMove>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_int_ {
    pub unsafe fn new_1(_v: *const ()) -> Self {
        let mut this = Self {
            v: (*unimplemented!()),
            alloc_constructed: false,
        };
        this
    }
}
impl ConvertibleFrom_int_ {
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, _u: *const ()) -> Self {
        let mut this = ConvertibleFrom_int_::new_1({ _u });
        this.alloc_constructed = true;
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleFrom_int_ {
    pub v: (),
    pub alloc_constructed: (),
}
impl Default for ConvertibleFrom_int_ {
    fn default() -> Self {
        unsafe {
            ConvertibleFrom_int_ {
                v: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_TracedCopyMove_ {
    pub unsafe fn new_1(_v: *const TracedCopyMove) -> Self {
        let mut this = Self {
            v: TracedCopyMove::new_1({ unimplemented!() }),
            alloc_constructed: false,
        };
        this
    }
}
impl ConvertibleFrom_TracedCopyMove_ {
    pub unsafe fn new_2(
        mut _a0: (),
        _a1: *const test_allocator_int_,
        _u: *const TracedCopyMove,
    ) -> Self {
        let mut this = ConvertibleFrom_TracedCopyMove_::new_1({ _u });
        this.alloc_constructed = true;
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ConvertibleFrom_TracedCopyMove_ {
    pub v: TracedCopyMove,
    pub alloc_constructed: (),
}
impl Default for ConvertibleFrom_TracedCopyMove_ {
    fn default() -> Self {
        unsafe {
            ConvertibleFrom_TracedCopyMove_ {
                v: <TracedCopyMove>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ExplicitConstructibleFrom_ConstMove_ {
    pub unsafe fn new_1(_v: *const ConstMove) -> Self {
        let mut this = Self {
            v: ConstMove::move_from({ unimplemented!() }),
            alloc_constructed: false,
        };
        this
    }
}
impl ExplicitConstructibleFrom_ConstMove_ {
    pub unsafe fn new_2(
        mut _a0: (),
        _a1: *const test_allocator_int_,
        _u: *const ConstMove,
    ) -> Self {
        let mut this = ExplicitConstructibleFrom_ConstMove_::new_1({ _u });
        this.alloc_constructed = true;
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ExplicitConstructibleFrom_ConstMove_ {
    pub v: ConstMove,
    pub alloc_constructed: (),
}
impl Default for ExplicitConstructibleFrom_ConstMove_ {
    fn default() -> Self {
        unsafe {
            ExplicitConstructibleFrom_ConstMove_ {
                v: <ConstMove>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ExplicitConstructibleFrom_int_ {
    pub unsafe fn new_1(_v: *const ()) -> Self {
        let mut this = Self {
            v: (*unimplemented!()),
            alloc_constructed: false,
        };
        this
    }
}
impl ExplicitConstructibleFrom_int_ {
    pub unsafe fn new_2(mut _a0: (), _a1: *const test_allocator_int_, _u: *const ()) -> Self {
        let mut this = ExplicitConstructibleFrom_int_::new_1({ _u });
        this.alloc_constructed = true;
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ExplicitConstructibleFrom_int_ {
    pub v: (),
    pub alloc_constructed: (),
}
impl Default for ExplicitConstructibleFrom_int_ {
    fn default() -> Self {
        unsafe {
            ExplicitConstructibleFrom_int_ {
                v: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
impl TracedCopyMove {
    pub unsafe fn new_2(
        mut _a0: (),
        _a1: *const test_allocator_int_,
        _u: *const TracedCopyMove,
    ) -> Self {
        let mut this = TracedCopyMove::new_1({ _u });
        this.alloc_constructed = true;
        this
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct TracedCopyMove {
    pub nonConstCopy: (),
    pub constCopy: (),
    pub nonConstMove: (),
    pub constMove: (),
    pub alloc_constructed: (),
}
impl TracedCopyMove {
    pub unsafe fn new_3(other: *const TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (((*other).constCopy) + (1_())),
            nonConstMove: (*other).nonConstMove,
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_4(other: *mut TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (((*other).nonConstCopy) + (1_())),
            constCopy: (*other).constCopy,
            nonConstMove: (*other).nonConstMove,
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_5(other: *mut TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (*other).constCopy,
            nonConstMove: (((*other).nonConstMove) + (1_())),
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_1(other: *const TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (*other).constCopy,
            nonConstMove: (*other).nonConstMove,
            constMove: (((*other).constMove) + (1_())),
            alloc_constructed: false,
        };
        this
    }
}
impl Clone for TracedCopyMove {
    fn clone(&self) -> Self {
        unsafe { TracedCopyMove::new_3(self as *const TracedCopyMove) }
    }
}
impl Default for TracedCopyMove {
    fn default() -> Self {
        unsafe {
            TracedCopyMove {
                nonConstCopy: 0_(),
                constCopy: 0_(),
                nonConstMove: 0_(),
                constMove: 0_(),
                alloc_constructed: false,
            }
        }
    }
}
pub unsafe fn nonConstCopyCtrCalled_4(obj: *const TracedCopyMove) -> () {
    return (((((*obj).nonConstCopy) == (1_())) && (((*obj).constCopy) == (0_())))
        && (((*obj).constMove) == (0_())))
        && (((*obj).nonConstMove) == (0_()));
}
pub unsafe fn constMoveCtrCalled_5(obj: *const TracedCopyMove) -> () {
    return (((((*obj).nonConstMove) == (0_())) && (((*obj).constMove) == (1_())))
        && (((*obj).constCopy) == (0_())))
        && (((*obj).nonConstCopy) == (0_()));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoConstructorFromInt {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CvtFromTupleRef {}
impl CvtFromTupleRef {
    pub unsafe fn new_1(other: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitCtrFromTupleRef {}
impl ExplicitCtrFromTupleRef {
    pub unsafe fn new_1(other: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CvtFromConstTupleRefRef {}
impl CvtFromConstTupleRefRef {
    pub unsafe fn new_1(other: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitCtrFromConstTupleRefRef {}
impl ExplicitCtrFromConstTupleRefRef {
    pub unsafe fn new_1(other: *mut ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct CopyAssign {
    pub val: (),
}
impl CopyAssign {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, Default)]
pub struct ConstCopyAssign {
    pub val: (),
}
impl ConstCopyAssign {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn copy_assign(&self, other: *const ConstCopyAssign) -> *const ConstCopyAssign {
        (*(&raw const self.val).cast_mut()) = (*other).val;
        return &(*(self as *const ConstCopyAssign));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct MoveAssign {
    pub val: (),
}
impl MoveAssign {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveAssign) -> *mut MoveAssign {
        self.val = (*_a0).val;
        return &mut (*(self as *mut MoveAssign));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct ConstMoveAssign {
    pub val: (),
}
impl ConstMoveAssign {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn move_assign(&self, other: *mut ConstMoveAssign) -> *const ConstMoveAssign {
        (*(&raw const self.val).cast_mut()) = (*other).val;
        return &(*(self as *const ConstMoveAssign));
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct TracedAssignment {
    pub copyAssign: (),
    pub constCopyAssign: (),
    pub moveAssign: (),
    pub constMoveAssign: (),
}
impl TracedAssignment {
    pub unsafe fn operator_assign_2(
        &mut self,
        _a0: *const TracedAssignment,
    ) -> *mut TracedAssignment {
        self.copyAssign.postfix_inc();
        return &mut (*(self as *mut TracedAssignment));
    }
    pub unsafe fn operator_assign_3(
        &self,
        _a0: *const TracedAssignment,
    ) -> *const TracedAssignment {
        (*(&raw const self.constCopyAssign).cast_mut()).postfix_inc();
        return &(*(self as *const TracedAssignment));
    }
    pub unsafe fn operator_assign_4(
        &mut self,
        _a0: *mut TracedAssignment,
    ) -> *mut TracedAssignment {
        self.moveAssign.postfix_inc();
        return &mut (*(self as *mut TracedAssignment));
    }
    pub unsafe fn operator_assign_5(&self, _a0: *mut TracedAssignment) -> *const TracedAssignment {
        (*(&raw const self.constMoveAssign).cast_mut()).postfix_inc();
        return &(*(self as *const TracedAssignment));
    }
}
impl Default for TracedAssignment {
    fn default() -> Self {
        unsafe {
            TracedAssignment {
                copyAssign: 0_(),
                constCopyAssign: 0_(),
                moveAssign: 0_(),
                constMoveAssign: 0_(),
            }
        }
    }
}
const _: () = assert!(
    true,
    "std::is_constructible_v<std::tuple<ConstMove, int>, std::allocator_arg_t, test_allocator<int>,\n                                      const std::pair<ConstMove, int>&&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v< std::tuple<ConstMove>, std::allocator_arg_t, test_allocator<int>,\n                                        const std::pair<ConstMove, int>&&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v< std::tuple<ConstMove, int, int>, std::allocator_arg_t, test_allocator<int>,\n                                        const std::pair<ConstMove, int>&&>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::tuple<int, int>, std::allocator_arg_t, test_allocator<int>,\n                                      const std::pair<int, int>&&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v< std::tuple<NoConstructorFromInt, int>, std::allocator_arg_t,\n                                        test_allocator<int>, const std::pair<int, int>&&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v< std::tuple<int, NoConstructorFromInt>, std::allocator_arg_t,\n                                        test_allocator<int>, const std::pair<int, int>&&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v< std::tuple<NoConstructorFromInt, NoConstructorFromInt>, std::allocator_arg_t,\n                                        test_allocator<int>, const std::pair<int, int>&&>"
);
const _: () = assert!(
    true,
    "ImplicitlyConstructible< std::tuple<ConvertibleFrom<ConstMove>, ConvertibleFrom<ConstMove>>, std::allocator_arg_t,\n                             test_allocator<int>, const std::pair<ConstMove, ConstMove>&&>"
);
const _: () = assert!(
    !(false),
    "!ImplicitlyConstructible<std::tuple<ConvertibleFrom<ConstMove>, ExplicitConstructibleFrom<ConstMove>>,\n                             std::allocator_arg_t, test_allocator<int>, const std::pair<ConstMove, ConstMove>&&>"
);
const _: () = assert!(
    !(false),
    "!ImplicitlyConstructible<std::tuple<ExplicitConstructibleFrom<ConstMove>, ConvertibleFrom<ConstMove>>,\n                             std::allocator_arg_t, test_allocator<int>, const std::pair<ConstMove, ConstMove>&&>"
);
pub unsafe fn test_6() -> () {
    {
        let p: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
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
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
}
