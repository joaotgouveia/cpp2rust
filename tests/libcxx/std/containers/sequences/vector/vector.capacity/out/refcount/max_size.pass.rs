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
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_4(
                (alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_4((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_1: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_2: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_int_ {}
impl CppMove for min_allocator_unsigned_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_int__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_int__unsigned_char__signed_char_ {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { data_: d }
    }
}
impl CppMove for sized_allocator_int__unsigned_char__signed_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for sized_allocator_int__unsigned_char__signed_char_ {
    fn default() -> Self {
        { sized_allocator_int__unsigned_char__signed_char_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_int__unsigned_short__short_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_int__unsigned_short__short_ {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { data_: d }
    }
}
impl CppMove for sized_allocator_int__unsigned_short__short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for sized_allocator_int__unsigned_short__short_ {
    fn default() -> Self {
        { sized_allocator_int__unsigned_short__short_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_int__unsigned_int__int_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_int__unsigned_int__int_ {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { data_: d }
    }
}
impl CppMove for sized_allocator_int__unsigned_int__int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for sized_allocator_int__unsigned_int__int_ {
    fn default() -> Self {
        { sized_allocator_int__unsigned_int__int_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_int__unsigned_long__long_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_int__unsigned_long__long_ {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { data_: d }
    }
}
impl CppMove for sized_allocator_int__unsigned_long__long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for sized_allocator_int__unsigned_long__long_ {
    fn default() -> Self {
        { sized_allocator_int__unsigned_long__long_::new(None) }
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
    pub static destructed_value_5: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_6: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_7(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_7(_x, _y)
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
        operator_eq_7(__lhs.clone(), __rhs.clone())
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
pub struct test_allocator_char__rebind_char_ {}
impl CppMove for test_allocator_char__rebind_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_char_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_char_ {
    pub fn new_1(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_char_>) -> Self {
        let __this: Value<test_allocator_char_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_char_> = __this.as_pointer();
        ({
            cpp2rust_assert_fail_4(
                (({ a.with(|__s| __s.data_) } != { destructed_value_5.with(|rc| *rc.borrow()) })
                    && ({ a.with(|__s| __s.id_) } != {
                        destructed_value_5.with(|rc| *rc.borrow())
                    }))
                    && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                        .is_null()),
            )
        });
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_char_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_char_> = Rc::new(RefCell::new(test_allocator_char_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_char_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_char_ {
    fn default() -> Self {
        {
            test_allocator_char_ {
                data_: 0_i32,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_unsigned_long_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_unsigned_long_ {
    pub fn new_1(mut i: i32) -> Self {
        Self { data_: i }
    }
}
impl CppMove for other_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for other_allocator_unsigned_long_ {
    fn default() -> Self {
        { other_allocator_unsigned_long_ { data_: 0_i32 } }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_10_ {
    #[offset(0)]
    pub outstanding_: u64,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl CppMove for limited_alloc_handle_10_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for limited_alloc_handle_10_ {
    fn default() -> Self {
        {
            limited_alloc_handle_10_ {
                outstanding_: 0_u64,
                last_alloc_: AnyPtr::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_18446744073709551615_ {
    #[offset(0)]
    pub outstanding_: u64,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl CppMove for limited_alloc_handle_18446744073709551615_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for limited_alloc_handle_18446744073709551615_ {
    fn default() -> Self {
        {
            limited_alloc_handle_18446744073709551615_ {
                outstanding_: 0_u64,
                last_alloc_: AnyPtr::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_9223372036854775807_ {
    #[offset(0)]
    pub outstanding_: u64,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl CppMove for limited_alloc_handle_9223372036854775807_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for limited_alloc_handle_9223372036854775807_ {
    fn default() -> Self {
        {
            limited_alloc_handle_9223372036854775807_ {
                outstanding_: 0_u64,
                last_alloc_: AnyPtr::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_4611686018427387903_ {
    #[offset(0)]
    pub outstanding_: u64,
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: AnyPtr,
}
impl CppMove for limited_alloc_handle_4611686018427387903_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for limited_alloc_handle_4611686018427387903_ {
    fn default() -> Self {
        {
            limited_alloc_handle_4611686018427387903_ {
                outstanding_: 0_u64,
                last_alloc_: AnyPtr::default(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_9() -> detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> = Rc::new(
        RefCell::new(<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>::default()),
    );
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block_  >::default() )) ;
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::allocate_1 ( alloc .as_pointer()  , 1_u64  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::copy_from({
        ptr.as_pointer()
    });
}
pub fn make_thread_unsafe_shared_10()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>::default(
            ),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_  >::default() )) ;
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::allocate_1 ( alloc .as_pointer()  , 1_u64  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(
        { ptr.as_pointer() },
    );
}
pub fn make_thread_unsafe_shared_11()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__>::default(),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block_  >::default() )) ;
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block__::allocate_1 ( alloc .as_pointer()  , 1_u64  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__::copy_from(
        { ptr.as_pointer() },
    );
}
pub fn make_thread_unsafe_shared_12()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__>::default(),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block_  >::default() )) ;
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block__::allocate_1 ( alloc .as_pointer()  , 1_u64  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__::copy_from(
        { ptr.as_pointer() },
    );
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_u64,
            content: <limited_alloc_handle_10_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    #[offset(0)]
    pub ref_count: u64,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_10_,
}
impl CppMove for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block {
                ref_count: 1_u64,
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
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| {
            __v.prefix_inc();
        });
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
impl CppMove for detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
            ref_count: 1_u64,
            content: <limited_alloc_handle_18446744073709551615_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block {
    #[offset(0)]
    pub ref_count: u64,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_18446744073709551615_,
}
impl CppMove
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block { ref_count : 1_u64  , content : <limited_alloc_handle_18446744073709551615_  >::default() , }
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
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| {
            __v.prefix_inc();
        });
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
impl CppMove for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block>::null()  , }
        }
    }
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_u64,
            content: <limited_alloc_handle_9223372036854775807_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block {
    #[offset(0)]
    pub ref_count: u64,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_9223372036854775807_,
}
impl CppMove
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block { ref_count : 1_u64  , content : <limited_alloc_handle_9223372036854775807_  >::default() , }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block,
    >,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__>,
    ) -> Self {
        let __this: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__,
        > = Rc::new(RefCell::new(Self {
            block: other.with(|__s| __s.block.clone()),
        }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| {
            __v.prefix_inc();
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__ {
    fn clone(&self) -> Self {
        let __src: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__,
        > = Rc::new(RefCell::new(
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__ {
                block: self.block.clone(),
            },
        ));
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl CppMove for detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block>::null()  , }
        }
    }
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_u64,
            content: <limited_alloc_handle_4611686018427387903_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block {
    #[offset(0)]
    pub ref_count: u64,
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_4611686018427387903_,
}
impl CppMove
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block { ref_count : 1_u64  , content : <limited_alloc_handle_4611686018427387903_  >::default() , }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block,
    >,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__>,
    ) -> Self {
        let __this: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__,
        > = Rc::new(RefCell::new(Self {
            block: other.with(|__s| __s.block.clone()),
        }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| {
            __v.prefix_inc();
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__ {
    fn clone(&self) -> Self {
        let __src: Value<
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__,
        > = Rc::new(RefCell::new(
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__ {
                block: self.block.clone(),
            },
        ));
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl CppMove for detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block>::null()  , }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__10__rebind_int_ {}
impl CppMove for limited_allocator_int__10__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_int__10_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
}
impl limited_allocator_int__10_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_9() }),
        }
    }
}
impl CppMove for limited_allocator_int__10_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for limited_allocator_int__10_ {
    fn default() -> Self {
        { limited_allocator_int__10_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__18446744073709551615__rebind_int_ {}
impl CppMove for limited_allocator_int__18446744073709551615__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_int__18446744073709551615_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_int__18446744073709551615_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_10() }),
        }
    }
}
impl CppMove for limited_allocator_int__18446744073709551615_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for limited_allocator_int__18446744073709551615_ {
    fn default() -> Self {
        { limited_allocator_int__18446744073709551615_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__9223372036854775807__rebind_int_ {}
impl CppMove for limited_allocator_int__9223372036854775807__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_int__9223372036854775807_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__,
}
impl limited_allocator_int__9223372036854775807_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_11() }),
        }
    }
}
impl CppMove for limited_allocator_int__9223372036854775807_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for limited_allocator_int__9223372036854775807_ {
    fn default() -> Self {
        { limited_allocator_int__9223372036854775807_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_int__4611686018427387903__rebind_int_ {}
impl CppMove for limited_allocator_int__4611686018427387903__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_int__4611686018427387903_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__,
}
impl limited_allocator_int__4611686018427387903_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_12() }),
        }
    }
}
impl CppMove for limited_allocator_int__4611686018427387903_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for limited_allocator_int__4611686018427387903_ {
    fn default() -> Self {
        { limited_allocator_int__4611686018427387903_::new() }
    }
}
pub fn test_13(v: Ptr<std_vector_unsigned_int__min_allocator_unsigned_int__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = ({
        let ___a: Value<min_allocator_unsigned_int_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_unsigned_int__min_allocator_unsigned_int__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_min_allocator_unsigned_int__::max_size(___a.as_pointer())
    });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_14(v: Ptr<std_vector_char__test_allocator_char__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = (({
        let ___a: Value<test_allocator_char_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref()) as std__Vector_base_char__test_allocator_char__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_test_allocator_char__::max_size(___a.as_pointer())
    }) as u64);
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_15(v: Ptr<std_vector_unsigned_long__other_allocator_unsigned_long__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = ({
        let ___a: Value<other_allocator_unsigned_long_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_unsigned_long__other_allocator_unsigned_long__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_other_allocator_unsigned_long__::max_size(___a.as_pointer())
    });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_16(v: Ptr<std_vector_int__sized_allocator_int__unsigned_char__signed_char__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = (({
        let ___a: Value<sized_allocator_int__unsigned_char__signed_char_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_int__sized_allocator_int__unsigned_char__signed_char__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_sized_allocator_int__unsigned_char__signed_char__::max_size(
            ___a.as_pointer(),
        )
    }) as u64);
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_17(v: Ptr<std_vector_int__sized_allocator_int__unsigned_short__short__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = (({
        let ___a: Value<sized_allocator_int__unsigned_short__short_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_int__sized_allocator_int__unsigned_short__short__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_sized_allocator_int__unsigned_short__short__::max_size(
            ___a.as_pointer(),
        )
    }) as u64);
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_18(v: Ptr<std_vector_int__sized_allocator_int__unsigned_int__int__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = (({
        let ___a: Value<sized_allocator_int__unsigned_int__int_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_int__sized_allocator_int__unsigned_int__int__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_sized_allocator_int__unsigned_int__int__::max_size(___a.as_pointer())
    }) as u64);
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_19(v: Ptr<std_vector_int__sized_allocator_int__unsigned_long__long__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = ({
        let ___a: Value<sized_allocator_int__unsigned_long__long_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_int__sized_allocator_int__unsigned_long__long__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_sized_allocator_int__unsigned_long__long__::max_size(___a.as_pointer())
    });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_20(v: Ptr<std_vector_int__limited_allocator_int__18446744073709551615__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = ({
        let ___a: Value<limited_allocator_int__18446744073709551615_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_int__limited_allocator_int__18446744073709551615__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_limited_allocator_int__18446744073709551615__::max_size(
            ___a.as_pointer(),
        )
    });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_21(v: Ptr<std_vector_int__limited_allocator_int__9223372036854775807__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = ({
        let ___a: Value<limited_allocator_int__9223372036854775807_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_int__limited_allocator_int__9223372036854775807__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_limited_allocator_int__9223372036854775807__::max_size(
            ___a.as_pointer(),
        )
    });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn test_22(v: Ptr<std_vector_int__limited_allocator_int__4611686018427387903__>) {
    let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
    let mut max_alloc: u64 = ({
        let ___a: Value<limited_allocator_int__4611686018427387903_> = Rc::new(RefCell::new(
            ({
                ((*v.upgrade().deref())
                    as std__Vector_base_int__limited_allocator_int__4611686018427387903__)
                    .get_allocator()
            }),
        ));
        std_allocator_traits_limited_allocator_int__4611686018427387903__::max_size(
            ___a.as_pointer(),
        )
    });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_dist })) });
    ({ cpp2rust_assert_fail_4(({ ({ (*v.upgrade().deref()).max_size() }) } <= { max_alloc })) });
    const _: () = assert!(true, "");;
}
pub fn tests_23() -> bool {
    {
        let c: Value<std_vector_int__limited_allocator_int__10__> = Rc::new(RefCell::new(
            <std_vector_int__limited_allocator_int__10__>::default(),
        ));
        ({ cpp2rust_assert_fail_4((({ (*c.borrow()).max_size() }) <= 10_u64)) });
        const _: () = assert!(true, "");;
    }
    {
        let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
        let c: Value<std_vector_int__limited_allocator_int__18446744073709551615__> =
            Rc::new(RefCell::new(
                <std_vector_int__limited_allocator_int__18446744073709551615__>::default(),
            ));
        ({ cpp2rust_assert_fail_4((({ (*c.borrow()).max_size() }) <= max_dist)) });
        const _: () = assert!(true, "");;
    }
    {
        let mut max_dist: u64 = (({ std_numeric_limits_long_::max() }) as u64);
        let c: Value<std_vector_char__std_allocator_char__> = Rc::new(RefCell::new(
            <std_vector_char__std_allocator_char__>::default(),
        ));
        ({ cpp2rust_assert_fail_4((({ (*c.borrow()).max_size() }) <= max_dist)) });
        ({
            cpp2rust_assert_fail_4(
                (({ (*c.borrow()).max_size() })
                    <= ({
                        let _a: Value<std_allocator_char_> = Rc::new(RefCell::new(
                            ({
                                ((*c.borrow()) as std__Vector_base_char__std_allocator_char__)
                                    .get_allocator()
                            }),
                        ));
                        alloc_max_size_24(_a.as_pointer())
                    })),
            )
        });
        const _: () = assert!(true, "");;
    }
    {
        ({
            let _v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
                <std_vector_int__std_allocator_int__>::default(),
            ));
            test_25(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_short__std_allocator_short__> = Rc::new(RefCell::new(
                <std_vector_short__std_allocator_short__>::default(),
            ));
            test_26(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_unsigned_int__min_allocator_unsigned_int__> = Rc::new(
                RefCell::new(<std_vector_unsigned_int__min_allocator_unsigned_int__>::default()),
            );
            test_13(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_char__test_allocator_char__> = Rc::new(RefCell::new({
                let __tmp_0: Value<test_allocator_char_> =
                    Rc::new(RefCell::new(test_allocator_char_::new_1({ 1 })));
                std_vector_char__test_allocator_char__::new_1({ __tmp_0.as_pointer() })
            }));
            test_14(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_unsigned_long__other_allocator_unsigned_long__> =
                Rc::new(RefCell::new({
                    let __tmp_1: Value<other_allocator_unsigned_long_> =
                        Rc::new(RefCell::new(other_allocator_unsigned_long_::new_1({ 5 })));
                    std_vector_unsigned_long__other_allocator_unsigned_long__::new_1({
                        __tmp_1.as_pointer()
                    })
                }));
            test_15(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_int__sized_allocator_int__unsigned_char__signed_char__> =
                Rc::new(RefCell::new(
                    <std_vector_int__sized_allocator_int__unsigned_char__signed_char__>::default(),
                ));
            test_16(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_int__sized_allocator_int__unsigned_short__short__> =
                Rc::new(RefCell::new(
                    <std_vector_int__sized_allocator_int__unsigned_short__short__>::default(),
                ));
            test_17(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_int__sized_allocator_int__unsigned_int__int__> = Rc::new(
                RefCell::new(<std_vector_int__sized_allocator_int__unsigned_int__int__>::default()),
            );
            test_18(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_int__sized_allocator_int__unsigned_long__long__> =
                Rc::new(RefCell::new(
                    <std_vector_int__sized_allocator_int__unsigned_long__long__>::default(),
                ));
            test_19(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_int__limited_allocator_int__18446744073709551615__> =
                Rc::new(RefCell::new(
                    <std_vector_int__limited_allocator_int__18446744073709551615__>::default(),
                ));
            test_20(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_int__limited_allocator_int__9223372036854775807__> =
                Rc::new(RefCell::new(
                    <std_vector_int__limited_allocator_int__9223372036854775807__>::default(),
                ));
            test_21(_v.as_pointer())
        });
        ({
            let _v: Value<std_vector_int__limited_allocator_int__4611686018427387903__> =
                Rc::new(RefCell::new(
                    <std_vector_int__limited_allocator_int__4611686018427387903__>::default(),
                ));
            test_22(_v.as_pointer())
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
    ({ tests_23() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_unsigned_int__std_integral_constant_unsigned_long__0__;
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
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| {
            __v.prefix_dec();
        });
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_u64)
        {
            return;
        };
        let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block_  >::default() )) ;
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_u64  , )
        });
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
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| {
            __v.prefix_dec();
        });
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_u64)
        {
            return;
        };
        let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block_  >::default() )) ;
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_u64  , )
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
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_4611686018427387903_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_4611686018427387903_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_4611686018427387903_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_4611686018427387903_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_4611686018427387903_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_4611686018427387903_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| {
            __v.prefix_dec();
        });
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_u64)
        {
            return;
        };
        let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block_  >::default() )) ;
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_u64  , )
        });
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_4611686018427387903_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_9223372036854775807_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_9223372036854775807_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_9223372036854775807_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_9223372036854775807_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_9223372036854775807_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_9223372036854775807_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__>
{
    fn destructor(&self) {
        field!((*self).with(|__s| __s.block.clone()), ref_count).with_mut(|__v| {
            __v.prefix_dec();
        });
        if ((*self)
            .with(|__s| __s.block.clone())
            .with(|__s| __s.ref_count)
            != 0_u64)
        {
            return;
        };
        let alloc : Value<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block_ > = Rc::new(RefCell::new(<std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block_  >::default() )) ;
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_u64  , )
        });
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_9223372036854775807_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait limited_alloc_handle_10_Impl {
    fn deallocate_2_int(&self, ptr: Ptr<i32>, N: u64);
}
impl limited_alloc_handle_10_Impl for Ptr<limited_alloc_handle_10_> {
    fn deallocate_2_int(&self, mut ptr: Ptr<i32>, mut N: u64) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(AnyPtr::default());
            ({ cpp2rust_assert_fail_4(((*self).with(|__s| __s.outstanding_) >= N)) });
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        ({ <std_allocator_int_>::default().deallocate((ptr).clone(), N) });
    }
}
pub trait limited_alloc_handle_18446744073709551615_Impl {
    fn deallocate_2_int(&self, ptr: Ptr<i32>, N: u64);
}
impl limited_alloc_handle_18446744073709551615_Impl
    for Ptr<limited_alloc_handle_18446744073709551615_>
{
    fn deallocate_2_int(&self, mut ptr: Ptr<i32>, mut N: u64) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(AnyPtr::default());
            ({ cpp2rust_assert_fail_4(((*self).with(|__s| __s.outstanding_) >= N)) });
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        ({ <std_allocator_int_>::default().deallocate((ptr).clone(), N) });
    }
}
pub trait limited_alloc_handle_4611686018427387903_Impl {
    fn deallocate_2_int(&self, ptr: Ptr<i32>, N: u64);
}
impl limited_alloc_handle_4611686018427387903_Impl
    for Ptr<limited_alloc_handle_4611686018427387903_>
{
    fn deallocate_2_int(&self, mut ptr: Ptr<i32>, mut N: u64) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(AnyPtr::default());
            ({ cpp2rust_assert_fail_4(((*self).with(|__s| __s.outstanding_) >= N)) });
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        ({ <std_allocator_int_>::default().deallocate((ptr).clone(), N) });
    }
}
pub trait limited_alloc_handle_9223372036854775807_Impl {
    fn deallocate_2_int(&self, ptr: Ptr<i32>, N: u64);
}
impl limited_alloc_handle_9223372036854775807_Impl
    for Ptr<limited_alloc_handle_9223372036854775807_>
{
    fn deallocate_2_int(&self, mut ptr: Ptr<i32>, mut N: u64) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(AnyPtr::default());
            ({ cpp2rust_assert_fail_4(((*self).with(|__s| __s.outstanding_) >= N)) });
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        ({ <std_allocator_int_>::default().deallocate((ptr).clone(), N) });
    }
}
pub trait limited_allocator_int__10_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: u64);
    fn max_size(&self) -> u64;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__10_Impl for Ptr<limited_allocator_int__10_> {
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u64) {
        ({
            limited_alloc_handle_10_Impl::deallocate_2_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> u64 {
        return 10_u64;
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_int__18446744073709551615_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: u64);
    fn max_size(&self) -> u64;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__18446744073709551615_Impl
    for Ptr<limited_allocator_int__18446744073709551615_>
{
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u64) {
        ({
            limited_alloc_handle_18446744073709551615_Impl::deallocate_2_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> u64 {
        return 18446744073709551615_u64;
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_int__4611686018427387903_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: u64);
    fn max_size(&self) -> u64;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_4611686018427387903_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__4611686018427387903_Impl
    for Ptr<limited_allocator_int__4611686018427387903_>
{
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u64) {
        ({
            limited_alloc_handle_4611686018427387903_Impl::deallocate_2_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_4611686018427387903__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> u64 {
        return 4611686018427387903_u64;
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_int__9223372036854775807_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: u64);
    fn max_size(&self) -> u64;
    fn getHandle(&self) -> Ptr<limited_alloc_handle_9223372036854775807_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_int__9223372036854775807_Impl
    for Ptr<limited_allocator_int__9223372036854775807_>
{
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u64) {
        ({
            limited_alloc_handle_9223372036854775807_Impl::deallocate_2_int(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_9223372036854775807__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> u64 {
        return 9223372036854775807_u64;
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait min_allocator_unsigned_int_Impl {
    fn allocate(
        &self,
        mut n: u64,
    ) -> min_pointer_unsigned_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        p: min_pointer_unsigned_int__std_integral_constant_unsigned_long__0__,
        n: u64,
    );
}
impl min_allocator_unsigned_int_Impl for Ptr<min_allocator_unsigned_int_> {
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_int__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({ <std_allocator_unsigned_int_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait other_allocator_unsigned_long_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<u64>, s: u64);
    fn select_on_container_copy_construction(&self) -> other_allocator_unsigned_long_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_unsigned_long_Impl for Ptr<other_allocator_unsigned_long_> {
    fn deallocate(&self, mut p: Ptr<u64>, mut s: u64) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: u64 = s;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub trait sized_allocator_int__unsigned_char__signed_char_Impl {
    fn allocate(&self, mut n: u8) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: u8);
    fn max_size(&self) -> u8;
}
impl sized_allocator_int__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_int__unsigned_char__signed_char_>
{
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u8) {
        ({ <std_allocator_int_>::default().deallocate((p).clone(), (n as u64)) });
    }
    fn max_size(&self) -> u8 {
        return (((({ std_numeric_limits_unsigned_char_::max() }) as u64)
            .wrapping_div((::std::mem::size_of::<i32>() as u64))) as u8);
    }
}
pub trait sized_allocator_int__unsigned_int__int_Impl {
    fn allocate(&self, mut n: u32) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: u32);
    fn max_size(&self) -> u32;
}
impl sized_allocator_int__unsigned_int__int_Impl for Ptr<sized_allocator_int__unsigned_int__int_> {
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u32) {
        ({ <std_allocator_int_>::default().deallocate((p).clone(), (n as u64)) });
    }
    fn max_size(&self) -> u32 {
        return (((({ std_numeric_limits_unsigned_int_::max() }) as u64)
            .wrapping_div((::std::mem::size_of::<i32>() as u64))) as u32);
    }
}
pub trait sized_allocator_int__unsigned_long__long_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: u64);
    fn max_size(&self) -> u64;
}
impl sized_allocator_int__unsigned_long__long_Impl
    for Ptr<sized_allocator_int__unsigned_long__long_>
{
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u64) {
        ({ <std_allocator_int_>::default().deallocate((p).clone(), (n as u64)) });
    }
    fn max_size(&self) -> u64 {
        return (({ std_numeric_limits_unsigned_long_::max() }) as u64)
            .wrapping_div((::std::mem::size_of::<i32>() as u64));
    }
}
pub trait sized_allocator_int__unsigned_short__short_Impl {
    fn allocate(&self, mut n: u16) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i32>, n: u16);
    fn max_size(&self) -> u16;
}
impl sized_allocator_int__unsigned_short__short_Impl
    for Ptr<sized_allocator_int__unsigned_short__short_>
{
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u16) {
        ({ <std_allocator_int_>::default().deallocate((p).clone(), (n as u64)) });
    }
    fn max_size(&self) -> u16 {
        return (((({ std_numeric_limits_unsigned_short_::max() }) as u64)
            .wrapping_div((::std::mem::size_of::<i32>() as u64))) as u16);
    }
}
pub trait test_allocator_char_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<i8>) -> Ptr<i8> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<i8> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<i8>, s: u32);
    fn max_size(&self) -> u32;
    fn destroy(&self, p: Ptr<i8>);
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_char_Impl for Ptr<test_allocator_char_> {
    fn destructor(&self) {
        ({
            cpp2rust_assert_fail_4(
                ((*self).with(|__s| __s.data_) != destructed_value_5.with(|rc| *rc.borrow())),
            )
        });
        ({
            cpp2rust_assert_fail_4(
                ((*self).with(|__s| __s.id_) != destructed_value_5.with(|rc| *rc.borrow())),
            )
        });
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_5.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_5.with(|rc| *rc.borrow()));
    }
    fn deallocate(&self, mut p: Ptr<i8>, mut s: u32) {
        ({
            cpp2rust_assert_fail_4(
                ((*self).with(|__s| __s.data_) != destructed_value_5.with(|rc| *rc.borrow())),
            )
        });
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_dec();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_sub(s)) as i32
            });
        }
        ({ <std_allocator_char_>::default().deallocate((p).clone(), (s as u64)) });
    }
    fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as u64)
            .wrapping_div((::std::mem::size_of::<i8>() as u64))) as u32);
    }
    fn destroy(&self, mut p: Ptr<i8>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count).with_mut(|__v| {
                __v.prefix_inc();
            });
        };
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        ({
            cpp2rust_assert_fail_4(
                ((*self).with(|__s| __s.count) == 0)
                    && (!(Ptr::<i8>::from_string_literal(b"clearing leaking allocator data?"))
                        .is_null()),
            )
        });
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
        field!((*self), data_).write(destructed_value_5.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_5.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_5.with(|_| ());
    let _ = moved_value_6.with(|_| ());
}
