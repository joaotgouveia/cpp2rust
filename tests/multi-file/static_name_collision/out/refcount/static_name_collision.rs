extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static same_name_different_type_0: Value<i32> = Rc::new(RefCell::new(1));
);
thread_local!(
    pub static same_name_same_type_1: Value<i32> = Rc::new(RefCell::new(5));
);
pub fn a_foo_2() -> i32 {
    return same_name_different_type_0.with(|rc| *rc.borrow());
}
pub fn a_bar_3() -> i32 {
    return same_name_same_type_1.with(|rc| *rc.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ a_foo_2() }) == 1) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"a_foo() == 1"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        13_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ b_foo_5() }) == 1.0E+0) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"b_foo() == 1.0f"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        14_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ a_bar_3() }) == 5) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"a_bar() == 5"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ b_bar_6() }) == 6) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"b_bar() == 6"),
                        Ptr::<i8>::from_string_literal(b"a.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
thread_local!(
    pub static same_name_different_type_7: Value<f32> = Rc::new(RefCell::new(1.0E+0));
);
thread_local!(
    pub static same_name_same_type_8: Value<i32> = Rc::new(RefCell::new(6));
);
pub fn b_foo_5() -> f32 {
    return same_name_different_type_7.with(|rc| *rc.borrow());
}
pub fn b_bar_6() -> i32 {
    return same_name_same_type_8.with(|rc| *rc.borrow());
}
pub fn __assert_fail_4(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = same_name_different_type_0.with(|_| ());
    let _ = same_name_same_type_1.with(|_| ());
    let _ = same_name_different_type_7.with(|_| ());
    let _ = same_name_same_type_8.with(|_| ());
}
