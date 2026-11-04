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
impl CppMove for test_allocator_statistics {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for test_allocator_void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for Ctor_Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
    pub fn new_2(
        _a0: std_allocator_arg_t,
        _a1: Ptr<test_allocator_int_>,
        o: Ptr<MutableCopy>,
    ) -> Self {
        let _a0: Value<std_allocator_arg_t> = Rc::new(RefCell::new(_a0));
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: true,
        }
    }
}
impl CppMove for MutableCopy {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    pub fn new_2(
        _a0: std_allocator_arg_t,
        _a1: Ptr<test_allocator_int_>,
        o: Ptr<ConstCopy>,
    ) -> Self {
        let _a0: Value<std_allocator_arg_t> = Rc::new(RefCell::new(_a0));
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: true,
        }
    }
}
impl CppMove for ConstCopy {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    pub fn new_2(
        _a0: std_allocator_arg_t,
        _a1: Ptr<test_allocator_int_>,
        o: Ptr<MutableMove>,
    ) -> Self {
        let _a0: Value<std_allocator_arg_t> = Rc::new(RefCell::new(_a0));
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: true,
        }
    }
}
impl CppMove for MutableMove {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
    pub fn new_2(
        _a0: std_allocator_arg_t,
        _a1: Ptr<test_allocator_int_>,
        o: Ptr<ConstMove>,
    ) -> Self {
        let _a0: Value<std_allocator_arg_t> = Rc::new(RefCell::new(_a0));
        Self {
            val: o.with(|__s| __s.val),
            alloc_constructed: true,
        }
    }
}
impl CppMove for ConstMove {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
    pub fn new_1(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: other.with(|__s| __s.nonConstCopy),
            constCopy: (other.with(|__s| __s.constCopy) + 1),
            nonConstMove: other.with(|__s| __s.nonConstMove),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_2(other: Ptr<TracedCopyMove>) -> Self {
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
        TracedCopyMove::new_1(__src.as_pointer())
    }
}
impl CppMove for TracedCopyMove {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for NoConstructorFromInt {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(std_tuple_CvtFromTupleRef_, new_1, mut_ptr)]
#[byte_size(20)]
pub struct CvtFromTupleRef {}
impl CvtFromTupleRef {
    pub fn new_1(other: Ptr<std_tuple_CvtFromTupleRef_>) -> Self {
        Self {}
    }
}
impl CppMove for CvtFromTupleRef {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(20)]
pub struct ExplicitCtrFromTupleRef {}
impl ExplicitCtrFromTupleRef {
    pub fn new_1(other: Ptr<std_tuple_ExplicitCtrFromTupleRef_>) -> Self {
        Self {}
    }
}
impl CppMove for ExplicitCtrFromTupleRef {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from_move(std_tuple_CvtFromConstTupleRefRef_, new_1, ptr)]
#[byte_size(20)]
pub struct CvtFromConstTupleRefRef {}
impl CvtFromConstTupleRefRef {
    pub fn new_1(other: Ptr<std_tuple_CvtFromConstTupleRefRef_>) -> Self {
        Self {}
    }
}
impl CppMove for CvtFromConstTupleRefRef {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(20)]
pub struct ExplicitCtrFromConstTupleRefRef {}
impl ExplicitCtrFromConstTupleRefRef {
    pub fn new_1(other: Ptr<std_tuple_const_ExplicitCtrFromConstTupleRefRef_>) -> Self {
        Self {}
    }
}
impl CppMove for ExplicitCtrFromConstTupleRefRef {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl AssignableFrom_ConstCopyAssign_ {
    pub fn new_1(u: Ptr<i32>) -> Self {
        Self {
            v: ConstCopyAssign::new_1({ (u.read()) }),
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, Default)]
#[cpp_from_move(i32, new_1, ptr)]
#[byte_size(4)]
pub struct AssignableFrom_ConstCopyAssign_ {
    #[offset(0)]
    #[byte_size(4)]
    pub v: ConstCopyAssign,
}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(16)]
pub struct AssignableFrom_TracedAssignment_ {
    #[offset(0)]
    #[byte_size(16)]
    pub v: TracedAssignment,
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
    "std::is_assignable_v<const std::tuple<int&>&, const std::tuple<long&>&>"
);
const _: () = assert!(
    true,
    "!std::is_assignable_v<const std::tuple<int&, int&>&, const std::tuple<long&>&>"
);
const _: () = assert!(
    true,
    "!std::is_assignable_v<const std::tuple<int&>&, const std::tuple<long&, long&>&>"
);
const _: () = assert!(
    true,
    "std::is_assignable_v<const std::tuple<AssignableFrom<ConstCopyAssign>>&,\n                                   const std::tuple<ConstCopyAssign>&>"
);
const _: () = assert!(
    true,
    "std::is_assignable_v<const std::tuple<AssignableFrom<ConstCopyAssign>, ConstCopyAssign>&,\n                                   const std::tuple<ConstCopyAssign, ConstCopyAssign>&>"
);
const _: () = assert!(
    true,
    "!std::is_assignable_v<const std::tuple<AssignableFrom<ConstCopyAssign>, CopyAssign>&,\n                                    const std::tuple<ConstCopyAssign, CopyAssign>&>"
);
pub fn test_6() -> bool {
    {
        let i1: Value<i32> = Rc::new(RefCell::new(1));
        let i2: Value<i32> = Rc::new(RefCell::new(2));
        let j1: Value<i64> = Rc::new(RefCell::new(3_i64));
        let j2: Value<i64> = Rc::new(RefCell::new(4_i64));
        let t1: Value<std_tuple_int_ref__int_ref_> = Rc::new(RefCell::new(
            std_tuple_int_ref__int_ref_::new_1({ i1.as_pointer() }, { i2.as_pointer() }),
        ));
        let t2: Value<std_tuple_long_ref__long_ref_> = Rc::new(RefCell::new(
            std_tuple_long_ref__long_ref_::new_1({ j1.as_pointer() }, { j2.as_pointer() }),
        ));
        (*t2.borrow_mut()) = (*t1.borrow()).clone();
        assert!(((({ get_7(t2.as_pointer(),) }).read()) == 1_i64));
        assert!(((({ get_8(t2.as_pointer(),) }).read()) == 2_i64));
    }
    {
        let t1: Value<std_tuple_ConstCopyAssign_> = Rc::new(RefCell::new(1));
        let t2: Value<std_tuple_AssignableFrom_ConstCopyAssign__> = Rc::new(RefCell::new(2));
        (*t2.borrow_mut()) = (*t1.borrow()).clone();
        assert!((({ get_9(t2.as_pointer(),) }).with(|__s| __s.v.val) == 1));
    }
    {
        let t1: Value<std_tuple_TracedAssignment_> =
            Rc::new(RefCell::new(<std_tuple_TracedAssignment_>::default()));
        let t2: Value<std_tuple_AssignableFrom_TracedAssignment__> = Rc::new(RefCell::new(
            <std_tuple_AssignableFrom_TracedAssignment__>::default(),
        ));
        (*t2.borrow_mut()) = (*t1.borrow()).clone();
        assert!((({ get_10(t2.as_pointer(),) }).with(|__s| __s.v.constCopyAssign) == 1));
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
pub trait AssignableFrom_ConstCopyAssign_Impl {
    fn operator_assign_5_struct_ConstCopyAssign_0(
        &self,
        t: Ptr<ConstCopyAssign>,
    ) -> Ptr<AssignableFrom_ConstCopyAssign_>;
}
impl AssignableFrom_ConstCopyAssign_Impl for Ptr<AssignableFrom_ConstCopyAssign_> {
    fn operator_assign_5_struct_ConstCopyAssign_0(
        &self,
        t: Ptr<ConstCopyAssign>,
    ) -> Ptr<AssignableFrom_ConstCopyAssign_> {
        ({
            let _other: Ptr<ConstCopyAssign> = (t).clone();
            ConstCopyAssignImpl::copy_assign(&field_ptr!((*self), v), _other)
        });
        return (*self).clone();
    }
}
pub trait AssignableFrom_TracedAssignment_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<AssignableFrom_TracedAssignment_>,
    ) -> Ptr<AssignableFrom_TracedAssignment_>;
    fn operator_assign_5_struct_TracedAssignment_0(
        &self,
        t: Ptr<TracedAssignment>,
    ) -> Ptr<AssignableFrom_TracedAssignment_>;
}
impl AssignableFrom_TracedAssignment_Impl for Ptr<AssignableFrom_TracedAssignment_> {
    fn move_assign(
        &self,
        _a0: Ptr<AssignableFrom_TracedAssignment_>,
    ) -> Ptr<AssignableFrom_TracedAssignment_> {
        ({
            let _arg0: Ptr<TracedAssignment> = field_ptr!(_a0, v);
            TracedAssignmentImpl::operator_assign_4(&field_ptr!((*self), v), _arg0)
        });
        return (*self).clone();
    }
    fn operator_assign_5_struct_TracedAssignment_0(
        &self,
        t: Ptr<TracedAssignment>,
    ) -> Ptr<AssignableFrom_TracedAssignment_> {
        ({
            let _arg0: Ptr<TracedAssignment> = (t).clone();
            TracedAssignmentImpl::operator_assign_3(&field_ptr!((*self), v), _arg0)
        });
        return (*self).clone();
    }
}
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
