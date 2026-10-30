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
pub unsafe fn operator_eq_4(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> bool {
    return true;
}
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
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
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
pub unsafe fn test_5(mut n: u32, x: *const i32, a: *const min_allocator_int_) {
    let mut d: std_deque_int__min_allocator_int__ =
        std_deque_int__min_allocator_int__::new_1({ (n as u64) }, { x }, { a });
    assert!(
        (unsafe {
            let _arg0: min_allocator_int_ = (unsafe { d.get_allocator() });
            let _arg1: min_allocator_int_ = (*a);
            operator_eq_4(_arg0, _arg1)
        })
    );
    assert!(((unsafe { d.size() }) == (n as u64)));
    assert!(
        (((unsafe {
            let ___first: UnsafeRandomAccessIterator<i32> = (unsafe { d.begin_24() });
            let ___last: UnsafeRandomAccessIterator<i32> = (unsafe { d.end_26() });
            distance_6(___first, ___last)
        }) as usize)
            == ((unsafe { d.size() }) as usize))
    );
    const _: () = assert!(true, "");;
    let mut i: UnsafeRandomAccessIterator<i32> =
        <UnsafeRandomAccessIterator<i32>>::from((unsafe { d.begin_24() }));
    let mut e: UnsafeRandomAccessIterator<i32> =
        <UnsafeRandomAccessIterator<i32>>::from((unsafe { d.end_26() }));
    'loop_: while !(i == e) {
        assert!(((*i.iter_deref()) == (*x)));
        i.iter_inc();
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
        let mut a: std_allocator_int_ = <std_allocator_int_>::default();
        (unsafe {
            let mut _x: i32 = 5;
            test_7(0_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 10;
            test_7(1_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 11;
            test_7(10_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = -11_i32;
            test_7(1023_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 25;
            test_7(1024_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 0;
            test_7(1025_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 110;
            test_7(2047_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = -500_i32;
            test_7(2048_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 654;
            test_7(2049_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 78;
            test_7(4095_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 1165;
            test_7(4096_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 157;
            test_7(4097_u32, &mut _x, &a)
        });
    }
    {
        let mut a: min_allocator_int_ = <min_allocator_int_>::default();
        (unsafe {
            let mut _x: i32 = 5;
            test_5(0_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 10;
            test_5(1_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 11;
            test_5(10_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = -11_i32;
            test_5(1023_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 25;
            test_5(1024_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 0;
            test_5(1025_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 110;
            test_5(2047_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = -500_i32;
            test_5(2048_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 654;
            test_5(2049_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 78;
            test_5(4095_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 1165;
            test_5(4096_u32, &mut _x, &a)
        });
        (unsafe {
            let mut _x: i32 = 157;
            test_5(4097_u32, &mut _x, &a)
        });
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
