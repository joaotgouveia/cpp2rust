extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut conjunction_v_0)));
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut conjunction_v_1)));
    assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut conjunction_v_2)));
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut conjunction_v_3)));
    assert!((!(*std::cell::LazyCell::force_mut(&mut *&raw mut conjunction_v_4))));
    assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut disjunction_v_5)));
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut disjunction_v_6)));
    assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut disjunction_v_7)));
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut disjunction_v_8)));
    assert!((!(*std::cell::LazyCell::force_mut(&mut *&raw mut disjunction_v_9))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
