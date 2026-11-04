extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn DoNotOptimize_0(value: Ptr<AnyPtr>) -> Ptr<AnyPtr> {
    b"";
    return (value).clone();
}
pub fn operator_eq_2(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_3(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: i32,
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_i32 };
        (*count_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for DefaultOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for DefaultOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> u64 {
        ({
            cpp2rust_assert_fail_8(
                (alloc_count_5.with(|rc| *rc.borrow()) >= dealloc_count_6.with(|rc| *rc.borrow())),
            )
        });
        return ((alloc_count_5.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_6.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        ({ cpp2rust_assert_fail_8((({ malloc_allocator_base::outstanding_alloc() }) == 0_u64)) });
        disable_default_constructor_7.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_4.with(|rc| *rc.borrow_mut() = 0_u64);
        alloc_count_5.with(|rc| *rc.borrow_mut() = 0_u64);
        dealloc_count_6.with(|rc| *rc.borrow_mut() = 0_u64);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_4: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static alloc_count_5: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static dealloc_count_6: Value<u64> = Rc::new(RefCell::new(0_u64));
);
thread_local!(
    pub static disable_default_constructor_7: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_eq_9(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppEq for min_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_9(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_10(
    mut _a0: min_allocator_DefaultOnly_,
    mut _a1: min_allocator_DefaultOnly_,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_DefaultOnly_ {}
impl CppEq for min_allocator_DefaultOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_DefaultOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_11(mut _a0: safe_allocator_int_, mut _a1: safe_allocator_int_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppEq for safe_allocator_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_11(__lhs.read(), __rhs.read())
    }
}
impl CppMove for safe_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        { safe_allocator_int_::new() }
    }
}
pub fn operator_eq_13(
    mut _a0: safe_allocator_DefaultOnly_,
    mut _a1: safe_allocator_DefaultOnly_,
) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_DefaultOnly_ {}
impl safe_allocator_DefaultOnly_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppEq for safe_allocator_DefaultOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.read(), __rhs.read())
    }
}
impl CppMove for safe_allocator_DefaultOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for safe_allocator_DefaultOnly_ {
    fn default() -> Self {
        { safe_allocator_DefaultOnly_::new() }
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
    pub static destructed_value_14: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_15: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_16(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_17(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_16(_x, _y)
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
        operator_eq_16(__lhs.clone(), __rhs.clone())
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
pub fn operator_eq_18(
    x: Ptr<test_allocator_DefaultOnly_>,
    y: Ptr<test_allocator_DefaultOnly_>,
) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_DefaultOnly__rebind_DefaultOnly_ {}
impl CppMove for test_allocator_DefaultOnly__rebind_DefaultOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_DefaultOnly_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_DefaultOnly_ {
    pub fn new_1(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_DefaultOnly_>) -> Self {
        let __this: Value<test_allocator_DefaultOnly_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_DefaultOnly_> = __this.as_pointer();
        ({
            cpp2rust_assert_fail_8(
                (({ a.with(|__s| __s.data_) } != { destructed_value_14.with(|rc| *rc.borrow()) })
                    && ({ a.with(|__s| __s.id_) } != {
                        destructed_value_14.with(|rc| *rc.borrow())
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
impl CppEq for test_allocator_DefaultOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_18(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_DefaultOnly_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_DefaultOnly_> =
            Rc::new(RefCell::new(test_allocator_DefaultOnly_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_DefaultOnly_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_DefaultOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_DefaultOnly_ {
    fn default() -> Self {
        {
            test_allocator_DefaultOnly_ {
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
pub fn tests_20() -> bool {
    ({ test_21(0_u64, None) });
    ({ test_21(50_u64, None) });
    ({ test_22(0_u64, None) });
    ({ test_22(50_u64, None) });
    ({ test_23(0_u64, None) });
    ({ test_23(50_u64, None) });
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
    ({ tests_20() });
    const _: () = assert!(true, "tests()");;
    ({ test_24(0_u64, None) });
    ({ test_24(500_u64, None) });
    ({ cpp2rust_assert_fail_8((count_1.with(|rc| *rc.borrow()) == 0)) });
    ({ test_25(0_u64, None) });
    ({ test_25(500_u64, None) });
    ({ test_26(0_u64, None) });
    ({ test_26(500_u64, None) });
    ({
        let _a: Value<test_allocator_DefaultOnly_> =
            Rc::new(RefCell::new(test_allocator_DefaultOnly_::new_1({ 23 })));
        test_27(0_u64, _a.as_pointer())
    });
    ({
        let _a: Value<test_allocator_DefaultOnly_> =
            Rc::new(RefCell::new(test_allocator_DefaultOnly_::new_1({ 23 })));
        test_27(100_u64, _a.as_pointer())
    });
    ({ cpp2rust_assert_fail_8((count_1.with(|rc| *rc.borrow()) == 0)) });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__;
pub trait DefaultOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl DefaultOnlyImpl for Ptr<DefaultOnly> {
    fn destructor(&self) {
        field!((*self), data_).write(0);
        (*count_1.with(Value::clone).borrow_mut()).prefix_dec();
    }
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
pub trait min_allocator_DefaultOnly_Impl {
    fn allocate(&self, n: u64)
    -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        n: u64,
    );
}
impl min_allocator_DefaultOnly_Impl for Ptr<min_allocator_DefaultOnly_> {
    fn allocate(
        &self,
        mut n: u64,
    ) -> min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_DefaultOnly_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({ <std_allocator_DefaultOnly_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: u64) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: u64);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: u64) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_int_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: u64,
    ) {
        ({ <std_allocator_int_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait safe_allocator_DefaultOnly_Impl {
    fn allocate(&self, n: u64) -> Ptr<DefaultOnly>;
    fn deallocate(&self, p: Ptr<DefaultOnly>, n: u64);
}
impl safe_allocator_DefaultOnly_Impl for Ptr<safe_allocator_DefaultOnly_> {
    fn allocate(&self, mut n: u64) -> Ptr<DefaultOnly> {
        let memory: Value<Ptr<DefaultOnly>> = Rc::new(RefCell::new(
            ({ <std_allocator_DefaultOnly_>::default().allocate(n) }),
        ));
        {
            ({
                memset_12(
                    (*memory.borrow()).to_any(),
                    0,
                    (4usize as u64).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<DefaultOnly>, mut n: u64) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(
                    ({ memset_12((p).to_any(), 0, (4usize as u64).wrapping_mul(n)) }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_DefaultOnly_>::default().deallocate((p).clone(), n) });
    }
}
pub trait safe_allocator_int_Impl {
    fn allocate(&self, n: u64) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, n: u64);
}
impl safe_allocator_int_Impl for Ptr<safe_allocator_int_> {
    fn allocate(&self, mut n: u64) -> Ptr<i32> {
        let memory: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ <std_allocator_int_>::default().allocate(n) }),
        ));
        {
            ({
                let ___s: AnyPtr = (*memory.borrow()).to_any();
                memset_12(
                    ___s,
                    0,
                    (::std::mem::size_of::<i32>() as u64).wrapping_mul(n),
                )
            });
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: u64) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new(
                    ({
                        let ___s: AnyPtr = (p).to_any();
                        memset_12(
                            ___s,
                            0,
                            (::std::mem::size_of::<i32>() as u64).wrapping_mul(n),
                        )
                    }),
                ));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_int_>::default().deallocate((p).clone(), n) });
    }
}
pub trait test_allocator_DefaultOnly_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn allocate(&self, n: u32, _a1: Option<AnyPtr>) -> Ptr<DefaultOnly>;
    fn deallocate(&self, p: Ptr<DefaultOnly>, s: u32);
    fn max_size(&self) -> u32;
    fn destroy(&self, p: Ptr<DefaultOnly>);
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_DefaultOnly_Impl for Ptr<test_allocator_DefaultOnly_> {
    fn destructor(&self) {
        ({
            cpp2rust_assert_fail_8(
                ((*self).with(|__s| __s.data_) != destructed_value_14.with(|rc| *rc.borrow())),
            )
        });
        ({
            cpp2rust_assert_fail_8(
                ((*self).with(|__s| __s.id_) != destructed_value_14.with(|rc| *rc.borrow())),
            )
        });
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: u32, _a1: Option<AnyPtr>) -> Ptr<DefaultOnly> {
        let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
        ({
            cpp2rust_assert_fail_8(
                ((*self).with(|__s| __s.data_) != destructed_value_14.with(|rc| *rc.borrow())),
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
                ({ __builtin_abort_19() });
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
        return ({ <std_allocator_DefaultOnly_>::default().allocate((n as u64)) });
    }
    fn deallocate(&self, mut p: Ptr<DefaultOnly>, mut s: u32) {
        ({
            cpp2rust_assert_fail_8(
                ((*self).with(|__s| __s.data_) != destructed_value_14.with(|rc| *rc.borrow())),
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
        ({ <std_allocator_DefaultOnly_>::default().deallocate((p).clone(), (s as u64)) });
    }
    fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as u64)
            .wrapping_div((4usize as u64))) as u32);
    }
    fn destroy(&self, mut p: Ptr<DefaultOnly>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        ({ DefaultOnlyImpl::destructor(&p) });
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        ({
            cpp2rust_assert_fail_8(
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
        field!((*self), data_).write(destructed_value_14.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_14.with(|rc| *rc.borrow()));
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
    let _ = count_1.with(|_| ());
    let _ = outstanding_bytes_4.with(|_| ());
    let _ = alloc_count_5.with(|_| ());
    let _ = dealloc_count_6.with(|_| ());
    let _ = disable_default_constructor_7.with(|_| ());
    let _ = destructed_value_14.with(|_| ());
    let _ = moved_value_15.with(|_| ());
}
