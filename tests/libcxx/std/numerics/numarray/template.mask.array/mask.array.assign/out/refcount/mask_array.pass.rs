extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ])));
        let N1: Value<usize> = Rc::new(RefCell::new(
            (::std::mem::size_of::<[i32; 16]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)),
        ));
        let b1: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([
            true, false, false, true, true, false, false, true, false, false, false, true, false,
            false, false, false,
        ])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            -1_i32, -2_i32, -3_i32, -4_i32, -5_i32, -6_i32, -7_i32, -8_i32,
        ])));
        let N2: Value<usize> = Rc::new(RefCell::new(
            (::std::mem::size_of::<[i32; 8]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)),
        ));
        let b2: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([
            true, false, true, true, false, false, true, true,
        ])));
        let v1: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a1.as_pointer() as Ptr<i32>) },
            { (*N1.borrow()) },
        )));
        let v2: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a2.as_pointer() as Ptr<i32>) },
            { (*N2.borrow()) },
        )));
        let vb1: Value<std_valarray_bool_> = Rc::new(RefCell::new(std_valarray_bool_::new_1(
            { (b1.as_pointer() as Ptr<bool>) },
            { (*N1.borrow()) },
        )));
        let vb2: Value<std_valarray_bool_> = Rc::new(RefCell::new(std_valarray_bool_::new_1(
            { (b2.as_pointer() as Ptr<bool>) },
            { (*N2.borrow()) },
        )));
        (*v1.borrow_mut())[(*vb1.borrow())] = ((*v2.borrow())[(*vb2.borrow())]).clone();
        assert!(({ (({ (*v1.borrow()).size() }) == 16_usize) }));
        assert!(({ ({ (*v1.borrow())[0_usize] } == -1_i32) }));
        assert!(({ ({ (*v1.borrow())[1_usize] } == 1) }));
        assert!(({ ({ (*v1.borrow())[2_usize] } == 2) }));
        assert!(({ ({ (*v1.borrow())[3_usize] } == -3_i32) }));
        assert!(({ ({ (*v1.borrow())[4_usize] } == -4_i32) }));
        assert!(({ ({ (*v1.borrow())[5_usize] } == 5) }));
        assert!(({ ({ (*v1.borrow())[6_usize] } == 6) }));
        assert!(({ ({ (*v1.borrow())[7_usize] } == -7_i32) }));
        assert!(({ ({ (*v1.borrow())[8_usize] } == 8) }));
        assert!(({ ({ (*v1.borrow())[9_usize] } == 9) }));
        assert!(({ ({ (*v1.borrow())[10_usize] } == 10) }));
        assert!(({ ({ (*v1.borrow())[11_usize] } == -8_i32) }));
        assert!(({ ({ (*v1.borrow())[12_usize] } == 12) }));
        assert!(({ ({ (*v1.borrow())[13_usize] } == 13) }));
        assert!(({ ({ (*v1.borrow())[14_usize] } == 14) }));
        assert!(({ ({ (*v1.borrow())[15_usize] } == 15) }));
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 4, 5])));
        let b1: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([true, false, true])));
        let v1: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a1.as_pointer() as Ptr<i32>) },
            { 3_usize },
        )));
        let v2: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a2.as_pointer() as Ptr<i32>) },
            { 3_usize },
        )));
        let vb1: Value<std_valarray_bool_> = Rc::new(RefCell::new(std_valarray_bool_::new_1(
            { (b1.as_pointer() as Ptr<bool>) },
            { 3_usize },
        )));
        let m1: Value<std_mask_array_int_> =
            Rc::new(RefCell::new(((*v1.borrow())[(*vb1.borrow())]).clone()));
        let m2: Value<std_mask_array_int_> =
            Rc::new(RefCell::new(((*v2.borrow())[(*vb1.borrow())]).clone()));
        let r: Ptr<std_mask_array_int_> = ((*m1.borrow_mut()) = (*m2.borrow()).clone());
        assert!(({ (r).clone() } == { (m1.as_pointer()) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
