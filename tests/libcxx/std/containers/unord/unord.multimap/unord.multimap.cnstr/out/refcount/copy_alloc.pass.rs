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
pub fn operator_eq_5(
    x: Ptr<test_allocator_std_pair_const_int__int__>,
    y: Ptr<test_allocator_std_pair_const_int__int__>,
) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {}
impl CppMove for test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__int__ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__int__ {
    pub fn new_1(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
}
impl CppEq for test_allocator_std_pair_const_int__int__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
impl Default for test_allocator_std_pair_const_int__int__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__int__ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
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
pub fn operator_eq_10(
    mut _a0: min_allocator_std_pair_const_int__int__,
    mut _a1: min_allocator_std_pair_const_int__int__,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl CppEq for min_allocator_std_pair_const_int__int__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_std_pair_const_int__int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_alloc_11(new_alloc: Ptr<min_allocator_std_pair_const_int__int__>) {
    {
        let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(4))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(1))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(2))),
        ])));
        let orig : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_1 ( { ( { begin_12 ( arr .as_pointer()  , ) } )   } , { (arr .as_pointer()  as Ptr< (Value<i32  >, Value<i32  >)  > ).to_end()   } , None , None , None , None , )  )) ;
        let copy : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        const _: () = assert!(true, "");;
        assert!(({ (({ (*copy.borrow()).size() }) == 4_u64) }));
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*copy.borrow()).find_60(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2)
        );
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*copy.borrow()).find_60(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4)
        );
        {
            let range: Value<(
                Value<std___detail__Node_iterator_std_pair_const_int__int___false__false_>,
                Value<std___detail__Node_iterator_std_pair_const_int__int___false__false_>,
            )> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(3));
                    (*copy.borrow()).equal_range_68(___x.as_pointer())
                }),
            ));
            let first_element: Value<
                std___detail__Node_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                0_isize as isize,
            )));
            let second_element: Value<
                std___detail__Node_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                1_isize as isize,
            )));
            let end: Value<std___detail__Node_iterator_std_pair_const_int__int___false__false_> =
                Rc::new(RefCell::new(libcc2rs::next_refcount(
                    ({ (*(*range.borrow()).0.borrow()).clone() }),
                    2_isize as isize,
                )));
            assert!(({ (*(*range.borrow()).1.borrow()) == (*end.borrow()) }));
            assert!(
                ({
                    ((*(*(*first_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        == 1)
                        || ((*(*(*first_element.borrow()).iter_deref().upgrade().deref())
                            .1
                            .borrow())
                            == 2)
                })
            );
            assert!(
                ({
                    ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        == 1)
                        || ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                            .1
                            .borrow())
                            == 2)
                })
            );
            assert!(
                ({
                    ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        != (*(*(*(*range.borrow()).0.borrow())
                            .iter_deref()
                            .upgrade()
                            .deref())
                        .1
                        .borrow()))
                })
            );
        }
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*copy.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_14(
                        (({ (*copy.borrow()).load_factor() })
                            - ((({ (*copy.borrow()).size() }) as f32)
                                / (({ (*copy.borrow()).bucket_count() }) as f32))),
                    )
                }) < 1.1920929e-07_f32)
            })
        );
        assert!(({ (({ (*copy.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                ({
                    let _arg0: min_allocator_std_pair_const_int__int__ =
                        ({ (*copy.borrow()).get_allocator() });
                    let _arg1: min_allocator_std_pair_const_int__int__ =
                        (*new_alloc.upgrade().deref()).clone();
                    operator_eq_10(_arg0, _arg1)
                })
            })
        );
        const _: () = assert!(true, "");;
        assert!(({ (({ (*orig.borrow()).size() }) == 4_u64) }));
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*orig.borrow()).find_62(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2)
        );
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*orig.borrow()).find_62(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4)
        );
        {
            let range: Value<(
                Value<std___detail__Node_const_iterator_std_pair_const_int__int___false__false_>,
                Value<std___detail__Node_const_iterator_std_pair_const_int__int___false__false_>,
            )> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(3));
                    (*orig.borrow()).equal_range_70(___x.as_pointer())
                }),
            ));
            let first_element: Value<
                std___detail__Node_const_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                0_isize as isize,
            )));
            let second_element: Value<
                std___detail__Node_const_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                1_isize as isize,
            )));
            let end: Value<
                std___detail__Node_const_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                2_isize as isize,
            )));
            assert!(({ (*(*range.borrow()).1.borrow()) == (*end.borrow()) }));
            assert!(
                ({
                    ((*(*(*first_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        == 1)
                        || ((*(*(*first_element.borrow()).iter_deref().upgrade().deref())
                            .1
                            .borrow())
                            == 2)
                })
            );
            assert!(
                ({
                    ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        == 1)
                        || ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                            .1
                            .borrow())
                            == 2)
                })
            );
            assert!(
                ({
                    ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        != (*(*(*(*range.borrow()).0.borrow())
                            .iter_deref()
                            .upgrade()
                            .deref())
                        .1
                        .borrow()))
                })
            );
        }
    }
    {
        let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([(
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(2)),
        )])));
        let orig : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_1 ( { ( { begin_15 ( arr .as_pointer()  , ) } )   } , { (arr .as_pointer()  as Ptr< (Value<i32  >, Value<i32  >)  > ).to_end()   } , None , None , None , None , )  )) ;
        let copy : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        const _: () = assert!(true, "");;
        assert!(({ (({ (*copy.borrow()).size() }) == 1_u64) }));
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*copy.borrow()).find_60(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*copy.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_14(
                        (({ (*copy.borrow()).load_factor() })
                            - ((({ (*copy.borrow()).size() }) as f32)
                                / (({ (*copy.borrow()).bucket_count() }) as f32))),
                    )
                }) < 1.1920929e-07_f32)
            })
        );
        assert!(({ (({ (*copy.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                ({
                    let _arg0: min_allocator_std_pair_const_int__int__ =
                        ({ (*copy.borrow()).get_allocator() });
                    let _arg1: min_allocator_std_pair_const_int__int__ =
                        (*new_alloc.upgrade().deref()).clone();
                    operator_eq_10(_arg0, _arg1)
                })
            })
        );
        const _: () = assert!(true, "");;
        assert!(({ (({ (*orig.borrow()).size() }) == 1_u64) }));
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*orig.borrow()).find_62(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2)
        );
    }
    {
        let orig : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___  >::default() )) ;
        let copy : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        const _: () = assert!(true, "");;
        assert!(({ (({ (*copy.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*copy.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*copy.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                ({
                    let _arg0: min_allocator_std_pair_const_int__int__ =
                        ({ (*copy.borrow()).get_allocator() });
                    let _arg1: min_allocator_std_pair_const_int__int__ =
                        (*new_alloc.upgrade().deref()).clone();
                    operator_eq_10(_arg0, _arg1)
                })
            })
        );
        const _: () = assert!(true, "");;
        assert!(({ (({ (*orig.borrow()).size() }) == 0_u64) }));
    }
    {
        let orig : Value<std_unordered_multimap_int__int__test_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new({ let __tmp_0 : Value<test_hash_int_ > = Rc::new(RefCell::new(test_hash_int_ :: new ( { Some ( 23  ) } , )  )); std_unordered_multimap_int__int__test_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_1 ( { 0_u64  } , { __tmp_0.as_pointer() } , None , None , )  } )) ;
        let copy : Value<std_unordered_multimap_int__int__test_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__test_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(23) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*copy.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_std_pair_const_int__int__ =
                        ({ (*copy.borrow()).get_allocator() });
                    let _arg1: min_allocator_std_pair_const_int__int__ =
                        (*new_alloc.upgrade().deref()).clone();
                    operator_eq_10(_arg0, _arg1)
                })
            })
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(23) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*orig.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
    }
    {
        let orig : Value<std_unordered_multimap_int__int__std_hash_int___test_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new({ let __tmp_1 : Value<std_hash_int_ > = Rc::new(RefCell::new(<std_hash_int_  >::default() ));let __tmp_2 : Value<test_equal_to_int_ > = Rc::new(RefCell::new(test_equal_to_int_ :: new_1 ( { 56  } , )  )); std_unordered_multimap_int__int__std_hash_int___test_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_1 ( { 0_u64  } , { __tmp_1.as_pointer() } , { __tmp_2.as_pointer() } , None , )  } )) ;
        let copy : Value<std_unordered_multimap_int__int__std_hash_int___test_equal_to_int___min_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___test_equal_to_int___min_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*copy.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 56 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_std_pair_const_int__int__ =
                        ({ (*copy.borrow()).get_allocator() });
                    let _arg1: min_allocator_std_pair_const_int__int__ =
                        (*new_alloc.upgrade().deref()).clone();
                    operator_eq_10(_arg0, _arg1)
                })
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*orig.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 56 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
    }
}
pub fn test_alloc_16(new_alloc: Ptr<test_allocator_std_pair_const_int__int__>) {
    {
        let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(4))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(1))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(2))),
        ])));
        let orig : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_1 ( { ( { begin_12 ( arr .as_pointer()  , ) } )   } , { (arr .as_pointer()  as Ptr< (Value<i32  >, Value<i32  >)  > ).to_end()   } , None , None , None , None , )  )) ;
        let copy : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        const _: () = assert!(true, "");;
        assert!(({ (({ (*copy.borrow()).size() }) == 4_u64) }));
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*copy.borrow()).find_60(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2)
        );
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*copy.borrow()).find_60(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4)
        );
        {
            let range: Value<(
                Value<std___detail__Node_iterator_std_pair_const_int__int___false__false_>,
                Value<std___detail__Node_iterator_std_pair_const_int__int___false__false_>,
            )> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(3));
                    (*copy.borrow()).equal_range_68(___x.as_pointer())
                }),
            ));
            let first_element: Value<
                std___detail__Node_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                0_isize as isize,
            )));
            let second_element: Value<
                std___detail__Node_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                1_isize as isize,
            )));
            let end: Value<std___detail__Node_iterator_std_pair_const_int__int___false__false_> =
                Rc::new(RefCell::new(libcc2rs::next_refcount(
                    ({ (*(*range.borrow()).0.borrow()).clone() }),
                    2_isize as isize,
                )));
            assert!(({ (*(*range.borrow()).1.borrow()) == (*end.borrow()) }));
            assert!(
                ({
                    ((*(*(*first_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        == 1)
                        || ((*(*(*first_element.borrow()).iter_deref().upgrade().deref())
                            .1
                            .borrow())
                            == 2)
                })
            );
            assert!(
                ({
                    ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        == 1)
                        || ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                            .1
                            .borrow())
                            == 2)
                })
            );
            assert!(
                ({
                    ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        != (*(*(*(*range.borrow()).0.borrow())
                            .iter_deref()
                            .upgrade()
                            .deref())
                        .1
                        .borrow()))
                })
            );
        }
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*copy.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_14(
                        (({ (*copy.borrow()).load_factor() })
                            - ((({ (*copy.borrow()).size() }) as f32)
                                / (({ (*copy.borrow()).bucket_count() }) as f32))),
                    )
                }) < 1.1920929e-07_f32)
            })
        );
        assert!(({ (({ (*copy.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                let _x: Value<test_allocator_std_pair_const_int__int__> =
                    Rc::new(RefCell::new(({ (*copy.borrow()).get_allocator() })));
                let _y: Ptr<test_allocator_std_pair_const_int__int__> = (new_alloc).clone();
                operator_eq_5(_x.as_pointer(), _y)
            })
        );
        const _: () = assert!(true, "");;
        assert!(({ (({ (*orig.borrow()).size() }) == 4_u64) }));
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*orig.borrow()).find_62(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2)
        );
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*orig.borrow()).find_62(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4)
        );
        {
            let range: Value<(
                Value<std___detail__Node_const_iterator_std_pair_const_int__int___false__false_>,
                Value<std___detail__Node_const_iterator_std_pair_const_int__int___false__false_>,
            )> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(3));
                    (*orig.borrow()).equal_range_70(___x.as_pointer())
                }),
            ));
            let first_element: Value<
                std___detail__Node_const_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                0_isize as isize,
            )));
            let second_element: Value<
                std___detail__Node_const_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                1_isize as isize,
            )));
            let end: Value<
                std___detail__Node_const_iterator_std_pair_const_int__int___false__false_,
            > = Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ (*(*range.borrow()).0.borrow()).clone() }),
                2_isize as isize,
            )));
            assert!(({ (*(*range.borrow()).1.borrow()) == (*end.borrow()) }));
            assert!(
                ({
                    ((*(*(*first_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        == 1)
                        || ((*(*(*first_element.borrow()).iter_deref().upgrade().deref())
                            .1
                            .borrow())
                            == 2)
                })
            );
            assert!(
                ({
                    ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        == 1)
                        || ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                            .1
                            .borrow())
                            == 2)
                })
            );
            assert!(
                ({
                    ((*(*(*second_element.borrow()).iter_deref().upgrade().deref())
                        .1
                        .borrow())
                        != (*(*(*(*range.borrow()).0.borrow())
                            .iter_deref()
                            .upgrade()
                            .deref())
                        .1
                        .borrow()))
                })
            );
        }
    }
    {
        let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([(
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(2)),
        )])));
        let orig : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_1 ( { ( { begin_15 ( arr .as_pointer()  , ) } )   } , { (arr .as_pointer()  as Ptr< (Value<i32  >, Value<i32  >)  > ).to_end()   } , None , None , None , None , )  )) ;
        let copy : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        const _: () = assert!(true, "");;
        assert!(({ (({ (*copy.borrow()).size() }) == 1_u64) }));
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*copy.borrow()).find_60(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2)
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*copy.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_14(
                        (({ (*copy.borrow()).load_factor() })
                            - ((({ (*copy.borrow()).size() }) as f32)
                                / (({ (*copy.borrow()).bucket_count() }) as f32))),
                    )
                }) < 1.1920929e-07_f32)
            })
        );
        assert!(({ (({ (*copy.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                let _x: Value<test_allocator_std_pair_const_int__int__> =
                    Rc::new(RefCell::new(({ (*copy.borrow()).get_allocator() })));
                let _y: Ptr<test_allocator_std_pair_const_int__int__> = (new_alloc).clone();
                operator_eq_5(_x.as_pointer(), _y)
            })
        );
        const _: () = assert!(true, "");;
        assert!(({ (({ (*orig.borrow()).size() }) == 1_u64) }));
        assert!(
            ((*(*({
                let ___x: Value<i32> = Rc::new(RefCell::new(1));
                (*orig.borrow()).find_62(___x.as_pointer())
            })
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2)
        );
    }
    {
        let orig : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___  >::default() )) ;
        let copy : Value<std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        const _: () = assert!(true, "");;
        assert!(({ (({ (*copy.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__int___false__false_   = ( { (*copy.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*copy.borrow()).size() }) as usize))
            })
        );
        assert!(({ (({ (*copy.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                let _x: Value<test_allocator_std_pair_const_int__int__> =
                    Rc::new(RefCell::new(({ (*copy.borrow()).get_allocator() })));
                let _y: Ptr<test_allocator_std_pair_const_int__int__> = (new_alloc).clone();
                operator_eq_5(_x.as_pointer(), _y)
            })
        );
        const _: () = assert!(true, "");;
        assert!(({ (({ (*orig.borrow()).size() }) == 0_u64) }));
    }
    {
        let orig : Value<std_unordered_multimap_int__int__test_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new({ let __tmp_3 : Value<test_hash_int_ > = Rc::new(RefCell::new(test_hash_int_ :: new ( { Some ( 23  ) } , )  )); std_unordered_multimap_int__int__test_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_1 ( { 0_u64  } , { __tmp_3.as_pointer() } , None , None , )  } )) ;
        let copy : Value<std_unordered_multimap_int__int__test_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__test_hash_int___std_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(23) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*copy.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _x: Value<test_allocator_std_pair_const_int__int__> =
                    Rc::new(RefCell::new(({ (*copy.borrow()).get_allocator() })));
                let _y: Ptr<test_allocator_std_pair_const_int__int__> = (new_alloc).clone();
                operator_eq_5(_x.as_pointer(), _y)
            })
        );
        assert!(
            ({
                let _c: Value<test_hash_int_> =
                    Rc::new(RefCell::new(test_hash_int_::new({ Some(23) })));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*orig.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
    }
    {
        let orig : Value<std_unordered_multimap_int__int__std_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new({ let __tmp_4 : Value<std_hash_int_ > = Rc::new(RefCell::new(<std_hash_int_  >::default() ));let __tmp_5 : Value<test_equal_to_int_ > = Rc::new(RefCell::new(test_equal_to_int_ :: new_1 ( { 56  } , )  )); std_unordered_multimap_int__int__std_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_1 ( { 0_u64  } , { __tmp_4.as_pointer() } , { __tmp_5.as_pointer() } , None , )  } )) ;
        let copy : Value<std_unordered_multimap_int__int__std_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__int___ > = Rc::new(RefCell::new(std_unordered_multimap_int__int__std_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__int___ :: new_2 ( { orig .as_pointer()  } , { (new_alloc ).clone() } , )  )) ;
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*copy.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 56 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Value<test_allocator_std_pair_const_int__int__> =
                    Rc::new(RefCell::new(({ (*copy.borrow()).get_allocator() })));
                let _y: Ptr<test_allocator_std_pair_const_int__int__> = (new_alloc).clone();
                operator_eq_5(_x.as_pointer(), _y)
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*orig.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1({ 56 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
    }
}
pub fn test_17() {
    ({
        let _new_alloc: Value<std_allocator_std_pair_const_int__int__> = Rc::new(RefCell::new(
            <std_allocator_std_pair_const_int__int__>::default(),
        ));
        test_alloc_18(_new_alloc.as_pointer())
    });
    ({
        let _new_alloc: Value<min_allocator_std_pair_const_int__int__> = Rc::new(RefCell::new(
            <min_allocator_std_pair_const_int__int__>::default(),
        ));
        test_alloc_11(_new_alloc.as_pointer())
    });
    ({
        let _new_alloc: Value<test_allocator_std_pair_const_int__int__> = Rc::new(RefCell::new(
            test_allocator_std_pair_const_int__int__::new_1({ 25 }),
        ));
        test_alloc_16(_new_alloc.as_pointer())
    });
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
    ({ test_17() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__;
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
pub trait min_allocator_std_pair_const_int__int__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__int__Impl for Ptr<min_allocator_std_pair_const_int__int__> {}
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
pub trait test_allocator_std_pair_const_int__int__Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<(Value<i32>, Value<i32>)>) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<(Value<i32>, Value<i32>)>) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<i32>)>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<(Value<i32>, Value<i32>)>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_std_pair_const_int__int__Impl
    for Ptr<test_allocator_std_pair_const_int__int__>
{
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
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl test_equal_to_int_Impl for Ptr<test_equal_to_int_> {}
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
