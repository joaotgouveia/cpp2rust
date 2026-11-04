extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test07_0() {
    write!(
        cout_1.with(|v| v.as_pointer()),
        "{:}\n",
        ({
            (*({
                ((*cin_2.with(Value::clone).borrow()) as std_basic_ios_char__std_char_traits_char__)
                    .rdbuf_17()
            })
            .upgrade()
            .deref())
            .in_avail()
        }),
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test07_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
