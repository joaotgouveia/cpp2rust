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
pub unsafe fn test_5(mut P: i32, c1: *mut Vec<i32>) {
    assert!(((P as usize) < ((unsafe { (*c1).size() }) as usize)));
    let mut c1_osize: usize = ((unsafe { (*c1).size() }) as usize);
    let mut i: UnsafeRandomAccessIterator<i32> = (unsafe {
        let ___position: UnsafeRandomAccessIterator<i32> =
            (unsafe { (*c1).cbegin() }).iter_add((P as isize));
        (*c1).erase_68(___position)
    });
    assert!(i == (unsafe { (*c1).begin_24() }).iter_add((P as isize)));
    assert!((((unsafe { (*c1).size() }) as usize) == ((c1_osize).wrapping_sub(1_usize))));
    assert!(
        (((unsafe {
            let ___first: UnsafeRandomAccessIterator<i32> = (unsafe { (*c1).begin_24() });
            let ___last: UnsafeRandomAccessIterator<i32> = (unsafe { (*c1).end_26() });
            distance_6(___first, ___last)
        }) as usize)
            == ((unsafe { (*c1).size() }) as usize))
    );
    i = (unsafe { (*c1).begin_24() }).clone();
    let mut j: i32 = 0;
    'loop_: while ((j) < (P)) {
        assert!(((*i.iter_deref()) == (j)));
        {
            j.prefix_inc();
            i.iter_inc()
        };
    }
    j.prefix_inc();
    'loop_: while ((j as usize) < (c1_osize)) {
        assert!(((*i.iter_deref()) == (j)));
        {
            j.prefix_inc();
            i.iter_inc()
        };
    }
}
pub unsafe fn testN_7(mut start: i32, mut N: i32) {
    let mut pstep: i32 = {
        let mut __tmp_1: i32 = ((N)
            / ({
                let mut __tmp_3: i32 = 1;
                (*{
                    let __a: *const _ = {
                        let mut __tmp_2: i32 = 10;
                        {
                            let __a: *const _ = &mut N;
                            let __b: *const _ = &mut __tmp_2;
                            if *__b < *__a { __b } else { __a }
                        }
                    };
                    let __b: *const _ = &mut __tmp_3;
                    if *__a < *__b { __b } else { __a }
                })
            }));
        let mut __tmp_4: i32 = 1;
        (*{
            let __a: *const _ = &mut __tmp_1;
            let __b: *const _ = &mut __tmp_4;
            if *__a < *__b { __b } else { __a }
        })
    };
    let mut p: i32 = 0;
    'loop_: while ((p) < (N)) {
        let mut c1: Vec<i32> = (unsafe { make_4(N, start) });
        (unsafe { test_5(p, &mut c1) });
        p += pstep;
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
                    testN_7(_start, _N)
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
                    testN_8(_start, _N)
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
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
