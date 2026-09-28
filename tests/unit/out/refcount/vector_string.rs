extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<Vec<Vec<u8>>> = Rc::new(RefCell::new(vec![{
        let mut __bytes = Ptr::<u8>::from_string_literal(b"a").to_c_bytes();
        __bytes.push(0);
        __bytes
    }]));
    assert!(
        (((if 0_usize as usize
            >= (*((v1.as_pointer() as Ptr<Vec<u8>>).to_last() as Ptr<Vec<u8>>)
                .upgrade()
                .deref())
            .len()
            .saturating_sub(1)
        {
            panic!("out of bounds access")
        } else {
            ((v1.as_pointer() as Ptr<Vec<u8>>).to_last() as Ptr<Vec<u8>>)
                .decay()
                .offset(0_usize as isize)
        }
        .read()) as i32)
            == (('a' as u8) as i32))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
