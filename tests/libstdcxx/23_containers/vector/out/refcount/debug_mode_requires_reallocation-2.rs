extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let a: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 0_u32;
        (*a.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 1_u32;
        (*a.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2_u32;
        (*a.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3_u32;
        (*a.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 4_u32;
        (*a.borrow_mut()).push(__a1)
    };
    if ((*a.borrow()).capacity() < 7_usize) {
        if 7_usize as usize > (*a.borrow()).capacity() as usize {
            let len_0 = (*a.borrow()).len();
            (*a.borrow_mut()).reserve_exact(7_usize as usize - len_0 as usize);
        };
    }
    {
        let mut __first = a.decay().begin();
        let __last = a.decay().begin().iter_add(2_isize);
        let mut __out = unimplemented!();
        while __first != __last {
            __out
                .iter_deref()
                .write(<Vec<u32> as CppFrom<u32>>::cpp_from(__first.iter_deref()));
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
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
