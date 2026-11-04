extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn destroy_at_0(mut p: Ptr<ConstCopy>) {}
pub fn destroy_1(mut first: Ptr<ConstCopy>, mut last: Ptr<ConstCopy>) {
    'loop_: while ({ (first).clone() } != { (last).clone() }) {
        ({ destroy_at_0(({ addressof_2((first).clone()) })) });
        first.prefix_inc();
    }
}
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
    pub static destructed_value_3: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_4: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_5(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_6(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_5(_x, _y)
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
        operator_eq_5(__lhs.clone(), __rhs.clone())
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
pub fn nonConstCopyCtrCalled_7(obj: Ptr<TracedCopyMove>) -> bool {
    return (((obj.with(|__s| __s.nonConstCopy) == 1) && (obj.with(|__s| __s.constCopy) == 0))
        && (obj.with(|__s| __s.constMove) == 0))
        && (obj.with(|__s| __s.nonConstMove) == 0);
}
pub fn constMoveCtrCalled_8(obj: Ptr<TracedCopyMove>) -> bool {
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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    pub data_: i32,
}
impl B {
    pub fn new() -> Self {
        let __this: B = Self { data_: 1 };
        (*population__10.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(b: Ptr<B>) -> Self {
        let __this: Value<B> = Rc::new(RefCell::new(Self { data_: 0_i32 }));
        let this: Ptr<B> = __this.as_pointer();
        (*count__9.with(Value::clone).borrow_mut()).prefix_inc();
        if (count__9.with(|rc| *rc.borrow()) == 3) {
            std::process::abort();
        }
        field!(this, data_).write({ b.with(|__s| __s.data_) });
        (*population__10.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for B {
    fn clone(&self) -> Self {
        let __src: Value<B> = Rc::new(RefCell::new(B {
            data_: self.data_.clone(),
        }));
        B::copy_from(__src.as_pointer())
    }
}
impl CppMove for B {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for B {
    fn default() -> Self {
        { B::new() }
    }
}
thread_local!(
    pub static count__9: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static population__10: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Nasty {
    #[offset(0)]
    pub i_: i32,
}
impl Nasty {
    pub fn new() -> Self {
        Self {
            i_: (*counter__11.with(Value::clone).borrow_mut()).postfix_inc(),
        }
    }
}
impl CppMove for Nasty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Nasty {
    fn default() -> Self {
        { Nasty::new() }
    }
}
thread_local!(
    pub static counter__11: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn test_12() -> bool {
    let mut n: i32 = 3;
    let value: Value<ConstCopy> = Rc::new(RefCell::new(ConstCopy::new_1({ 42 })));
    let alloc: Value<std_allocator_ConstCopy_> =
        Rc::new(RefCell::new(<std_allocator_ConstCopy_>::default()));
    let mut out: Ptr<ConstCopy> = ({ (*alloc.borrow()).allocate((n as usize)) });
    ({
        let ___first: Ptr<ConstCopy> = (out).clone();
        let ___last: Ptr<ConstCopy> = out.offset((n) as isize);
        let ___x: Ptr<ConstCopy> = value.as_pointer();
        uninitialized_fill_13(___first, ___last, ___x)
    });
    let mut i: i32 = 0;
    'loop_: while (i != n) {
        assert!(({ ({ elem!(out, i).with(|__s| __s.val) } == { { (*value.borrow()).val } }) }));
        i.prefix_inc();
    }
    ({
        let _first: Ptr<ConstCopy> = (out).clone();
        let _last: Ptr<ConstCopy> = out.offset((n) as isize);
        destroy_1(_first, _last)
    });
    ({ (*alloc.borrow()).deallocate((out).clone(), (n as usize)) });
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
    {
        let mut N: i32 = 5;
        let pool: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut bp: Ptr<B> = (pool.as_pointer() as Ptr<i8>).reinterpret_cast::<B>();
        assert!((population__10.with(|rc| *rc.borrow()) == 0));
        count__9.with(|rc| *rc.borrow_mut() = 0);
        ({
            let ___first: Ptr<B> = (bp).clone();
            let ___last: Ptr<B> = bp.offset((2) as isize);
            let ___x: Value<B> = Rc::new(RefCell::new(B::new()));
            uninitialized_fill_14(___first, ___last, ___x.as_pointer())
        });
        let mut i: i32 = 0;
        'loop_: while (i < 2) {
            assert!((elem!(bp, i).with(|__s| __s.data_) == 1));
            i.prefix_inc();
        }
        assert!((population__10.with(|rc| *rc.borrow()) == 2));
    }
    {
        let mut N: i32 = 5;
        let pool: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
            0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
        ])));
        let mut bp: Ptr<Nasty> = (pool.as_pointer() as Ptr<i8>).reinterpret_cast::<Nasty>();
        counter__11.with(|rc| *rc.borrow_mut() = 23);
        ({
            let ___first: Ptr<Nasty> = (bp).clone();
            let ___last: Ptr<Nasty> = bp.offset((N) as isize);
            let ___x: Value<Nasty> = Rc::new(RefCell::new(Nasty::new()));
            uninitialized_fill_15(___first, ___last, ___x.as_pointer())
        });
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            assert!((elem!(bp, i).with(|__s| __s.i_) == 23));
            i.prefix_inc();
        }
    }
    ({ test_12() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct test_allocator_int_;
pub trait BImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<B>) -> Ptr<B>;
}
impl BImpl for Ptr<B> {
    fn destructor(&self) {
        field!((*self), data_).write(0);
        (*population__10.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<B>) -> Ptr<B> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
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
pub trait NastyImpl {
    fn operator_addr(&self) -> Ptr<Nasty>;
}
impl NastyImpl for Ptr<Nasty> {
    fn operator_addr(&self) -> Ptr<Nasty> {
        return Ptr::<Nasty>::null();
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
        field!((*self), data_).write(destructed_value_3.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_3.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_3.with(|_| ());
    let _ = moved_value_4.with(|_| ());
    let _ = count__9.with(|_| ());
    let _ = population__10.with(|_| ());
    let _ = counter__11.with(|_| ());
}
