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
pub fn operator_eq_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_6(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_7(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow_mut());
        }
    }
    return std::cmp::Ordering::Equal;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: i32,
}
impl MoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0);
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_13(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_14(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow_mut());
        }
    }
    return std::cmp::Ordering::Equal;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: i32,
}
impl TrivialMoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl CppLess for TrivialMoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_10(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_16.with(|rc| *rc.borrow()) >= dealloc_count_17.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_16.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_17.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_18.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_15.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_16.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_17.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_15: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_16: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_17: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_18: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_MoveOnly_ {}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {
    pub fn new_1(mut _a0: min_allocator_MoveOnly_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct safe_allocator_MoveOnly_ {}
impl safe_allocator_MoveOnly_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for safe_allocator_MoveOnly_ {
    fn default() -> Self {
        { safe_allocator_MoveOnly_::new() }
    }
}
impl safe_allocator_MoveOnly_ptr_ {
    pub fn new_1(mut _a0: safe_allocator_MoveOnly_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct safe_allocator_MoveOnly_ptr_ {}
impl safe_allocator_MoveOnly_ptr_ {}
pub fn make_19(mut size: i32, mut start: i32) -> Vec<MoveOnly> {
    let mut b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if (start > 0) {
        init = (((start + 1) / b) + ((((start + 1) % b) != 0) as i32));
        init *= b;
        init.prefix_dec();
    }
    let c: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(
        std_deque_MoveOnly__std_allocator_MoveOnly__::new_1({ (init as u64) }, None),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < (init - start)) {
        ({ (*c.borrow_mut()).pop_back() });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        (*c.borrow_mut()).push(MoveOnly::new({ Some(i) }));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < start) {
        (*c.borrow_mut()).remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut (*c.borrow_mut()));
}
pub fn test_20(mut size: i32) {
    let mut rng: [i32; 11] = [0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049];
    let mut N: i32 = (((::std::mem::size_of::<[i32; 11]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut j: i32 = 0;
    'loop_: while (j < N) {
        let c: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(({ make_19(size, rng[(j) as usize]) })));
        let it: Value<RandomAccessIterator<MoveOnly>> = Rc::new(RefCell::new(
            <RandomAccessIterator<MoveOnly>>::from(({ (*c.borrow_mut()).begin_24() })),
        ));
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while (i < size) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*it.borrow()).iter_deref();
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(i) })));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            {
                i.prefix_inc();
                &((*it.borrow_mut()).iter_inc())
            };
        }
        j.prefix_inc();
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
        let mut rng: [i32; 14] = [
            0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049, 4094, 4095, 4096,
        ];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 14]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as i32);
        let mut j: i32 = 0;
        'loop_: while (j < N) {
            ({ test_20(rng[(j) as usize]) });
            j.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 14] = [
            0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049, 4094, 4095, 4096,
        ];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 14]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as i32);
        let mut j: i32 = 0;
        'loop_: while (j < N) {
            ({ test_21(rng[(j) as usize]) });
            j.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 14] = [
            0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049, 4094, 4095, 4096,
        ];
        let mut N: i32 = (((::std::mem::size_of::<[i32; 14]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as i32);
        let mut j: i32 = 0;
        'loop_: while (j < N) {
            ({ test_22(rng[(j) as usize]) });
            j.prefix_inc();
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0);
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait min_allocator_MoveOnly_Impl {
    fn allocate(&self, n: usize) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_MoveOnly_Impl for Ptr<min_allocator_MoveOnly_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_MoveOnly_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({ <std_allocator_MoveOnly_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub trait min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , n : usize  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___>
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( { <std_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___  >::default() . allocate ( n  , ) } )   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({
            <std_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___  >::default() . deallocate ( (p . ptr_  ).clone() , n  , )
        });
    }
}
pub trait safe_allocator_MoveOnly_Impl {
    fn allocate(&self, n: usize) -> Ptr<MoveOnly>;
    fn deallocate(&self, p: Ptr<MoveOnly>, n: usize);
}
impl safe_allocator_MoveOnly_Impl for Ptr<safe_allocator_MoveOnly_> {
    fn allocate(&self, mut n: usize) -> Ptr<MoveOnly> {
        let memory: Value<Ptr<MoveOnly>> = Rc::new(RefCell::new(
            ({ <std_allocator_MoveOnly_>::default().allocate(n) }),
        ));
        {
            {
                (*memory.borrow()).to_any().memset(
                    (0) as u8,
                    ((4usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                );
                (*memory.borrow()).to_any()
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<MoveOnly>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new({
                    (p).to_any().memset(
                        (0) as u8,
                        ((4usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                    );
                    (p).to_any()
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_MoveOnly_>::default().deallocate((p).clone(), n) });
    }
}
pub trait safe_allocator_MoveOnly_ptr_Impl {
    fn allocate(&self, n: usize) -> Ptr<Ptr<MoveOnly>>;
    fn deallocate(&self, p: Ptr<Ptr<MoveOnly>>, n: usize);
}
impl safe_allocator_MoveOnly_ptr_Impl for Ptr<safe_allocator_MoveOnly_ptr_> {
    fn allocate(&self, mut n: usize) -> Ptr<Ptr<MoveOnly>> {
        let memory: Value<Ptr<Ptr<MoveOnly>>> = Rc::new(RefCell::new(
            ({ <std_allocator_MoveOnly_ptr_>::default().allocate(n) }),
        ));
        {
            {
                (*memory.borrow()).to_any().memset(
                    (0) as u8,
                    ((8usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                );
                (*memory.borrow()).to_any()
            };
        }
        return (*memory.borrow()).clone();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<MoveOnly>>, mut n: usize) {
        {
            ({
                let _value: Value<AnyPtr> = Rc::new(RefCell::new({
                    (p).to_any().memset(
                        (0) as u8,
                        ((8usize as u64).wrapping_mul((n as u64)) as usize) as usize,
                    );
                    (p).to_any()
                }));
                DoNotOptimize_0(_value.as_pointer())
            });
        }
        ({ <std_allocator_MoveOnly_ptr_>::default().deallocate((p).clone(), n) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_15.with(|_| ());
    let _ = alloc_count_16.with(|_| ());
    let _ = dealloc_count_17.with(|_| ());
    let _ = disable_default_constructor_18.with(|_| ());
}
