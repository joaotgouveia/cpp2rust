extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(rhs: Ptr<()>) {
    let input: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        64_(),
        256_(),
        768_(),
        2048_(),
        5120_(),
    ])));
    let mut expected: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut N: () =
        (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let value: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_1(rhs: Ptr<()>) {
    let input: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        64_(),
        256_(),
        768_(),
        2048_(),
        5120_(),
    ])));
    let mut expected: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut N: () =
        (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let value: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_2(rhs: Ptr<()>) {
    let input: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        64_(),
        256_(),
        768_(),
        2048_(),
        5120_(),
    ])));
    let mut expected: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut N: () =
        (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let value: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_3(rhs: Ptr<()>) {
    let input: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        64_(),
        256_(),
        768_(),
        2048_(),
        5120_(),
    ])));
    let mut expected: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut N: () =
        (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let value: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_4(rhs: Ptr<()>) {
    let input: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        64_(),
        256_(),
        768_(),
        2048_(),
        5120_(),
    ])));
    let mut expected: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut N: () =
        (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let value: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        unimplemented!();
        i.prefix_inc();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    let input: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([6_(), 7_(), 8_(), 9_(), 10_()])));
    let mut N: () =
        (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mask: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let indices: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 1_(), 2_(), 3_(), 4_()])));
    let indirect: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let zero: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let value: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_0(value.as_pointer()) });
        ({
            let _rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_1(_rhs.as_pointer())
        });
        ({
            let _rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_2(_rhs.as_pointer())
        });
        ({
            let _rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_3(_rhs.as_pointer())
        });
        ({
            let _rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            test_4(_rhs.as_pointer())
        });
        ({
            let _rhs: Value<
                std___val_expr_std__BinaryOp_std_plus_int___std_valarray_int___std_valarray_int___,
            > = Rc::new(RefCell::new(unimplemented!()));
            test_5(_rhs.as_pointer())
        });
    }
    {
        let value: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ test_0(value.as_pointer()) });
        ({
            let _rhs: Value<std___val_expr_std___slice_expr_const_std_valarray_int__ref__> =
                Rc::new(RefCell::new(unimplemented!()));
            test_6(_rhs.as_pointer())
        });
        ({
            let _rhs: Value<std___val_expr_std___indirect_expr_const_std_valarray_int__ref__> =
                Rc::new(RefCell::new(unimplemented!()));
            test_7(_rhs.as_pointer())
        });
        ({
            let _rhs: Value<std___val_expr_std___mask_expr_const_std_valarray_int__ref__> =
                Rc::new(RefCell::new(unimplemented!()));
            test_8(_rhs.as_pointer())
        });
        ({
            let _rhs: Value<std___val_expr_std___indirect_expr_const_std_valarray_int__ref__> =
                Rc::new(RefCell::new(unimplemented!()));
            test_7(_rhs.as_pointer())
        });
        ({
            let _rhs: Value<
                std___val_expr_std__BinaryOp_std_plus_int___std_valarray_int___std_valarray_int___,
            > = Rc::new(RefCell::new(unimplemented!()));
            test_5(_rhs.as_pointer())
        });
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
