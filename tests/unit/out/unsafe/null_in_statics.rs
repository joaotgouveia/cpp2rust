extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
pub static mut p_mut_0: std::cell::LazyCell<*mut i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut p_const_1: std::cell::LazyCell<*const i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut cp_2: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut arr_of_ptr_3: std::cell::LazyCell<[*mut i32; 4]> =
    std::cell::LazyCell::new(|| unsafe { [std::ptr::null_mut(); 4] });
pub static mut pp_4: std::cell::LazyCell<*mut *mut i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut const_arr_of_ptr_5: std::cell::LazyCell<[*const i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [std::ptr::null(); 3] });
pub static mut cp_explicit_null_6: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut p_zero_7: std::cell::LazyCell<*mut i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut p_mut_0)).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"p_mut == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut p_const_1)).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"p_const == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut cp_2)).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"cp == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while ((i) < (4)) {
        (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut arr_of_ptr_3))[(i) as usize]).is_null()
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_8(
                    c"arr_of_ptr[i] == nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut pp_4)).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"pp == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut const_arr_of_ptr_5))[(i) as usize])
            .is_null()
        {
            &(0);
        } else {
            (unsafe {
                __assert_fail_8(
                    c"const_arr_of_ptr[i] == nullptr".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        i.prefix_inc();
    }
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut cp_explicit_null_6)).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"cp_explicit_null == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut p_zero_7)).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_8(
                c"p_zero == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_8(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const p_mut_0);
    std::cell::LazyCell::force(&*&raw const p_const_1);
    std::cell::LazyCell::force(&*&raw const cp_2);
    std::cell::LazyCell::force(&*&raw const arr_of_ptr_3);
    std::cell::LazyCell::force(&*&raw const pp_4);
    std::cell::LazyCell::force(&*&raw const const_arr_of_ptr_5);
    std::cell::LazyCell::force(&*&raw const cp_explicit_null_6);
    std::cell::LazyCell::force(&*&raw const p_zero_7);
}
