extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_bool__unsigned_char__signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { data_: d }
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    fn default() -> Self {
        { sized_allocator_unsigned_long__unsigned_char__signed_char_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_bool__unsigned_short__short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_short__short_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { data_: d }
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_short__short_ {
    fn default() -> Self {
        { sized_allocator_unsigned_long__unsigned_short__short_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_bool__unsigned_int__int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_int__int_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { data_: d }
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_int__int_ {
    fn default() -> Self {
        { sized_allocator_unsigned_long__unsigned_int__int_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_bool__unsigned_long__long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { data_: d }
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_long__long_ {
    fn default() -> Self {
        { sized_allocator_unsigned_long__unsigned_long__long_::new(None) }
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
    pub static destructed_value_4: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_5: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_6(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_7(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_6(_x, _y)
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
        operator_eq_6(__lhs.clone(), __rhs.clone())
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
pub struct test_allocator_bool__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_bool__rebind_bool_ {}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_bool_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_bool_ {
    pub fn new_1(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
}
impl Default for test_allocator_bool_ {
    fn default() -> Self {
        {
            test_allocator_bool_ {
                data_: <i32>::default(),
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_unsigned_long_ {
    pub fn new_1(a: Ptr<test_allocator_bool_>) -> Self {
        let __this: Value<test_allocator_unsigned_long_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_unsigned_long_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_unsigned_long_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_unsigned_long_ {
    pub fn copy_from(a: Ptr<test_allocator_unsigned_long_>) -> Self {
        let __this: Value<test_allocator_unsigned_long_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_unsigned_long_> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_4.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_4.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_unsigned_long_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_unsigned_long_> =
            Rc::new(RefCell::new(test_allocator_unsigned_long_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_unsigned_long_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_unsigned_long_ {
    fn default() -> Self {
        {
            test_allocator_unsigned_long_ {
                data_: <i32>::default(),
                id_: <i32>::default(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_bool_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_bool_ {
    pub fn new_1(mut i: i32) -> Self {
        Self { data_: i }
    }
}
impl Default for other_allocator_bool_ {
    fn default() -> Self {
        {
            other_allocator_bool_ {
                data_: <i32>::default(),
            }
        }
    }
}
impl other_allocator_unsigned_long_ {
    pub fn new_1(a: Ptr<other_allocator_bool_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_unsigned_long_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_unsigned_long_ {}
impl Default for other_allocator_unsigned_long_ {
    fn default() -> Self {
        {
            other_allocator_unsigned_long_ {
                data_: <i32>::default(),
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
pub struct limited_alloc_handle_18446744073709551615_ {
    #[offset(0)]
    pub outstanding_: usize,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl Default for limited_alloc_handle_18446744073709551615_ {
    fn default() -> Self {
        {
            limited_alloc_handle_18446744073709551615_ {
                outstanding_: 0_usize,
                last_alloc_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_302405640552615600_ {
    #[offset(0)]
    pub outstanding_: usize,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl Default for limited_alloc_handle_302405640552615600_ {
    fn default() -> Self {
        {
            limited_alloc_handle_302405640552615600_ {
                outstanding_: 0_usize,
                last_alloc_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_292805461487453200_ {
    #[offset(0)]
    pub outstanding_: usize,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl Default for limited_alloc_handle_292805461487453200_ {
    fn default() -> Self {
        {
            limited_alloc_handle_292805461487453200_ {
                outstanding_: 0_usize,
                last_alloc_: Ptr::<()>::null(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_8()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>::default(
            ),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_  >::default() )) ;
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::allocate_1 ( alloc .as_pointer()  , 1_usize  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(
        { ptr.as_pointer() },
    );
}
pub fn make_thread_unsafe_shared_9()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__>::default(),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block_  >::default() )) ;
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::allocate_1 ( alloc .as_pointer()  , 1_usize  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::copy_from({
        ptr.as_pointer()
    });
}
pub fn make_thread_unsafe_shared_10()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__>::default(),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block_  >::default() )) ;
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::allocate_1 ( alloc .as_pointer()  , 1_usize  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::copy_from({
        ptr.as_pointer()
    });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_18446744073709551615_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    #[offset(0)]
    pub ref_count: usize,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_18446744073709551615_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block { ref_count : 1_usize  , content : <limited_alloc_handle_18446744073709551615_  >::default() , }
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
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_302405640552615600_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block {
    #[offset(0)]
    pub ref_count: usize,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_302405640552615600_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block { ref_count : 1_usize  , content : <limited_alloc_handle_302405640552615600_  >::default() , }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block,
    >,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__>,
    ) -> Self {
        let __this: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
        > = Rc::new(RefCell::new(Self {
            block: other.with(|__s| __s.block.clone()),
        }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    fn clone(&self) -> Self {
        let __src: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
        > = Rc::new(RefCell::new(
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
                block: self.block.clone(),
            },
        ));
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block>::null()  , }
        }
    }
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_usize,
            content: <limited_alloc_handle_292805461487453200_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block {
    #[offset(0)]
    pub ref_count: usize,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_292805461487453200_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block { ref_count : 1_usize  , content : <limited_alloc_handle_292805461487453200_  >::default() , }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block,
    >,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__>,
    ) -> Self {
        let __this: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
        > = Rc::new(RefCell::new(Self {
            block: other.with(|__s| __s.block.clone()),
        }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    fn clone(&self) -> Self {
        let __src: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
        > = Rc::new(RefCell::new(
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
                block: self.block.clone(),
            },
        ));
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block>::null()  , }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__18446744073709551615__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__18446744073709551615__rebind_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_bool__18446744073709551615_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_bool__18446744073709551615_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__18446744073709551615_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_unsigned_long__18446744073709551615_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_8() }),
        }
    }
}
impl Default for limited_allocator_unsigned_long__18446744073709551615_ {
    fn default() -> Self {
        { limited_allocator_unsigned_long__18446744073709551615_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__302405640552615600__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__302405640552615600__rebind_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_bool__302405640552615600_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
}
impl limited_allocator_bool__302405640552615600_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__302405640552615600_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
}
impl limited_allocator_unsigned_long__302405640552615600_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_9() }),
        }
    }
}
impl Default for limited_allocator_unsigned_long__302405640552615600_ {
    fn default() -> Self {
        { limited_allocator_unsigned_long__302405640552615600_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__292805461487453200__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__292805461487453200__rebind_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_bool__292805461487453200_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
}
impl limited_allocator_bool__292805461487453200_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__292805461487453200_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
}
impl limited_allocator_unsigned_long__292805461487453200_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_10() }),
        }
    }
}
impl Default for limited_allocator_unsigned_long__292805461487453200_ {
    fn default() -> Self {
        { limited_allocator_unsigned_long__292805461487453200_::new() }
    }
}
pub fn test_11(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_12(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_13(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_14(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_15(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_16(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_17(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_18(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_19(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn test_20(v: Ptr<Vec<bool>>) {
    let mut max_dist: usize = (<i64>::MAX as usize);
    assert!(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist }));
}
pub fn tests_21() -> bool {
    {
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_22(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_22(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_11(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new({
                let __tmp_0: Value<test_allocator_bool_> =
                    Rc::new(RefCell::new(test_allocator_bool_::new_1({ 1 })));
                std_vector_bool__test_allocator_bool__::new_1({ __tmp_0.as_pointer() })
            }));
            test_12(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new({
                let __tmp_1: Value<other_allocator_bool_> =
                    Rc::new(RefCell::new(other_allocator_bool_::new_1({ 5 })));
                std_vector_bool__other_allocator_bool__::new_1({ __tmp_1.as_pointer() })
            }));
            test_13(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_14(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_15(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_16(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_17(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_18(_v.as_pointer())
        });
    }
    {
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_19(_v.as_pointer())
        });
        ({
            let _v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
            test_20(_v.as_pointer())
        });
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
    ({ tests_21() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__;
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
            != 0_usize)
        {
            return;
        };
        let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_  >::default() )) ;
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_usize  , )
        });
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
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_292805461487453200_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_dec());
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_usize)
        {
            return;
        };
        let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block_  >::default() )) ;
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_usize  , )
        });
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_302405640552615600_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_dec());
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_usize)
        {
            return;
        };
        let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block_  >::default() )) ;
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_usize  , )
        });
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait limited_alloc_handle_18446744073709551615_Impl {
    fn deallocate_2_unsigned_long(&self, ptr: Ptr<u64>, N: usize);
}
impl limited_alloc_handle_18446744073709551615_Impl
    for Ptr<limited_alloc_handle_18446744073709551615_>
{
    fn deallocate_2_unsigned_long(&self, mut ptr: Ptr<u64>, mut N: usize) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(Ptr::<()>::null());
            assert!(((*self).with(|__s| __s.outstanding_) >= N));
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        ({
            let ___p: Ptr<u64> = (ptr).clone();
            let ___n: usize = N;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub trait limited_alloc_handle_292805461487453200_Impl {
    fn deallocate_2_unsigned_long(&self, ptr: Ptr<u64>, N: usize);
}
impl limited_alloc_handle_292805461487453200_Impl
    for Ptr<limited_alloc_handle_292805461487453200_>
{
    fn deallocate_2_unsigned_long(&self, mut ptr: Ptr<u64>, mut N: usize) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(Ptr::<()>::null());
            assert!(((*self).with(|__s| __s.outstanding_) >= N));
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        ({
            let ___p: Ptr<u64> = (ptr).clone();
            let ___n: usize = N;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub trait limited_alloc_handle_302405640552615600_Impl {
    fn deallocate_2_unsigned_long(&self, ptr: Ptr<u64>, N: usize);
}
impl limited_alloc_handle_302405640552615600_Impl
    for Ptr<limited_alloc_handle_302405640552615600_>
{
    fn deallocate_2_unsigned_long(&self, mut ptr: Ptr<u64>, mut N: usize) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(Ptr::<()>::null());
            assert!(((*self).with(|__s| __s.outstanding_) >= N));
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        ({
            let ___p: Ptr<u64> = (ptr).clone();
            let ___n: usize = N;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub trait limited_allocator_bool__18446744073709551615_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize {
        unimplemented!()
    }
    fn getHandle(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_bool__18446744073709551615_Impl
    for Ptr<limited_allocator_bool__18446744073709551615_>
{
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_bool__292805461487453200_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize {
        unimplemented!()
    }
    fn getHandle(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_bool__292805461487453200_Impl
    for Ptr<limited_allocator_bool__292805461487453200_>
{
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_bool__302405640552615600_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize {
        unimplemented!()
    }
    fn getHandle(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_bool__302405640552615600_Impl
    for Ptr<limited_allocator_bool__302405640552615600_>
{
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_unsigned_long__18446744073709551615_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, n: usize);
    fn max_size(&self) -> usize;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__18446744073709551615_Impl
    for Ptr<limited_allocator_unsigned_long__18446744073709551615_>
{
    fn deallocate(&self, mut p: Ptr<u64>, mut n: usize) {
        ({
            let _ptr: Ptr<u64> = (p).clone();
            let _N: usize = n;
            limited_alloc_handle_18446744073709551615_Impl::deallocate_2_unsigned_long(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                _ptr,
                _N,
            )
        });
    }
    fn max_size(&self) -> usize {
        return (18446744073709551615_u64 as usize);
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_unsigned_long__292805461487453200_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, n: usize);
    fn max_size(&self) -> usize;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__292805461487453200_Impl
    for Ptr<limited_allocator_unsigned_long__292805461487453200_>
{
    fn deallocate(&self, mut p: Ptr<u64>, mut n: usize) {
        ({
            let _ptr: Ptr<u64> = (p).clone();
            let _N: usize = n;
            limited_alloc_handle_292805461487453200_Impl::deallocate_2_unsigned_long(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                _ptr,
                _N,
            )
        });
    }
    fn max_size(&self) -> usize {
        return (292805461487453200_u64 as usize);
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_unsigned_long__302405640552615600_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, n: usize);
    fn max_size(&self) -> usize;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__302405640552615600_Impl
    for Ptr<limited_allocator_unsigned_long__302405640552615600_>
{
    fn deallocate(&self, mut p: Ptr<u64>, mut n: usize) {
        ({
            let _ptr: Ptr<u64> = (p).clone();
            let _N: usize = n;
            limited_alloc_handle_302405640552615600_Impl::deallocate_2_unsigned_long(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                _ptr,
                _N,
            )
        });
    }
    fn max_size(&self) -> usize {
        return (302405640552615600_u64 as usize);
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({
            let ___p: Ptr<u64> = (p.ptr_).clone();
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub trait other_allocator_bool_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut s: usize) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_bool_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_bool_Impl for Ptr<other_allocator_bool_> {}
pub trait other_allocator_unsigned_long_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, s: usize);
    fn select_on_container_copy_construction(&self) -> other_allocator_unsigned_long_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_unsigned_long_Impl for Ptr<other_allocator_unsigned_long_> {
    fn deallocate(&self, mut p: Ptr<u64>, mut s: usize) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = s;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub trait sized_allocator_bool__unsigned_char__signed_char_Impl {
    fn allocate(&self, mut n: u8) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: u8) {
        unimplemented!()
    }
    fn max_size(&self) -> u8 {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_bool__unsigned_char__signed_char_>
{
}
pub trait sized_allocator_bool__unsigned_int__int_Impl {
    fn allocate(&self, mut n: u32) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_int__int_Impl
    for Ptr<sized_allocator_bool__unsigned_int__int_>
{
}
pub trait sized_allocator_bool__unsigned_long__long_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_long__long_Impl
    for Ptr<sized_allocator_bool__unsigned_long__long_>
{
}
pub trait sized_allocator_bool__unsigned_short__short_Impl {
    fn allocate(&self, mut n: u16) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: u16) {
        unimplemented!()
    }
    fn max_size(&self) -> u16 {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_short__short_Impl
    for Ptr<sized_allocator_bool__unsigned_short__short_>
{
}
pub trait sized_allocator_unsigned_long__unsigned_char__signed_char_Impl {
    fn allocate(&self, mut n: u8) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, n: u8);
    fn max_size(&self) -> u8;
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_char__signed_char_>
{
    fn deallocate(&self, mut p: Ptr<u64>, mut n: u8) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u8 {
        return (((<u8>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u8);
    }
}
pub trait sized_allocator_unsigned_long__unsigned_int__int_Impl {
    fn allocate(&self, mut n: u32) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, n: u32);
    fn max_size(&self) -> u32;
}
impl sized_allocator_unsigned_long__unsigned_int__int_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_int__int_>
{
    fn deallocate(&self, mut p: Ptr<u64>, mut n: u32) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u32 {
        return (((<u32>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u32);
    }
}
pub trait sized_allocator_unsigned_long__unsigned_long__long_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, n: u64);
    fn max_size(&self) -> u64;
}
impl sized_allocator_unsigned_long__unsigned_long__long_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_long__long_>
{
    fn deallocate(&self, mut p: Ptr<u64>, mut n: u64) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u64 {
        return (<u64>::MAX as u64).wrapping_div((::std::mem::size_of::<u64>() as u64));
    }
}
pub trait sized_allocator_unsigned_long__unsigned_short__short_Impl {
    fn allocate(&self, mut n: u16) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, n: u16);
    fn max_size(&self) -> u16;
}
impl sized_allocator_unsigned_long__unsigned_short__short_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_short__short_>
{
    fn deallocate(&self, mut p: Ptr<u64>, mut n: u16) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u16 {
        return (((<u16>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u16);
    }
}
pub trait test_allocator_bool_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<bool>) -> Ptr<bool> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<bool>) -> Ptr<bool> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<bool>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_bool_Impl for Ptr<test_allocator_bool_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_4.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_4.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
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
pub trait test_allocator_unsigned_long_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<u64>) -> Ptr<u64> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<u64>) -> Ptr<u64> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, s: u32);
    fn max_size(&self) -> u32;
    fn destroy(&self, mut p: Ptr<u64>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_unsigned_long_Impl for Ptr<test_allocator_unsigned_long_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_4.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_4.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_4.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_sub(s)) as i32
            });
        }
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (s as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<u64>() as usize))) as u32);
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
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
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
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = destructed_value_4.with(|_| ());
    let _ = moved_value_5.with(|_| ());
}
