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
    pub static destructed_value_1: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_2: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_4(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
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
        operator_eq_3(__lhs.clone(), __rhs.clone())
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
impl test_allocator_bool_ {
    pub fn new_1(a: Ptr<test_allocator_unsigned_long_>) -> Self {
        let __this: Value<test_allocator_bool_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_bool_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_bool__rebind_unsigned_long_ {}
impl CppMove for test_allocator_bool__rebind_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_bool__rebind_bool_ {}
impl CppMove for test_allocator_bool__rebind_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(test_allocator_unsigned_long_, new_1, ptr)]
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
    pub fn new_2(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
}
impl Default for test_allocator_bool_ {
    fn default() -> Self {
        {
            test_allocator_bool_ {
                data_: 0_i32,
                id_: 0_i32,
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
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn operator_eq_5(
    x: Ptr<test_allocator_unsigned_long_>,
    y: Ptr<test_allocator_unsigned_long_>,
) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(test_allocator_bool_, new_1, ptr)]
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
        ({
            cpp2rust_assert_fail_0(
                (({ a.with(|__s| __s.data_) } != { destructed_value_1.with(|rc| *rc.borrow()) })
                    && ({ a.with(|__s| __s.id_) } != {
                        destructed_value_1.with(|rc| *rc.borrow())
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
impl CppEq for test_allocator_unsigned_long_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
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
impl CppMove for test_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_unsigned_long_ {
    fn default() -> Self {
        {
            test_allocator_unsigned_long_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl other_allocator_bool_ {
    pub fn new_1(a: Ptr<other_allocator_unsigned_long_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
pub fn operator_eq_7(x: Ptr<other_allocator_bool_>, y: Ptr<other_allocator_bool_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(other_allocator_unsigned_long_, new_1, ptr)]
#[byte_size(4)]
pub struct other_allocator_bool_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_bool_ {
    pub fn new_2(mut i: i32) -> Self {
        Self { data_: i }
    }
}
impl CppEq for other_allocator_bool_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for other_allocator_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for other_allocator_bool_ {
    fn default() -> Self {
        { other_allocator_bool_ { data_: 0_i32 } }
    }
}
impl other_allocator_unsigned_long_ {
    pub fn new_1(a: Ptr<other_allocator_bool_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
pub fn operator_eq_8(
    x: Ptr<other_allocator_unsigned_long_>,
    y: Ptr<other_allocator_unsigned_long_>,
) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(other_allocator_bool_, new_1, ptr)]
#[byte_size(4)]
pub struct other_allocator_unsigned_long_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_unsigned_long_ {}
impl CppEq for other_allocator_unsigned_long_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_0(
                (alloc_count_10.with(|rc| *rc.borrow())
                    >= dealloc_count_11.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_10.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_11.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_0((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_12.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_9.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_10.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_11.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_9: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_10: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_11: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_12: Value<bool> = Rc::new(RefCell::new(false));
);
impl min_allocator_bool_ {
    pub fn new_1(mut _a0: min_allocator_unsigned_long_) -> Self {
        Self {}
    }
}
pub fn operator_eq_13(mut _a0: min_allocator_bool_, mut _a1: min_allocator_bool_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_unsigned_long_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_bool_ {}
impl CppEq for min_allocator_bool_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_unsigned_long_ {
    pub fn new_1(mut _a0: min_allocator_bool_) -> Self {
        Self {}
    }
}
pub fn operator_eq_14(
    mut _a0: min_allocator_unsigned_long_,
    mut _a1: min_allocator_unsigned_long_,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_bool_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl CppEq for min_allocator_unsigned_long_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_14(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {}
pub fn tests_15() -> bool {
    {
        let v1: Value<std_vector_bool__std_allocator_bool__> = Rc::new(RefCell::new(
            std_vector_bool__std_allocator_bool__::new_1({ 100_u64 }, None),
        ));
        let v2: Value<std_vector_bool__std_allocator_bool__> = Rc::new(RefCell::new(
            std_vector_bool__std_allocator_bool__::new_1({ 200_u64 }, None),
        ));
        ({ (*v1.borrow()).swap_50(v2.as_pointer()) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).size() }) == 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).capacity() }) >= 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).size() }) == 100_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).capacity() }) >= 100_u64)) });
    }
    {
        let v1: Value<std_vector_bool__test_allocator_bool__> = Rc::new(RefCell::new({
            let __tmp_0: Value<bool> = Rc::new(RefCell::new(true));
            let __tmp_1: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 1 }, { 1 })));
            std_vector_bool__test_allocator_bool__::new_1({ 100_u64 }, { __tmp_0.as_pointer() }, {
                __tmp_1.as_pointer()
            })
        }));
        let v2: Value<std_vector_bool__test_allocator_bool__> = Rc::new(RefCell::new({
            let __tmp_2: Value<bool> = Rc::new(RefCell::new(false));
            let __tmp_3: Value<test_allocator_bool_> =
                Rc::new(RefCell::new(test_allocator_bool_::new_2({ 1 }, { 2 })));
            std_vector_bool__test_allocator_bool__::new_1({ 200_u64 }, { __tmp_2.as_pointer() }, {
                __tmp_3.as_pointer()
            })
        }));
        ({ swap_16(v1.as_pointer(), v2.as_pointer()) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).size() }) == 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).capacity() }) >= 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).size() }) == 100_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).capacity() }) >= 100_u64)) });
        ({
            cpp2rust_assert_fail_0(
                (({
                    test_allocator_bool_Impl::get_id(
                        &Rc::new(RefCell::new(({ (*v1.borrow()).get_allocator() }))).as_pointer(),
                    )
                }) == 1),
            )
        });
        ({
            cpp2rust_assert_fail_0(
                (({
                    test_allocator_bool_Impl::get_id(
                        &Rc::new(RefCell::new(({ (*v2.borrow()).get_allocator() }))).as_pointer(),
                    )
                }) == 2),
            )
        });
    }
    {
        let v1: Value<std_vector_bool__other_allocator_bool__> = Rc::new(RefCell::new({
            let __tmp_4: Value<bool> = Rc::new(RefCell::new(true));
            let __tmp_5: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 1 })));
            std_vector_bool__other_allocator_bool__::new_1({ 100_u64 }, { __tmp_4.as_pointer() }, {
                __tmp_5.as_pointer()
            })
        }));
        let v2: Value<std_vector_bool__other_allocator_bool__> = Rc::new(RefCell::new({
            let __tmp_6: Value<bool> = Rc::new(RefCell::new(false));
            let __tmp_7: Value<other_allocator_bool_> =
                Rc::new(RefCell::new(other_allocator_bool_::new_2({ 2 })));
            std_vector_bool__other_allocator_bool__::new_1({ 200_u64 }, { __tmp_6.as_pointer() }, {
                __tmp_7.as_pointer()
            })
        }));
        ({ swap_17(v1.as_pointer(), v2.as_pointer()) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).size() }) == 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).capacity() }) >= 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).size() }) == 100_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).capacity() }) >= 100_u64)) });
        ({
            cpp2rust_assert_fail_0(
                ({
                    let _x: Value<other_allocator_bool_> =
                        Rc::new(RefCell::new(({ (*v1.borrow()).get_allocator() })));
                    let _y: Value<other_allocator_bool_> =
                        Rc::new(RefCell::new(other_allocator_bool_::new_2({ 2 })));
                    operator_eq_7(_x.as_pointer(), _y.as_pointer())
                }),
            )
        });
        ({
            cpp2rust_assert_fail_0(
                ({
                    let _x: Value<other_allocator_bool_> =
                        Rc::new(RefCell::new(({ (*v2.borrow()).get_allocator() })));
                    let _y: Value<other_allocator_bool_> =
                        Rc::new(RefCell::new(other_allocator_bool_::new_2({ 1 })));
                    operator_eq_7(_x.as_pointer(), _y.as_pointer())
                }),
            )
        });
    }
    {
        let v: Value<std_vector_bool__std_allocator_bool__> = Rc::new(RefCell::new(
            std_vector_bool__std_allocator_bool__::new_1({ 2_u64 }, None),
        ));
        let r1: Value<std__Bit_reference> = Rc::new(RefCell::new(((*v.borrow())[0_u64]).clone()));
        let r2: Value<std__Bit_reference> = Rc::new(RefCell::new(((*v.borrow())[1_u64]).clone()));
        (*r1.borrow_mut()) = true;
        ({ swap_18((*r1.borrow()).clone(), (*r2.borrow()).clone()) });
        ({ cpp2rust_assert_fail_0(((({ (*v.borrow())[0_u64]() }) as i32) == (false as i32))) });
        ({ cpp2rust_assert_fail_0(((({ (*v.borrow())[1_u64]() }) as i32) == (true as i32))) });
    }
    {
        let v1: Value<std_vector_bool__min_allocator_bool__> = Rc::new(RefCell::new(
            std_vector_bool__min_allocator_bool__::new_1({ 100_u64 }, None),
        ));
        let v2: Value<std_vector_bool__min_allocator_bool__> = Rc::new(RefCell::new(
            std_vector_bool__min_allocator_bool__::new_1({ 200_u64 }, None),
        ));
        ({ (*v1.borrow()).swap_50(v2.as_pointer()) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).size() }) == 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).capacity() }) >= 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).size() }) == 100_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).capacity() }) >= 100_u64)) });
    }
    {
        let v1: Value<std_vector_bool__min_allocator_bool__> = Rc::new(RefCell::new({
            let __tmp_8: Value<bool> = Rc::new(RefCell::new(true));
            let __tmp_9: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            std_vector_bool__min_allocator_bool__::new_2({ 100_u64 }, { __tmp_8.as_pointer() }, {
                __tmp_9.as_pointer()
            })
        }));
        let v2: Value<std_vector_bool__min_allocator_bool__> = Rc::new(RefCell::new({
            let __tmp_10: Value<bool> = Rc::new(RefCell::new(false));
            let __tmp_11: Value<min_allocator_bool_> =
                Rc::new(RefCell::new(<min_allocator_bool_>::default()));
            std_vector_bool__min_allocator_bool__::new_2({ 200_u64 }, { __tmp_10.as_pointer() }, {
                __tmp_11.as_pointer()
            })
        }));
        ({ swap_19(v1.as_pointer(), v2.as_pointer()) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).size() }) == 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v1.borrow()).capacity() }) >= 200_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).size() }) == 100_u64)) });
        ({ cpp2rust_assert_fail_0((({ (*v2.borrow()).capacity() }) >= 100_u64)) });
        ({
            cpp2rust_assert_fail_0(
                ({
                    let _arg0: min_allocator_bool_ = ({ (*v1.borrow()).get_allocator() });
                    operator_eq_13(_arg0, <min_allocator_bool_>::default())
                }),
            )
        });
        ({
            cpp2rust_assert_fail_0(
                ({
                    let _arg0: min_allocator_bool_ = ({ (*v2.borrow()).get_allocator() });
                    operator_eq_13(_arg0, <min_allocator_bool_>::default())
                }),
            )
        });
    }
    {
        let v: Value<std_vector_bool__min_allocator_bool__> = Rc::new(RefCell::new(
            std_vector_bool__min_allocator_bool__::new_1({ 2_u64 }, None),
        ));
        let r1: Value<std__Bit_reference> = Rc::new(RefCell::new(((*v.borrow())[0_u64]).clone()));
        let r2: Value<std__Bit_reference> = Rc::new(RefCell::new(((*v.borrow())[1_u64]).clone()));
        (*r1.borrow_mut()) = true;
        ({ swap_18((*r1.borrow()).clone(), (*r2.borrow()).clone()) });
        ({ cpp2rust_assert_fail_0(((({ (*v.borrow())[0_u64]() }) as i32) == (false as i32))) });
        ({ cpp2rust_assert_fail_0(((({ (*v.borrow())[1_u64]() }) as i32) == (true as i32))) });
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
    ({ tests_15() });
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
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: u64) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: u64,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: u64,
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: u64,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_unsigned_long_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({
            let ___p: Ptr<u64> = (p.ptr_).clone();
            let ___n: u64 = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub trait other_allocator_bool_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut s: u64) {
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
    fn allocate(&self, n: u64) -> Ptr<u64>;
    fn deallocate(&self, p: Ptr<u64>, s: u64);
    fn select_on_container_copy_construction(&self) -> other_allocator_unsigned_long_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_unsigned_long_Impl for Ptr<other_allocator_unsigned_long_> {
    fn allocate(&self, mut n: u64) -> Ptr<u64> {
        return ({ <std_allocator_unsigned_long_>::default().allocate(n) });
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut s: u64) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: u64 = s;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
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
    fn get_id(&self) -> i32;
}
impl test_allocator_bool_Impl for Ptr<test_allocator_bool_> {
    fn destructor(&self) {
        ({
            cpp2rust_assert_fail_0(
                ((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())),
            )
        });
        ({
            cpp2rust_assert_fail_0(
                ((*self).with(|__s| __s.id_) != destructed_value_1.with(|rc| *rc.borrow())),
            )
        });
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        ({
            cpp2rust_assert_fail_0(
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
pub trait test_allocator_unsigned_long_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<u64>) -> Ptr<u64> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<u64>) -> Ptr<u64> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<u64>;
    fn deallocate(&self, p: Ptr<u64>, s: u32);
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
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
        ({
            cpp2rust_assert_fail_0(
                ((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())),
            )
        });
        ({
            cpp2rust_assert_fail_0(
                ((*self).with(|__s| __s.id_) != destructed_value_1.with(|rc| *rc.borrow())),
            )
        });
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<u64> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        ({
            cpp2rust_assert_fail_0(
                ((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())),
            )
        });
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
                ({ __builtin_abort_6() });
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_add(n)) as i32
            });
        }
        return ({ <std_allocator_unsigned_long_>::default().allocate((n as u64)) });
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut s: u32) {
        ({
            cpp2rust_assert_fail_0(
                ((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())),
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
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: u64 = (s as u64);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
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
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
    let _ = outstanding_bytes_9.with(|_| ());
    let _ = alloc_count_10.with(|_| ());
    let _ = dealloc_count_11.with(|_| ());
    let _ = disable_default_constructor_12.with(|_| ());
}
