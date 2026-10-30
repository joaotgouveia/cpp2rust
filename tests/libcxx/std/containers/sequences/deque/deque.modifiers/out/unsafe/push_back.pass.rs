extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_int_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_int_>::default().deallocate(p.ptr_, n) });
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( unsafe { <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ >::default() . allocate  ( n , ) } )  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            <std_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>::default()
                .deallocate(p.ptr_, n)
        });
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub unsafe fn make_4(mut size: i32, mut start: i32) -> Vec<i32> {
    let b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if ((start) > (0)) {
        init = ((((start) + (1)) / (b)) + (((((start) + (1)) % (b)) != (0)) as i32));
        init *= b;
        init.prefix_dec();
    }
    let mut c: Vec<i32> = {
        let mut __tmp_0: i32 = 0;
        std_deque_int__std_allocator_int__::new_1({ (init as u64) }, { &mut __tmp_0 }, None)
    };
    let mut i: i32 = 0;
    'loop_: while ((i) < ((init) - (start))) {
        (unsafe { c.pop_back() });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        c.push(i);
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (start)) {
        c.remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub unsafe fn test_5(mut size: i32) {
    let mut rng: [i32; 11] = [0, 1, 2, 3, 1023, 1024, 1025, 2046, 2047, 2048, 2049];
    let N: i32 = (((::std::mem::size_of::<[i32; 11]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut j: i32 = 0;
    'loop_: while ((j) < (N)) {
        let mut c: Vec<i32> = (unsafe { make_4(size, rng[(j) as usize]) });
        let mut it: UnsafeRandomAccessIterator<i32> =
            <UnsafeRandomAccessIterator<i32>>::from((unsafe { c.begin_24() }));
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while ((i) < (size)) {
            assert!(((*it.iter_deref()) == (i)));
            {
                i.prefix_inc();
                it.iter_inc()
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
            (unsafe { test_5(rng[(j) as usize]) });
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
            (unsafe { test_6(rng[(j) as usize]) });
            j.prefix_inc();
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
