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
pub type TdEnum_enum = u32;
pub const TdEnum_enum_TD_A: TdEnum_enum = 0;
pub const TdEnum_enum_TD_B: TdEnum_enum = 1;
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
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    {
        &(::std::mem::size_of::<i32>());
        { {} }
    };
    let mut td: TdEnum_enum = TdEnum_enum_TD_A;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((td as u32) == ((TdEnum_enum_TD_A as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"td == TD_A".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous_enum_c.c".as_ptr().cast_mut()).cast_const(),
                        41_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    td = TdEnum_enum_TD_B;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((td as u32) == ((TdEnum_enum_TD_B as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"td == TD_B".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous_enum_c.c".as_ptr().cast_mut()).cast_const(),
                        43_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    let mut w: WithAnonField = <WithAnonField>::default();
    w.field = anon_2_FIELD_A;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((w.field as u32) == ((anon_2_FIELD_A as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"w.field == FIELD_A".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous_enum_c.c".as_ptr().cast_mut()).cast_const(),
                        47_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
    w.field = anon_2_FIELD_B;
    {
        &(::std::mem::size_of::<i32>());
        {
            if ((((w.field as u32) == ((anon_2_FIELD_B as i32) as u32)) as i32) != 0) {
            } else {
                (unsafe {
                    __assert_fail_4(
                        (c"w.field == FIELD_B".as_ptr().cast_mut()).cast_const(),
                        (c"anonymous_enum_c.c".as_ptr().cast_mut()).cast_const(),
                        49_u32,
                        c"int main()".as_ptr(),
                    )
                });
            }
        }
    };
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
