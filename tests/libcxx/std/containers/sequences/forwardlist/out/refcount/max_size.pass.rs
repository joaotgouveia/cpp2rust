extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn alloc_max_size_0(a: Ptr<()>) -> () {
    return ({ std_allocator_traits_std_allocator_char__::max_size((a).clone()) });
}
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
    pub static destructed_value_1: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_2: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_4(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_3(_x, _y)
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
            operator_eq_3(
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
#[byte_size(16)]
pub struct limited_alloc_handle_10_ {
    #[offset(0)]
    pub outstanding_: (),
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: (),
}
impl Default for limited_alloc_handle_10_ {
    fn default() -> Self {
        {
            limited_alloc_handle_10_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_18446744073709551615_ {
    #[offset(0)]
    pub outstanding_: (),
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: (),
}
impl Default for limited_alloc_handle_18446744073709551615_ {
    fn default() -> Self {
        {
            limited_alloc_handle_18446744073709551615_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_5() -> detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> = Rc::new(
        RefCell::new(<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>::default()),
    );
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = unimplemented!();
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::copy_from({
        ptr.as_pointer()
    });
}
pub fn make_thread_unsafe_shared_6()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>::default(
            ),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = unimplemented!();
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(
        { ptr.as_pointer() },
    );
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_10_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    #[offset(0)]
    pub ref_count: (),
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_10_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
                ref_count: 1_(),
                content: <limited_alloc_handle_10_>::default(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block>,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>,
    ) -> Self {
        let __this: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> =
            Rc::new(RefCell::new(Self {
                block: other.with(|__s| __s.block.clone()),
            }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    fn clone(&self) -> Self {
        let __src: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> = Rc::new(
            RefCell::new(detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
                block: self.block.clone(),
            }),
        );
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::copy_from(__src.as_pointer())
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block>::null()  , }
        }
    }
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_18446744073709551615_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    #[offset(0)]
    pub ref_count: (),
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_18446744073709551615_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block { ref_count : 1_()  , content : <limited_alloc_handle_18446744073709551615_  >::default() , }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block,
    >,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>,
    ) -> Self {
        let __this: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
        > = Rc::new(RefCell::new(Self {
            block: other.with(|__s| __s.block.clone()),
        }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    fn clone(&self) -> Self {
        let __src: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
        > = Rc::new(RefCell::new(
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
                block: self.block.clone(),
            },
        ));
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block>::null()  , }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__10__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_int__10_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
}
impl limited_allocator_int__10_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__18446744073709551615__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_int__18446744073709551615_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_int__18446744073709551615_ {}
pub fn test_7() -> () {
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let mut max_dist: () = unimplemented!();
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let mut max_dist: () = unimplemented!();
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    unimplemented!();
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
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_10_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_dec());
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_())
        {
            return;
        };
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        unimplemented!();
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_10_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_18446744073709551615_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_dec());
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_())
        {
            return;
        };
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        unimplemented!();
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait limited_allocator_int__10_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn getHandle(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__10_Impl for Ptr<limited_allocator_int__10_> {
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_int__18446744073709551615_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn getHandle(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__18446744073709551615_Impl
    for Ptr<limited_allocator_int__18446744073709551615_>
{
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
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
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
}
