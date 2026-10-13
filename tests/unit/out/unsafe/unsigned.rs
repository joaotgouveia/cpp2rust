extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type std_float_round_style = i32;
pub const std_float_round_style_round_indeterminate: std_float_round_style = -1;
pub const std_float_round_style_round_toward_zero: std_float_round_style = 0;
pub const std_float_round_style_round_to_nearest: std_float_round_style = 1;
pub const std_float_round_style_round_toward_infinity: std_float_round_style = 2;
pub const std_float_round_style_round_toward_neg_infinity: std_float_round_style = 3;
pub type std_float_denorm_style = i32;
pub const std_float_denorm_style_denorm_indeterminate: std_float_denorm_style = -1;
pub const std_float_denorm_style_denorm_absent: std_float_denorm_style = 0;
pub const std_float_denorm_style_denorm_present: std_float_denorm_style = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___numeric_limits_base {}
impl Clone for std___numeric_limits_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___numeric_limits_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_0 {}
impl Clone for numeric_limits_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_0 {
    pub unsafe fn min() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_1 {}
impl Clone for numeric_limits_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_numeric_limits_char__is_signed_2: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl numeric_limits_1 {
    pub unsafe fn min() -> libc::c_char {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_3 {}
impl Clone for numeric_limits_3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_3 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_3 {
    pub unsafe fn min() -> i8 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_4 {}
impl Clone for numeric_limits_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_4 {
    pub unsafe fn min() -> u8 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_5 {}
impl Clone for numeric_limits_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_5 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_numeric_limits_wchar_t__is_signed_6: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl numeric_limits_5 {
    pub unsafe fn min() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_7 {}
impl Clone for numeric_limits_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_numeric_limits_char16_t__is_signed_8: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl numeric_limits_7 {
    pub unsafe fn min() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_9 {}
impl Clone for numeric_limits_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_numeric_limits_char32_t__is_signed_10: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl numeric_limits_9 {
    pub unsafe fn min() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_11 {}
impl Clone for numeric_limits_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_11 {
    pub unsafe fn min() -> i16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_12 {}
impl Clone for numeric_limits_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_12 {
    pub unsafe fn min() -> u16 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_13 {}
impl Clone for numeric_limits_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_13 {
    pub unsafe fn min() -> i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_14 {}
impl Clone for numeric_limits_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_14 {
    pub unsafe fn min() -> u32 {
        unimplemented!()
    }
}
impl numeric_limits_14 {
    pub unsafe fn max() -> u32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_15 {}
impl Clone for numeric_limits_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_15 {
    pub unsafe fn min() -> i64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_16 {}
impl Clone for numeric_limits_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_16 {
    pub unsafe fn min() -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_17 {}
impl Clone for numeric_limits_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_17 {
    pub unsafe fn min() -> i64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_18 {}
impl Clone for numeric_limits_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_18 {
    pub unsafe fn min() -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_19 {}
impl Clone for numeric_limits_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_19 {
    pub unsafe fn min() -> i128 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_20 {}
impl Clone for numeric_limits_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl numeric_limits_20 {
    pub unsafe fn min() -> u128 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct numeric_limits_21 {}
impl Clone for numeric_limits_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_numeric_limits___float128__has_signaling_NaN_22: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl numeric_limits_21 {
    pub unsafe fn max() -> f64 {
        unimplemented!()
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: u32 = (-1_i32 as u32);
    (if ((x) == (unsafe { numeric_limits_14::max() })) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_23(
                c"x == std::numeric_limits<unsigned int>::max()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut v1: i32 = (((x) & (1_u32)) as i32);
    let mut v2: u32 = ((x) & (1_u32));
    let mut p: *mut u32 = (&mut x as *mut u32);
    let mut b: bool = (((*p) & (255_u32)) != 0);
    let mut a: i32 = (((*p) & (255_u32)) as i32);
    (if ((a) == (255)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_23(
                c"a == 255".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_23(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
