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
    pub time_to_throw: i32,
    pub throw_after: i32,
    pub count: i32,
    pub alloc_count: i32,
    pub allocated_size: i32,
    pub construct_count: i32,
    pub destroy_count: i32,
    pub copied: i32,
    pub moved: i32,
    pub converted: i32,
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        assert!(
            ((self.count) == (0)) && (!(c"clearing leaking allocator data?".as_ptr()).is_null())
        );
        self.count = 0;
        self.time_to_throw = 0;
        self.alloc_count = 0;
        self.allocated_size = 0;
        self.construct_count = 0;
        self.destroy_count = 0;
        self.throw_after = 2147483647;
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0;
        self.moved = 0;
        self.converted = 0;
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
pub static mut destructed_value_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_2(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_2(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: i32, mut id: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: i32,
        mut id: i32,
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
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> i32 {
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
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct MutableCopy {
    pub val: i32,
    pub alloc_constructed: bool,
}
impl MutableCopy {
    pub unsafe fn new_1(mut _val: i32) -> Self {
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
                val: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct ConstCopy {
    pub val: i32,
    pub alloc_constructed: bool,
}
impl ConstCopy {
    pub unsafe fn new_1(mut _val: i32) -> Self {
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
                val: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct MutableMove {
    pub val: i32,
    pub alloc_constructed: bool,
}
impl MutableMove {
    pub unsafe fn new_1(mut _val: i32) -> Self {
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
                val: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct ConstMove {
    pub val: i32,
    pub alloc_constructed: bool,
}
impl ConstMove {
    pub unsafe fn new_1(mut _val: i32) -> Self {
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
                val: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleFrom_X_ {
    pub v: X,
    pub alloc_constructed: bool,
}
impl Default for ConvertibleFrom_X_ {
    fn default() -> Self {
        unsafe {
            ConvertibleFrom_X_ {
                v: <X>::default(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertibleFrom_Y_ {
    pub v: Y,
    pub alloc_constructed: bool,
}
impl Default for ConvertibleFrom_Y_ {
    fn default() -> Self {
        unsafe {
            ConvertibleFrom_Y_ {
                v: <Y>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_MutableCopy_ {
    pub unsafe fn new_1(_v: *mut MutableCopy) -> Self {
        let mut this = Self {
            v: (*_v),
            alloc_constructed: false,
        };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe)]
#[cpp_from(MutableCopy, new_1, mut_ptr)]
pub struct ConvertibleFrom_MutableCopy_ {
    pub v: MutableCopy,
    pub alloc_constructed: bool,
}
impl Default for ConvertibleFrom_MutableCopy_ {
    fn default() -> Self {
        unsafe {
            ConvertibleFrom_MutableCopy_ {
                v: <MutableCopy>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_int_ {
    pub unsafe fn new_1(_v: *mut i32) -> Self {
        let mut this = Self {
            v: (*_v),
            alloc_constructed: false,
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, mut_ptr)]
pub struct ConvertibleFrom_int_ {
    pub v: i32,
    pub alloc_constructed: bool,
}
impl Default for ConvertibleFrom_int_ {
    fn default() -> Self {
        unsafe {
            ConvertibleFrom_int_ {
                v: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_TracedCopyMove_ {
    pub unsafe fn new_1(_v: *mut TracedCopyMove) -> Self {
        let mut this = Self {
            v: TracedCopyMove::new_1({ _v }),
            alloc_constructed: false,
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(TracedCopyMove, new_1, mut_ptr)]
pub struct ConvertibleFrom_TracedCopyMove_ {
    pub v: TracedCopyMove,
    pub alloc_constructed: bool,
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ExplicitConstructibleFrom_Y_ {
    pub v: Y,
    pub alloc_constructed: bool,
}
impl Default for ExplicitConstructibleFrom_Y_ {
    fn default() -> Self {
        unsafe {
            ExplicitConstructibleFrom_Y_ {
                v: <Y>::default(),
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ExplicitConstructibleFrom_X_ {
    pub v: X,
    pub alloc_constructed: bool,
}
impl Default for ExplicitConstructibleFrom_X_ {
    fn default() -> Self {
        unsafe {
            ExplicitConstructibleFrom_X_ {
                v: <X>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ExplicitConstructibleFrom_MutableCopy_ {
    pub unsafe fn new_1(_v: *mut MutableCopy) -> Self {
        let mut this = Self {
            v: (*_v),
            alloc_constructed: false,
        };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ExplicitConstructibleFrom_MutableCopy_ {
    pub v: MutableCopy,
    pub alloc_constructed: bool,
}
impl Default for ExplicitConstructibleFrom_MutableCopy_ {
    fn default() -> Self {
        unsafe {
            ExplicitConstructibleFrom_MutableCopy_ {
                v: <MutableCopy>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ExplicitConstructibleFrom_int_ {
    pub unsafe fn new_1(_v: *mut i32) -> Self {
        let mut this = Self {
            v: (*_v),
            alloc_constructed: false,
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ExplicitConstructibleFrom_int_ {
    pub v: i32,
    pub alloc_constructed: bool,
}
impl Default for ExplicitConstructibleFrom_int_ {
    fn default() -> Self {
        unsafe {
            ExplicitConstructibleFrom_int_ {
                v: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct TracedCopyMove {
    pub nonConstCopy: i32,
    pub constCopy: i32,
    pub nonConstMove: i32,
    pub constMove: i32,
    pub alloc_constructed: bool,
}
impl TracedCopyMove {
    pub unsafe fn new_2(other: *const TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (((*other).constCopy) + (1)),
            nonConstMove: (*other).nonConstMove,
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_1(other: *mut TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (((*other).nonConstCopy) + (1)),
            constCopy: (*other).constCopy,
            nonConstMove: (*other).nonConstMove,
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_3(other: *mut TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (*other).constCopy,
            nonConstMove: (((*other).nonConstMove) + (1)),
            constMove: (*other).constMove,
            alloc_constructed: false,
        };
        this
    }
    pub unsafe fn new_4(other: *const TracedCopyMove) -> Self {
        let mut this = Self {
            nonConstCopy: (*other).nonConstCopy,
            constCopy: (*other).constCopy,
            nonConstMove: (*other).nonConstMove,
            constMove: (((*other).constMove) + (1)),
            alloc_constructed: false,
        };
        this
    }
}
impl Clone for TracedCopyMove {
    fn clone(&self) -> Self {
        unsafe { TracedCopyMove::new_2(self as *const TracedCopyMove) }
    }
}
impl Default for TracedCopyMove {
    fn default() -> Self {
        unsafe {
            TracedCopyMove {
                nonConstCopy: 0,
                constCopy: 0,
                nonConstMove: 0,
                constMove: 0,
                alloc_constructed: false,
            }
        }
    }
}
pub unsafe fn nonConstCopyCtrCalled_4(obj: *const TracedCopyMove) -> bool {
    return (((((*obj).nonConstCopy) == (1)) && (((*obj).constCopy) == (0)))
        && (((*obj).constMove) == (0)))
        && (((*obj).nonConstMove) == (0));
}
pub unsafe fn constMoveCtrCalled_5(obj: *const TracedCopyMove) -> bool {
    return (((((*obj).nonConstMove) == (0)) && (((*obj).constMove) == (1)))
        && (((*obj).constCopy) == (0)))
        && (((*obj).nonConstCopy) == (0));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoConstructorFromInt {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(() , new_1, mut_ptr)]
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from_move(() , new_1, ptr)]
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
#[derive(FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new_1, value)]
pub struct CopyAssign {
    pub val: i32,
}
impl CopyAssign {
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { val: v };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new_1, value)]
pub struct ConstCopyAssign {
    pub val: i32,
}
impl ConstCopyAssign {
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn copy_assign(&self, other: *const ConstCopyAssign) -> *const ConstCopyAssign {
        (*(&raw const self.val).cast_mut()) = (*other).val;
        return &(*(self as *const ConstCopyAssign));
    }
}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new_1, value)]
pub struct MoveAssign {
    pub val: i32,
}
impl MoveAssign {
    pub unsafe fn new_1(mut v: i32) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveAssign) -> *mut MoveAssign {
        self.val = (*_a0).val;
        return &mut (*(self as *mut MoveAssign));
    }
}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new_1, value)]
pub struct ConstMoveAssign {
    pub val: i32,
}
impl ConstMoveAssign {
    pub unsafe fn new_1(mut v: i32) -> Self {
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
    pub copyAssign: i32,
    pub constCopyAssign: i32,
    pub moveAssign: i32,
    pub constMoveAssign: i32,
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
                copyAssign: 0,
                constCopyAssign: 0,
                moveAssign: 0,
                constMoveAssign: 0,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Y {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotConvertibleToXorY {}
const _: () = assert!(
    true,
    "std::is_constructible_v<std::pair<X, Y>, std::pair<X, Y>&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::pair<X, Y>, std::pair<NotConvertibleToXorY, Y>&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::pair<X, Y>, std::pair<X, NotConvertibleToXorY>&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::pair<X, Y>, std::pair<NotConvertibleToXorY, NotConvertibleToXorY>&>"
);
const _: () = assert!(
    true,
    "std::is_convertible_v<std::pair<X, Y>&, std::pair<ConvertibleFrom<X>, ConvertibleFrom<Y>>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<std::pair<X, Y>&, std::pair<ConvertibleFrom<X>, ExplicitConstructibleFrom<Y>>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<std::pair<X, Y>&, std::pair<ExplicitConstructibleFrom<X>, ConvertibleFrom<Y>>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<std::pair<X, Y>&, std::pair<ExplicitConstructibleFrom<X>, ExplicitConstructibleFrom<Y>>>"
);
pub unsafe fn test_6() -> bool {
    {
        let mut p1: (i32, i32) = (1, 2);
        let mut p2: (*mut i32, *mut i32) = {
            let __a0 = (p1).clone();
            (
                <*mut i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.0),
                <*mut i32 as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
            )
        };
        assert!(((&mut (*p2.0) as *mut i32) == (&mut (p1.0) as *mut i32)));
        assert!(((&mut (*p2.1) as *mut i32) == (&mut (p1.1) as *mut i32)));
    }
    {
        let mut p1: (MutableCopy, i32) = (MutableCopy::new_1({ 1 }), 2);
        let mut p2: (ConvertibleFrom_MutableCopy_, ConvertibleFrom_int_) = {
            let __a0 = (p1).clone();
            (
                <ConvertibleFrom_MutableCopy_ as CppFromUnsafe<MutableCopy>>::cpp_from(&__a0.0),
                <ConvertibleFrom_int_ as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
            )
        };
        assert!(((p2.0.v.val) == (1)));
        assert!(((p2.1.v) == (2)));
    }
    {
        let mut p1: (MutableCopy, i32) = (MutableCopy::new_1({ 1 }), 2);
        let mut p2: (
            ExplicitConstructibleFrom_MutableCopy_,
            ExplicitConstructibleFrom_int_,
        ) = {
            let __a0 = (p1).clone();
            (
                <ExplicitConstructibleFrom_MutableCopy_ as CppFromUnsafe<MutableCopy>>::cpp_from(
                    &__a0.0,
                ),
                <ExplicitConstructibleFrom_int_ as CppFromUnsafe<i32>>::cpp_from(&__a0.1),
            )
        };
        assert!(((p2.0.v.val) == (1)));
        assert!(((p2.1.v) == (2)));
    }
    {
        let mut p1: (TracedCopyMove, TracedCopyMove) = unimplemented!();
        let mut p2: (
            ConvertibleFrom_TracedCopyMove_,
            ConvertibleFrom_TracedCopyMove_,
        ) = {
            let __a0 = (p1).clone();
            (
                <ConvertibleFrom_TracedCopyMove_ as CppFromUnsafe<TracedCopyMove>>::cpp_from(
                    &__a0.0,
                ),
                <ConvertibleFrom_TracedCopyMove_ as CppFromUnsafe<TracedCopyMove>>::cpp_from(
                    &__a0.1,
                ),
            )
        };
        assert!((unsafe { nonConstCopyCtrCalled_4(&p2.0.v,) }));
        assert!((unsafe { nonConstCopyCtrCalled_4(&p2.1.v,) }));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct test_allocator_int_;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
}
