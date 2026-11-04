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
    pub static destructed_value_0: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_1: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_2(_x, _y)
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
        operator_eq_2(__lhs.clone(), __rhs.clone())
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
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_short_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_short_ {
    pub fn new_1(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
}
impl Default for test_allocator_short_ {
    fn default() -> Self {
        {
            test_allocator_short_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_int_ {
    pub fn new_1(a: Ptr<test_allocator_short_>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
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
pub struct test_allocator_int__rebind_int_ {}
impl CppMove for test_allocator_int__rebind_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ptr_ {}
impl CppMove for test_allocator_int__rebind_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(test_allocator_short_, new_1, ptr)]
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
    pub fn copy_from(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
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
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_int_> = Rc::new(RefCell::new(test_allocator_int_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_int_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl test_allocator_long_ {
    pub fn new_1(a: Ptr<test_allocator_short_>) -> Self {
        let __this: Value<test_allocator_long_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_long_> = __this.as_pointer();
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
pub struct test_allocator_long__rebind_long_ {}
impl CppMove for test_allocator_long__rebind_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_long__rebind_long_ptr_ {}
impl CppMove for test_allocator_long__rebind_long_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(test_allocator_short_, new_1, ptr)]
#[byte_size(16)]
pub struct test_allocator_long_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_long_ {
    pub fn copy_from(a: Ptr<test_allocator_long_>) -> Self {
        let __this: Value<test_allocator_long_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_long_> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
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
impl Clone for test_allocator_long_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_long_> = Rc::new(RefCell::new(test_allocator_long_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_long_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_long_ {
    fn default() -> Self {
        {
            test_allocator_long_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_int_ptr_ {
    pub fn new_1(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_ptr_> = __this.as_pointer();
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
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(test_allocator_int_, new_1, ptr)]
#[byte_size(16)]
pub struct test_allocator_int_ptr_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ptr_ {}
impl Default for test_allocator_int_ptr_ {
    fn default() -> Self {
        {
            test_allocator_int_ptr_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_long_ptr_ {
    pub fn new_1(a: Ptr<test_allocator_long_>) -> Self {
        let __this: Value<test_allocator_long_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_long_ptr_> = __this.as_pointer();
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
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, Destructor)]
#[cpp_from(test_allocator_long_, new_1, ptr)]
#[byte_size(16)]
pub struct test_allocator_long_ptr_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_long_ptr_ {}
impl Default for test_allocator_long_ptr_ {
    fn default() -> Self {
        {
            test_allocator_long_ptr_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_int_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_int_ {}
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
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: i32,
}
impl test_less_int_ {}
impl CppMove for test_less_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_4() {
    let m : Value<std_flat_map_int__long__std_less_int___std_vector_int__test_allocator_int____std_vector_long__test_allocator_long___ > = Rc::new(RefCell::new({ let __tmp_0 : Value<test_allocator_short_ > = Rc::new(RefCell::new(test_allocator_short_ :: new_1 ( { 0  } , { 5  } , )  )); std_flat_map_int__long__std_less_int___std_vector_int__test_allocator_int____std_vector_long__test_allocator_long___ :: new_1 ( { __tmp_0.as_pointer() } , )  } )) ;
    assert!(
        ({
            ({
                ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_vector_int__test_allocator_int____std_vector_long__test_allocator_long____false_ ) . empty ( )
            })
        })
    );
    assert!(
        ({
            ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_vector_int__test_allocator_int____std_vector_long__test_allocator_long____false_ ) . begin_31 ( )
        }) == ({
            ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_vector_int__test_allocator_int____std_vector_long__test_allocator_long____false_ ) . end_33 ( )
        })
    );
    assert!(
        (({
            test_allocator_int_Impl :: get_id ( &Rc::new(RefCell::new(( { ( (*( { ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_vector_int__test_allocator_int____std_vector_long__test_allocator_long____false_ ) . keys ( ) } ) .upgrade().deref()) as std__Vector_base_int__test_allocator_int__ ) . get_allocator  ( ) } )  )).as_pointer() , )
        }) == 5)
    );
    assert!(
        (({
            test_allocator_long_Impl :: get_id ( &Rc::new(RefCell::new(( { ( (*( { ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_vector_int__test_allocator_int____std_vector_long__test_allocator_long____false_ ) . values ( ) } ) .upgrade().deref()) as std__Vector_base_long__test_allocator_long__ ) . get_allocator  ( ) } )  )).as_pointer() , )
        }) == 5)
    );
}
pub fn test_5() {
    let m : Value<std_flat_map_int__long__std_less_int___std_deque_int__test_allocator_int____std_deque_long__test_allocator_long___ > = Rc::new(RefCell::new({ let __tmp_1 : Value<test_allocator_short_ > = Rc::new(RefCell::new(test_allocator_short_ :: new_1 ( { 0  } , { 5  } , )  )); std_flat_map_int__long__std_less_int___std_deque_int__test_allocator_int____std_deque_long__test_allocator_long___ :: new_1 ( { __tmp_1.as_pointer() } , )  } )) ;
    assert!(
        ({
            ({
                ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_deque_int__test_allocator_int____std_deque_long__test_allocator_long____false_ ) . empty ( )
            })
        })
    );
    assert!(
        ({
            ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_deque_int__test_allocator_int____std_deque_long__test_allocator_long____false_ ) . begin_31 ( )
        }) == ({
            ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_deque_int__test_allocator_int____std_deque_long__test_allocator_long____false_ ) . end_33 ( )
        })
    );
    assert!(
        (({
            test_allocator_int_Impl :: get_id ( &Rc::new(RefCell::new(( { (*( { ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_deque_int__test_allocator_int____std_deque_long__test_allocator_long____false_ ) . keys ( ) } ) .upgrade().deref()) . get_allocator  ( ) } )  )).as_pointer() , )
        }) == 5)
    );
    assert!(
        (({
            test_allocator_long_Impl :: get_id ( &Rc::new(RefCell::new(( { (*( { ( (*m.borrow()) as std__Flat_map_impl_int__long__std_less_int___std_deque_int__test_allocator_int____std_deque_long__test_allocator_long____false_ ) . values ( ) } ) .upgrade().deref()) . get_allocator  ( ) } )  )).as_pointer() , )
        }) == 5)
    );
}
pub fn test_6() -> bool {
    {
        const _: () = assert!(true, "std::is_constructible_v<M1, const A1&>");;
        const _: () = assert!(true, "!std::is_constructible_v<M1, const A2&>");;
        const _: () = assert!(true, "!std::is_constructible_v<M2, const A2&>");;
        const _: () = assert!(true, "!std::is_constructible_v<M3, const A2&>");;
    }
    {
        const _: () = assert!(true, "std::is_constructible_v<M, test_allocator<int>>");;
        const _: () = assert!(true, "!std::is_convertible_v<test_allocator<int>, M>");;
    }
    ({ test_4() });
    {
        ({ test_5() });
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
    ({ test_6() });
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
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, s: u32);
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<i32>);
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32;
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<i32> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
                std::process::abort();
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
        return ({ <std_allocator_int_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
        ({ <std_allocator_int_>::default().deallocate((p).clone(), (s as usize)) });
    }
    fn destroy(&self, mut p: Ptr<i32>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count).with_mut(|__v| {
                __v.prefix_inc();
            });
        };
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait test_allocator_int_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<Ptr<i32>>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<Ptr<i32>>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<Ptr<i32>>;
    fn deallocate(&self, p: Ptr<Ptr<i32>>, s: u32);
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<i32>>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_int_ptr_Impl for Ptr<test_allocator_int_ptr_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<Ptr<i32>> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
                std::process::abort();
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
        return ({ <std_allocator_int_ptr_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<Ptr<i32>>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
        ({ <std_allocator_int_ptr_>::default().deallocate((p).clone(), (s as usize)) });
    }
}
pub trait test_allocator_long_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<i64>) -> Ptr<i64> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<i64>) -> Ptr<i64> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<i64>;
    fn deallocate(&self, p: Ptr<i64>, s: u32);
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<i64>);
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32;
}
impl test_allocator_long_Impl for Ptr<test_allocator_long_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<i64> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
                std::process::abort();
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
        return ({ <std_allocator_long_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<i64>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
        ({ <std_allocator_long_>::default().deallocate((p).clone(), (s as usize)) });
    }
    fn destroy(&self, mut p: Ptr<i64>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count).with_mut(|__v| {
                __v.prefix_inc();
            });
        };
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait test_allocator_long_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<Ptr<i64>>) -> Ptr<Ptr<i64>> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<Ptr<i64>>) -> Ptr<Ptr<i64>> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<Ptr<i64>>;
    fn deallocate(&self, p: Ptr<Ptr<i64>>, s: u32);
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<i64>>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_long_ptr_Impl for Ptr<test_allocator_long_ptr_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<Ptr<i64>> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
                std::process::abort();
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
        return ({ <std_allocator_long_ptr_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<Ptr<i64>>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
        ({ <std_allocator_long_ptr_>::default().deallocate((p).clone(), (s as usize)) });
    }
}
pub trait test_allocator_short_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<i16>) -> Ptr<i16> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<i16>) -> Ptr<i16> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<i16> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i16>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<i16>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_short_Impl for Ptr<test_allocator_short_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
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
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
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
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool {
        unimplemented!()
    }
}
impl test_less_int_Impl for Ptr<test_less_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
}
