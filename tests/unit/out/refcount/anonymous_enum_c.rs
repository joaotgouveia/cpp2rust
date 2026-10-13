extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_FIRST_A: anon_0 = 0;
pub const anon_0_FIRST_B: anon_0 = 1;
pub type anon_1 = u32;
pub const anon_1_SECOND_A: anon_1 = 0;
pub const anon_1_SECOND_B: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub a: i32,
}
pub type TdEnum_enum = u32;
pub const TdEnum_enum_TD_A: TdEnum_enum = 0;
pub const TdEnum_enum_TD_B: TdEnum_enum = 1;
pub type anon_2 = u32;
pub const anon_2_FIELD_A: anon_2 = 0;
pub const anon_2_FIELD_B: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct WithAnonField {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub field: anon_2,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    {
        &(4usize);
        { {} }
    };
    let mut td: TdEnum_enum = TdEnum_enum_TD_A;
    {
        &(4usize);
        {
            if ((((td as u32) == ((TdEnum_enum_TD_A as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"td == TD_A"),
                        Ptr::<i8>::from_string_literal(b"anonymous_enum_c.c"),
                        41_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    td = TdEnum_enum_TD_B;
    {
        &(4usize);
        {
            if ((((td as u32) == ((TdEnum_enum_TD_B as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"td == TD_B"),
                        Ptr::<i8>::from_string_literal(b"anonymous_enum_c.c"),
                        43_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    let mut w: WithAnonField = <WithAnonField>::default();
    w.field = anon_2_FIELD_A;
    {
        &(4usize);
        {
            if ((((w.field as u32) == ((anon_2_FIELD_A as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"w.field == FIELD_A"),
                        Ptr::<i8>::from_string_literal(b"anonymous_enum_c.c"),
                        47_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    w.field = anon_2_FIELD_B;
    {
        &(4usize);
        {
            if ((((w.field as u32) == ((anon_2_FIELD_B as i32) as u32)) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"w.field == FIELD_B"),
                        Ptr::<i8>::from_string_literal(b"anonymous_enum_c.c"),
                        49_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
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
pub fn __assert_fail_4(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
