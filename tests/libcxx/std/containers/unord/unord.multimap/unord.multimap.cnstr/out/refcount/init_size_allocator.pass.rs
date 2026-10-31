extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn CheckConsecutiveKeys_0(pos: (), end: (), mut key: (), values: Ptr<()>) {
    let pos: Value<()> = Rc::new(RefCell::new(pos));
    let end: Value<()> = Rc::new(RefCell::new(end));
    'loop_: while !(unimplemented!()) {
        assert!((*pos.borrow()) != (*end.borrow()));
        assert!(((*(*(*pos.borrow()).iter_deref().upgrade().deref()).0.borrow()) == key));
        assert!(unimplemented!() != unimplemented!());
        unimplemented!();
        (*pos.borrow_mut()).iter_inc();
    }
    assert!(
        ((*pos.borrow()) == (*end.borrow()))
            || ((*(*(*pos.borrow()).iter_deref().upgrade().deref()).0.borrow()) != key)
    );
}
pub fn operator_eq_1(a: Ptr<test_equal_to_int_>, b: Ptr<test_equal_to_int_>) -> bool {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_equal_to_int_ {
    #[offset(0)]
    pub data_: i32,
}
impl test_equal_to_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0 }
    }
}
impl CppEq for test_equal_to_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Callable2<Ptr<i32>, Ptr<i32>, bool> for test_equal_to_int_ {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            test_equal_to_int_Impl::operator_call(__it, a1, a2)
        })
    }
}
impl Default for test_equal_to_int_ {
    fn default() -> Self {
        { test_equal_to_int_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_hash_int_ {
    #[offset(0)]
    data_: i32,
}
impl test_hash_int_ {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 0);
        Self { data_: data }
    }
}
impl CppEq for test_hash_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        test_hash_int_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Callable1<Ptr<i32>, usize> for test_hash_int_ {
    fn call(&self, a1: Ptr<i32>) -> usize {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            test_hash_int_Impl::operator_call(__it, a1)
        })
    }
}
impl Default for test_hash_int_ {
    fn default() -> Self {
        { test_hash_int_::new(None) }
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
    pub static destructed_value_2: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_3: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_4(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_5(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_4(_x, _y)
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
        operator_eq_4(__lhs.clone(), __rhs.clone())
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
pub fn operator_eq_6(
    x : Ptr< test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >,
    y : Ptr< test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >,
) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__std_basic_string_char____rebind_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { pub fn new_1 ( mut data : i32  , ) -> Self { Self { data_ : data  , id_ : 0  , stats_ : Ptr::<test_allocator_statistics>::null()  , }  } }
impl CppEq for test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_6(__lhs.clone(), __rhs.clone()) } }
impl Default for test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn default() -> Self { { test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { data_ : 0_i32 , id_ : 0  , stats_ : Ptr::<test_allocator_statistics>::null()  , } } } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
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
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_8.with(|rc| *rc.borrow()) >= dealloc_count_9.with(|rc| *rc.borrow())));
        return ((alloc_count_8.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_9.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_10.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_7.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_8.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_9.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_7: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_8: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_9: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_10: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<(Value<i32>, Value<Vec<i8>>)>,
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_11(
    mut _a0 : min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
    mut _a1 : min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
impl CppEq for min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_11(__lhs.read(), __rhs.read()) } }
pub fn operator_eq_12(
    mut _a0 : explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
    mut _a1 : explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
impl explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { pub fn new ( ) -> Self { Self { }  } }
impl CppEq for explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_12(__lhs.read(), __rhs.read()) } }
impl Default for explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn default() -> Self { { explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ :: new ( )  } } }
pub fn test_13(
    alloc : Ptr< test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >,
) {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    assert!((unimplemented!() == 6_()));
    let eq: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new({
        let __a0 = unimplemented!();
        (
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a0.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a0.1.as_pointer(),
            ))),
        )
    }));
    assert!((unimplemented!() == 2_isize));
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    ({
        let _pos: () = <()>::from(unimplemented!());
        let _end: () = <()>::from(unimplemented!());
        CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
    });
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 2_isize));
    unimplemented!();
    unimplemented!();
    ({
        let _pos: () = <()>::from(unimplemented!());
        let _end: () = <()>::from(unimplemented!());
        CheckConsecutiveKeys_0(_pos, _end, (2 as ()), s.as_pointer())
    });
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 1_isize));
    let i: Value<()> = Rc::new(RefCell::new((*(*eq.borrow()).0.borrow()).clone()));
    assert!(((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
    assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
        (*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*i.borrow())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 1_isize));
    (*i.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
    assert!(((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
    assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
        (*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*i.borrow())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
    assert!((unimplemented!() < 1.1920929e-07_f32));
    assert!((unimplemented!() == 1_f32));
    assert!(
        ({
            let _c: Value<test_hash_int_> = Rc::new(RefCell::new(test_hash_int_::new(None)));
            test_hash_int_Impl::operator_eq(
                &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                _c.as_pointer(),
            )
        })
    );
    assert!(
        ({
            let _a: Value<test_equal_to_int_> = Rc::new(RefCell::new(unimplemented!()));
            let _b: Value<test_equal_to_int_> = Rc::new(RefCell::new(test_equal_to_int_::new_1()));
            operator_eq_1(_a.as_pointer(), _b.as_pointer())
        })
    );
    assert!(
        ({
            let _x : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(unimplemented!()  ));
            let _y: Ptr< test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >  = (alloc ).clone() ;
            operator_eq_6(_x.as_pointer(), _y)
        })
    );
}
pub fn test_14(
    alloc : Ptr< min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >,
) {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    assert!((unimplemented!() == 6_()));
    let eq: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new({
        let __a0 = unimplemented!();
        (
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a0.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a0.1.as_pointer(),
            ))),
        )
    }));
    assert!((unimplemented!() == 2_isize));
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    ({
        let _pos: () = <()>::from(unimplemented!());
        let _end: () = <()>::from(unimplemented!());
        CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
    });
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 2_isize));
    unimplemented!();
    unimplemented!();
    ({
        let _pos: () = <()>::from(unimplemented!());
        let _end: () = <()>::from(unimplemented!());
        CheckConsecutiveKeys_0(_pos, _end, (2 as ()), s.as_pointer())
    });
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 1_isize));
    let i: Value<()> = Rc::new(RefCell::new((*(*eq.borrow()).0.borrow()).clone()));
    assert!(((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
    assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
        (*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*i.borrow())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 1_isize));
    (*i.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
    assert!(((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
    assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
        (*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*i.borrow())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
    assert!((unimplemented!() < 1.1920929e-07_f32));
    assert!((unimplemented!() == 1_f32));
    assert!(
        ({
            let _c: Value<test_hash_int_> = Rc::new(RefCell::new(test_hash_int_::new(None)));
            test_hash_int_Impl::operator_eq(
                &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                _c.as_pointer(),
            )
        })
    );
    assert!(
        ({
            let _a: Value<test_equal_to_int_> = Rc::new(RefCell::new(unimplemented!()));
            let _b: Value<test_equal_to_int_> = Rc::new(RefCell::new(test_equal_to_int_::new_1()));
            operator_eq_1(_a.as_pointer(), _b.as_pointer())
        })
    );
    assert!(
        ({
            let _arg0: min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____   = unimplemented!()   ;
            let _arg1: min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____   = ((*alloc.upgrade().deref()) ).clone()  ;
            operator_eq_11(_arg0, _arg1)
        })
    );
}
pub fn test_15(
    alloc : Ptr< explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >,
) {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    assert!((unimplemented!() == 6_()));
    let eq: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new({
        let __a0 = unimplemented!();
        (
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a0.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a0.1.as_pointer(),
            ))),
        )
    }));
    assert!((unimplemented!() == 2_isize));
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    ({
        let _pos: () = <()>::from(unimplemented!());
        let _end: () = <()>::from(unimplemented!());
        CheckConsecutiveKeys_0(_pos, _end, (1 as ()), s.as_pointer())
    });
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 2_isize));
    unimplemented!();
    unimplemented!();
    ({
        let _pos: () = <()>::from(unimplemented!());
        let _end: () = <()>::from(unimplemented!());
        CheckConsecutiveKeys_0(_pos, _end, (2 as ()), s.as_pointer())
    });
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 1_isize));
    let i: Value<()> = Rc::new(RefCell::new((*(*eq.borrow()).0.borrow()).clone()));
    assert!(((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
    assert!(Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
        (*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*i.borrow())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    {
        let __a1 = unimplemented!();
        eq.as_pointer().write((
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.0.as_pointer(),
            ))),
            Rc::new(RefCell::new(<() as CppFrom<()>>::cpp_from_move(
                __a1.1.as_pointer(),
            ))),
        ))
    };
    assert!((unimplemented!() == 1_isize));
    (*i.borrow_mut()) = (*(*eq.borrow()).0.borrow()).clone();
    assert!(((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4));
    assert!(Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
        (*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow())[..(*(*(*i.borrow())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
        .len()
        .saturating_sub(1)]
            == *__s
    }));
    assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
    assert!((unimplemented!() < 1.1920929e-07_f32));
    assert!((unimplemented!() == 1_f32));
    assert!(
        ({
            let _c: Value<test_hash_int_> = Rc::new(RefCell::new(test_hash_int_::new(None)));
            test_hash_int_Impl::operator_eq(
                &Rc::new(RefCell::new(unimplemented!())).as_pointer(),
                _c.as_pointer(),
            )
        })
    );
    assert!(
        ({
            let _a: Value<test_equal_to_int_> = Rc::new(RefCell::new(unimplemented!()));
            let _b: Value<test_equal_to_int_> = Rc::new(RefCell::new(test_equal_to_int_::new_1()));
            operator_eq_1(_a.as_pointer(), _b.as_pointer())
        })
    );
    assert!(
        ({
            let _arg0: explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____   = unimplemented!()   ;
            let _arg1: explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____   = ((*alloc.upgrade().deref()) ).clone()  ;
            operator_eq_12(_arg0, _arg1)
        })
    );
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
    ({
        let _alloc : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ :: new_1 ( { 10  } , )  ));
        test_13(_alloc.as_pointer())
    });
    ({
        let _alloc : Value<min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() ));
        test_14(_alloc.as_pointer())
    });
    ({
        let _alloc : Value<explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ :: new ( )  ));
        test_15(_alloc.as_pointer())
    });
    return 0;
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
pub trait explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl
{
    fn allocate(&self, mut n: usize) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<Vec<i8>>)>, mut n: usize) {
        unimplemented!()
    }
}
impl explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl for Ptr<explicit_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____> {
}
pub trait min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl for Ptr<min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____> {
}
pub trait min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : isize  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
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
pub trait test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl
{
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<Vec<i8>>)>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<(Value<i32>, Value<Vec<i8>>)>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____Impl for Ptr<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____> {
fn destructor ( &self , ) { assert!(( (* self ) .with(|__s| __s . data_ ) != destructed_value_2 .with(|rc| *rc.borrow()) ) )  ;
 assert!(( (* self ) .with(|__s| __s . id_ ) != destructed_value_2 .with(|rc| *rc.borrow()) ) )  ;
 if ! ( ( (* self ) .with(|__s| __s . stats_ .clone()) ).is_null() ) { field!((* self ) .with(|__s| __s . stats_ .clone()) , count) .with_mut(|__v| __v. prefix_dec ()) ;
 } field!((* self ) , data_) .write( destructed_value_2 .with(|rc| *rc.borrow())  ) ;
 field!((* self ) , id_) .write( destructed_value_2 .with(|rc| *rc.borrow())  ) ;
 } }
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_2.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_2.with(|rc| *rc.borrow()));
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
pub trait test_equal_to_int_Impl {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool;
}
impl test_equal_to_int_Impl for Ptr<test_equal_to_int_> {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool {
        return ({ (a.read()) } == { (b.read()) });
    }
}
pub trait test_hash_int_Impl {
    fn operator_call(&self, x: Ptr<i32>) -> usize;
    fn operator_eq(&self, c: Ptr<test_hash_int_>) -> bool;
}
impl test_hash_int_Impl for Ptr<test_hash_int_> {
    fn operator_call(&self, x: Ptr<i32>) -> usize {
        return unimplemented!();
    }
    fn operator_eq(&self, c: Ptr<test_hash_int_>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } == { c.with(|__s| __s.data_) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_2.with(|_| ());
    let _ = moved_value_3.with(|_| ());
    let _ = outstanding_bytes_7.with(|_| ());
    let _ = alloc_count_8.with(|_| ());
    let _ = dealloc_count_9.with(|_| ());
    let _ = disable_default_constructor_10.with(|_| ());
}
