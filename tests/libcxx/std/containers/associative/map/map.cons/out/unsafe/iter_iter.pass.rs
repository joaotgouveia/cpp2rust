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
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
pub unsafe fn test_4() -> () {
    {
        let mut ar: [(); 9] = [
            {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            {
                let mut __tmp_2: () = 1_();
                let mut __tmp_3: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_2 }, { &mut __tmp_3 })
            },
            {
                let mut __tmp_4: () = 1_();
                let mut __tmp_5: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
            },
            {
                let mut __tmp_6: () = 2_();
                let mut __tmp_7: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
            },
            {
                let mut __tmp_8: () = 2_();
                let mut __tmp_9: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_8 }, { &mut __tmp_9 })
            },
            {
                let mut __tmp_10: () = 2_();
                let mut __tmp_11: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
            },
            {
                let mut __tmp_12: () = 3_();
                let mut __tmp_13: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
            },
            {
                let mut __tmp_14: () = 3_();
                let mut __tmp_15: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_14 }, { &mut __tmp_15 })
            },
            {
                let mut __tmp_16: () = 3_();
                let mut __tmp_17: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
            },
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ar: [(); 9] = [
            {
                let mut __tmp_18: () = 1_();
                let mut __tmp_19: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
            },
            {
                let mut __tmp_20: () = 1_();
                let mut __tmp_21: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_20 }, { &mut __tmp_21 })
            },
            {
                let mut __tmp_22: () = 1_();
                let mut __tmp_23: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
            },
            {
                let mut __tmp_24: () = 2_();
                let mut __tmp_25: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
            },
            {
                let mut __tmp_26: () = 2_();
                let mut __tmp_27: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_26 }, { &mut __tmp_27 })
            },
            {
                let mut __tmp_28: () = 2_();
                let mut __tmp_29: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
            },
            {
                let mut __tmp_30: () = 3_();
                let mut __tmp_31: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
            },
            {
                let mut __tmp_32: () = 3_();
                let mut __tmp_33: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_32 }, { &mut __tmp_33 })
            },
            {
                let mut __tmp_34: () = 3_();
                let mut __tmp_35: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
            },
        ];
        let mut m: () = unimplemented!();
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
    (unsafe { test_4() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
