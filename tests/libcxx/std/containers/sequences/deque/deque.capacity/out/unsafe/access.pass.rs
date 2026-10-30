extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_3))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_4)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_3))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_4))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_5)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_2)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_3)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_4)) = 0_usize;
    }
}
pub static mut outstanding_bytes_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_3: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_4: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_5: std::cell::LazyCell<bool> =
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
pub unsafe fn make_6(mut size: i32, mut start: Option<i32>) -> Vec<i32> {
    let mut start: i32 = start.unwrap_or_else(|| unsafe { 0 });
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
        let mut c: Vec<i32> = (unsafe { make_6(10, None) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((c[(i as u64) as usize]) == (i)));
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((*(unsafe { c.at_45((i as u64),) })) == (i)));
            i.prefix_inc();
        }
        assert!(((*((c).first_mut().unwrap())) == (0)));
        assert!(((*(c.last_mut().unwrap())) == (9)));
        const _: () = assert!(true, "");;
    }
    {
        let c: Vec<i32> = (unsafe { make_6(10, None) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((c[(i as u64) as usize]) == (i)));
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((*(unsafe { c.at_46((i as u64),) })) == (i)));
            i.prefix_inc();
        }
        assert!(((*(unsafe { c.front_48() })) == (0)));
        assert!(((*(unsafe { c.back_50() })) == (9)));
        const _: () = assert!(true, "");;
    }
    {
        let mut c: std_deque_int__min_allocator_int__ = (unsafe { make_7(10, None) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((c[(i as u64) as usize]) == (i)));
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((*(unsafe { c.at_45((i as u64),) })) == (i)));
            i.prefix_inc();
        }
        assert!(((*(unsafe { c.front_47() })) == (0)));
        assert!(((*(unsafe { c.back_49() })) == (9)));
        const _: () = assert!(true, "");;
    }
    {
        let c: std_deque_int__min_allocator_int__ = (unsafe { make_7(10, None) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((c[(i as u64) as usize]) == (i)));
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while ((i) < (10)) {
            assert!(((*(unsafe { c.at_46((i as u64),) })) == (i)));
            i.prefix_inc();
        }
        assert!(((*(unsafe { c.front_48() })) == (0)));
        assert!(((*(unsafe { c.back_50() })) == (9)));
        const _: () = assert!(true, "");;
    }
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_2);
    std::cell::LazyCell::force(&*&raw const alloc_count_3);
    std::cell::LazyCell::force(&*&raw const dealloc_count_4);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_5);
}
