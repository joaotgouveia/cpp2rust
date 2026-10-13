extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static value_0: Value<i32> = Rc::new(RefCell::new(5));
);
pub fn param_shadow_1(mut value: i32) -> i32 {
    return (value + 1);
}
pub fn local_shadow_2() -> i32 {
    let mut value: i32 = 99;
    return value;
}
pub fn read_global_3() -> i32 {
    return value_0.with(|rc| *rc.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        &(4usize);
        {
            if (((({ param_shadow_1(10) }) == 11) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"param_shadow(10) == 11"),
                        Ptr::<i8>::from_string_literal(b"static_shadow.c"),
                        15_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ local_shadow_2() }) == 99) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"local_shadow() == 99"),
                        Ptr::<i8>::from_string_literal(b"static_shadow.c"),
                        16_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if (((({ read_global_3() }) == 5) as i32) != 0) {
            } else {
                ({
                    __assert_fail_4(
                        Ptr::<i8>::from_string_literal(b"read_global() == 5"),
                        Ptr::<i8>::from_string_literal(b"static_shadow.c"),
                        17_u32,
                        Ptr::<i8>::from_string_literal(b"int main()"),
                    )
                });
            }
        }
    };
    return 0;
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
    let _ = value_0.with(|_| ());
}
