extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct widget {
    #[offset(0)]
    pub id: i32,
}
pub fn a_value_0() -> i32 {
    let mut w: widget = <widget>::default();
    w.id = 11;
    return w.id;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ a_value_0() }) == 11) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"a_value() == 11"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ b_value_2() }) == 2) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"b_value() == 2"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        17_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub type widget_enum = u32;
pub const widget_enum_WIDGET_A: widget_enum = 0;
pub const widget_enum_WIDGET_B: widget_enum = 1;
pub const widget_enum_WIDGET_C: widget_enum = 2;
pub fn b_value_2() -> i32 {
    let mut w: widget_enum = widget_enum_WIDGET_C;
    return (w as i32);
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
