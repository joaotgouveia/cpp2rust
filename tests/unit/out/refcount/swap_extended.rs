extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn swap_by_ptr_1(mut a: Ptr<i32>, mut b: Ptr<i32>) {
    let mut tmp: i32 = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn swap_by_ref_2(a: Ptr<i32>, b: Ptr<i32>) {
    let mut tmp: i32 = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: i32 = 0;
    {
        let __s = {
            let __s = libcc2rs::cout();
            write_int_refcount(&__s, x);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    let mut a: i32 = 1;
    let mut b: i32 = ({ identity_0(a) });
    {
        let __s = {
            let __s = libcc2rs::cout();
            write_int_refcount(&__s, b);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    let c: Value<i32> = Rc::new(RefCell::new(2));
    let mut p: Ptr<i32> = (c.as_pointer());
    {
        let __s = {
            let __s = libcc2rs::cout();
            write_int_refcount(&__s, (p.read()));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    let d: Value<i32> = Rc::new(RefCell::new(3));
    let e: Value<i32> = Rc::new(RefCell::new(4));
    ({ swap_by_ptr_1((d.as_pointer()), (e.as_pointer())) });
    let f: Value<i32> = Rc::new(RefCell::new(4));
    let g: Value<i32> = Rc::new(RefCell::new(5));
    ({ swap_by_ref_2(f.as_pointer(), g.as_pointer()) });
    let mut h: Ptr<i32> = Ptr::alloc(6);
    {
        let __s = {
            let __s = libcc2rs::cout();
            write_int_refcount(&__s, (h.read()));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    h.delete();
    let mut i: Ptr<i32> = Ptr::alloc_array(Box::new([7, 8, 0_i32]));
    {
        let __s = {
            let __s = {
                let __s = {
                    let __s = libcc2rs::cout();
                    write_int_refcount(&__s, (elem!(i, 0).read()));
                    __s
                };
                let _ = __s.write_all(&[(' ' as i8) as u8]);
                __s
            };
            write_int_refcount(&__s, (elem!(i, 1).read()));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    i.delete();
    ({ swap_by_ptr_1(Ptr::alloc(7), Ptr::alloc(8)) });
    ({
        swap_by_ptr_1(
            Ptr::alloc(7).offset((0) as isize),
            Ptr::alloc(8).offset((0) as isize),
        )
    });
    ({ swap_by_ref_2(Ptr::alloc(9), Ptr::alloc(10)) });
    ({
        swap_by_ref_2(
            (Ptr::alloc(9)).offset((0) as isize),
            (Ptr::alloc(10)).offset((0) as isize),
        )
    });
    let mut j: Option<Value<i32>> = Ptr::alloc(11).to_owned_opt();
    let mut k: Ptr<i32> = j.as_pointer();
    {
        let __s = {
            let __s = libcc2rs::cout();
            write_int_refcount(&__s, (k.read()));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    let mut l: Option<Value<i32>> = Some(Rc::new(RefCell::new(11)));
    let mut m: Ptr<i32> = l.as_pointer();
    {
        let __s = {
            let __s = libcc2rs::cout();
            write_int_refcount(&__s, (m.read()));
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    assert!(({ ((*c.borrow()) == 2) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
