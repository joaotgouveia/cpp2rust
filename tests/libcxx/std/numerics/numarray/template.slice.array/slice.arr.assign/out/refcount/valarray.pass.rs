extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_void__void_ {}
impl CppMove for test_macros_detail_is_same_void__void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
    let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ])));
    let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        -1_i32, -2_i32, -3_i32, -4_i32, -5_i32,
    ])));
    let v1: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
        { (a1.as_pointer() as Ptr<i32>) },
        {
            (::std::mem::size_of::<[i32; 16]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))
        },
    )));
    let v2: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
        { (a2.as_pointer() as Ptr<i32>) },
        {
            (::std::mem::size_of::<[i32; 5]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))
        },
    )));
    let s1: Value<std_slice_array_int_> = Rc::new(RefCell::new(
        elem!(
            v1.as_pointer(),
            std_slice::new_1({ 1_usize }, { 5_usize }, { 3_usize },)
        )
        .read(),
    ));
    (*s1.borrow_mut()) = (*v2.borrow()).clone();
    assert!(({ (({ (*v1.borrow()).size() }) == 16_usize) }));
    assert!(({ ({ (*v1.borrow())[0_usize] } == 0) }));
    assert!(({ ({ (*v1.borrow())[1_usize] } == -1_i32) }));
    assert!(({ ({ (*v1.borrow())[2_usize] } == 2) }));
    assert!(({ ({ (*v1.borrow())[3_usize] } == 3) }));
    assert!(({ ({ (*v1.borrow())[4_usize] } == -2_i32) }));
    assert!(({ ({ (*v1.borrow())[5_usize] } == 5) }));
    assert!(({ ({ (*v1.borrow())[6_usize] } == 6) }));
    assert!(({ ({ (*v1.borrow())[7_usize] } == -3_i32) }));
    assert!(({ ({ (*v1.borrow())[8_usize] } == 8) }));
    assert!(({ ({ (*v1.borrow())[9_usize] } == 9) }));
    assert!(({ ({ (*v1.borrow())[10_usize] } == -4_i32) }));
    assert!(({ ({ (*v1.borrow())[11_usize] } == 11) }));
    assert!(({ ({ (*v1.borrow())[12_usize] } == 12) }));
    assert!(({ ({ (*v1.borrow())[13_usize] } == -5_i32) }));
    assert!(({ ({ (*v1.borrow())[14_usize] } == 14) }));
    assert!(({ ({ (*v1.borrow())[15_usize] } == 15) }));
    const _: () = assert!(true, "");;
    let m: Value<std_valarray_double_> = Rc::new(RefCell::new(std_valarray_double_::new_1({
        Vec::from([0_f64, 0_f64, 0_f64])
    })));
    let s2: Value<std_slice_array_double_> = Rc::new(RefCell::new(
        elem!(
            m.as_pointer(),
            std_slice::new_1({ 0_usize }, { 3_usize }, { 1_usize },)
        )
        .read(),
    ));
    (*s2.borrow_mut()) = std_valarray_double_::new_1({ Vec::from([1_f64, 2_f64, 3_f64]) });
    assert!(({ ({ (*m.borrow())[0_usize] } == 1_f64) }));
    assert!(({ ({ (*m.borrow())[1_usize] } == 2_f64) }));
    assert!(({ ({ (*m.borrow())[2_usize] } == 3_f64) }));
    const _: () = assert!(true, "");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
