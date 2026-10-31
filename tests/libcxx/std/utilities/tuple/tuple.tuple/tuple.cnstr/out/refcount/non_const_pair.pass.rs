extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: i32,
    #[offset(4)]
    pub throw_after: i32,
    #[offset(8)]
    pub count: i32,
    #[offset(12)]
    pub alloc_count: i32,
    #[offset(16)]
    pub allocated_size: i32,
    #[offset(20)]
    pub construct_count: i32,
    #[offset(24)]
    pub destroy_count: i32,
    #[offset(28)]
    pub copied: i32,
    #[offset(32)]
    pub moved: i32,
    #[offset(36)]
    pub converted: i32,
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
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
thread_local!(
    pub static destructed_value_0: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_1: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_2(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: i32, mut id: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct MutableCopy {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub alloc_constructed: bool,
}
impl MutableCopy {
    pub fn new_1(mut _val: i32) -> Self {
        Self {
            val: _val,
            alloc_constructed: false,
        }
    }
    pub fn new_2(_a0: (), _a1: Ptr<test_allocator_int_>, o: Ptr<MutableCopy>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: true,
        }
    }
}
impl Default for MutableCopy {
    fn default() -> Self {
        {
            MutableCopy {
                val: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct ConstCopy {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub alloc_constructed: bool,
}
impl ConstCopy {
    pub fn new_1(mut _val: i32) -> Self {
        Self {
            val: _val,
            alloc_constructed: false,
        }
    }
    pub fn new_2(_a0: (), _a1: Ptr<test_allocator_int_>, o: Ptr<ConstCopy>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: true,
        }
    }
}
impl Default for ConstCopy {
    fn default() -> Self {
        {
            ConstCopy {
                val: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct MutableMove {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub alloc_constructed: bool,
}
impl MutableMove {
    pub fn new_1(mut _val: i32) -> Self {
        Self {
            val: _val,
            alloc_constructed: false,
        }
    }
    pub fn move_from(_a0: Ptr<MutableMove>) -> Self {
        Self {
            val: { (*_a0.upgrade().deref()).val },
            alloc_constructed: { (*_a0.upgrade().deref()).alloc_constructed },
        }
    }
    pub fn new_2(_a0: (), _a1: Ptr<test_allocator_int_>, o: Ptr<MutableMove>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: true,
        }
    }
}
impl Default for MutableMove {
    fn default() -> Self {
        {
            MutableMove {
                val: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct ConstMove {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub alloc_constructed: bool,
}
impl ConstMove {
    pub fn new_1(mut _val: i32) -> Self {
        Self {
            val: _val,
            alloc_constructed: false,
        }
    }
    pub fn move_from(o: Ptr<ConstMove>) -> Self {
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: false,
        }
    }
    pub fn new_2(_a0: (), _a1: Ptr<test_allocator_int_>, o: Ptr<ConstMove>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: true,
        }
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        {
            ConstMove {
                val: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_MutableCopy_ {
    pub fn new_1(_v: Ptr<MutableCopy>) -> Self {
        Self {
            v: (*_v.upgrade().deref()).clone(),
            alloc_constructed: false,
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom)]
#[cpp_from(MutableCopy, new_1, mut_ptr)]
#[byte_size(12)]
pub struct ConvertibleFrom_MutableCopy_ {
    #[offset(0)]
    #[byte_size(8)]
    pub v: MutableCopy,
    #[offset(8)]
    pub alloc_constructed: bool,
}
impl Default for ConvertibleFrom_MutableCopy_ {
    fn default() -> Self {
        {
            ConvertibleFrom_MutableCopy_ {
                v: <MutableCopy>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_int_ {
    pub fn new_1(_v: Ptr<i32>) -> Self {
        Self {
            v: (_v.read()),
            alloc_constructed: false,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, mut_ptr)]
#[byte_size(8)]
pub struct ConvertibleFrom_int_ {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub alloc_constructed: bool,
}
impl Default for ConvertibleFrom_int_ {
    fn default() -> Self {
        {
            ConvertibleFrom_int_ {
                v: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_TracedCopyMove_ {
    pub fn new_1(_v: Ptr<TracedCopyMove>) -> Self {
        Self {
            v: TracedCopyMove::new_1({ (_v).clone() }),
            alloc_constructed: false,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(TracedCopyMove, new_1, mut_ptr)]
#[byte_size(24)]
pub struct ConvertibleFrom_TracedCopyMove_ {
    #[offset(0)]
    #[byte_size(20)]
    pub v: TracedCopyMove,
    #[offset(20)]
    pub alloc_constructed: bool,
}
impl Default for ConvertibleFrom_TracedCopyMove_ {
    fn default() -> Self {
        {
            ConvertibleFrom_TracedCopyMove_ {
                v: <TracedCopyMove>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ExplicitConstructibleFrom_MutableCopy_ {
    pub fn new_1(_v: Ptr<MutableCopy>) -> Self {
        Self {
            v: (*_v.upgrade().deref()).clone(),
            alloc_constructed: false,
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct ExplicitConstructibleFrom_MutableCopy_ {
    #[offset(0)]
    #[byte_size(8)]
    pub v: MutableCopy,
    #[offset(8)]
    pub alloc_constructed: bool,
}
impl Default for ExplicitConstructibleFrom_MutableCopy_ {
    fn default() -> Self {
        {
            ExplicitConstructibleFrom_MutableCopy_ {
                v: <MutableCopy>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ExplicitConstructibleFrom_int_ {
    pub fn new_1(_v: Ptr<i32>) -> Self {
        Self {
            v: (_v.read()),
            alloc_constructed: false,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ExplicitConstructibleFrom_int_ {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub alloc_constructed: bool,
}
impl Default for ExplicitConstructibleFrom_int_ {
    fn default() -> Self {
        {
            ExplicitConstructibleFrom_int_ {
                v: 0_i32,
                alloc_constructed: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(20)]
pub struct TracedCopyMove {
    #[offset(0)]
    pub nonConstCopy: i32,
    #[offset(4)]
    pub constCopy: i32,
    #[offset(8)]
    pub nonConstMove: i32,
    #[offset(12)]
    pub constMove: i32,
    #[offset(16)]
    pub alloc_constructed: bool,
}
impl TracedCopyMove {
    pub fn new_2(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: other.with(|__s| __s.nonConstCopy),
            constCopy: (other.with(|__s| __s.constCopy) + 1),
            nonConstMove: other.with(|__s| __s.nonConstMove),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_1(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: (other.with(|__s| __s.nonConstCopy) + 1),
            constCopy: other.with(|__s| __s.constCopy),
            nonConstMove: other.with(|__s| __s.nonConstMove),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_3(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: other.with(|__s| __s.nonConstCopy),
            constCopy: other.with(|__s| __s.constCopy),
            nonConstMove: (other.with(|__s| __s.nonConstMove) + 1),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_4(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: other.with(|__s| __s.nonConstCopy),
            constCopy: other.with(|__s| __s.constCopy),
            nonConstMove: other.with(|__s| __s.nonConstMove),
            constMove: (other.with(|__s| __s.constMove) + 1),
            alloc_constructed: false,
        }
    }
}
impl Clone for TracedCopyMove {
    fn clone(&self) -> Self {
        let __src: Value<TracedCopyMove> = Rc::new(RefCell::new(TracedCopyMove {
            nonConstCopy: self.nonConstCopy.clone(),
            constCopy: self.constCopy.clone(),
            nonConstMove: self.nonConstMove.clone(),
            constMove: self.constMove.clone(),
            alloc_constructed: self.alloc_constructed.clone(),
        }));
        TracedCopyMove::new_2(__src.as_pointer())
    }
}
impl Default for TracedCopyMove {
    fn default() -> Self {
        {
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
pub fn nonConstCopyCtrCalled_4(obj: Ptr<TracedCopyMove>) -> bool {
    return (((obj.with(|__s| __s.nonConstCopy) == 1) && (obj.with(|__s| __s.constCopy) == 0))
        && (obj.with(|__s| __s.constMove) == 0))
        && (obj.with(|__s| __s.nonConstMove) == 0);
}
pub fn constMoveCtrCalled_5(obj: Ptr<TracedCopyMove>) -> bool {
    return (((obj.with(|__s| __s.nonConstMove) == 0) && (obj.with(|__s| __s.constMove) == 1))
        && (obj.with(|__s| __s.constCopy) == 0))
        && (obj.with(|__s| __s.nonConstCopy) == 0);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoConstructorFromInt {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(()  , new_1, mut_ptr)]
#[byte_size(20)]
pub struct CvtFromTupleRef {}
impl CvtFromTupleRef {
    pub fn new_1(other: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(20)]
pub struct ExplicitCtrFromTupleRef {}
impl ExplicitCtrFromTupleRef {
    pub fn new_1(other: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from_move(()  , new_1, ptr)]
#[byte_size(20)]
pub struct CvtFromConstTupleRefRef {}
impl CvtFromConstTupleRefRef {
    pub fn new_1(other: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(20)]
pub struct ExplicitCtrFromConstTupleRefRef {}
impl ExplicitCtrFromConstTupleRefRef {
    pub fn new_1(other: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct CopyAssign {
    #[offset(0)]
    pub val: i32,
}
impl CopyAssign {
    pub fn new_1(mut v: i32) -> Self {
        Self { val: v }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct ConstCopyAssign {
    #[offset(0)]
    pub val: i32,
}
impl ConstCopyAssign {
    pub fn new_1(mut v: i32) -> Self {
        Self { val: v }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, MoveAssign, Default)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct MoveAssign {
    #[offset(0)]
    pub val: i32,
}
impl MoveAssign {
    pub fn new_1(mut v: i32) -> Self {
        Self { val: v }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, MoveAssign, Default)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct ConstMoveAssign {
    #[offset(0)]
    pub val: i32,
}
impl ConstMoveAssign {
    pub fn new_1(mut v: i32) -> Self {
        Self { val: v }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct TracedAssignment {
    #[offset(0)]
    pub copyAssign: i32,
    #[offset(4)]
    pub constCopyAssign: i32,
    #[offset(8)]
    pub moveAssign: i32,
    #[offset(12)]
    pub constMoveAssign: i32,
}
impl Default for TracedAssignment {
    fn default() -> Self {
        {
            TracedAssignment {
                copyAssign: 0,
                constCopyAssign: 0,
                moveAssign: 0,
                constMoveAssign: 0,
            }
        }
    }
}
const _: () = assert!(
    true,
    "std::is_constructible_v<std::tuple<MutableCopy, int>, std::pair<MutableCopy, int>&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::tuple<MutableCopy>, std::pair<MutableCopy, int>&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::tuple<MutableCopy, int, int>, std::pair<MutableCopy, int>&>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::tuple<int, int>, std::pair<int, int>&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::tuple<NoConstructorFromInt, int>, std::pair<int, int>&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::tuple<int, NoConstructorFromInt>, std::pair<int, int>&>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v< std::tuple<NoConstructorFromInt, NoConstructorFromInt>, std::pair<int, int>&>"
);
const _: () = assert!(
    true,
    "std::is_convertible_v<std::pair<MutableCopy, MutableCopy>&,\n                                    std::tuple<ConvertibleFrom<MutableCopy>, ConvertibleFrom<MutableCopy>>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<std::pair<MutableCopy, MutableCopy>&,\n                                     std::tuple<ExplicitConstructibleFrom<MutableCopy>, ConvertibleFrom<MutableCopy>>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<std::pair<MutableCopy, MutableCopy>&,\n                                     std::tuple<ConvertibleFrom<MutableCopy>, ExplicitConstructibleFrom<MutableCopy>>>"
);
pub fn test_6() -> bool {
    {
        let p: Value<(Value<MutableCopy>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(MutableCopy::new_1({ 1 }))),
            Rc::new(RefCell::new(2)),
        )));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ (unimplemented!()).clone().v.val } == 1));
        assert!(({ (unimplemented!()).clone().v } == 2));
    }
    {
        let p: Value<(Value<MutableCopy>, Value<i32>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(MutableCopy::new_1({ 1 }))),
            Rc::new(RefCell::new(2)),
        )));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ (unimplemented!()).clone().v.val } == 1));
        assert!(({ (unimplemented!()).clone().v } == 2));
    }
    {
        let p: Value<(Value<TracedCopyMove>, Value<TracedCopyMove>)> =
            Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ nonConstCopyCtrCalled_4(field_ptr!(unimplemented!(), v),) }));
        assert!(({ nonConstCopyCtrCalled_4(unimplemented!(),) }));
    }
    return true;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_6() });
    const _: () = assert!(true, "test()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct test_allocator_int_;
pub trait ConstCopyAssignImpl {
    fn copy_assign(&self, other: Ptr<ConstCopyAssign>) -> Ptr<ConstCopyAssign>;
}
impl ConstCopyAssignImpl for Ptr<ConstCopyAssign> {
    fn copy_assign(&self, other: Ptr<ConstCopyAssign>) -> Ptr<ConstCopyAssign> {
        field!((*self), val).write({ other.with(|__s| __s.val) });
        return (*self).clone();
    }
}
pub trait ConstMoveAssignImpl {
    fn move_assign(&self, other: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
}
impl ConstMoveAssignImpl for Ptr<ConstMoveAssign> {
    fn move_assign(&self, other: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), val).write({ other.with(|__s| __s.val) });
        return (*self).clone();
    }
}
pub trait MoveAssignImpl {
    fn move_assign(&self, _a0: Ptr<MoveAssign>) -> Ptr<MoveAssign>;
}
impl MoveAssignImpl for Ptr<MoveAssign> {
    fn move_assign(&self, _a0: Ptr<MoveAssign>) -> Ptr<MoveAssign> {
        field!((*self), val).write({ { (*_a0.upgrade().deref()).val } });
        return (*self).clone();
    }
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait TracedAssignmentImpl {
    fn operator_assign_2(&self, _a0: Ptr<TracedAssignment>) -> Ptr<TracedAssignment>;
    fn operator_assign_3(&self, _a0: Ptr<TracedAssignment>) -> Ptr<TracedAssignment>;
    fn operator_assign_4(&self, _a0: Ptr<TracedAssignment>) -> Ptr<TracedAssignment>;
    fn operator_assign_5(&self, _a0: Ptr<TracedAssignment>) -> Ptr<TracedAssignment>;
}
impl TracedAssignmentImpl for Ptr<TracedAssignment> {
    fn operator_assign_2(&self, _a0: Ptr<TracedAssignment>) -> Ptr<TracedAssignment> {
        field!((*self), copyAssign).with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
    fn operator_assign_3(&self, _a0: Ptr<TracedAssignment>) -> Ptr<TracedAssignment> {
        field!((*self), constCopyAssign).with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
    fn operator_assign_4(&self, _a0: Ptr<TracedAssignment>) -> Ptr<TracedAssignment> {
        field!((*self), moveAssign).with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
    fn operator_assign_5(&self, _a0: Ptr<TracedAssignment>) -> Ptr<TracedAssignment> {
        field!((*self), constMoveAssign).with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        assert!(
            ((*self).with(|__s| __s.count) == 0)
                && (!(Ptr::<i8>::from_string_literal(b"clearing leaking allocator data?"))
                    .is_null())
        );
        field!((*self), count).write(0);
        field!((*self), time_to_throw).write(0);
        field!((*self), alloc_count).write(0);
        field!((*self), allocated_size).write(0);
        field!((*self), construct_count).write(0);
        field!((*self), destroy_count).write(0);
        field!((*self), throw_after).write(2147483647);
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0);
        field!((*self), moved).write(0);
        field!((*self), converted).write(0);
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
}
