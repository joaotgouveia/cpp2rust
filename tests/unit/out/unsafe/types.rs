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
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct imaxdiv_t {
    pub quot: i64,
    pub rem: i64,
}
impl Clone for imaxdiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for imaxdiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
const _: () = assert!(
    ((::std::mem::size_of::<usize>()) == (::std::mem::size_of::<*mut ::libc::c_void>())),
    "sizeof(std::size_t) == sizeof(void *)"
);
const _: () = assert!(
    ((::std::mem::size_of::<*mut i32>()) == (8_usize)),
    "sizeof(int *) == 8"
);
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let xu8: u8 = 8_u8;
    let xu16: u16 = 16_u16;
    let mut xu32: u32 = 32_u32;
    let mut xu64: u64 = 64_u64;
    let xsz1: usize = 64_usize;
    let mut xsz2: usize = 64_usize;
    let xi1: i8 = (-8_i32 as i8);
    let xi2: i16 = 16_i16;
    let mut xi3: i32 = 32;
    let mut xi4: i64 = 64_i64;
    let mut b: bool = ((xu64) == (64_u64));
    let mut xld: f64 = 1.5E+0;
    let mut xwc: i32 = 65_i32;
    let mut xc8: u8 = 66_u8;
    let mut xc16: u16 = 67_u16;
    let mut xc32: u32 = 68_u32;
    let mut xnp: *mut ::libc::c_void = Default::default();
    (if (((((((((((((xu8 as i32) + (xu16 as i32)) as u32).wrapping_add(xu32)) as u64)
        .wrapping_add(xu64))
    .wrapping_add((xsz1 as u64)))
    .wrapping_add((xsz2 as u64)))
    .wrapping_add((xi1 as u64)))
    .wrapping_add((xi2 as u64)))
    .wrapping_add((xi3 as u64)))
    .wrapping_add((xi4 as u64)))
        == (352_u64))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"xu8 + xu16 + xu32 + xu64 + xsz1 + xsz2 + xi1 + xi2 + xi3 + xi4 == 352".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((xld) * (2_f64)) == (3_f64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"xld * 2 == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut pi: f64 = 3.14159265358979E+0;
    (if ((pi) < (3.1415926540000001E+0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"pi < 3.141592654".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut dmax: f64 = -1.7976931348623157E+308;
    (if ((dmax) < (-1.0E+308)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"dmax < -1e308".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut big: f64 = f64::INFINITY;
    (if ((big) > (1.7976931348623157E+308)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"big > DBL_MAX".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut fbig: f32 = f64::INFINITY;
    (if ((fbig) > (3.40282347E+38)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"fbig > FLT_MAX".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((((((xwc) + (xc8 as i32)) + (xc16 as i32)) as u32).wrapping_add(xc32)) == (266_u32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"xwc + xc8 + xc16 + xc32 == 266".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (std::ptr::null_mut::<::libc::c_void>()).is_null() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"xnp == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ip: *mut i32 = std::ptr::null_mut();
    (if ((::std::mem::size_of::<*mut i32>()) == (::std::mem::size_of::<*mut ::libc::c_void>())) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"sizeof(ip) == sizeof(void *)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
