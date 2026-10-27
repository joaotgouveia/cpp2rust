extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const ()) -> *const () {
    c"";
    return value;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_1(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_2(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn operator_eq_3(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_4(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_5(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_6(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_7(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_8(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_9(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
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
        unsafe { operator_cmp_9(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_3(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_13(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_14(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_15(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_16(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: (),
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
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
    pub unsafe fn get(&self) -> () {
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
            operator_cmp_16(
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
            operator_eq_10(
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
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_18))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_19))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_20)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_17)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_18)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_19)) = 0_();
    }
}
pub static mut outstanding_bytes_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_MoveOnly_ {}
impl safe_allocator_MoveOnly_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut MoveOnly {
        let mut memory: *mut MoveOnly = unimplemented!();
        {
            (unsafe {
                memset_21(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<MoveOnly>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut MoveOnly, mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_21(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<MoveOnly>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_MoveOnly_ {
    fn default() -> Self {
        unsafe { safe_allocator_MoveOnly_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_MoveOnly_ptr_ {}
impl safe_allocator_MoveOnly_ptr_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut *mut MoveOnly {
        let mut memory: *mut *mut MoveOnly = unimplemented!();
        {
            (unsafe {
                memset_21(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<*mut MoveOnly>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut MoveOnly, mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_21(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<*mut MoveOnly>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_MoveOnly_ptr_ {
    fn default() -> Self {
        unsafe { safe_allocator_MoveOnly_ptr_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_const_MoveOnly_ptr_ {}
impl safe_allocator_const_MoveOnly_ptr_ {}
pub unsafe fn make_22(mut size: (), mut start: ()) -> () {
    let b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if ((start) > (0_())) {
        init = ((((start) + (1_())) / (b)) + ((((start) + (1_())) % (b)) != (0_())));
        init *= b;
        init.prefix_dec();
    }
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (start)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn make_23(mut size: (), mut start: ()) -> () {
    let b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if ((start) > (0_())) {
        init = ((((start) + (1_())) / (b)) + ((((start) + (1_())) % (b)) != (0_())));
        init *= b;
        init.prefix_dec();
    }
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (start)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn test_24(c1: *mut (), mut x: ()) {
    let mut c1_osize: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut j: () = 0_();
    'loop_: while ((j) < (c1_osize)) {
        unimplemented!();
        {
            j.prefix_inc();
            &(unimplemented!())
        };
    }
}
pub unsafe fn test_25(c1: *mut (), mut x: ()) {
    let mut c1_osize: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut j: () = 0_();
    'loop_: while ((j) < (c1_osize)) {
        unimplemented!();
        {
            j.prefix_inc();
            &(unimplemented!())
        };
    }
}
pub unsafe fn testN_26(mut start: (), mut N: ()) {
    let mut c1: () = (unsafe { make_22(N, start) });
    (unsafe { test_24(&mut c1, -10_()) });
}
pub unsafe fn testN_27(mut start: (), mut N: ()) {
    let mut c1: () = (unsafe { make_23(N, start) });
    (unsafe { test_25(&mut c1, -10_()) });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            let mut j: () = 0_();
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_26(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            let mut j: () = 0_();
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_27(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_17);
    std::cell::LazyCell::force(&*&raw const alloc_count_18);
    std::cell::LazyCell::force(&*&raw const dealloc_count_19);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_20);
}
