extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    static __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
    pub static g_0: Ptr<i32> = __tmp_0.with(Value::as_pointer);
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if ((g_0.with(Ptr::clone).read()) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"g == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = g_0.with(|_| ());
}
