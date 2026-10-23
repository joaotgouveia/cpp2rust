extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Buffer_Counted__1_ {
    #[offset(0)]
    #[byte_size(8)]
    pub buffer: Box<[()]>,
}
impl Default for Buffer_Counted__1_ {
    fn default() -> Self {
        {
            Buffer_Counted__1_ {
                buffer: Box::new([
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct Buffer_Counted__5_ {
    #[offset(0)]
    #[byte_size(40)]
    pub buffer: Box<[()]>,
}
impl Default for Buffer_Counted__5_ {
    fn default() -> Self {
        {
            Buffer_Counted__5_ {
                buffer: Box::new([
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Buffer_double__3_ {
    #[offset(0)]
    #[byte_size(24)]
    pub buffer: Box<[()]>,
}
impl Default for Buffer_double__3_ {
    fn default() -> Self {
        {
            Buffer_double__3_ {
                buffer: Box::new([
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Buffer_Counted__3_ {
    #[offset(0)]
    #[byte_size(24)]
    pub buffer: Box<[()]>,
}
impl Default for Buffer_Counted__3_ {
    fn default() -> Self {
        {
            Buffer_Counted__3_ {
                buffer: Box::new([
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Buffer_int__3_ {
    #[offset(0)]
    #[byte_size(12)]
    pub buffer: Box<[()]>,
}
impl Default for Buffer_int__3_ {
    fn default() -> Self {
        {
            Buffer_int__3_ {
                buffer: Box::new([
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
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
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_in_out_result_Counted_ptr__Counted_ptr___std_ranges_in_out_result_Counted_ptr__Counted_ptr__
{}
pub fn operator_eq_6(l: Ptr<Counted>, r: Ptr<Counted>) -> () {
    return ({ l.with(|__s| __s.value) } == { r.with(|__s| __s.value) });
}
pub fn operator_ne_7(l: Ptr<Counted>, r: Ptr<Counted>) -> () {
    return !({
        let _l: Ptr<Counted> = (l).clone();
        let _r: Ptr<Counted> = (r).clone();
        operator_eq_6(_l, _r)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub value: (),
    #[offset(4)]
    pub moved_from: (),
}
impl Counted {
    pub fn new_1() -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            value: unimplemented!(),
            moved_from: false,
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        ({ CountedImpl::check_throw(&this) });
        ({ CountedImpl::increase_counters(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(mut v: ()) -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            value: v,
            moved_from: false,
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        ({ CountedImpl::check_throw(&this) });
        ({ CountedImpl::increase_counters(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn reset() {
        current_objects_1.with(|rc| {
            *rc.borrow_mut() = {
                total_objects_2.with(|rc| *rc.borrow_mut() = 0_());
                total_objects_2.with(|rc| *rc.borrow())
            }
        });
        total_copies_3.with(|rc| {
            *rc.borrow_mut() = {
                total_moves_4.with(|rc| *rc.borrow_mut() = 0_());
                total_moves_4.with(|rc| *rc.borrow())
            }
        });
        throw_on_5.with(|rc| *rc.borrow_mut() = -1_());
    }
    pub fn copy_from(rhs: Ptr<Counted>) -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            value: rhs.with(|__s| __s.value),
            moved_from: false,
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        ({ CountedImpl::check_throw(&this) });
        ({ CountedImpl::increase_counters(&this) });
        (*total_copies_3.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(rhs: Ptr<Counted>) -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            value: rhs.with(|__s| __s.value),
            moved_from: false,
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        ({ CountedImpl::check_throw(&this) });
        ({ CountedImpl::increase_counters(&this) });
        field!(rhs, moved_from).write(true);
        (*total_moves_4.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_6(
                Rc::new(RefCell::new(Counted {
                    value: self.value.clone(),
                    moved_from: self.moved_from.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Counted {
                    value: other.value.clone(),
                    moved_from: other.moved_from.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Counted {}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted {
            value: self.value.clone(),
            moved_from: self.moved_from.clone(),
        }));
        Counted::copy_from(__src.as_pointer())
    }
}
impl Default for Counted {
    fn default() -> Self {
        { Counted::new_1() }
    }
}
thread_local!(
    pub static current_objects_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static total_objects_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static total_copies_3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static total_moves_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static throw_on_5: Value<()> = Rc::new(RefCell::new(-1_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct overload_compare_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
}
impl overload_compare_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<()>) -> Self {
        Self { it_: (it).clone() }
    }
}
impl std::cmp::PartialEq for overload_compare_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            overload_compare_iterator_int_ptr_Impl::operator_eq(
                &Rc::new(RefCell::new(overload_compare_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(overload_compare_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for overload_compare_iterator_int_ptr_ {}
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
    pub static destructed_value_8: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_9: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_10(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_11(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_10(_x, _y)
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
            operator_eq_10(
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
    pub fn new_1(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: other.with(|__s| __s.nonConstCopy),
            constCopy: (other.with(|__s| __s.constCopy) + 1_()),
            nonConstMove: other.with(|__s| __s.nonConstMove),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_2(other: Ptr<TracedCopyMove>) -> Self {
        Self {
            nonConstCopy: (other.with(|__s| __s.nonConstCopy) + 1_()),
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
            nonConstMove: (other.with(|__s| __s.nonConstMove) + 1_()),
            constMove: other.with(|__s| __s.constMove),
            alloc_constructed: false,
        }
    }
    pub fn new_4(other: Ptr<TracedCopyMove>) -> Self {
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
        TracedCopyMove::new_1(__src.as_pointer())
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
pub fn nonConstCopyCtrCalled_12(obj: Ptr<TracedCopyMove>) -> () {
    return (((obj.with(|__s| __s.nonConstCopy) == 1_())
        && (obj.with(|__s| __s.constCopy) == 0_()))
        && (obj.with(|__s| __s.constMove) == 0_()))
        && (obj.with(|__s| __s.nonConstMove) == 0_());
}
pub fn constMoveCtrCalled_13(obj: Ptr<TracedCopyMove>) -> () {
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct support_double_move_tracker {
    #[offset(0)]
    moved_from_: (),
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        unimplemented!();
        field!(other, moved_from_).write(true);
        __this
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        { support_double_move_tracker::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_14(
    x: Ptr<forward_iterator_Counted_ptr_>,
    y: Ptr<forward_iterator_Counted_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn base_15(i: Ptr<forward_iterator_Counted_ptr_>) -> Ptr<Counted> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(16)]
pub struct forward_iterator_Counted_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Counted>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl forward_iterator_Counted_ptr_ {
    pub fn new_1(mut it: Ptr<Counted>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<forward_iterator_Counted_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for forward_iterator_Counted_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_14(
                Rc::new(RefCell::new(forward_iterator_Counted_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_iterator_Counted_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_Counted_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
pub fn base_16(i: Ptr<cpp20_input_iterator_const_int_ptr_>) -> Ptr<()> {
    return i.with(|__s| __s.it_.clone());
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_const_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_const_int_ptr_ {
    pub fn new(mut it: Ptr<()>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_const_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct IteratorOpCounts {
    #[offset(0)]
    pub increments: (),
    #[offset(8)]
    pub decrements: (),
    #[offset(16)]
    pub zero_moves: (),
    #[offset(24)]
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_forward_iterator_Counted_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<Counted>,
}
impl sentinel_wrapper_forward_iterator_Counted_ptr__ {
    pub fn new_1(it: Ptr<forward_iterator_Counted_ptr_>) -> Self {
        Self {
            base_: ({ base_15((it).clone()) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct sentinel_wrapper_cpp20_input_iterator_const_int_ptr__ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<()>,
}
impl sentinel_wrapper_cpp20_input_iterator_const_int_ptr__ {
    pub fn new_1(it: Ptr<cpp20_input_iterator_const_int_ptr_>) -> Self {
        Self {
            base_: ({ base_16((it).clone()) }),
        }
    }
}
pub fn operator_eq_17(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> () {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_17(
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<()>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_18(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_19(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_18(_i, n)
    });
}
pub fn iter_swap_20(a: adl_Iterator, b: adl_Iterator) {
    let a: Value<adl_Iterator> = Rc::new(RefCell::new(a));
    let b: Value<adl_Iterator> = Rc::new(RefCell::new(b));
    ({
        let ___x: Ptr<Ptr<()>> = field_ptr!(a.as_pointer(), ptr_);
        let ___y: Ptr<Ptr<()>> = field_ptr!(b.as_pointer(), ptr_);
        swap_21(___x, ___y)
    });
    if !({ (*a.borrow()).iter_swaps_.clone() }).is_null() {
        { (*a.borrow()).iter_swaps_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_22(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) }).reinterpret_cast::<()>();
}
pub fn operator_eq_23(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_24(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return std::cmp::Ord::cmp(
        &(lhs.with(|__s| __s.ptr_.clone())),
        &(rhs.with(|__s| __s.ptr_.clone())),
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct adl_Iterator {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<()>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<()>, mut iter_moves: Ptr<()>, mut iter_swaps: Ptr<()>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<()>, iter_moves: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<()>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<()>::null() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<()>, iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_24(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_23(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<()>::null(),
                iter_moves_: Ptr::<()>::null(),
                iter_swaps_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
}
impl rvalue_iterator_int_ptr_ {}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
const _: () = assert!(
    true,
    "std::is_class_v<decltype(std::ranges::uninitialized_copy)>"
);
const _: () = assert!(
    true,
    "std::is_invocable_v<decltype(std::ranges::uninitialized_copy), int*, int*, long*, long*>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotConvertibleFromInt {}
const _: () = assert!(
    true,
    "!std::is_invocable_v<decltype(std::ranges::uninitialized_copy), int*, int*, NotConvertibleFromInt*,\n                                   NotConvertibleFromInt*>"
);
pub fn test_25() -> () {
    let n: Value<()> = Rc::new(RefCell::new(3_()));
    let in_: Value<Box<[ConstCopy]>> = Rc::new(RefCell::new(Box::new([
        ConstCopy::new_1({ 1_() }),
        ConstCopy::new_1({ 2_() }),
        ConstCopy::new_1({ 3_() }),
    ])));
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let mut out: Ptr<ConstCopy> = unimplemented!();
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___ifirst: Ptr<ConstCopy> = (in_.as_pointer() as Ptr<ConstCopy>);
                let ___ilast: Ptr<ConstCopy> =
                    (in_.as_pointer() as Ptr<ConstCopy>).offset((*n.borrow()) as isize);
                let ___ofirst: Ptr<ConstCopy> = (out).clone();
                let ___olast: Ptr<ConstCopy> = out.offset((*n.borrow()) as isize);
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i != (*n.borrow())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: Ptr<ConstCopy> = unimplemented!();
        let out_range : Value<std_ranges_subrange_ConstCopy_ptr__ConstCopy_ptr__std_ranges_subrange_kind_sized_ > = Rc::new(RefCell::new(std_ranges_subrange_ConstCopy_ptr__ConstCopy_ptr__std_ranges_subrange_kind_sized_ :: new_1 ( { (out ).clone() } , { out . offset ( ( (*n.borrow()) ) as isize )  } , )  )) ;
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    in_.as_pointer(),
                    out_range.as_pointer(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i != (*n.borrow())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = 2_();
        let mut out: Ptr<ConstCopy> = unimplemented!();
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___ifirst: Ptr<ConstCopy> = (in_.as_pointer() as Ptr<ConstCopy>);
                let ___ilast: Ptr<ConstCopy> =
                    (in_.as_pointer() as Ptr<ConstCopy>).offset((*n.borrow()) as isize);
                let ___ofirst: Ptr<ConstCopy> = (out).clone();
                let ___olast: Ptr<ConstCopy> = out.offset((m) as isize);
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i != m) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
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
    {
        let in_: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([Counted::new_1()])));
        let out: Value<Buffer_Counted__1_> = Rc::new(RefCell::new(<Buffer_Counted__1_>::default()));
        ({ Counted::reset() });
        {
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    let ___ifirst: Ptr<Counted> = (in_.as_pointer() as Ptr<Counted>);
                    let ___ilast: Ptr<Counted> = (in_.as_pointer() as Ptr<Counted>);
                    let ___ofirst: Ptr<Counted> =
                        ({ Buffer_Counted__1_Impl::begin(&out.as_pointer()) });
                    let ___olast: Ptr<Counted> =
                        ({ Buffer_Counted__1_Impl::end(&out.as_pointer()) });
                    (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                        ___ifirst, ___ilast, ___ofirst, ___olast,
                    )
                })
                .clone(),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                        view.as_pointer(),
                        out.as_pointer(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let it: Value<forward_iterator_Counted_ptr_> =
                Rc::new(RefCell::new(forward_iterator_Counted_ptr_::new_1({
                    (in_.as_pointer() as Ptr<Counted>)
                })));
            let range : Value<std_ranges_subrange_forward_iterator_Counted_ptr___sentinel_wrapper_forward_iterator_Counted_ptr____std_ranges_subrange_kind_unsized_ > = Rc::new(RefCell::new(std_ranges_subrange_forward_iterator_Counted_ptr___sentinel_wrapper_forward_iterator_Counted_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { ((*it.borrow()) ).clone()  } , { sentinel_wrapper_forward_iterator_Counted_ptr__ :: new_1 ( { it .as_pointer()  } , )   } , )  )) ;
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    let ___ifirst: forward_iterator_Counted_ptr_ =
                        ({ (*range.borrow()).begin_8() });
                    let ___ilast: sentinel_wrapper_forward_iterator_Counted_ptr__ =
                        ({ (*range.borrow()).end() });
                    let ___ofirst: Ptr<Counted> =
                        ({ Buffer_Counted__1_Impl::begin(&out.as_pointer()) });
                    let ___olast: Ptr<Counted> =
                        ({ Buffer_Counted__1_Impl::end(&out.as_pointer()) });
                    (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                        ___ifirst, ___ilast, ___ofirst, ___olast,
                    )
                })
                .clone(),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let it: Value<forward_iterator_Counted_ptr_> =
                Rc::new(RefCell::new(forward_iterator_Counted_ptr_::new_1({
                    (in_.as_pointer() as Ptr<Counted>)
                })));
            let range : Value<std_ranges_subrange_forward_iterator_Counted_ptr___sentinel_wrapper_forward_iterator_Counted_ptr____std_ranges_subrange_kind_unsized_ > = Rc::new(RefCell::new(std_ranges_subrange_forward_iterator_Counted_ptr___sentinel_wrapper_forward_iterator_Counted_ptr____std_ranges_subrange_kind_unsized_ :: new_1 ( { ((*it.borrow()) ).clone()  } , { sentinel_wrapper_forward_iterator_Counted_ptr__ :: new_1 ( { it .as_pointer()  } , )   } , )  )) ;
            let result: Value<()> = Rc::new(RefCell::new(
                ({
                    (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                        range.as_pointer(),
                        out.as_pointer(),
                    )
                }),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ Counted::reset() });
    }
    {
        let N: Value<()> = Rc::new(RefCell::new(5_()));
        let in_: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ])));
        let out: Value<Buffer_Counted__5_> = Rc::new(RefCell::new(<Buffer_Counted__5_>::default()));
        ({ Counted::reset() });
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___ifirst: Ptr<Counted> = (in_.as_pointer() as Ptr<Counted>);
                let ___ilast: Ptr<Counted> =
                    (in_.as_pointer() as Ptr<Counted>).offset((*N.borrow()) as isize);
                let ___ofirst: Ptr<Counted> =
                    ({ Buffer_Counted__5_Impl::begin(&out.as_pointer()) });
                let ___olast: Ptr<Counted> = ({ Buffer_Counted__5_Impl::end(&out.as_pointer()) });
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            })
            .clone(),
        ));
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ({ Counted::reset() });
    {
        let N: Value<()> = Rc::new(RefCell::new(5_()));
        let in_: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ])));
        let out: Value<Buffer_Counted__5_> = Rc::new(RefCell::new(<Buffer_Counted__5_>::default()));
        ({ Counted::reset() });
        let range: Value<
            std_ranges_subrange_Counted_ptr__Counted_ptr__std_ranges_subrange_kind_sized_,
        > = Rc::new(RefCell::new(
            std_ranges_subrange_Counted_ptr__Counted_ptr__std_ranges_subrange_kind_sized_::new_1(
                { (in_.as_pointer() as Ptr<Counted>) },
                { (in_.as_pointer() as Ptr<Counted>).offset((*N.borrow()) as isize) },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    out.as_pointer(),
                )
            }),
        ));
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ({ Counted::reset() });
    {
        let mut N: () = 3_();
        let in_: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ])));
        let out: Value<Buffer_Counted__5_> = Rc::new(RefCell::new(<Buffer_Counted__5_>::default()));
        ({ Counted::reset() });
        let iter: Value<()> = Rc::new(RefCell::new(std_counted_iterator_Counted_ptr_::new_1(
            { (in_.as_pointer() as Ptr<Counted>) },
            { N },
        )));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___ofirst: Ptr<Counted> =
                    ({ Buffer_Counted__5_Impl::begin(&out.as_pointer()) });
                let ___olast: Ptr<Counted> = ({ Buffer_Counted__5_Impl::end(&out.as_pointer()) });
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    unimplemented!(),
                    unimplemented!(),
                    ___ofirst,
                    ___olast,
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ({ Counted::reset() });
    {
        let N: Value<()> = Rc::new(RefCell::new(3_()));
        let in_: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ])));
        let out: Value<Buffer_Counted__5_> = Rc::new(RefCell::new(<Buffer_Counted__5_>::default()));
        ({ Counted::reset() });
        let view: Value<()> = Rc::new(RefCell::new(
            ({ (*counted_27.with(Value::clone).borrow_mut())(in_.as_pointer(), N.as_pointer()) }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    view.as_pointer(),
                    out.as_pointer(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ({ Counted::reset() });
    {
        let N: Value<()> = Rc::new(RefCell::new(3_()));
        let in_: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ])));
        let out: Value<Buffer_Counted__5_> = Rc::new(RefCell::new(<Buffer_Counted__5_>::default()));
        ({ Counted::reset() });
        let range: Value<
            std_ranges_subrange_Counted_ptr__Counted_ptr__std_ranges_subrange_kind_sized_,
        > = Rc::new(RefCell::new(
            std_ranges_subrange_Counted_ptr__Counted_ptr__std_ranges_subrange_kind_sized_::new_1(
                { (in_.as_pointer() as Ptr<Counted>) },
                { (in_.as_pointer() as Ptr<Counted>).offset((*N.borrow()) as isize) },
            ),
        ));
        let view: Value<()> = Rc::new(RefCell::new(
            ({ (*reverse_28.with(Value::clone).borrow_mut())(range.as_pointer()) }),
        ));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    view.as_pointer(),
                    out.as_pointer(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let expected: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([
            Counted::new_2({ 3_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 1_() }),
        ])));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ({ Counted::reset() });
    {
        let N: Value<()> = Rc::new(RefCell::new(5_()));
        let in_: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
        let out: Value<Box<[()]>> =
            Rc::new(RefCell::new(Box::new([6_(), 7_(), 8_(), 9_(), 10_()])));
        unimplemented!();
        ({
            let ___ifirst: Ptr<()> = (in_.as_pointer() as Ptr<()>);
            let ___ilast: Ptr<()> = (in_.as_pointer() as Ptr<()>).offset((1_()) as isize);
            let ___ofirst: Ptr<()> = (out.as_pointer() as Ptr<()>);
            let ___olast: Ptr<()> = (out.as_pointer() as Ptr<()>).offset((*N.borrow()) as isize);
            (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        unimplemented!();
        unimplemented!();
        ({
            let ___ifirst: Ptr<()> = (in_.as_pointer() as Ptr<()>);
            let ___ilast: Ptr<()> = (in_.as_pointer() as Ptr<()>).offset((*N.borrow()) as isize);
            let ___ofirst: Ptr<()> = (out.as_pointer() as Ptr<()>);
            let ___olast: Ptr<()> = (out.as_pointer() as Ptr<()>).offset((*N.borrow()) as isize);
            (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        unimplemented!();
    }
    {
        let N: Value<()> = Rc::new(RefCell::new(3_()));
        let in_: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let out: Value<Buffer_double__3_> = Rc::new(RefCell::new(<Buffer_double__3_>::default()));
        ({
            let ___ifirst: Ptr<()> = (in_.as_pointer() as Ptr<()>);
            let ___ilast: Ptr<()> = (in_.as_pointer() as Ptr<()>).offset((*N.borrow()) as isize);
            let ___ofirst: Ptr<()> = ({ Buffer_double__3_Impl::begin(&out.as_pointer()) });
            let ___olast: Ptr<()> = ({ Buffer_double__3_Impl::end(&out.as_pointer()) });
            (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                ___ifirst, ___ilast, ___ofirst, ___olast,
            )
        });
        unimplemented!();
    }
    {
        let N: Value<()> = Rc::new(RefCell::new(3_()));
        let in_: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let out: Value<Buffer_double__3_> = Rc::new(RefCell::new(<Buffer_double__3_>::default()));
        ({
            (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                in_.as_pointer(),
                out.as_pointer(),
            )
        });
        unimplemented!();
    }
    {
        let mut M: () = 3_();
        let N: Value<()> = Rc::new(RefCell::new(5_()));
        let in_: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ])));
        let out: Value<Buffer_Counted__3_> = Rc::new(RefCell::new(<Buffer_Counted__3_>::default()));
        ({ Counted::reset() });
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                let ___ifirst: Ptr<Counted> = (in_.as_pointer() as Ptr<Counted>);
                let ___ilast: Ptr<Counted> =
                    (in_.as_pointer() as Ptr<Counted>).offset((*N.borrow()) as isize);
                let ___ofirst: Ptr<Counted> =
                    ({ Buffer_Counted__3_Impl::begin(&out.as_pointer()) });
                let ___olast: Ptr<Counted> = ({ Buffer_Counted__3_Impl::end(&out.as_pointer()) });
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            })
            .clone(),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut M: () = 3_();
        let N: Value<()> = Rc::new(RefCell::new(5_()));
        let in_: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new([
            Counted::new_2({ 1_() }),
            Counted::new_2({ 2_() }),
            Counted::new_2({ 3_() }),
            Counted::new_2({ 4_() }),
            Counted::new_2({ 5_() }),
        ])));
        let out: Value<Buffer_Counted__3_> = Rc::new(RefCell::new(<Buffer_Counted__3_>::default()));
        ({ Counted::reset() });
        let range: Value<
            std_ranges_subrange_Counted_ptr__Counted_ptr__std_ranges_subrange_kind_sized_,
        > = Rc::new(RefCell::new(
            std_ranges_subrange_Counted_ptr__Counted_ptr__std_ranges_subrange_kind_sized_::new_1(
                { (in_.as_pointer() as Ptr<Counted>) },
                { (in_.as_pointer() as Ptr<Counted>).offset((*N.borrow()) as isize) },
            ),
        ));
        let result: Value<()> = Rc::new(RefCell::new(
            ({
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    range.as_pointer(),
                    out.as_pointer(),
                )
            }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible_v<MoveOnlyIter>");;
        let N: Value<()> = Rc::new(RefCell::new(3_()));
        const _: () = assert!(true, "std::ranges::input_range<MoveOnlyRange>");;
        let in_: Value<MoveOnlyRange_29> = Rc::new(RefCell::new(<MoveOnlyRange_29>::default()));
        {
            let out: Value<Buffer_int__3_> = Rc::new(RefCell::new(<Buffer_int__3_>::default()));
            ({
                let ___ifirst: cpp20_input_iterator_const_int_ptr_ =
                    ({ MoveOnlyRange_29Impl::begin(&in_.as_pointer()) });
                let ___ilast: sentinel_wrapper_cpp20_input_iterator_const_int_ptr__ =
                    ({ MoveOnlyRange_29Impl::end(&in_.as_pointer()) });
                let ___ofirst: Ptr<()> = ({ Buffer_int__3_Impl::begin(&out.as_pointer()) });
                let ___olast: Ptr<()> = ({ Buffer_int__3_Impl::end(&out.as_pointer()) });
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    ___ifirst, ___ilast, ___ofirst, ___olast,
                )
            });
        }
        {
            let out: Value<Buffer_int__3_> = Rc::new(RefCell::new(<Buffer_int__3_>::default()));
            ({
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    in_.as_pointer(),
                    out.as_pointer(),
                )
            });
        }
    }
    {
        let N: Value<()> = Rc::new(RefCell::new(5_()));
        {
            let pool: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                0_(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ])));
            let mut p: Ptr<()> = (pool.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
            let mut p_end: Ptr<()> = (pool.as_pointer() as Ptr<()>)
                .reinterpret_cast::<()>()
                .offset((*N.borrow()) as isize);
            let array: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
            ({
                let ___ifirst: overload_compare_iterator_int_ptr_ =
                    overload_compare_iterator_int_ptr_::new_1({ (array.as_pointer() as Ptr<()>) });
                let ___ilast: overload_compare_iterator_int_ptr_ =
                    overload_compare_iterator_int_ptr_::new_1({
                        (array.as_pointer() as Ptr<()>).offset((*N.borrow()) as isize)
                    });
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    ___ifirst,
                    ___ilast,
                    (p).clone(),
                    (p_end).clone(),
                )
            });
            let mut i: () = 0_();
            'loop_: while (i != (*N.borrow())) {
                unimplemented!();
                i.prefix_inc();
            }
        }
        {
            let pool: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                0_(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ])));
            let mut p: Ptr<()> = (pool.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
            let mut p_end: Ptr<()> = (pool.as_pointer() as Ptr<()>)
                .reinterpret_cast::<()>()
                .offset((*N.borrow()) as isize);
            let array: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
            ({
                let ___ifirst: Ptr<()> = (array.as_pointer() as Ptr<()>);
                let ___ilast: Ptr<()> =
                    (array.as_pointer() as Ptr<()>).offset((*N.borrow()) as isize);
                (*uninitialized_copy_26.with(Value::clone).borrow_mut())(
                    ___ifirst,
                    ___ilast,
                    overload_compare_iterator_int_ptr_::new_1({ (p).clone() }),
                    overload_compare_iterator_int_ptr_::new_1({ (p_end).clone() }),
                )
            });
            let mut i: () = 0_();
            'loop_: while (i != (*N.borrow())) {
                unimplemented!();
                i.prefix_inc();
            }
        }
    }
    ({ test_25() });
    return 0_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct MoveOnlyRange_29 {
    #[offset(0)]
    #[byte_size(12)]
    pub buffer: Box<[()]>,
}
impl Default for MoveOnlyRange_29 {
    fn default() -> Self {
        {
            MoveOnlyRange_29 {
                buffer: Box::new([1_(), 2_(), 3_()]),
            }
        }
    }
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct test_allocator_int_;
pub trait Buffer_Counted__1_Impl {
    fn begin(&self) -> Ptr<Counted>;
    fn end(&self) -> Ptr<Counted>;
    fn cbegin(&self) -> Ptr<Counted> {
        unimplemented!()
    }
    fn cend(&self) -> Ptr<Counted> {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl Buffer_Counted__1_Impl for Ptr<Buffer_Counted__1_> {
    fn begin(&self) -> Ptr<Counted> {
        return (array_field_ptr!((*self), buffer) as Ptr<()>).reinterpret_cast::<Counted>();
    }
    fn end(&self) -> Ptr<Counted> {
        return ({ Buffer_Counted__1_Impl::begin(self) }).offset((1_()) as isize);
    }
}
pub trait Buffer_Counted__3_Impl {
    fn begin(&self) -> Ptr<Counted>;
    fn end(&self) -> Ptr<Counted>;
    fn cbegin(&self) -> Ptr<Counted> {
        unimplemented!()
    }
    fn cend(&self) -> Ptr<Counted> {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl Buffer_Counted__3_Impl for Ptr<Buffer_Counted__3_> {
    fn begin(&self) -> Ptr<Counted> {
        return (array_field_ptr!((*self), buffer) as Ptr<()>).reinterpret_cast::<Counted>();
    }
    fn end(&self) -> Ptr<Counted> {
        return ({ Buffer_Counted__3_Impl::begin(self) }).offset((3_()) as isize);
    }
}
pub trait Buffer_Counted__5_Impl {
    fn begin(&self) -> Ptr<Counted>;
    fn end(&self) -> Ptr<Counted>;
    fn cbegin(&self) -> Ptr<Counted> {
        unimplemented!()
    }
    fn cend(&self) -> Ptr<Counted> {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl Buffer_Counted__5_Impl for Ptr<Buffer_Counted__5_> {
    fn begin(&self) -> Ptr<Counted> {
        return (array_field_ptr!((*self), buffer) as Ptr<()>).reinterpret_cast::<Counted>();
    }
    fn end(&self) -> Ptr<Counted> {
        return ({ Buffer_Counted__5_Impl::begin(self) }).offset((5_()) as isize);
    }
}
pub trait Buffer_double__3_Impl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
    fn cbegin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn cend(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl Buffer_double__3_Impl for Ptr<Buffer_double__3_> {
    fn begin(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), buffer) as Ptr<()>).reinterpret_cast::<()>();
    }
    fn end(&self) -> Ptr<()> {
        return ({ Buffer_double__3_Impl::begin(self) }).offset((3_()) as isize);
    }
}
pub trait Buffer_int__3_Impl {
    fn begin(&self) -> Ptr<()>;
    fn end(&self) -> Ptr<()>;
    fn cbegin(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn cend(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
}
impl Buffer_int__3_Impl for Ptr<Buffer_int__3_> {
    fn begin(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), buffer) as Ptr<()>).reinterpret_cast::<()>();
    }
    fn end(&self) -> Ptr<()> {
        return ({ Buffer_int__3_Impl::begin(self) }).offset((3_()) as isize);
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
pub trait CountedImpl {
    fn destructor(&self);
    fn check_throw(&self);
    fn increase_counters(&self);
}
impl CountedImpl for Ptr<Counted> {
    fn destructor(&self) {
        (*current_objects_1.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn check_throw(&self) {
        if (throw_on_5.with(|rc| *rc.borrow()) == total_objects_2.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    fn increase_counters(&self) {
        (*current_objects_1.with(Value::clone).borrow_mut()).prefix_inc();
        (*total_objects_2.with(Value::clone).borrow_mut()).prefix_inc();
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
pub trait MoveOnlyRange_29Impl {
    fn begin(&self) -> cpp20_input_iterator_const_int_ptr_;
    fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_const_int_ptr__;
}
impl MoveOnlyRange_29Impl for Ptr<MoveOnlyRange_29> {
    fn begin(&self) -> cpp20_input_iterator_const_int_ptr_ {
        return cpp20_input_iterator_const_int_ptr_::new({
            (array_field_ptr!((*self), buffer) as Ptr<()>)
        });
    }
    fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_const_int_ptr__ {
        return {
            let __tmp_0: Value<cpp20_input_iterator_const_int_ptr_> =
                Rc::new(RefCell::new(cpp20_input_iterator_const_int_ptr_::new({
                    (array_field_ptr!((*self), buffer) as Ptr<()>)
                })));
            sentinel_wrapper_cpp20_input_iterator_const_int_ptr__::new_1({ __tmp_0.as_pointer() })
        };
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
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> ();
    fn iter_swaps(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_sub_10(&self, n: ()) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> ();
    fn operator_add_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: ()) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: ()) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as () as ());
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_inc_14(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return prev;
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {}
pub trait cpp20_input_iterator_const_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_const_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_const_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_const_int_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: ()) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_const_int_ptr_Impl for Ptr<cpp20_input_iterator_const_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_const_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_const_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_const_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_int_ptr_Impl for Ptr<cpp20_input_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp20_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp20_output_iterator_int_ptr_Impl for Ptr<cpp20_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
}
pub trait forward_iterator_Counted_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<forward_iterator_Counted_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> forward_iterator_Counted_ptr_ {
        unimplemented!()
    }
}
impl forward_iterator_Counted_ptr_Impl for Ptr<forward_iterator_Counted_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<forward_iterator_Counted_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait overload_compare_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_arrow(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<overload_compare_iterator_int_ptr_>;
    fn operator_post_inc_10(&self, mut _a0: ()) -> overload_compare_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_eq(&self, other: Ptr<overload_compare_iterator_int_ptr_>) -> ();
    fn operator_ne(&self, other: Ptr<overload_compare_iterator_int_ptr_>) -> ();
}
impl overload_compare_iterator_int_ptr_Impl for Ptr<overload_compare_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_9(&self) -> Ptr<overload_compare_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_eq(&self, other: Ptr<overload_compare_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { other.with(|__s| __s.it_.clone()) });
    }
    fn operator_ne(&self, other: Ptr<overload_compare_iterator_int_ptr_>) -> () {
        return !({ overload_compare_iterator_int_ptr_Impl::operator_eq(self, (other).clone()) });
    }
}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait sentinel_wrapper_cpp20_input_iterator_const_int_ptr__Impl {
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_const_int_ptr_>) -> ();
}
impl sentinel_wrapper_cpp20_input_iterator_const_int_ptr__Impl
    for Ptr<sentinel_wrapper_cpp20_input_iterator_const_int_ptr__>
{
    fn operator_eq(&self, other: Ptr<cpp20_input_iterator_const_int_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_16((other).clone()) }) });
    }
}
pub trait sentinel_wrapper_forward_iterator_Counted_ptr__Impl {
    fn operator_eq(&self, other: Ptr<forward_iterator_Counted_ptr_>) -> ();
}
impl sentinel_wrapper_forward_iterator_Counted_ptr__Impl
    for Ptr<sentinel_wrapper_forward_iterator_Counted_ptr__>
{
    fn operator_eq(&self, other: Ptr<forward_iterator_Counted_ptr_>) -> () {
        {}
        return ({ (*self).with(|__s| __s.base_.clone()) } == { ({ base_15((other).clone()) }) });
    }
}
pub trait support_double_move_trackerImpl {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker>;
}
impl support_double_move_trackerImpl for Ptr<support_double_move_tracker> {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker> {
        unimplemented!();
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
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
        field!((*self), data_).write(destructed_value_8.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_8.with(|rc| *rc.borrow()));
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
    let _ = current_objects_1.with(|_| ());
    let _ = total_objects_2.with(|_| ());
    let _ = total_copies_3.with(|_| ());
    let _ = total_moves_4.with(|_| ());
    let _ = throw_on_5.with(|_| ());
    let _ = destructed_value_8.with(|_| ());
    let _ = moved_value_9.with(|_| ());
}
