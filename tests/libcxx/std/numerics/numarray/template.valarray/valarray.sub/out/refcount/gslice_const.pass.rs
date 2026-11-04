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
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        ])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            -0_i32, -1_i32, -2_i32, -3_i32, -4_i32, -5_i32, -6_i32, -7_i32, -8_i32, -9_i32,
            -10_i32, -11_i32, -12_i32, -13_i32, -14_i32, -15_i32, -16_i32, -17_i32, -18_i32,
            -19_i32, -20_i32, -21_i32, -22_i32, -23_i32,
        ])));
        let v1: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a1.as_pointer() as Ptr<i32>) },
            {
                (::std::mem::size_of::<[i32; 41]>() as usize)
                    .wrapping_div((::std::mem::size_of::<i32>() as usize))
            },
        )));
        let v2: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a2.as_pointer() as Ptr<i32>) },
            {
                (::std::mem::size_of::<[i32; 24]>() as usize)
                    .wrapping_div((::std::mem::size_of::<i32>() as usize))
            },
        )));
        let sz: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([2_usize, 4_usize, 3_usize])));
        let st: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([19_usize, 4_usize, 1_usize])));
        elem!(v1.as_pointer(), {
            let __tmp_0: Value<std_valarray_unsigned_long_> =
                Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1(
                    { (sz.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>() },
                    {
                        (::std::mem::size_of::<[usize; 3]>() as usize)
                            .wrapping_div((::std::mem::size_of::<usize>() as usize))
                    },
                )));
            let __tmp_1: Value<std_valarray_unsigned_long_> =
                Rc::new(RefCell::new(std_valarray_unsigned_long_::new_1(
                    { (st.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>() },
                    {
                        (::std::mem::size_of::<[usize; 3]>() as usize)
                            .wrapping_div((::std::mem::size_of::<usize>() as usize))
                    },
                )));
            std_gslice::new_1({ 3_usize }, { __tmp_0.as_pointer() }, {
                __tmp_1.as_pointer()
            })
        })
        .write((*v2.borrow()).clone());
        assert!(({ (({ (*v1.borrow()).size() }) == 41_usize) }));
        assert!(({ ({ (*v1.borrow())[0_usize] } == 0) }));
        assert!(({ ({ (*v1.borrow())[1_usize] } == 1) }));
        assert!(({ ({ (*v1.borrow())[2_usize] } == 2) }));
        assert!(({ ({ (*v1.borrow())[3_usize] } == 0) }));
        assert!(({ ({ (*v1.borrow())[4_usize] } == -1_i32) }));
        assert!(({ ({ (*v1.borrow())[5_usize] } == -2_i32) }));
        assert!(({ ({ (*v1.borrow())[6_usize] } == 6) }));
        assert!(({ ({ (*v1.borrow())[7_usize] } == -3_i32) }));
        assert!(({ ({ (*v1.borrow())[8_usize] } == -4_i32) }));
        assert!(({ ({ (*v1.borrow())[9_usize] } == -5_i32) }));
        assert!(({ ({ (*v1.borrow())[10_usize] } == 10) }));
        assert!(({ ({ (*v1.borrow())[11_usize] } == -6_i32) }));
        assert!(({ ({ (*v1.borrow())[12_usize] } == -7_i32) }));
        assert!(({ ({ (*v1.borrow())[13_usize] } == -8_i32) }));
        assert!(({ ({ (*v1.borrow())[14_usize] } == 14) }));
        assert!(({ ({ (*v1.borrow())[15_usize] } == -9_i32) }));
        assert!(({ ({ (*v1.borrow())[16_usize] } == -10_i32) }));
        assert!(({ ({ (*v1.borrow())[17_usize] } == -11_i32) }));
        assert!(({ ({ (*v1.borrow())[18_usize] } == 18) }));
        assert!(({ ({ (*v1.borrow())[19_usize] } == 19) }));
        assert!(({ ({ (*v1.borrow())[20_usize] } == 20) }));
        assert!(({ ({ (*v1.borrow())[21_usize] } == 21) }));
        assert!(({ ({ (*v1.borrow())[22_usize] } == -12_i32) }));
        assert!(({ ({ (*v1.borrow())[23_usize] } == -13_i32) }));
        assert!(({ ({ (*v1.borrow())[24_usize] } == -14_i32) }));
        assert!(({ ({ (*v1.borrow())[25_usize] } == 25) }));
        assert!(({ ({ (*v1.borrow())[26_usize] } == -15_i32) }));
        assert!(({ ({ (*v1.borrow())[27_usize] } == -16_i32) }));
        assert!(({ ({ (*v1.borrow())[28_usize] } == -17_i32) }));
        assert!(({ ({ (*v1.borrow())[29_usize] } == 29) }));
        assert!(({ ({ (*v1.borrow())[30_usize] } == -18_i32) }));
        assert!(({ ({ (*v1.borrow())[31_usize] } == -19_i32) }));
        assert!(({ ({ (*v1.borrow())[32_usize] } == -20_i32) }));
        assert!(({ ({ (*v1.borrow())[33_usize] } == 33) }));
        assert!(({ ({ (*v1.borrow())[34_usize] } == -21_i32) }));
        assert!(({ ({ (*v1.borrow())[35_usize] } == -22_i32) }));
        assert!(({ ({ (*v1.borrow())[36_usize] } == -23_i32) }));
        assert!(({ ({ (*v1.borrow())[37_usize] } == 37) }));
        assert!(({ ({ (*v1.borrow())[38_usize] } == 38) }));
        assert!(({ ({ (*v1.borrow())[39_usize] } == 39) }));
        assert!(({ ({ (*v1.borrow())[40_usize] } == 40) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
