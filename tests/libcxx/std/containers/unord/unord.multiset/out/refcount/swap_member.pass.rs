extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(a: Ptr<test_equal_to_int_>, b: Ptr<test_equal_to_int_>) -> bool {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct test_equal_to_int_ {
    #[offset(0)]
    pub data_: i32,
}
impl test_equal_to_int_ {
    pub fn new_1(mut data: i32) -> Self {
        Self { data_: data }
    }
}
impl CppEq for test_equal_to_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Callable2<Ptr<i32>, Ptr<i32>, bool> for test_equal_to_int_ {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            test_equal_to_int_Impl::operator_call(__it, a1, a2)
        })
    }
}
impl CppMove for test_equal_to_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for test_hash_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ {}
impl CppMove for test_allocator_int__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_int_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ {
    pub fn new_1(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_5(x: Ptr<other_allocator_int_>, y: Ptr<other_allocator_int_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_int_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_int_ {
    pub fn new_1(mut i: i32) -> Self {
        Self { data_: i }
    }
}
impl CppEq for other_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for other_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for other_allocator_int_ {
    fn default() -> Self {
        { other_allocator_int_ { data_: 0_i32 } }
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
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_7.with(|rc| *rc.borrow()) >= dealloc_count_8.with(|rc| *rc.borrow())));
        return ((alloc_count_7.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_8.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_9.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_6.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_7.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_8.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_6: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_7: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_8: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_9: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_eq_10(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_0: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_1: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_2: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 1 }, { 1 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_0.as_pointer() } , { __tmp_1.as_pointer() } , { __tmp_2.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_3: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_4: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_5: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 1 }, { 2 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_3.as_pointer() } , { __tmp_4.as_pointer() } , { __tmp_5.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c1.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            (({
                test_allocator_int_Impl::get_id(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() }))).as_pointer(),
                )
            }) == 1)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            (({
                test_allocator_int_Impl::get_id(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).get_allocator() }))).as_pointer(),
                )
            }) == 2)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a2: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([10, 20, 30, 40, 50, 60, 70, 80])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_6: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_7: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_8: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 1 }, { 1 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_6.as_pointer() } , { __tmp_7.as_pointer() } , { __tmp_8.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_9: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_10: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_11: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 1 }, { 2 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__ :: new_2 ( { ( { begin_13 ( a2 .as_pointer()  , ) } )   } , { (a2 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_9.as_pointer() } , { __tmp_10.as_pointer() } , { __tmp_11.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        assert!(({ (({ (*c1.borrow()).bucket_count() }) >= 8_u64) }));
        assert!(({ (({ (*c1.borrow()).size() }) == 8_u64) }));
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 10)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(20));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 20)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(30));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 30)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(40));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 40)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(50));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 50)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(60));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 60)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(70));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 70)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(80));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 80)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            (({
                test_allocator_int_Impl::get_id(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() }))).as_pointer(),
                )
            }) == 1)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            (({
                test_allocator_int_Impl::get_id(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).get_allocator() }))).as_pointer(),
                )
            }) == 2)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_12: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_13: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_14: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 1 }, { 1 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__ :: new_2 ( { ( { begin_14 ( a1 .as_pointer()  , ) } )   } , { (a1 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_12.as_pointer() } , { __tmp_13.as_pointer() } , { __tmp_14.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_15: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_16: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_17: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 1 }, { 2 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_15.as_pointer() } , { __tmp_16.as_pointer() } , { __tmp_17.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c1.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            (({
                test_allocator_int_Impl::get_id(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() }))).as_pointer(),
                )
            }) == 1)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        assert!(({ (({ (*c2.borrow()).bucket_count() }) >= 6_u64) }));
        assert!(({ (({ (*c2.borrow()).size() }) == 6_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            (({
                test_allocator_int_Impl::get_id(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).get_allocator() }))).as_pointer(),
                )
            }) == 2)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let a2: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([10, 20, 30, 40, 50, 60, 70, 80])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_18: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_19: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_20: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 1 }, { 1 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__ :: new_2 ( { ( { begin_14 ( a1 .as_pointer()  , ) } )   } , { (a1 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_18.as_pointer() } , { __tmp_19.as_pointer() } , { __tmp_20.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_21: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_22: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_23: Value<test_allocator_int_> =
                Rc::new(RefCell::new(test_allocator_int_::new_1({ 1 }, { 2 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___test_allocator_int__ :: new_2 ( { ( { begin_13 ( a2 .as_pointer()  , ) } )   } , { (a2 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_21.as_pointer() } , { __tmp_22.as_pointer() } , { __tmp_23.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        assert!(({ (({ (*c1.borrow()).bucket_count() }) >= 8_u64) }));
        assert!(({ (({ (*c1.borrow()).size() }) == 8_u64) }));
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 10)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(20));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 20)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(30));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 30)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(40));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 40)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(50));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 50)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(60));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 60)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(70));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 70)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(80));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 80)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            (({
                test_allocator_int_Impl::get_id(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() }))).as_pointer(),
                )
            }) == 1)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        assert!(({ (({ (*c2.borrow()).bucket_count() }) >= 6_u64) }));
        assert!(({ (({ (*c2.borrow()).size() }) == 6_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            (({
                test_allocator_int_Impl::get_id(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).get_allocator() }))).as_pointer(),
                )
            }) == 2)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_24: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_25: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_26: Value<other_allocator_int_> =
                Rc::new(RefCell::new(other_allocator_int_::new_1({ 1 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_24.as_pointer() } , { __tmp_25.as_pointer() } , { __tmp_26.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_27: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_28: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_29: Value<other_allocator_int_> =
                Rc::new(RefCell::new(other_allocator_int_::new_1({ 2 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_27.as_pointer() } , { __tmp_28.as_pointer() } , { __tmp_29.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c1.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 2 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).get_allocator() })));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 1 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a2: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([10, 20, 30, 40, 50, 60, 70, 80])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_30: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_31: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_32: Value<other_allocator_int_> =
                Rc::new(RefCell::new(other_allocator_int_::new_1({ 1 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_30.as_pointer() } , { __tmp_31.as_pointer() } , { __tmp_32.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_33: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_34: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_35: Value<other_allocator_int_> =
                Rc::new(RefCell::new(other_allocator_int_::new_1({ 2 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__ :: new_2 ( { ( { begin_13 ( a2 .as_pointer()  , ) } )   } , { (a2 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_33.as_pointer() } , { __tmp_34.as_pointer() } , { __tmp_35.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        assert!(({ (({ (*c1.borrow()).bucket_count() }) >= 8_u64) }));
        assert!(({ (({ (*c1.borrow()).size() }) == 8_u64) }));
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 10)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(20));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 20)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(30));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 30)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(40));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 40)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(50));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 50)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(60));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 60)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(70));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 70)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(80));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 80)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 2 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).get_allocator() })));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 1 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_36: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_37: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_38: Value<other_allocator_int_> =
                Rc::new(RefCell::new(other_allocator_int_::new_1({ 1 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__ :: new_2 ( { ( { begin_14 ( a1 .as_pointer()  , ) } )   } , { (a1 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_36.as_pointer() } , { __tmp_37.as_pointer() } , { __tmp_38.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_39: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_40: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_41: Value<other_allocator_int_> =
                Rc::new(RefCell::new(other_allocator_int_::new_1({ 2 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_39.as_pointer() } , { __tmp_40.as_pointer() } , { __tmp_41.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c1.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 2 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        assert!(({ (({ (*c2.borrow()).bucket_count() }) >= 6_u64) }));
        assert!(({ (({ (*c2.borrow()).size() }) == 6_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).get_allocator() })));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 1 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let a2: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([10, 20, 30, 40, 50, 60, 70, 80])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_42: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_43: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_44: Value<other_allocator_int_> =
                Rc::new(RefCell::new(other_allocator_int_::new_1({ 1 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__ :: new_2 ( { ( { begin_14 ( a1 .as_pointer()  , ) } )   } , { (a1 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_42.as_pointer() } , { __tmp_43.as_pointer() } , { __tmp_44.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_45: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_46: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_47: Value<other_allocator_int_> =
                Rc::new(RefCell::new(other_allocator_int_::new_1({ 2 })));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___other_allocator_int__ :: new_2 ( { ( { begin_13 ( a2 .as_pointer()  , ) } )   } , { (a2 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_45.as_pointer() } , { __tmp_46.as_pointer() } , { __tmp_47.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        assert!(({ (({ (*c1.borrow()).bucket_count() }) >= 8_u64) }));
        assert!(({ (({ (*c1.borrow()).size() }) == 8_u64) }));
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 10)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(20));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 20)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(30));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 30)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(40));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 40)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(50));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 50)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(60));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 60)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(70));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 70)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(80));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 80)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 2 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        assert!(({ (({ (*c2.borrow()).bucket_count() }) >= 6_u64) }));
        assert!(({ (({ (*c2.borrow()).size() }) == 6_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).get_allocator() })));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 1 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_48: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_49: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_50: Value<min_allocator_int_> =
                Rc::new(RefCell::new(<min_allocator_int_>::default()));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_48.as_pointer() } , { __tmp_49.as_pointer() } , { __tmp_50.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_51: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_52: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_53: Value<min_allocator_int_> =
                Rc::new(RefCell::new(<min_allocator_int_>::default()));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_51.as_pointer() } , { __tmp_52.as_pointer() } , { __tmp_53.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c1.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_int_ = ({ (*c1.borrow()).get_allocator() });
                    operator_eq_10(_arg0, <min_allocator_int_>::default())
                })
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_int_ = ({ (*c2.borrow()).get_allocator() });
                    operator_eq_10(_arg0, <min_allocator_int_>::default())
                })
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a2: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([10, 20, 30, 40, 50, 60, 70, 80])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_54: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_55: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_56: Value<min_allocator_int_> =
                Rc::new(RefCell::new(<min_allocator_int_>::default()));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_54.as_pointer() } , { __tmp_55.as_pointer() } , { __tmp_56.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_57: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_58: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_59: Value<min_allocator_int_> =
                Rc::new(RefCell::new(<min_allocator_int_>::default()));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__ :: new_2 ( { ( { begin_13 ( a2 .as_pointer()  , ) } )   } , { (a2 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_57.as_pointer() } , { __tmp_58.as_pointer() } , { __tmp_59.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        assert!(({ (({ (*c1.borrow()).bucket_count() }) >= 8_u64) }));
        assert!(({ (({ (*c1.borrow()).size() }) == 8_u64) }));
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 10)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(20));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 20)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(30));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 30)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(40));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 40)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(50));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 50)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(60));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 60)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(70));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 70)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(80));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 80)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_int_ = ({ (*c1.borrow()).get_allocator() });
                    operator_eq_10(_arg0, <min_allocator_int_>::default())
                })
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_int_ = ({ (*c2.borrow()).get_allocator() });
                    operator_eq_10(_arg0, <min_allocator_int_>::default())
                })
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_60: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_61: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_62: Value<min_allocator_int_> =
                Rc::new(RefCell::new(<min_allocator_int_>::default()));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__ :: new_2 ( { ( { begin_14 ( a1 .as_pointer()  , ) } )   } , { (a1 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_60.as_pointer() } , { __tmp_61.as_pointer() } , { __tmp_62.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_63: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_64: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_65: Value<min_allocator_int_> =
                Rc::new(RefCell::new(<min_allocator_int_>::default()));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__ :: new_1 ( { 0_u64  } , { __tmp_63.as_pointer() } , { __tmp_64.as_pointer() } , { __tmp_65.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        const _: () = assert!(true, "");;
        assert!(({ (({ (*c1.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_int_ = ({ (*c1.borrow()).get_allocator() });
                    operator_eq_10(_arg0, <min_allocator_int_>::default())
                })
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        assert!(({ (({ (*c2.borrow()).bucket_count() }) >= 6_u64) }));
        assert!(({ (({ (*c2.borrow()).size() }) == 6_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_int_ = ({ (*c2.borrow()).get_allocator() });
                    operator_eq_10(_arg0, <min_allocator_int_>::default())
                })
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 1, 2])));
        let a2: Value<Box<[i32]>> =
            Rc::new(RefCell::new(Box::new([10, 20, 30, 40, 50, 60, 70, 80])));
        let c1: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_66: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
            let __tmp_67: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
            let __tmp_68: Value<min_allocator_int_> =
                Rc::new(RefCell::new(<min_allocator_int_>::default()));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__ :: new_2 ( { ( { begin_14 ( a1 .as_pointer()  , ) } )   } , { (a1 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_66.as_pointer() } , { __tmp_67.as_pointer() } , { __tmp_68.as_pointer() } , )
        }));
        let c2: Value<
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__,
        > = Rc::new(RefCell::new({
            let __tmp_69: Value<test_hash_int_> =
                Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
            let __tmp_70: Value<test_equal_to_int_> =
                Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
            let __tmp_71: Value<min_allocator_int_> =
                Rc::new(RefCell::new(<min_allocator_int_>::default()));
            std_unordered_multiset_int__test_hash_int___test_equal_to_int___min_allocator_int__ :: new_2 ( { ( { begin_13 ( a2 .as_pointer()  , ) } )   } , { (a2 .as_pointer()  as Ptr< i32  > ).to_end()   } , { 0_u64  } , { __tmp_69.as_pointer() } , { __tmp_70.as_pointer() } , { __tmp_71.as_pointer() } , )
        }));
        ({ (*c2.borrow()).max_load_factor_82(2_f32) });
        ({ (*c1.borrow()).swap(c2.as_pointer()) });
        assert!(({ (({ (*c1.borrow()).bucket_count() }) >= 8_u64) }));
        assert!(({ (({ (*c1.borrow()).size() }) == 8_u64) }));
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 10)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(20));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 20)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(30));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 30)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(40));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 40)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(50));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 50)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(60));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 60)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(70));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 70)
        );
        assert!(
            ((({
                let ___x: Value<i32> = Rc::new(RefCell::new(80));
                (*c1.borrow()).find_58(___x.as_pointer())
            })
            .iter_deref()
            .read())
                == 80)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(2) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c1.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 2 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_int_ = ({ (*c1.borrow()).get_allocator() });
                    operator_eq_10(_arg0, <min_allocator_int_>::default())
                })
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c1.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c1.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c1.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c1.borrow()).max_load_factor_81() }) == 2_f32) }));
        assert!(({ (({ (*c2.borrow()).bucket_count() }) >= 6_u64) }));
        assert!(({ (({ (*c2.borrow()).size() }) == 6_u64) }));
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 2_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            (({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c2.borrow()).count(___x.as_pointer())
            }) == 1_u64)
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(1) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c2.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c2.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 1 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_int_ = ({ (*c2.borrow()).get_allocator() });
                    operator_eq_10(_arg0, <min_allocator_int_>::default())
                })
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).begin_27() });
                    let ___last: std___detail__Node_iterator_int__true__true_ =
                        ({ (*c2.borrow()).end_29() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cbegin_31() });
                    let ___last: std___detail__Node_const_iterator_int__true__true_ =
                        ({ (*c2.borrow()).cend_32() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (({ (*c2.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*c2.borrow()).max_load_factor_81() }) == 1_f32) }));
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
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
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait other_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: usize) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_int_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_int_Impl for Ptr<other_allocator_int_> {}
pub trait test_allocator_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<i32>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32;
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_1.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_1.with(|rc| *rc.borrow())));
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
        return ({ <std_hash_int_>::default()((x.read())) });
    }
    fn operator_eq(&self, c: Ptr<test_hash_int_>) -> bool {
        return ({ (*self).with(|__s| __s.data_) } == { c.with(|__s| __s.data_) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
    let _ = outstanding_bytes_6.with(|_| ());
    let _ = alloc_count_7.with(|_| ());
    let _ = dealloc_count_8.with(|_| ());
    let _ = disable_default_constructor_9.with(|_| ());
}
