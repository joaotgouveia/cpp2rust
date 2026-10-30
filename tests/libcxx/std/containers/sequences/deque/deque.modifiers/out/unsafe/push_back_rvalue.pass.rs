extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const *mut ::libc::c_void) -> *const *mut ::libc::c_void {
    c"";
    return value;
}
pub unsafe fn operator_eq_1(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_2(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_3(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_4(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_5(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_6(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_7(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_7(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_13(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_14(
    _a0: *const TrivialMoveOnly,
    _a1: *const TrivialMoveOnly,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: i32,
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_14(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_8(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_16))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_17)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_16))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_17))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_18)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_15)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_16)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_17)) = 0_usize;
    }
}
pub static mut outstanding_bytes_15: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_16: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_17: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_18: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_MoveOnly_ {}
impl min_allocator_MoveOnly_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_MoveOnly_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_MoveOnly_>::default().deallocate(p.ptr_, n) });
    }
}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_MoveOnly_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( unsafe { <std_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ >::default() . allocate  ( n , ) } )  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            <std_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ >::default() . deallocate  ( p . ptr_  , n , )
        });
    }
}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_MoveOnly_ {}
impl safe_allocator_MoveOnly_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut MoveOnly {
        let mut memory: *mut MoveOnly =
            (unsafe { <std_allocator_MoveOnly_>::default().allocate(n) });
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in 0..((::std::mem::size_of::<MoveOnly>() as u64)
                    .wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut MoveOnly, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..((::std::mem::size_of::<MoveOnly>() as u64)
                        .wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        (unsafe { <std_allocator_MoveOnly_>::default().deallocate(p, n) });
    }
}
impl Default for safe_allocator_MoveOnly_ {
    fn default() -> Self {
        unsafe { safe_allocator_MoveOnly_::new() }
    }
}
impl safe_allocator_MoveOnly_ptr_ {
    pub unsafe fn new_1(mut _a0: safe_allocator_MoveOnly_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_MoveOnly_ptr_ {}
impl safe_allocator_MoveOnly_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut MoveOnly {
        let mut memory: *mut *mut MoveOnly =
            (unsafe { <std_allocator_MoveOnly_ptr_>::default().allocate(n) });
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in 0..((::std::mem::size_of::<*mut MoveOnly>() as u64)
                    .wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut MoveOnly, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..((::std::mem::size_of::<*mut MoveOnly>() as u64)
                        .wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        (unsafe { <std_allocator_MoveOnly_ptr_>::default().deallocate(p, n) });
    }
}
pub unsafe fn make_19(mut size: i32, mut start: i32) -> Vec<MoveOnly> {
    let b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if ((start) > (0)) {
        init = ((((start) + (1)) / (b)) + (((((start) + (1)) % (b)) != (0)) as i32));
        init *= b;
        init.prefix_dec();
    }
    let mut c: Vec<MoveOnly> =
        std_deque_MoveOnly__std_allocator_MoveOnly__::new_1({ (init as u64) }, None);
    let mut i: i32 = 0;
    'loop_: while ((i) < ((init) - (start))) {
        (unsafe { c.pop_back() });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        c.push(MoveOnly::new({ Some(i) }));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (start)) {
        c.remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub unsafe fn test_20(mut size: i32) {
    let mut rng: [i32; 11] = [0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049];
    let N: i32 = (((::std::mem::size_of::<[i32; 11]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut j: i32 = 0;
    'loop_: while ((j) < (N)) {
        let mut c: Vec<MoveOnly> = (unsafe { make_19(size, rng[(j) as usize]) });
        let mut it: UnsafeRandomAccessIterator<MoveOnly> =
            <UnsafeRandomAccessIterator<MoveOnly>>::from((unsafe { c.begin_24() }));
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while ((i) < (size)) {
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*it.iter_deref());
                    let mut _y: MoveOnly = MoveOnly::new({ Some(i) });
                    operator_eq_1(_x, &mut _y)
                })
            );
            {
                i.prefix_inc();
                &(it.iter_inc())
            };
        }
        j.prefix_inc();
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut rng: [i32; 14] = [
            0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049, 4094, 4095, 4096,
        ];
        let N: i32 = (((::std::mem::size_of::<[i32; 14]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut j: i32 = 0;
        'loop_: while ((j) < (N)) {
            (unsafe { test_20(rng[(j) as usize]) });
            j.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 14] = [
            0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049, 4094, 4095, 4096,
        ];
        let N: i32 = (((::std::mem::size_of::<[i32; 14]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut j: i32 = 0;
        'loop_: while ((j) < (N)) {
            (unsafe { test_21(rng[(j) as usize]) });
            j.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 14] = [
            0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049, 4094, 4095, 4096,
        ];
        let N: i32 = (((::std::mem::size_of::<[i32; 14]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut j: i32 = 0;
        'loop_: while ((j) < (N)) {
            (unsafe { test_22(rng[(j) as usize]) });
            j.prefix_inc();
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_15);
    std::cell::LazyCell::force(&*&raw const alloc_count_16);
    std::cell::LazyCell::force(&*&raw const dealloc_count_17);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_18);
}
