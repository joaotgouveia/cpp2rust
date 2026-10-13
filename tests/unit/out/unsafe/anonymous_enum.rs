extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_FIRST_A: anon_0 = 0;
pub const anon_0_FIRST_B: anon_0 = 1;
pub type anon_1 = u32;
pub const anon_1_SECOND_A: anon_1 = 0;
pub const anon_1_SECOND_B: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub a: i32,
}
pub type TdEnum = u32;
pub const TdEnum_TD_A: TdEnum = 0;
pub const TdEnum_TD_B: TdEnum = 1;
pub type anon_2 = u32;
pub const anon_2_FIELD_A: anon_2 = 0;
pub const anon_2_FIELD_B: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithAnonField {
    pub a: i32,
    pub field: anon_2,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((anon_0_FIRST_A as i32) != (anon_0_FIRST_B as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"FIRST_A != FIRST_B".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((anon_1_SECOND_A as i32) != (anon_1_SECOND_B as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"S::SECOND_A != S::SECOND_B".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((anon_3_THIRD_A as i32) != (anon_3_THIRD_B as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"THIRD_A != THIRD_B".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut td: TdEnum = TdEnum_TD_A;
    (if ((td as i32) == (TdEnum_TD_A as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"td == TD_A".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    td = TdEnum_TD_B;
    (if ((td as i32) == (TdEnum_TD_B as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"td == TD_B".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut w: WithAnonField = <WithAnonField>::default();
    w.field = anon_2_FIELD_A;
    (if ((w.field as i32) == (anon_2_FIELD_A as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"w.field == WithAnonField::FIELD_A".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    w.field = anon_2_FIELD_B;
    (if ((w.field as i32) == (anon_2_FIELD_B as i32)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"w.field == WithAnonField::FIELD_B".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub type anon_3 = u32;
pub const anon_3_THIRD_A: anon_3 = 0;
pub const anon_3_THIRD_B: anon_3 = 1;
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
