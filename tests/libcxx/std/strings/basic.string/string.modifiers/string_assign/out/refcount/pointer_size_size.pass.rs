extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<()>) -> Ptr<()> {
    b"";
    return (value).clone();
}
pub fn is_string_asan_correct_1(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_2(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_3(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_string_asan_correct_4(_a0: Ptr<()>) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_6.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_7.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_8.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_5.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_7.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_7: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_8: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_char_ {}
impl safe_allocator_char_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_char_ {
    fn default() -> Self {
        { safe_allocator_char_::new() }
    }
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
    pub static destructed_value_10: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_11: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_12(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_13(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_12(_x, _y)
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
            operator_eq_12(
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
pub struct limited_alloc_handle_33_ {
    #[offset(0)]
    pub outstanding_: (),
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: (),
}
impl Default for limited_alloc_handle_33_ {
    fn default() -> Self {
        {
            limited_alloc_handle_33_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_14() -> detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__> = Rc::new(
        RefCell::new(<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__>::default()),
    );
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block__::allocate ( alloc .as_pointer()  , 1_()  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__::copy_from({
        ptr.as_pointer()
    });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_33_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block {
    #[offset(0)]
    pub ref_count: (),
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_33_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block {
                ref_count: 1_(),
                content: <limited_alloc_handle_33_>::default(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__ {
    #[offset(0)]
    #[byte_size(8)]
    block: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block>,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__ {
    pub fn copy_from(
        other: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__>,
    ) -> Self {
        let __this: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__> =
            Rc::new(RefCell::new(Self {
                block: other.with(|__s| __s.block.clone()),
            }));
        let this: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__> =
            __this.as_pointer();
        field!(this.with(|__s| __s.block.clone()), ref_count).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__ {
    fn clone(&self) -> Self {
        let __src: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__> = Rc::new(
            RefCell::new(detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__ {
                block: self.block.clone(),
            }),
        );
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__::copy_from(__src.as_pointer())
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__ {
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__ { block : Ptr::<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block>::null()  , }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_char__33__rebind_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_char__33_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__,
}
impl limited_allocator_char__33_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_14() }),
        }
    }
}
impl Default for limited_allocator_char__33_ {
    fn default() -> Self {
        { limited_allocator_char__33_::new() }
    }
}
pub fn test_15(s: (), mut str: Ptr<()>, mut pos: (), mut n: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let mut str_len: () = unimplemented!();
    if (pos <= str_len) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_16(s: (), mut str: Ptr<()>, mut pos: (), mut n: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let mut str_len: () = unimplemented!();
    if (pos <= str_len) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_17(s: (), mut str: Ptr<()>, mut pos: (), mut n: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let mut str_len: () = unimplemented!();
    if (pos <= str_len) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_18(s: (), mut str: Ptr<()>, mut pos: (), mut n: (), mut expected: ()) {
    let s: Value<()> = Rc::new(RefCell::new(s));
    let mut str_len: () = unimplemented!();
    if (pos <= str_len) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_string_19() {
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            0_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"123") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"2345") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            3_(),
            15_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"45") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            5_(),
            15_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            6_(),
            15_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"2") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"345") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            12_(),
            13_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"34567890") },
                None,
            ),
        )
    });
    ({
        test_15(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            21_(),
            13_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"12345"),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"34") },
                None,
            ),
        )
    });
    ({
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"1234567890"),
            0_(),
            100_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"12345"),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"234") },
                None,
            ),
        )
    });
    ({
        test_15 ( std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { Ptr::<() >::from_string_literal(b"12345678901234567890" )  } , None , )   , Ptr::<() >::from_string_literal(b"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890" )  , 5_()  , 10_()  , std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { Ptr::<() >::from_string_literal(b"6789012345" )  } , None , )   , )
    });
}
pub fn test_string_20() {
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            0_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"123") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"2345") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            3_(),
            15_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"45") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            5_(),
            15_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            6_(),
            15_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"2") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"345") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            12_(),
            13_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"34567890") },
                None,
            ),
        )
    });
    ({
        test_16(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            21_(),
            13_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"12345"),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"34") },
                None,
            ),
        )
    });
    ({
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"1234567890"),
            0_(),
            100_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"12345"),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"234") },
                None,
            ),
        )
    });
    ({
        test_16 ( std_basic_string_char__std_char_traits_char___min_allocator_char__ :: new_1 ( { Ptr::<() >::from_string_literal(b"12345678901234567890" )  } , None , )   , Ptr::<() >::from_string_literal(b"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890" )  , 5_()  , 10_()  , std_basic_string_char__std_char_traits_char___min_allocator_char__ :: new_1 ( { Ptr::<() >::from_string_literal(b"6789012345" )  } , None , )   , )
    });
}
pub fn test_string_21() {
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            0_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"123") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"2345") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            3_(),
            15_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"45") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            5_(),
            15_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            6_(),
            15_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"2") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"345") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            12_(),
            13_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"34567890") },
                None,
            ),
        )
    });
    ({
        test_17(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            21_(),
            13_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"12345"),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"34") },
                None,
            ),
        )
    });
    ({
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"1234567890"),
            0_(),
            100_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"12345"),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"234") },
                None,
            ),
        )
    });
    ({
        test_17 ( std_basic_string_char__std_char_traits_char___safe_allocator_char__ :: new_1 ( { Ptr::<() >::from_string_literal(b"12345678901234567890" )  } , None , )   , Ptr::<() >::from_string_literal(b"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890" )  , 5_()  , 10_()  , std_basic_string_char__std_char_traits_char___safe_allocator_char__ :: new_1 ( { Ptr::<() >::from_string_literal(b"6789012345" )  } , None , )   , )
    });
}
pub fn test_string_22() {
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            0_(),
            3_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"123") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            1_(),
            4_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"2345") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            3_(),
            15_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"45") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            5_(),
            15_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345"),
            6_(),
            15_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            1_(),
            1_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"2") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            2_(),
            3_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"345") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            12_(),
            13_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"34567890") },
                None,
            ),
        )
    });
    ({
        test_18(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"12345678901234567890"),
            21_(),
            13_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"not happening") },
                None,
            ),
        )
    });
    ({
        test_18(
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_18(
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"12345"),
            2_(),
            2_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"34") },
                None,
            ),
        )
    });
    ({
        test_18(
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"12345") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"1234567890"),
            0_(),
            100_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"1234567890") },
                None,
            ),
        )
    });
    ({
        test_18(
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        test_18(
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"12345678901234567890") },
                None,
            ),
            Ptr::<()>::from_string_literal(b"12345"),
            1_(),
            3_(),
            std_basic_string_char__std_char_traits_char___limited_allocator_char__33__::new_1(
                { Ptr::<()>::from_string_literal(b"234") },
                None,
            ),
        )
    });
    ({
        test_18 ( std_basic_string_char__std_char_traits_char___limited_allocator_char__33__ :: new_1 ( { Ptr::<() >::from_string_literal(b"12345678901234567890" )  } , None , )   , Ptr::<() >::from_string_literal(b"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890" )  , 5_()  , 10_()  , std_basic_string_char__std_char_traits_char___limited_allocator_char__33__ :: new_1 ( { Ptr::<() >::from_string_literal(b"6789012345" )  } , None , )   , )
    });
}
pub fn test_23() -> () {
    ({ test_string_19() });
    ({ test_string_20() });
    ({ test_string_21() });
    ({ test_string_22() });
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
    ({ test_23() });
    const _: () = assert!(true, "test()");;
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
pub trait detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__Impl {
    fn destructor(&self);
    fn operator_deref_4(&self) -> Ptr<limited_alloc_handle_33_> {
        unimplemented!()
    }
    fn operator_arrow_5(&self) -> Ptr<limited_alloc_handle_33_> {
        unimplemented!()
    }
    fn operator_deref_6(&self) -> Ptr<limited_alloc_handle_33_> {
        unimplemented!()
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_33_>;
    fn get_8(&self) -> Ptr<limited_alloc_handle_33_> {
        unimplemented!()
    }
    fn get_9(&self) -> Ptr<limited_alloc_handle_33_> {
        unimplemented!()
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__>;
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__Impl
    for Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__>
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
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_33___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_()  , )
        });
    }
    fn operator_arrow_7(&self) -> Ptr<limited_alloc_handle_33_> {
        return (field_ptr!((*self).with(|__s| __s.block.clone()), content));
    }
    fn copy_assign(
        &self,
        _a0: Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__>,
    ) -> Ptr<detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__> {
        field!((*self), block).write({ _a0.with(|__s| __s.block.clone()) });
        return (*self).clone();
    }
}
pub trait limited_alloc_handle_33_Impl {
    fn allocate_1_char(&self, N: ()) -> Ptr<()>;
    fn deallocate_2_char(&self, ptr: Ptr<()>, N: ());
}
impl limited_alloc_handle_33_Impl for Ptr<limited_alloc_handle_33_> {
    fn allocate_1_char(&self, mut N: ()) -> Ptr<()> {
        if ((N).wrapping_add((*self).with(|__s| __s.outstanding_)) > 33_()) {
            unimplemented!();
        }
        let alloc: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        field!((*self), last_alloc_).write((*alloc.borrow()).to_any());
        field!((*self), outstanding_)
            .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_add(N) });
        return (*alloc.borrow()).clone();
    }
    fn deallocate_2_char(&self, mut ptr: Ptr<()>, mut N: ()) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(unimplemented!());
            unimplemented!();
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
}
pub trait limited_allocator_char__33_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
    fn getHandle(&self) -> Ptr<limited_alloc_handle_33_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_char__33_Impl for Ptr<limited_allocator_char__33_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        return ({
            limited_alloc_handle_33_Impl::allocate_1_char(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                n,
            )
        });
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        ({
            limited_alloc_handle_33_Impl::deallocate_2_char(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_33__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                (p).clone(),
                n,
            )
        });
    }
    fn max_size(&self) -> () {
        return 33_();
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait min_allocator_char_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char_Impl for Ptr<min_allocator_char_> {
    fn allocate(&self, mut n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait safe_allocator_char_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
}
impl safe_allocator_char_Impl for Ptr<safe_allocator_char_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        let memory: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                memset_9(
                    (*memory.borrow()).to_any(),
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        {
            ({
                let _value: Value<()> = Rc::new(RefCell::new(
                    ({
                        memset_9(
                            (p).to_any(),
                            0_(),
                            (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                        )
                    }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
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
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_10.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_10.with(|rc| *rc.borrow()));
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
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
    let _ = destructed_value_10.with(|_| ());
    let _ = moved_value_11.with(|_| ());
}
