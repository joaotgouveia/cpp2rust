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
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_bool__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_4(
    mut x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_5(
    mut x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_4(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<()>::null(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
impl min_allocator_bool_ {
    pub fn new_1(mut _a0: min_allocator_unsigned_long_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {
    pub fn new_1(mut _a0: min_allocator_bool_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {}
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub fn new_1(a: Ptr<sized_allocator_unsigned_char__unsigned_char__signed_char_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_bool__unsigned_char__signed_char_ {}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_char__signed_char_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub fn new_2(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 0_());
        Self { data_: d }
    }
}
impl Default for sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    fn default() -> Self {
        { sized_allocator_unsigned_char__unsigned_char__signed_char_::new_2(None) }
    }
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub fn new_1(a: Ptr<sized_allocator_unsigned_short__unsigned_short__short_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_bool__unsigned_short__short_ {}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_short__short_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_short__unsigned_short__short_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub fn new_2(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 0_());
        Self { data_: d }
    }
}
impl Default for sized_allocator_unsigned_short__unsigned_short__short_ {
    fn default() -> Self {
        { sized_allocator_unsigned_short__unsigned_short__short_::new_2(None) }
    }
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub fn new_1(a: Ptr<sized_allocator_unsigned_int__unsigned_int__int_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_bool__unsigned_int__int_ {}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_int__int_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_int__unsigned_int__int_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub fn new_2(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 0_());
        Self { data_: d }
    }
}
impl Default for sized_allocator_unsigned_int__unsigned_int__int_ {
    fn default() -> Self {
        { sized_allocator_unsigned_int__unsigned_int__int_::new_2(None) }
    }
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub fn new_1(a: Ptr<sized_allocator_unsigned_long__unsigned_long__long_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_bool__unsigned_long__long_ {}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_long__long_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub fn new_2(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 0_());
        Self { data_: d }
    }
}
impl Default for sized_allocator_unsigned_long__unsigned_long__long_ {
    fn default() -> Self {
        { sized_allocator_unsigned_long__unsigned_long__long_::new_2(None) }
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
    pub static destructed_value_6: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_7: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_8(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_9(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_8(_x, _y)
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
            operator_eq_8(
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
impl test_allocator_bool_ {
    pub fn new_1(a: Ptr<test_allocator_unsigned_int_>) -> Self {
        let __this: Value<test_allocator_bool_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_bool_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_bool__rebind_unsigned_int_ {}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_bool_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_bool_ {
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
}
impl Default for test_allocator_bool_ {
    fn default() -> Self {
        {
            test_allocator_bool_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_unsigned_int_ {
    pub fn new_1(a: Ptr<test_allocator_bool_>) -> Self {
        let __this: Value<test_allocator_unsigned_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_unsigned_int_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_unsigned_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_unsigned_int_ {}
impl Default for test_allocator_unsigned_int_ {
    fn default() -> Self {
        {
            test_allocator_unsigned_int_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_bool_ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_bool_ {
    pub fn new_2(mut i: ()) -> Self {
        Self { data_: i }
    }
}
impl Default for other_allocator_bool_ {
    fn default() -> Self {
        {
            other_allocator_bool_ {
                data_: unimplemented!(),
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
    data_: (),
}
impl other_allocator_unsigned_long_ {}
impl Default for other_allocator_unsigned_long_ {
    fn default() -> Self {
        {
            other_allocator_unsigned_long_ {
                data_: unimplemented!(),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_302405640552615600_ {
    #[offset(0)]
    pub outstanding_: (),
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: (),
}
impl Default for limited_alloc_handle_302405640552615600_ {
    fn default() -> Self {
        {
            limited_alloc_handle_302405640552615600_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct limited_alloc_handle_292805461487453200_ {
    #[offset(0)]
    pub outstanding_: (),
    #[offset(8)]
    #[byte_size(8)]
    pub last_alloc_: (),
}
impl Default for limited_alloc_handle_292805461487453200_ {
    fn default() -> Self {
        {
            limited_alloc_handle_292805461487453200_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
pub fn make_thread_unsafe_shared_10() -> detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__> = Rc::new(
        RefCell::new(<detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__>::default()),
    );
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::allocate ( alloc .as_pointer()  , 1_()  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__::copy_from({
        ptr.as_pointer()
    });
}
pub fn make_thread_unsafe_shared_11()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__>::default(
            ),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::allocate ( alloc .as_pointer()  , 1_()  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__::copy_from(
        { ptr.as_pointer() },
    );
}
pub fn make_thread_unsafe_shared_12()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__>::default(),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::allocate ( alloc .as_pointer()  , 1_()  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::copy_from({
        ptr.as_pointer()
    });
}
pub fn make_thread_unsafe_shared_13()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__ {
    let ptr: Value<detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__> =
        Rc::new(RefCell::new(
            <detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__>::default(),
        ));
    let _dtor_ptr = ScopedDestructor::new(&ptr, |__p| __p.destructor());
    let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    (*ptr.borrow_mut()).block = ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::allocate ( alloc .as_pointer()  , 1_()  , )
    });
    ({
        std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::construct ( alloc .as_pointer()  , { (*ptr.borrow()) . block .clone() }  , )
    });
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::copy_from({
        ptr.as_pointer()
    });
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
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block {
    pub fn new() -> Self {
        Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_302405640552615600_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block {
    #[offset(0)]
    pub ref_count: (),
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_302405640552615600_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block { ref_count : 1_()  , content : <limited_alloc_handle_302405640552615600_  >::default() , }
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
            ref_count: 1_(),
            content: <limited_alloc_handle_292805461487453200_>::default(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block {
    #[offset(0)]
    pub ref_count: (),
    #[offset(8)]
    #[byte_size(16)]
    pub content: limited_alloc_handle_292805461487453200_,
}
impl Default
    for detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block
{
    fn default() -> Self {
        {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block { ref_count : 1_()  , content : <limited_alloc_handle_292805461487453200_  >::default() , }
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
pub struct limited_allocator_bool__10__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_bool__10_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
}
impl limited_allocator_bool__10_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__10_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__,
}
impl limited_allocator_unsigned_long__10_ {
    pub fn new() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_10() }),
        }
    }
}
impl Default for limited_allocator_unsigned_long__10_ {
    fn default() -> Self {
        { limited_allocator_unsigned_long__10_::new() }
    }
}
impl limited_allocator_bool__18446744073709551615_ {
    pub fn new_1(other: Ptr<limited_allocator_unsigned_long__18446744073709551615_>) -> Self {
        Self { handle_ : detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ :: copy_from ( { field_ptr!(other , handle_)  } , )   , }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__18446744073709551615__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_bool__18446744073709551615_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_bool__18446744073709551615_ {}
impl limited_allocator_unsigned_long__18446744073709551615_ {
    pub fn new_1(other: Ptr<limited_allocator_bool__18446744073709551615_>) -> Self {
        Self { handle_ : detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__ :: copy_from ( { field_ptr!(other , handle_)  } , )   , }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__18446744073709551615_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__,
}
impl limited_allocator_unsigned_long__18446744073709551615_ {
    pub fn new_2() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_11() }),
        }
    }
}
impl Default for limited_allocator_unsigned_long__18446744073709551615_ {
    fn default() -> Self {
        { limited_allocator_unsigned_long__18446744073709551615_::new_2() }
    }
}
impl limited_allocator_bool__302405640552615600_ {
    pub fn new_1(other: Ptr<limited_allocator_unsigned_long__302405640552615600_>) -> Self {
        Self {
            handle_:
                detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::copy_from(
                    { field_ptr!(other, handle_) },
                ),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__302405640552615600__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_bool__302405640552615600_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
}
impl limited_allocator_bool__302405640552615600_ {}
impl limited_allocator_unsigned_long__302405640552615600_ {
    pub fn new_1(other: Ptr<limited_allocator_bool__302405640552615600_>) -> Self {
        Self {
            handle_:
                detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__::copy_from(
                    { field_ptr!(other, handle_) },
                ),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__302405640552615600_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__,
}
impl limited_allocator_unsigned_long__302405640552615600_ {
    pub fn new_2() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_12() }),
        }
    }
}
impl Default for limited_allocator_unsigned_long__302405640552615600_ {
    fn default() -> Self {
        { limited_allocator_unsigned_long__302405640552615600_::new_2() }
    }
}
impl limited_allocator_bool__292805461487453200_ {
    pub fn new_1(other: Ptr<limited_allocator_unsigned_long__292805461487453200_>) -> Self {
        Self {
            handle_:
                detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::copy_from(
                    { field_ptr!(other, handle_) },
                ),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct limited_allocator_bool__292805461487453200__rebind_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct limited_allocator_bool__292805461487453200_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
}
impl limited_allocator_bool__292805461487453200_ {}
impl limited_allocator_unsigned_long__292805461487453200_ {
    pub fn new_1(other: Ptr<limited_allocator_bool__292805461487453200_>) -> Self {
        Self {
            handle_:
                detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__::copy_from(
                    { field_ptr!(other, handle_) },
                ),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct limited_allocator_unsigned_long__292805461487453200_ {
    #[offset(0)]
    #[byte_size(8)]
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__,
}
impl limited_allocator_unsigned_long__292805461487453200_ {
    pub fn new_2() -> Self {
        Self {
            handle_: ({ make_thread_unsafe_shared_13() }),
        }
    }
}
impl Default for limited_allocator_unsigned_long__292805461487453200_ {
    fn default() -> Self {
        { limited_allocator_unsigned_long__292805461487453200_::new_2() }
    }
}
pub fn test_14(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_long_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let _arg0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        std_allocator_traits_std_allocator_unsigned_long__::max_size(_arg0.as_pointer())
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_15(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_long_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let _arg0: Value<min_allocator_unsigned_long_> =
            Rc::new(RefCell::new(min_allocator_unsigned_long_::new_1({
                unimplemented!()
            })));
        std_allocator_traits_min_allocator_unsigned_long__::max_size(_arg0.as_pointer())
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_16(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_int_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let ___a: Value<test_allocator_unsigned_int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<test_allocator_bool_> = Rc::new(RefCell::new(unimplemented!()));
            test_allocator_unsigned_int_::new_1({ __tmp_0.as_pointer() })
        }));
        std_allocator_traits_test_allocator_unsigned_int__::max_size(___a.as_pointer())
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_17(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_long_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let _arg0: Value<other_allocator_unsigned_long_> = Rc::new(RefCell::new({
            let __tmp_1: Value<other_allocator_bool_> = Rc::new(RefCell::new(unimplemented!()));
            other_allocator_unsigned_long_::new_1({ __tmp_1.as_pointer() })
        }));
        std_allocator_traits_other_allocator_unsigned_long__::max_size(_arg0.as_pointer())
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_18(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_signed_char_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let ___a: Value<sized_allocator_unsigned_char__unsigned_char__signed_char_> =
            Rc::new(RefCell::new({
                let __tmp_2: Value<sized_allocator_bool__unsigned_char__signed_char_> =
                    Rc::new(RefCell::new(unimplemented!()));
                sized_allocator_unsigned_char__unsigned_char__signed_char_::new_1({
                    __tmp_2.as_pointer()
                })
            }));
        std_allocator_traits_sized_allocator_unsigned_char__unsigned_char__signed_char__::max_size(
            ___a.as_pointer(),
        )
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_19(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_short_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let ___a: Value<sized_allocator_unsigned_short__unsigned_short__short_> =
            Rc::new(RefCell::new({
                let __tmp_3: Value<sized_allocator_bool__unsigned_short__short_> =
                    Rc::new(RefCell::new(unimplemented!()));
                sized_allocator_unsigned_short__unsigned_short__short_::new_1({
                    __tmp_3.as_pointer()
                })
            }));
        std_allocator_traits_sized_allocator_unsigned_short__unsigned_short__short__::max_size(
            ___a.as_pointer(),
        )
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_20(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_int_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let ___a: Value<sized_allocator_unsigned_int__unsigned_int__int_> =
            Rc::new(RefCell::new({
                let __tmp_4: Value<sized_allocator_bool__unsigned_int__int_> =
                    Rc::new(RefCell::new(unimplemented!()));
                sized_allocator_unsigned_int__unsigned_int__int_::new_1({ __tmp_4.as_pointer() })
            }));
        std_allocator_traits_sized_allocator_unsigned_int__unsigned_int__int__::max_size(
            ___a.as_pointer(),
        )
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_21(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_long_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let ___a: Value<sized_allocator_unsigned_long__unsigned_long__long_> =
            Rc::new(RefCell::new({
                let __tmp_5: Value<sized_allocator_bool__unsigned_long__long_> =
                    Rc::new(RefCell::new(unimplemented!()));
                sized_allocator_unsigned_long__unsigned_long__long_::new_1({ __tmp_5.as_pointer() })
            }));
        std_allocator_traits_sized_allocator_unsigned_long__unsigned_long__long__::max_size(
            ___a.as_pointer(),
        )
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_22(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_long_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let ___a: Value<limited_allocator_unsigned_long__18446744073709551615_> =
            Rc::new(RefCell::new({
                let __tmp_6: Value<limited_allocator_bool__18446744073709551615_> =
                    Rc::new(RefCell::new(unimplemented!()));
                limited_allocator_unsigned_long__18446744073709551615_::new_1({
                    __tmp_6.as_pointer()
                })
            }));
        std_allocator_traits_limited_allocator_unsigned_long__18446744073709551615__::max_size(
            ___a.as_pointer(),
        )
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_23(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_long_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let ___a: Value<limited_allocator_unsigned_long__302405640552615600_> =
            Rc::new(RefCell::new({
                let __tmp_7: Value<limited_allocator_bool__302405640552615600_> =
                    Rc::new(RefCell::new(unimplemented!()));
                limited_allocator_unsigned_long__302405640552615600_::new_1({
                    __tmp_7.as_pointer()
                })
            }));
        std_allocator_traits_limited_allocator_unsigned_long__302405640552615600__::max_size(
            ___a.as_pointer(),
        )
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn test_24(v: Ptr<()>) {
    let mut max_dist: () = ({ std_numeric_limits_long_::max() });
    unimplemented!();
    let mut max_alloc: () = ({
        let ___a: Value<limited_allocator_unsigned_long__292805461487453200_> =
            Rc::new(RefCell::new({
                let __tmp_8: Value<limited_allocator_bool__292805461487453200_> =
                    Rc::new(RefCell::new(unimplemented!()));
                limited_allocator_unsigned_long__292805461487453200_::new_1({
                    __tmp_8.as_pointer()
                })
            }));
        std_allocator_traits_limited_allocator_unsigned_long__292805461487453200__::max_size(
            ___a.as_pointer(),
        )
    });
    let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let mut max_size: () = if ((max_dist).wrapping_div(bits_per_word) < max_alloc) {
        max_dist
    } else {
        (max_alloc).wrapping_mul(bits_per_word)
    };
    unimplemented!();
    unimplemented!();
}
pub fn tests_25() -> () {
    {
        let mut v: () = unimplemented!();
        let mut bits_per_word: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
        unimplemented!();
    }
    {
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_14(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_14(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_15(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_16(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_17(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_18(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_19(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_20(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_21(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_22(_v.as_pointer())
        });
    }
    {
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_23(_v.as_pointer())
        });
        ({
            let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_24(_v.as_pointer())
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ tests_25() });
    const _: () = assert!(true, "tests()");;
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
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_10___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_()  , )
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
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_()  , )
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
            != 0_())
        {
            return;
        };
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_()  , )
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
            != 0_())
        {
            return;
        };
        let alloc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::destroy ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , )
        });
        ({
            std_allocator_traits_std_allocator_detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600___control_block__::deallocate ( alloc .as_pointer()  , (* self ) .with(|__s| __s . block .clone())  , 1_()  , )
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
pub trait limited_alloc_handle_10_Impl {
    fn deallocate_2_unsigned_long(&self, ptr: Ptr<()>, N: ());
}
impl limited_alloc_handle_10_Impl for Ptr<limited_alloc_handle_10_> {
    fn deallocate_2_unsigned_long(&self, mut ptr: Ptr<()>, mut N: ()) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(unimplemented!());
            unimplemented!();
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
}
pub trait limited_alloc_handle_18446744073709551615_Impl {
    fn deallocate_2_unsigned_long(&self, ptr: Ptr<()>, N: ());
}
impl limited_alloc_handle_18446744073709551615_Impl
    for Ptr<limited_alloc_handle_18446744073709551615_>
{
    fn deallocate_2_unsigned_long(&self, mut ptr: Ptr<()>, mut N: ()) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(unimplemented!());
            unimplemented!();
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
}
pub trait limited_alloc_handle_292805461487453200_Impl {
    fn deallocate_2_unsigned_long(&self, ptr: Ptr<()>, N: ());
}
impl limited_alloc_handle_292805461487453200_Impl
    for Ptr<limited_alloc_handle_292805461487453200_>
{
    fn deallocate_2_unsigned_long(&self, mut ptr: Ptr<()>, mut N: ()) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(unimplemented!());
            unimplemented!();
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
}
pub trait limited_alloc_handle_302405640552615600_Impl {
    fn deallocate_2_unsigned_long(&self, ptr: Ptr<()>, N: ());
}
impl limited_alloc_handle_302405640552615600_Impl
    for Ptr<limited_alloc_handle_302405640552615600_>
{
    fn deallocate_2_unsigned_long(&self, mut ptr: Ptr<()>, mut N: ()) {
        if ({ (ptr).to_any() } == { (*self).with(|__s| __s.last_alloc_.clone()) }) {
            field!((*self), last_alloc_).write(unimplemented!());
            unimplemented!();
            field!((*self), outstanding_)
                .write({ ((*self).with(|__s| __s.outstanding_)).wrapping_sub(N) });
        }
        unimplemented!();
    }
}
pub trait limited_allocator_bool__10_Impl {
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
impl limited_allocator_bool__10_Impl for Ptr<limited_allocator_bool__10_> {
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_bool__18446744073709551615_Impl {
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
impl limited_allocator_bool__18446744073709551615_Impl
    for Ptr<limited_allocator_bool__18446744073709551615_>
{
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_bool__292805461487453200_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
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
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
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
pub trait limited_allocator_unsigned_long__10_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
    fn getHandle(&self) -> Ptr<limited_alloc_handle_10_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__10_Impl for Ptr<limited_allocator_unsigned_long__10_> {
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        ({
            let _ptr: Ptr<()> = (p).clone();
            let _N: () = n;
            limited_alloc_handle_10_Impl::deallocate_2_unsigned_long(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_10__Impl::operator_arrow_7(
                        &field_ptr!((*self), handle_),
                    )
                }),
                _ptr,
                _N,
            )
        });
    }
    fn max_size(&self) -> () {
        return 10_();
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_unsigned_long__18446744073709551615_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
    fn getHandle(&self) -> Ptr<limited_alloc_handle_18446744073709551615_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__18446744073709551615_Impl
    for Ptr<limited_allocator_unsigned_long__18446744073709551615_>
{
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        ({
            let _ptr: Ptr<()> = (p).clone();
            let _N: () = n;
            limited_alloc_handle_18446744073709551615_Impl::deallocate_2_unsigned_long(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_18446744073709551615__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                _ptr,
                _N,
            )
        });
    }
    fn max_size(&self) -> () {
        return 18446744073709551615_();
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_unsigned_long__292805461487453200_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
    fn getHandle(&self) -> Ptr<limited_alloc_handle_292805461487453200_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__292805461487453200_Impl
    for Ptr<limited_allocator_unsigned_long__292805461487453200_>
{
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        ({
            let _ptr: Ptr<()> = (p).clone();
            let _N: () = n;
            limited_alloc_handle_292805461487453200_Impl::deallocate_2_unsigned_long(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_292805461487453200__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                _ptr,
                _N,
            )
        });
    }
    fn max_size(&self) -> () {
        return 292805461487453200_();
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait limited_allocator_unsigned_long__302405640552615600_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
    fn getHandle(&self) -> Ptr<limited_alloc_handle_302405640552615600_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl limited_allocator_unsigned_long__302405640552615600_Impl
    for Ptr<limited_allocator_unsigned_long__302405640552615600_>
{
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        ({
            let _ptr: Ptr<()> = (p).clone();
            let _N: () = n;
            limited_alloc_handle_302405640552615600_Impl::deallocate_2_unsigned_long(
                &({
                    detail_thread_unsafe_shared_ptr_limited_alloc_handle_302405640552615600__Impl :: operator_arrow_7 ( &field_ptr!((* self )  , handle_)  , )
                }),
                _ptr,
                _N,
            )
        });
    }
    fn max_size(&self) -> () {
        return 302405640552615600_();
    }
    fn destructor(&self) {
        field_ptr!(self, handle_).destructor();
    }
}
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_bool__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_bool__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_bool__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>
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
pub trait other_allocator_bool_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_bool_ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_bool_Impl for Ptr<other_allocator_bool_> {}
pub trait other_allocator_unsigned_long_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn select_on_container_copy_construction(&self) -> other_allocator_unsigned_long_ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_unsigned_long_Impl for Ptr<other_allocator_unsigned_long_> {
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
    }
}
pub trait sized_allocator_bool__unsigned_char__signed_char_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_bool__unsigned_char__signed_char_>
{
}
pub trait sized_allocator_bool__unsigned_int__int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_int__int_Impl
    for Ptr<sized_allocator_bool__unsigned_int__int_>
{
}
pub trait sized_allocator_bool__unsigned_long__long_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_long__long_Impl
    for Ptr<sized_allocator_bool__unsigned_long__long_>
{
}
pub trait sized_allocator_bool__unsigned_short__short_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_short__short_Impl
    for Ptr<sized_allocator_bool__unsigned_short__short_>
{
}
pub trait sized_allocator_unsigned_char__unsigned_char__signed_char_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_unsigned_char__unsigned_char__signed_char_>
{
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return ({ std_numeric_limits_unsigned_char_::max() })
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait sized_allocator_unsigned_int__unsigned_int__int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl sized_allocator_unsigned_int__unsigned_int__int_Impl
    for Ptr<sized_allocator_unsigned_int__unsigned_int__int_>
{
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return ({ std_numeric_limits_unsigned_int_::max() })
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait sized_allocator_unsigned_long__unsigned_long__long_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl sized_allocator_unsigned_long__unsigned_long__long_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_long__long_>
{
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (({ std_numeric_limits_unsigned_long_::max() }) as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait sized_allocator_unsigned_short__unsigned_short__short_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl sized_allocator_unsigned_short__unsigned_short__short_Impl
    for Ptr<sized_allocator_unsigned_short__unsigned_short__short_>
{
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return ({ std_numeric_limits_unsigned_short_::max() })
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait test_allocator_bool_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_bool_Impl for Ptr<test_allocator_bool_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_6.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_6.with(|rc| *rc.borrow()));
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
pub trait test_allocator_unsigned_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> ();
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_unsigned_int_Impl for Ptr<test_allocator_unsigned_int_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_6.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_6.with(|rc| *rc.borrow()));
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
            .wrapping_div((::std::mem::size_of::<()>() as ()));
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
        field!((*self), data_).write(destructed_value_6.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_6.with(|rc| *rc.borrow()));
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
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = destructed_value_6.with(|_| ());
    let _ = moved_value_7.with(|_| ());
}
