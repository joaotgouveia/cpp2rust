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
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
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
impl min_allocator_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct explicit_allocator_bool_ {}
impl explicit_allocator_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct explicit_allocator_unsigned_long_ {}
impl explicit_allocator_unsigned_long_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for explicit_allocator_unsigned_long_ {
    fn default() -> Self {
        { explicit_allocator_unsigned_long_::new() }
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
    pub static destructed_value_4: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_5: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_6(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_7(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_6(_x, _y)
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
        operator_eq_6(__lhs.clone(), __rhs.clone())
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
pub fn tests_8() -> bool {
    {
        let mut v: Vec<bool> = Vec::new();
        if 10_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(10_usize as usize - len_0 as usize);
        };
        assert!((v.capacity() >= 10_usize));
    }
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        assert!((v.capacity() >= 100_usize));
        if 50_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(50_usize as usize - len_0 as usize);
        };
        assert!((v.len() == 100_usize));
        assert!((v.capacity() >= 100_usize));
        if 150_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(150_usize as usize - len_0 as usize);
        };
        assert!((v.len() == 100_usize));
        assert!((v.capacity() >= 150_usize));
    }
    {
        let mut v: Vec<bool> = Vec::new();
        if 10_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(10_usize as usize - len_0 as usize);
        };
        assert!((v.capacity() >= 10_usize));
    }
    {
        let mut v: Vec<bool> = Vec::new();
        if 10_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(10_usize as usize - len_0 as usize);
        };
        assert!((v.capacity() >= 10_usize));
    }
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        assert!((v.capacity() >= 100_usize));
        if 50_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(50_usize as usize - len_0 as usize);
        };
        assert!((v.len() == 100_usize));
        assert!((v.capacity() >= 100_usize));
        if 150_usize as usize > v.capacity() as usize {
            let len_0 = v.len();
            v.reserve_exact(150_usize as usize - len_0 as usize);
        };
        assert!((v.len() == 100_usize));
        assert!((v.capacity() >= 150_usize));
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
    ({ tests_8() });
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
pub trait explicit_allocator_bool_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: usize) {
        unimplemented!()
    }
}
impl explicit_allocator_bool_Impl for Ptr<explicit_allocator_bool_> {}
pub trait explicit_allocator_unsigned_long_Impl {
    fn allocate(&self, n: usize) -> Ptr<u64>;
    fn deallocate(&self, p: Ptr<u64>, n: usize);
}
impl explicit_allocator_unsigned_long_Impl for Ptr<explicit_allocator_unsigned_long_> {
    fn allocate(&self, mut n: usize) -> Ptr<u64> {
        return ({ <std_allocator_unsigned_long_>::default().allocate(n) })
            .reinterpret_cast::<u64>();
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: usize) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub trait min_allocator_bool_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_bool__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_bool__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_bool_Impl for Ptr<min_allocator_bool_> {}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_unsigned_long_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({
            let ___p: Ptr<u64> = (p.ptr_).clone();
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
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
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
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
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = destructed_value_4.with(|_| ());
    let _ = moved_value_5.with(|_| ());
}
