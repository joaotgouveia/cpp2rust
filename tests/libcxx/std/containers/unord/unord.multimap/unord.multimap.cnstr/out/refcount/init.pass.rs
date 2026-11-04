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
    x : Ptr< test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >,
    y : Ptr< test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >,
) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__std_basic_string_char____rebind_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
impl CppMove for test_allocator_std_pair_const_int__std_basic_string_char____rebind_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
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
impl CppEq for test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_5(__lhs.clone(), __rhs.clone()) } }
impl Default for test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn default() -> Self { { test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { data_ : 0  , id_ : 0  , stats_ : Ptr::<test_allocator_statistics>::null()  , } } } }
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
    mut _a0 : min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
    mut _a1 : min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
impl CppEq for min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_10(__lhs.read(), __rhs.read()) } }
impl CppMove for min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
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
        let c : Value<std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { Vec::from([ match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"one" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_0,) => { (
        Rc::new(RefCell::new(((*__tmp_0.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_0.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"two" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_1,) => { (
        Rc::new(RefCell::new(((*__tmp_1.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_1.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(3 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"three" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_2,) => { (
        Rc::new(RefCell::new(((*__tmp_2.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_2.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(4 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_3,) => { (
        Rc::new(RefCell::new(((*__tmp_3.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_3.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_4,) => { (
        Rc::new(RefCell::new(((*__tmp_4.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_4.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_5,) => { (
        Rc::new(RefCell::new(((*__tmp_5.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_5.upgrade().deref()) .1.as_pointer()))),
    ) } }  , ] )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        assert!(({ (({ (*c.borrow()).size() }) == 6_u64) }));
        let eq : Value<(Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >, Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >) > = Rc::new(RefCell::new(match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_6,) => { (
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_6.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_6.upgrade().deref()) .1.as_pointer()))),
    ) } } )) ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 2_isize)
            })
        );
        let s : Value<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() )) ;
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_12(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_7,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_7.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_7.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_12(_pos, _end, 2, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_8,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_8.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_8.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 1_isize)
            })
        );
        let i : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_ > = Rc::new(RefCell::new(((*(*eq.borrow()) .0.borrow()) ).clone() )) ;
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_9,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_10,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_10.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_10.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 1_isize)
            })
        );
        {
            let __a1 = ({ (*(*eq.borrow()).0.borrow()).clone() });
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_11,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_14(
                        ((({ (*c.borrow()).load_factor() })
                            - ((({ (*c.borrow()).size() }) as f32)
                                / (({ (*c.borrow()).bucket_count() }) as f32)))
                            as f64),
                    )
                }) < (1.1920929e-07_f32 as f64))
            })
        );
        assert!(({ (({ (*c.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> = Rc::new(RefCell::new(test_hash_int_::new(None)));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1()));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(( { (*c.borrow()) . get_allocator ( ) } )  ));
                let _y : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() ));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    {
        let c : Value<std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_1 ( { Vec::from([ match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"one" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_12,) => { (
        Rc::new(RefCell::new(((*__tmp_12.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_12.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"two" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_13,) => { (
        Rc::new(RefCell::new(((*__tmp_13.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_13.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(3 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"three" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_14,) => { (
        Rc::new(RefCell::new(((*__tmp_14.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_14.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(4 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_15,) => { (
        Rc::new(RefCell::new(((*__tmp_15.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_15.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_16,) => { (
        Rc::new(RefCell::new(((*__tmp_16.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_16.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_17,) => { (
        Rc::new(RefCell::new(((*__tmp_17.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_17.upgrade().deref()) .1.as_pointer()))),
    ) } }  , ] )  } , None , None , None , None , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 7_u64) }));
        assert!(({ (({ (*c.borrow()).size() }) == 6_u64) }));
        let eq : Value<(Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >, Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >) > = Rc::new(RefCell::new(match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_18,) => { (
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_18.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_18.upgrade().deref()) .1.as_pointer()))),
    ) } } )) ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 2_isize)
            })
        );
        let s : Value<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() )) ;
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_12(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_19,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_19.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_19.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_12(_pos, _end, 2, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_20,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_20.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_20.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 1_isize)
            })
        );
        let i : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_ > = Rc::new(RefCell::new(((*(*eq.borrow()) .0.borrow()) ).clone() )) ;
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_21,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_21.upgrade().deref())
                            [..(*__tmp_21.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_22,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_22.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_22.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 1_isize)
            })
        );
        {
            let __a1 = ({ (*(*eq.borrow()).0.borrow()).clone() });
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_23,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_23.upgrade().deref())
                            [..(*__tmp_23.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_14(
                        ((({ (*c.borrow()).load_factor() })
                            - ((({ (*c.borrow()).size() }) as f32)
                                / (({ (*c.borrow()).bucket_count() }) as f32)))
                            as f64),
                    )
                }) < (1.1920929e-07_f32 as f64))
            })
        );
        assert!(({ (({ (*c.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> = Rc::new(RefCell::new(test_hash_int_::new(None)));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1()));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                ({
                    let _arg0: min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____   = ( { (*c.borrow()) . get_allocator ( ) } )   ;
                    operator_eq_10 ( _arg0 , <min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default()  , )
                })
            })
        );
    }
    {
        let a : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ :: new_1 ( { 42  } , )  )) ;
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let c : Value<std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_2 ( { Vec::from([ match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"one" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_24,) => { (
        Rc::new(RefCell::new(((*__tmp_24.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_24.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"two" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_25,) => { (
        Rc::new(RefCell::new(((*__tmp_25.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_25.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(3 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"three" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_26,) => { (
        Rc::new(RefCell::new(((*__tmp_26.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_26.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(4 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_27,) => { (
        Rc::new(RefCell::new(((*__tmp_27.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_27.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_28,) => { (
        Rc::new(RefCell::new(((*__tmp_28.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_28.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_29,) => { (
        Rc::new(RefCell::new(((*__tmp_29.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_29.upgrade().deref()) .1.as_pointer()))),
    ) } }  , ] )  } , { 12_u64  } , { a .as_pointer()  } , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 12_u64) }));
        assert!(({ (({ (*c.borrow()).size() }) == 6_u64) }));
        let eq : Value<(Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >, Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >) > = Rc::new(RefCell::new(match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_30,) => { (
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_30.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_30.upgrade().deref()) .1.as_pointer()))),
    ) } } )) ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 2_isize)
            })
        );
        let s : Value<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() )) ;
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_12(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_31,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_31.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_31.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_12(_pos, _end, 2, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_32,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_32.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_32.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 1_isize)
            })
        );
        let i : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_ > = Rc::new(RefCell::new(((*(*eq.borrow()) .0.borrow()) ).clone() )) ;
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_33,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_33.upgrade().deref())
                            [..(*__tmp_33.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_34,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_34.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_34.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 1_isize)
            })
        );
        {
            let __a1 = ({ (*(*eq.borrow()).0.borrow()).clone() });
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_35,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_35.upgrade().deref())
                            [..(*__tmp_35.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_14(
                        ((({ (*c.borrow()).load_factor() })
                            - ((({ (*c.borrow()).size() }) as f32)
                                / (({ (*c.borrow()).bucket_count() }) as f32)))
                            as f64),
                    )
                }) < (1.1920929e-07_f32 as f64))
            })
        );
        assert!(({ (({ (*c.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                let _c: Value<test_hash_int_> = Rc::new(RefCell::new(test_hash_int_::new(None)));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1()));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(( { (*c.borrow()) . get_allocator ( ) } )  ));
                operator_eq_5(_x.as_pointer(), a.as_pointer())
            })
        );
        assert!(
            !({
                let _x : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(( { (*c.borrow()) . get_allocator ( ) } )  ));
                let _y : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() ));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    {
        let hf: Value<test_hash_int_> = Rc::new(RefCell::new(test_hash_int_::new({ Some(42) })));
        let a : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ :: new_1 ( { 43  } , )  )) ;
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let c : Value<std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ > = Rc::new(RefCell::new(std_unordered_multimap_int__std_basic_string_char__std_char_traits_char___std_allocator_char____test_hash_int___test_equal_to_int___test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____ :: new_3 ( { Vec::from([ match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"one" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_36,) => { (
        Rc::new(RefCell::new(((*__tmp_36.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_36.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"two" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_37,) => { (
        Rc::new(RefCell::new(((*__tmp_37.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_37.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(3 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"three" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_38,) => { (
        Rc::new(RefCell::new(((*__tmp_38.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_38.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(4 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_39,) => { (
        Rc::new(RefCell::new(((*__tmp_39.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_39.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(1 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_40,) => { (
        Rc::new(RefCell::new(((*__tmp_40.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_40.upgrade().deref()) .1.as_pointer()))),
    ) } }  , match (Rc::new(RefCell::new((Rc::new(RefCell::new(2 )), Rc::new(RefCell::new({let mut __bytes = Ptr::<i8 >::from_string_literal(b"four" ) .to_c_bytes();
    __bytes.push(0_i8);
    __bytes}  ))) )).as_pointer(),) { (__tmp_41,) => { (
        Rc::new(RefCell::new(((*__tmp_41.upgrade().deref()) .0.as_pointer().read()))),
        Rc::new(RefCell::new(<Vec<i8  >   as CppFrom<Vec<i8  >  >>::cpp_from_move((*__tmp_41.upgrade().deref()) .1.as_pointer()))),
    ) } }  , ] )  } , { 12_u64  } , { hf .as_pointer()  } , { a .as_pointer()  } , )  )) ;
        assert!(({ (({ (*c.borrow()).bucket_count() }) >= 12_u64) }));
        assert!(({ (({ (*c.borrow()).size() }) == 6_u64) }));
        let eq : Value<(Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >, Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >) > = Rc::new(RefCell::new(match (Rc::new(RefCell::new(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . equal_range_68 ( ___x.as_pointer() , ) } )  )).as_pointer(),) { (__tmp_42,) => { (
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_42.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_42.upgrade().deref()) .1.as_pointer()))),
    ) } } )) ;
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 2_isize)
            })
        );
        let s : Value<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<std_multiset_std_basic_string_char__std_char_traits_char___std_allocator_char____std_less_std_basic_string_char__std_char_traits_char___std_allocator_char_____std_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() )) ;
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"one").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(1 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_12(_pos, _end, 1, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(2));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_43,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_43.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_43.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 2_isize)
            })
        );
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"two").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<Vec<i8>> = Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"four").to_c_bytes();
                __bytes.push(0_i8);
                __bytes
            }));
            (*s.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let _pos: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { let ___x : Value<i32 > = Rc::new(RefCell::new(2 )); (*c.borrow()) . find_60 ( ___x.as_pointer() , ) } )  )  ;
            let _end: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = <std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
            CheckConsecutiveKeys_12(_pos, _end, 2, s.as_pointer())
        });
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(3));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_44,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_44.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_44.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 1_isize)
            })
        );
        let i : Value<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_ > = Rc::new(RefCell::new(((*(*eq.borrow()) .0.borrow()) ).clone() )) ;
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_45,) => Ptr::<i8>::from_string_literal(b"three").with_c_str(|__s| {
                        (*__tmp_45.upgrade().deref())
                            [..(*__tmp_45.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        match (Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*c.borrow()).equal_range_68(___x.as_pointer())
            }),
        ))
        .as_pointer(),)
        {
            (__tmp_46,) => {
                eq .as_pointer() .write((
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_46.upgrade().deref()) .0.as_pointer()))),
        Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   as CppFrom<std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_  >>::cpp_from_move((*__tmp_46.upgrade().deref()) .1.as_pointer()))),
    ));
                eq.as_pointer()
            }
        };
        assert!(
            ({
                (({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .0.borrow()) ).clone()  ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ((*(*eq.borrow()) .1.borrow()) ).clone()  ;
                    distance_11(___first, ___last)
                }) == 1_isize)
            })
        );
        {
            let __a1 = ({ (*(*eq.borrow()).0.borrow()).clone() });
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 4) }));
        assert!(
            ({
                match ((*(*i.borrow()).iter_deref().upgrade().deref())
                    .1
                    .as_pointer(),)
                {
                    (__tmp_47,) => Ptr::<i8>::from_string_literal(b"four").with_c_str(|__s| {
                        (*__tmp_47.upgrade().deref())
                            [..(*__tmp_47.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . begin_27 ( ) } )   ;
                    let ___last: std___detail__Node_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . end_30 ( ) } )   ;
                    distance_13(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                ((({
                    let ___first: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cbegin_29 ( ) } )   ;
                    let ___last: std___detail__Node_const_iterator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____false__true_   = ( { (*c.borrow()) . cend_32 ( ) } )   ;
                    distance_11(___first, ___last)
                }) as usize)
                    == (({ (*c.borrow()).size() }) as usize))
            })
        );
        assert!(
            ({
                (({
                    fabs_14(
                        ((({ (*c.borrow()).load_factor() })
                            - ((({ (*c.borrow()).size() }) as f32)
                                / (({ (*c.borrow()).bucket_count() }) as f32)))
                            as f64),
                    )
                }) < (1.1920929e-07_f32 as f64))
            })
        );
        assert!(({ (({ (*c.borrow()).max_load_factor_83() }) == 1_f32) }));
        assert!(
            ({
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c.borrow()).hash_function() }))).as_pointer(),
                    hf.as_pointer(),
                )
            })
        );
        assert!(
            !({
                let _c: Value<test_hash_int_> = Rc::new(RefCell::new(test_hash_int_::new(None)));
                test_hash_int_Impl::operator_eq(
                    &Rc::new(RefCell::new(({ (*c.borrow()).hash_function() }))).as_pointer(),
                    _c.as_pointer(),
                )
            })
        );
        assert!(
            ({
                let _a: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(({ (*c.borrow()).key_eq() })));
                let _b: Value<test_equal_to_int_> =
                    Rc::new(RefCell::new(test_equal_to_int_::new_1()));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(
            ({
                let _x : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(( { (*c.borrow()) . get_allocator ( ) } )  ));
                operator_eq_5(_x.as_pointer(), a.as_pointer())
            })
        );
        assert!(
            !({
                let _x : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(( { (*c.borrow()) . get_allocator ( ) } )  ));
                let _y : Value<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ > = Rc::new(RefCell::new(<test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  >::default() ));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__;
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
    fn address_11(
        &self,
        x: Ptr<(Value<i32>, Value<Vec<i8>>)>,
    ) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
        unimplemented!()
    }
    fn address_12(
        &self,
        x: Ptr<(Value<i32>, Value<Vec<i8>>)>,
    ) -> Ptr<(Value<i32>, Value<Vec<i8>>)> {
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
fn destructor ( &self , ) { assert!(( (* self ) .with(|__s| __s . data_ ) != destructed_value_1 .with(|rc| *rc.borrow()) ) )  ;
 assert!(( (* self ) .with(|__s| __s . id_ ) != destructed_value_1 .with(|rc| *rc.borrow()) ) )  ;
 if ! ( ( (* self ) .with(|__s| __s . stats_ .clone()) ).is_null() ) { field!((* self ) .with(|__s| __s . stats_ .clone()) , count) .with_mut(|__v| { __v. prefix_dec (); }) ;
 } field!((* self ) , data_) .write( destructed_value_1 .with(|rc| *rc.borrow())  ) ;
 field!((* self ) , id_) .write( destructed_value_1 .with(|rc| *rc.borrow())  ) ;
 } }
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
