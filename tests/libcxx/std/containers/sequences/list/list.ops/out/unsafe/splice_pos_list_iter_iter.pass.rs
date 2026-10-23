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
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn test_4() -> () {
    let mut a1: [(); 3] = [1_(), 2_(), 3_()];
    let mut a2: [(); 3] = [4_(), 5_(), 6_()];
    {
        let mut l1: () = std_list_int__std_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_0: () = unimplemented!();
            std___list_const_iterator_int__void_ptr_::new_1({ &mut __tmp_0 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__std_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_1: () = unimplemented!();
            std___list_const_iterator_int__void_ptr_::new_1({ &mut __tmp_1 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__std_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_2: () = unimplemented!();
            std___list_const_iterator_int__void_ptr_::new_1({ &mut __tmp_2 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__std_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        let mut l2: () = std_list_int__std_allocator_int__::new_1({ a2.as_mut_ptr() }, {
            a2.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_3: () = unimplemented!();
            std___list_const_iterator_int__void_ptr_::new_1({ &mut __tmp_3 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__std_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        let mut l2: () = std_list_int__std_allocator_int__::new_1({ a2.as_mut_ptr() }, {
            a2.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_4: () = unimplemented!();
            std___list_const_iterator_int__void_ptr_::new_1({ &mut __tmp_4 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__std_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        let mut l2: () = std_list_int__std_allocator_int__::new_1({ a2.as_mut_ptr() }, {
            a2.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_5: () = unimplemented!();
            std___list_const_iterator_int__void_ptr_::new_1({ &mut __tmp_5 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__min_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_6: () = unimplemented!();
            std___list_const_iterator_int__min_pointer_void__std_integral_constant_unsigned_long__0___ :: new_1 ( { & mut __tmp_6 } , )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__min_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_7: () = unimplemented!();
            std___list_const_iterator_int__min_pointer_void__std_integral_constant_unsigned_long__0___ :: new_1 ( { & mut __tmp_7 } , )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__min_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_8: () = unimplemented!();
            std___list_const_iterator_int__min_pointer_void__std_integral_constant_unsigned_long__0___ :: new_1 ( { & mut __tmp_8 } , )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__min_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        let mut l2: () = std_list_int__min_allocator_int__::new_1({ a2.as_mut_ptr() }, {
            a2.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_9: () = unimplemented!();
            std___list_const_iterator_int__min_pointer_void__std_integral_constant_unsigned_long__0___ :: new_1 ( { & mut __tmp_9 } , )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__min_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        let mut l2: () = std_list_int__min_allocator_int__::new_1({ a2.as_mut_ptr() }, {
            a2.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_10: () = unimplemented!();
            std___list_const_iterator_int__min_pointer_void__std_integral_constant_unsigned_long__0___ :: new_1 ( { & mut __tmp_10 } , )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = std_list_int__min_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset((3_()) as isize)
        });
        let mut l2: () = std_list_int__min_allocator_int__::new_1({ a2.as_mut_ptr() }, {
            a2.as_mut_ptr().offset((3_()) as isize)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = {
            let mut __tmp_11: () = unimplemented!();
            std___list_const_iterator_int__min_pointer_void__std_integral_constant_unsigned_long__0___ :: new_1 ( { & mut __tmp_11 } , )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
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
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
