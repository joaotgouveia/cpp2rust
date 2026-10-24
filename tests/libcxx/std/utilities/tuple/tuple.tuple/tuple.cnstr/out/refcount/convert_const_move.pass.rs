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
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
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
thread_local!(
    pub static destructed_value_0: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_1: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
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
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
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
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
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
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct MutableCopy {
    #[offset(0)]
    pub val: (),
    #[offset(4)]
    pub alloc_constructed: (),
}
impl MutableCopy {
    pub fn new_1(mut _val: ()) -> Self {
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
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ConstCopy {
    #[offset(0)]
    pub val: (),
    #[offset(4)]
    pub alloc_constructed: (),
}
impl ConstCopy {
    pub fn new_1(mut _val: ()) -> Self {
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
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(8)]
pub struct MutableMove {
    #[offset(0)]
    pub val: (),
    #[offset(4)]
    pub alloc_constructed: (),
}
impl MutableMove {
    pub fn new_1(mut _val: ()) -> Self {
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
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(8)]
pub struct ConstMove {
    #[offset(0)]
    pub val: (),
    #[offset(4)]
    pub alloc_constructed: (),
}
impl ConstMove {
    pub fn new_1(mut _val: ()) -> Self {
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
                val: unimplemented!(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_ConstMove_ {
    pub fn new_1(_v: Ptr<ConstMove>) -> Self {
        Self {
            v: ConstMove::move_from({ unimplemented!() }),
            alloc_constructed: false,
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct ConvertibleFrom_ConstMove_ {
    #[offset(0)]
    #[byte_size(8)]
    pub v: ConstMove,
    #[offset(8)]
    pub alloc_constructed: (),
}
impl Default for ConvertibleFrom_ConstMove_ {
    fn default() -> Self {
        {
            ConvertibleFrom_ConstMove_ {
                v: <ConstMove>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_TracedCopyMove_ {
    pub fn new_1(_v: Ptr<TracedCopyMove>) -> Self {
        Self {
            v: TracedCopyMove::new_1({ unimplemented!() }),
            alloc_constructed: false,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct ConvertibleFrom_TracedCopyMove_ {
    #[offset(0)]
    #[byte_size(20)]
    pub v: TracedCopyMove,
    #[offset(20)]
    pub alloc_constructed: (),
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
impl ConvertibleFrom_CvtFromConstTupleRefRef_ {
    pub fn new_1(_v: Ptr<CvtFromConstTupleRefRef>) -> Self {
        Self {
            v: (*unimplemented!().upgrade().deref()).clone(),
            alloc_constructed: false,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct ConvertibleFrom_CvtFromConstTupleRefRef_ {
    #[offset(0)]
    #[byte_size(20)]
    pub v: CvtFromConstTupleRefRef,
    #[offset(20)]
    pub alloc_constructed: (),
}
impl Default for ConvertibleFrom_CvtFromConstTupleRefRef_ {
    fn default() -> Self {
        {
            ConvertibleFrom_CvtFromConstTupleRefRef_ {
                v: <CvtFromConstTupleRefRef>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ConvertibleFrom_ExplicitCtrFromConstTupleRefRef_ {
    pub fn new_1(_v: Ptr<ExplicitCtrFromConstTupleRefRef>) -> Self {
        Self {
            v: (*unimplemented!().upgrade().deref()).clone(),
            alloc_constructed: false,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct ConvertibleFrom_ExplicitCtrFromConstTupleRefRef_ {
    #[offset(0)]
    #[byte_size(20)]
    pub v: ExplicitCtrFromConstTupleRefRef,
    #[offset(20)]
    pub alloc_constructed: (),
}
impl Default for ConvertibleFrom_ExplicitCtrFromConstTupleRefRef_ {
    fn default() -> Self {
        {
            ConvertibleFrom_ExplicitCtrFromConstTupleRefRef_ {
                v: <ExplicitCtrFromConstTupleRefRef>::default(),
                alloc_constructed: false,
            }
        }
    }
}
impl ExplicitConstructibleFrom_ConstMove_ {
    pub fn new_1(_v: Ptr<ConstMove>) -> Self {
        Self {
            v: ConstMove::move_from({ unimplemented!() }),
            alloc_constructed: false,
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct ExplicitConstructibleFrom_ConstMove_ {
    #[offset(0)]
    #[byte_size(8)]
    pub v: ConstMove,
    #[offset(8)]
    pub alloc_constructed: (),
}
impl Default for ExplicitConstructibleFrom_ConstMove_ {
    fn default() -> Self {
        {
            ExplicitConstructibleFrom_ConstMove_ {
                v: <ConstMove>::default(),
                alloc_constructed: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(20)]
pub struct TracedCopyMove {
    #[offset(0)]
    pub nonConstCopy: (),
    #[offset(4)]
    pub constCopy: (),
    #[offset(8)]
    pub nonConstMove: (),
    #[offset(12)]
    pub constMove: (),
    #[offset(16)]
    pub alloc_constructed: (),
}
impl TracedCopyMove {
    pub fn new_2(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: other.with(|__s| __s.nonConstCopy),
            constCopy: (other.with(|__s| __s.constCopy) + 1_()),
            nonConstMove: other.with(|__s| __s.nonConstMove),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_3(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: (other.with(|__s| __s.nonConstCopy) + 1_()),
            constCopy: other.with(|__s| __s.constCopy),
            nonConstMove: other.with(|__s| __s.nonConstMove),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_4(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: other.with(|__s| __s.nonConstCopy),
            constCopy: other.with(|__s| __s.constCopy),
            nonConstMove: (other.with(|__s| __s.nonConstMove) + 1_()),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_1(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: other.with(|__s| __s.nonConstCopy),
            constCopy: other.with(|__s| __s.constCopy),
            nonConstMove: other.with(|__s| __s.nonConstMove),
            constMove: (other.with(|__s| __s.constMove) + 1_()),
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
                nonConstCopy: 0_(),
                constCopy: 0_(),
                nonConstMove: 0_(),
                constMove: 0_(),
                alloc_constructed: false,
            }
        }
    }
}
pub fn nonConstCopyCtrCalled_4(obj: Ptr<TracedCopyMove>) -> () {
    return (((obj.with(|__s| __s.nonConstCopy) == 1_())
        && (obj.with(|__s| __s.constCopy) == 0_()))
        && (obj.with(|__s| __s.constMove) == 0_()))
        && (obj.with(|__s| __s.nonConstMove) == 0_());
}
pub fn constMoveCtrCalled_5(obj: Ptr<TracedCopyMove>) -> () {
    return (((obj.with(|__s| __s.nonConstMove) == 0_())
        && (obj.with(|__s| __s.constMove) == 1_()))
        && (obj.with(|__s| __s.constCopy) == 0_()))
        && (obj.with(|__s| __s.nonConstCopy) == 0_());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoConstructorFromInt {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct CopyAssign {
    #[offset(0)]
    pub val: (),
}
impl CopyAssign {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct ConstCopyAssign {
    #[offset(0)]
    pub val: (),
}
impl ConstCopyAssign {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(4)]
pub struct MoveAssign {
    #[offset(0)]
    pub val: (),
}
impl MoveAssign {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(4)]
pub struct ConstMoveAssign {
    #[offset(0)]
    pub val: (),
}
impl ConstMoveAssign {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct TracedAssignment {
    #[offset(0)]
    pub copyAssign: (),
    #[offset(4)]
    pub constCopyAssign: (),
    #[offset(8)]
    pub moveAssign: (),
    #[offset(12)]
    pub constMoveAssign: (),
}
impl Default for TracedAssignment {
    fn default() -> Self {
        {
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
    "std::is_convertible_v<const std::tuple<ConstMove>&&, std::tuple<ConvertibleFrom<ConstMove>>>"
);
const _: () = assert!(
    true,
    "std::is_convertible_v<const std::tuple<ConstMove, ConstMove>&&,\n                                    std::tuple<ConvertibleFrom<ConstMove>, ConvertibleFrom<ConstMove>>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<const std::tuple<MutableCopy>&&, std::tuple<ExplicitConstructibleFrom<ConstMove>>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<const std::tuple<ConstMove, ConstMove>&&,\n                                     std::tuple<ConvertibleFrom<ConstMove>, ExplicitConstructibleFrom<ConstMove>>>"
);
pub fn test_6() -> () {
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::tuple<int, int>, const std::tuple<int>&&>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::tuple<int, int, int>, const std::tuple<int, int>&&>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<std::tuple<int, NoConstructorFromInt>, const std::tuple<int, int>&&>"
    );;
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
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
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
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
