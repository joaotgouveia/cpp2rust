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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)) = 0_usize;
    }
}
pub static mut outstanding_bytes_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_3: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_4: std::cell::LazyCell<bool> =
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        let mut memory: *mut i32 = (unsafe { <std_allocator_int_>::default().allocate(n) });
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in
                    0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in
                        0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        (unsafe { <std_allocator_int_>::default().deallocate(p, n) });
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        unsafe { safe_allocator_int_::new() }
    }
}
impl safe_allocator_int_ptr_ {
    pub unsafe fn new_1(mut _a0: safe_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_int_ptr_ {}
impl safe_allocator_int_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut i32 {
        let mut memory: *mut *mut i32 =
            (unsafe { <std_allocator_int_ptr_>::default().allocate(n) });
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in 0..((::std::mem::size_of::<*mut i32>() as u64)
                    .wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..((::std::mem::size_of::<*mut i32>() as u64)
                        .wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        (unsafe { <std_allocator_int_ptr_>::default().deallocate(p, n) });
    }
}
pub unsafe fn make_5(mut size: i32, mut start: i32) -> Vec<i32> {
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
pub unsafe fn test_6(c1: *mut Vec<i32>) {
    let mut c1_osize: usize = ((unsafe { (*c1).size() }) as usize);
    (unsafe { (*c1).pop_back() });
    assert!((((unsafe { (*c1).size() }) as usize) == ((c1_osize).wrapping_sub(1_usize))));
    assert!(
        (((unsafe {
            let ___first: UnsafeRandomAccessIterator<i32> = (unsafe { (*c1).begin_24() });
            let ___last: UnsafeRandomAccessIterator<i32> = (unsafe { (*c1).end_26() });
            distance_7(___first, ___last)
        }) as usize)
            == ((unsafe { (*c1).size() }) as usize))
    );
    const _: () = assert!(true, "");;
    let mut i: UnsafeRandomAccessIterator<i32> = (unsafe { (*c1).begin_24() });
    let mut j: i32 = 0;
    'loop_: while ((j as usize) < ((unsafe { (*c1).size() }) as usize)) {
        assert!(((*i.iter_deref()) == (j)));
        {
            j.prefix_inc();
            i.iter_inc()
        };
    }
}
pub unsafe fn testN_8(mut start: i32, mut N: i32) {
    if ((N) != (0)) {
        let mut c1: Vec<i32> = (unsafe { make_5(N, start) });
        (unsafe { test_6(&mut c1) });
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
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_8(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_9(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_10(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
