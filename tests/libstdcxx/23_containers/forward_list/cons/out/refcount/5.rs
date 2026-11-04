extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let flv: Value<std_forward_list_float__std_allocator_float__> = Rc::new(RefCell::new({
        let __tmp_0: Value<f32> = Rc::new(RefCell::new(5_f32));
        std_forward_list_float__std_allocator_float__::new_1(
            { 10_u64 },
            { __tmp_0.as_pointer() },
            None,
        )
    }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
