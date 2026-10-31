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
pub fn operator_eq_4(x: Ptr<test_allocator_int_>, y: Ptr<test_allocator_int_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_5(x: Ptr<test_allocator_int_>, y: Ptr<test_allocator_int_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_int_> = (x).clone();
        let _y: Ptr<test_allocator_int_> = (y).clone();
        operator_eq_4(_x, _y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ptr_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
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
    pub fn new_1(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
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
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for test_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
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
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: 0_i32,
                id_: 0,
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
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
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
pub fn operator_eq_6(x: Ptr<other_allocator_int_>, y: Ptr<other_allocator_int_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_7(x: Ptr<other_allocator_int_>, y: Ptr<other_allocator_int_>) -> bool {
    return !({
        let _x: Ptr<other_allocator_int_> = (x).clone();
        let _y: Ptr<other_allocator_int_> = (y).clone();
        operator_eq_6(_x, _y)
    });
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
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
impl Default for other_allocator_int_ {
    fn default() -> Self {
        { other_allocator_int_ { data_: 0_i32 } }
    }
}
impl other_allocator_int_ptr_ {
    pub fn new_1(a: Ptr<other_allocator_int_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(other_allocator_int_, new_1, ptr)]
#[byte_size(4)]
pub struct other_allocator_int_ptr_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_int_ptr_ {}
impl Default for other_allocator_int_ptr_ {
    fn default() -> Self {
        { other_allocator_int_ptr_ { data_: 0_i32 } }
    }
}
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
        assert!(
            (alloc_count_9.with(|rc| *rc.borrow()) >= dealloc_count_10.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_9.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_10.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_11.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_8.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_9.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_10.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_8: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_9: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_10: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_11: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_13(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<i32>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl CppEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_15(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_16(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_17(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_18(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_17(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(AnyPtr, new_1, value)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn new_2 ( mut p : Ptr< min_pointer_int__std_integral_constant_unsigned_long__0__  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } pub fn new_1 ( mut _a0 : AnyPtr  , ) -> Self { Self { ptr_ : Ptr::<min_pointer_int__std_integral_constant_unsigned_long__0__>::null()  , }  } }
impl CppLess for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_lt_16(__lhs.read(), __rhs.read()) } }
impl CppEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_17(__lhs.read(), __rhs.read()) } }
impl CppAdvance for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_8(); } fn cpp_dec(__it: &Ptr<Self>) { __it.operator_dec_10(); } }
pub fn operator_eq_19(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
pub fn operator_ne_20(mut x: min_allocator_int_, mut y: min_allocator_int_) -> bool {
    return !({
        let _arg0: min_allocator_int_ = x;
        operator_eq_19(_arg0, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_19(__lhs.read(), __rhs.read())
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_allocator_int_, new_1, value)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub fn test_21(x: Ptr<Vec<i32>>) {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    c.as_pointer().write((*x.upgrade().deref()).clone());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_22(x: Ptr<Vec<i32>>) {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    c.as_pointer().write((*x.upgrade().deref()).clone());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
        let ab: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45,
        ])));
        let mut an: Ptr<i32> = (ab.as_pointer() as Ptr<i32>).offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        ({
            let _x: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
            test_21(_x.as_pointer())
        });
    }
    {
        let l: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        l2.as_pointer().write((*l.borrow()).clone());
        assert!(unimplemented!());
        assert!(
            ({
                let _x: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
                let _y: Value<test_allocator_int_> =
                    Rc::new(RefCell::new(test_allocator_int_::new_1({ 3 })));
                operator_eq_4(_x.as_pointer(), _y.as_pointer())
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let l: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        l2.as_pointer().write((*l.borrow()).clone());
        assert!(unimplemented!());
        assert!(
            ({
                let _x: Value<other_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
                let _y: Value<other_allocator_int_> =
                    Rc::new(RefCell::new(other_allocator_int_::new_1({ 5 })));
                operator_eq_6(_x.as_pointer(), _y.as_pointer())
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let ab: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45,
        ])));
        let mut an: Ptr<i32> = (ab.as_pointer() as Ptr<i32>).offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        ({
            let _x: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
            test_22(_x.as_pointer())
        });
    }
    {
        let l: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        l2.as_pointer().write((*l.borrow()).clone());
        assert!(unimplemented!());
        assert!(
            ({
                let _arg0: min_allocator_int_ = unimplemented!();
                operator_eq_19(_arg0, <min_allocator_int_>::default())
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
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
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_2 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: i32,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: isize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_arrow(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_add ( &self , n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn operator_index(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_bool ( &self , ) -> bool  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_int__std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_dec ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() - n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } }
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait other_allocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, s: usize);
    fn select_on_container_copy_construction(&self) -> other_allocator_int_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_int_Impl for Ptr<other_allocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: usize) {
        unimplemented!();
    }
}
pub trait other_allocator_int_ptr_Impl {
    fn allocate(&self, n: usize) -> Ptr<Ptr<i32>>;
    fn deallocate(&self, p: Ptr<Ptr<i32>>, s: usize);
    fn select_on_container_copy_construction(&self) -> other_allocator_int_ptr_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_int_ptr_Impl for Ptr<other_allocator_int_ptr_> {
    fn allocate(&self, mut n: usize) -> Ptr<Ptr<i32>> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<i32>>, mut s: usize) {
        unimplemented!();
    }
}
pub trait test_allocator_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<i32> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<i32> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, s: u32);
    fn max_size(&self) -> u32;
    fn destroy(&self, p: Ptr<i32>);
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
    fn construct_16_const_int_ref(&self, p: Ptr<i32>, val: Ptr<i32>);
    fn construct_16_int(&self, p: Ptr<i32>, val: Ptr<i32>);
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
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
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_add(n)) as i32
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
        unimplemented!();
    }
    fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    }
    fn destroy(&self, mut p: Ptr<i32>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_const_int_ref(&self, mut p: Ptr<i32>, val: Ptr<i32>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
    fn construct_16_int(&self, mut p: Ptr<i32>, val: Ptr<i32>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_int_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<i32>> {
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
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
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
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                (((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size) as u32)
                    .wrapping_add(n)) as i32
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<i32>>, mut s: u32) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
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
        unimplemented!();
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
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
    let _ = outstanding_bytes_8.with(|_| ());
    let _ = alloc_count_9.with(|_| ());
    let _ = dealloc_count_10.with(|_| ());
    let _ = disable_default_constructor_11.with(|_| ());
}
