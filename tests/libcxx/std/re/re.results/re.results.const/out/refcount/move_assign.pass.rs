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
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_4(
    x: Ptr<test_allocator_std_sub_match_const_char_ptr__>,
    y: Ptr<test_allocator_std_sub_match_const_char_ptr__>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_sub_match_const_char_ptr___rebind_std_sub_match_const_char_ptr__ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_sub_match_const_char_ptr__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_sub_match_const_char_ptr__ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_std_sub_match_const_char_ptr__>) -> Self {
        let __this: Value<test_allocator_std_sub_match_const_char_ptr__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_sub_match_const_char_ptr__> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for test_allocator_std_sub_match_const_char_ptr__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_std_sub_match_const_char_ptr__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_sub_match_const_char_ptr__> = Rc::new(RefCell::new(
            test_allocator_std_sub_match_const_char_ptr__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            },
        ));
        test_allocator_std_sub_match_const_char_ptr__::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_std_sub_match_const_char_ptr__ {
    fn default() -> Self {
        {
            test_allocator_std_sub_match_const_char_ptr__ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_5(
    x: Ptr<test_allocator_std_sub_match_const_wchar_t_ptr__>,
    y: Ptr<test_allocator_std_sub_match_const_wchar_t_ptr__>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_sub_match_const_wchar_t_ptr___rebind_std_sub_match_const_wchar_t_ptr__
{}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_sub_match_const_wchar_t_ptr__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_sub_match_const_wchar_t_ptr__ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_std_sub_match_const_wchar_t_ptr__>) -> Self {
        let __this: Value<test_allocator_std_sub_match_const_wchar_t_ptr__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_sub_match_const_wchar_t_ptr__> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for test_allocator_std_sub_match_const_wchar_t_ptr__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_std_sub_match_const_wchar_t_ptr__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_sub_match_const_wchar_t_ptr__> = Rc::new(RefCell::new(
            test_allocator_std_sub_match_const_wchar_t_ptr__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            },
        ));
        test_allocator_std_sub_match_const_wchar_t_ptr__::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_std_sub_match_const_wchar_t_ptr__ {
    fn default() -> Self {
        {
            test_allocator_std_sub_match_const_wchar_t_ptr__ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_6(
    x: Ptr<other_allocator_std_sub_match_const_char_ptr__>,
    y: Ptr<other_allocator_std_sub_match_const_char_ptr__>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_std_sub_match_const_char_ptr__ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_std_sub_match_const_char_ptr__ {
    pub fn new_1() -> Self {
        Self { data_: -1_() }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { data_: i }
    }
}
impl CppEq for other_allocator_std_sub_match_const_char_ptr__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
impl Default for other_allocator_std_sub_match_const_char_ptr__ {
    fn default() -> Self {
        { other_allocator_std_sub_match_const_char_ptr__::new_1() }
    }
}
pub fn operator_eq_7(
    x: Ptr<other_allocator_std_sub_match_const_wchar_t_ptr__>,
    y: Ptr<other_allocator_std_sub_match_const_wchar_t_ptr__>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_std_sub_match_const_wchar_t_ptr__ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_std_sub_match_const_wchar_t_ptr__ {
    pub fn new_1() -> Self {
        Self { data_: -1_() }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { data_: i }
    }
}
impl CppEq for other_allocator_std_sub_match_const_wchar_t_ptr__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl Default for other_allocator_std_sub_match_const_wchar_t_ptr__ {
    fn default() -> Self {
        { other_allocator_std_sub_match_const_wchar_t_ptr__::new_1() }
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
pub fn test_8(a: Ptr<()>) {
    let m0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_9(a: Ptr<()>) {
    let m0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_10(a: Ptr<test_allocator_std_sub_match_const_char_ptr__>) {
    let m0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_11(a: Ptr<test_allocator_std_sub_match_const_wchar_t_ptr__>) {
    let m0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_12(a: Ptr<other_allocator_std_sub_match_const_char_ptr__>) {
    let m0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub fn test_13(a: Ptr<other_allocator_std_sub_match_const_wchar_t_ptr__>) {
    let m0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
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
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_8(_a.as_pointer())
    });
    ({
        let _a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(_a.as_pointer())
    });
    ({
        let _a: Value<test_allocator_std_sub_match_const_char_ptr__> = Rc::new(RefCell::new(
            test_allocator_std_sub_match_const_char_ptr__::new_1({ 3_() }),
        ));
        test_10(_a.as_pointer())
    });
    ({
        let _a: Value<test_allocator_std_sub_match_const_wchar_t_ptr__> = Rc::new(RefCell::new(
            test_allocator_std_sub_match_const_wchar_t_ptr__::new_1({ 3_() }),
        ));
        test_11(_a.as_pointer())
    });
    ({
        let _a: Value<other_allocator_std_sub_match_const_char_ptr__> = Rc::new(RefCell::new(
            other_allocator_std_sub_match_const_char_ptr__::new_2({ 3_() }),
        ));
        test_12(_a.as_pointer())
    });
    ({
        let _a: Value<other_allocator_std_sub_match_const_wchar_t_ptr__> = Rc::new(RefCell::new(
            other_allocator_std_sub_match_const_wchar_t_ptr__::new_2({ 3_() }),
        ));
        test_13(_a.as_pointer())
    });
    return 0_();
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
pub trait other_allocator_std_sub_match_const_char_ptr__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn select_on_container_copy_construction(
        &self,
    ) -> other_allocator_std_sub_match_const_char_ptr__ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_std_sub_match_const_char_ptr__Impl
    for Ptr<other_allocator_std_sub_match_const_char_ptr__>
{
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
    }
}
pub trait other_allocator_std_sub_match_const_wchar_t_ptr__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn select_on_container_copy_construction(
        &self,
    ) -> other_allocator_std_sub_match_const_wchar_t_ptr__ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_std_sub_match_const_wchar_t_ptr__Impl
    for Ptr<other_allocator_std_sub_match_const_wchar_t_ptr__>
{
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
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
pub trait test_allocator_std_sub_match_const_char_ptr__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, p: Ptr<()>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_class_std_sub_match_const_char_ptr_(&self, p: Ptr<()>, val: Ptr<()>);
}
impl test_allocator_std_sub_match_const_char_ptr__Impl
    for Ptr<test_allocator_std_sub_match_const_char_ptr__>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((24usize as ()));
    }
    fn destroy(&self, mut p: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_class_std_sub_match_const_char_ptr_(&self, mut p: Ptr<()>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_std_sub_match_const_wchar_t_ptr__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, p: Ptr<()>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_class_std_sub_match_const_wchar_t_ptr_(&self, p: Ptr<()>, val: Ptr<()>);
}
impl test_allocator_std_sub_match_const_wchar_t_ptr__Impl
    for Ptr<test_allocator_std_sub_match_const_wchar_t_ptr__>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((24usize as ()));
    }
    fn destroy(&self, mut p: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_class_std_sub_match_const_wchar_t_ptr_(&self, mut p: Ptr<()>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
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
