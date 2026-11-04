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
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    ])));
    let v1: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
        { (a.as_pointer() as Ptr<i32>) },
        {
            (::std::mem::size_of::<[i32; 41]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))
        },
    )));
    let sz: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([2_usize, 4_usize, 3_usize])));
    let st: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([19_usize, 4_usize, 1_usize])));
    let v: Value<std_valarray_int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<std_gslice_array_int_> = Rc::new(RefCell::new(
            elem!(v1.as_pointer(), {
                let __tmp_1: Value<std_valarray_unsigned_long_> =
                    Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1(
                        { (sz.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>() },
                        {
                            (::std::mem::size_of::<[usize; 3]>() as usize)
                                .wrapping_div((::std::mem::size_of::<usize>() as usize))
                        },
                    )));
                let __tmp_2: Value<std_valarray_unsigned_long_> =
                    Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1(
                        { (st.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>() },
                        {
                            (::std::mem::size_of::<[usize; 3]>() as usize)
                                .wrapping_div((::std::mem::size_of::<usize>() as usize))
                        },
                    )));
                std_gslice::new_1({ 3_usize }, { __tmp_1.as_pointer() }, {
                    __tmp_2.as_pointer()
                })
            })
            .read(),
        ));
        std_valarray_int_::new_2({ __tmp_0.as_pointer() })
    }));
    assert!(({ (({ (*v.borrow()).size() }) == 24_usize) }));
    assert!(({ ({ (*v.borrow())[0_usize] } == 3) }));
    assert!(({ ({ (*v.borrow())[1_usize] } == 4) }));
    assert!(({ ({ (*v.borrow())[2_usize] } == 5) }));
    assert!(({ ({ (*v.borrow())[3_usize] } == 7) }));
    assert!(({ ({ (*v.borrow())[4_usize] } == 8) }));
    assert!(({ ({ (*v.borrow())[5_usize] } == 9) }));
    assert!(({ ({ (*v.borrow())[6_usize] } == 11) }));
    assert!(({ ({ (*v.borrow())[7_usize] } == 12) }));
    assert!(({ ({ (*v.borrow())[8_usize] } == 13) }));
    assert!(({ ({ (*v.borrow())[9_usize] } == 15) }));
    assert!(({ ({ (*v.borrow())[10_usize] } == 16) }));
    assert!(({ ({ (*v.borrow())[11_usize] } == 17) }));
    assert!(({ ({ (*v.borrow())[12_usize] } == 22) }));
    assert!(({ ({ (*v.borrow())[13_usize] } == 23) }));
    assert!(({ ({ (*v.borrow())[14_usize] } == 24) }));
    assert!(({ ({ (*v.borrow())[15_usize] } == 26) }));
    assert!(({ ({ (*v.borrow())[16_usize] } == 27) }));
    assert!(({ ({ (*v.borrow())[17_usize] } == 28) }));
    assert!(({ ({ (*v.borrow())[18_usize] } == 30) }));
    assert!(({ ({ (*v.borrow())[19_usize] } == 31) }));
    assert!(({ ({ (*v.borrow())[20_usize] } == 32) }));
    assert!(({ ({ (*v.borrow())[21_usize] } == 34) }));
    assert!(({ ({ (*v.borrow())[22_usize] } == 35) }));
    assert!(({ ({ (*v.borrow())[23_usize] } == 36) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
