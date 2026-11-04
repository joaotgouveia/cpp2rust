extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test07_0() {
    {
        let __s = {
            let __s = wcout_1.with(|v| v.as_pointer());
            write_int_refcount(
                &__s,
                ({
                    ({
                        (*({
                            ((*wcin_2.with(Value::clone).borrow())
                                as std_basic_ios_wchar_t__std_char_traits_wchar_t__)
                                .rdbuf_17()
                        })
                        .upgrade()
                        .deref())
                        .in_avail()
                    })
                }),
            );
            __s
        };
        libcc2rs::endl_refcount.call(__s)
    };
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
