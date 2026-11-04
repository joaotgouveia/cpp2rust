extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fw_0(mut x: f64) -> f64 {
    return (x + 1_f64);
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
    {
        let pa: Value<std_discrete_distribution___param_type> = Rc::new(RefCell::new(
            std_discrete_distribution___param_type::new_1({ 0_usize }, { 0_f64 }, { 1_f64 }, {
                FnPtr::<fn(f64) -> f64>::new(fw_0)
            }),
        ));
        let p: Value<Vec<f64>> = Rc::new(RefCell::new(({ (*pa.borrow()).probabilities() })));
        assert!(({ ((*p.borrow()).len() == 1_usize) }));
        assert!(((p.decay().offset(0_usize as isize).read()) == 1_f64));
    }
    {
        let pa: Value<std_discrete_distribution___param_type> = Rc::new(RefCell::new(
            std_discrete_distribution___param_type::new_1({ 1_usize }, { 0_f64 }, { 1_f64 }, {
                FnPtr::<fn(f64) -> f64>::new(fw_0)
            }),
        ));
        let p: Value<Vec<f64>> = Rc::new(RefCell::new(({ (*pa.borrow()).probabilities() })));
        assert!(({ ((*p.borrow()).len() == 1_usize) }));
        assert!(((p.decay().offset(0_usize as isize).read()) == 1_f64));
    }
    {
        let pa: Value<std_discrete_distribution___param_type> = Rc::new(RefCell::new(
            std_discrete_distribution___param_type::new_1({ 2_usize }, { 0.5_f64 }, { 1.5_f64 }, {
                FnPtr::<fn(f64) -> f64>::new(fw_0)
            }),
        ));
        let p: Value<Vec<f64>> = Rc::new(RefCell::new(({ (*pa.borrow()).probabilities() })));
        assert!(({ ((*p.borrow()).len() == 2_usize) }));
        assert!(((p.decay().offset(0_usize as isize).read()) == 0.4375_f64));
        assert!(((p.decay().offset(1_usize as isize).read()) == 0.5625_f64));
    }
    {
        let pa: Value<std_discrete_distribution___param_type> = Rc::new(RefCell::new(
            std_discrete_distribution___param_type::new_1({ 4_usize }, { 0_f64 }, { 2_f64 }, {
                FnPtr::<fn(f64) -> f64>::new(fw_0)
            }),
        ));
        let p: Value<Vec<f64>> = Rc::new(RefCell::new(({ (*pa.borrow()).probabilities() })));
        assert!(({ ((*p.borrow()).len() == 4_usize) }));
        assert!(((p.decay().offset(0_usize as isize).read()) == 0.15625_f64));
        assert!(((p.decay().offset(1_usize as isize).read()) == 0.21875_f64));
        assert!(((p.decay().offset(2_usize as isize).read()) == 0.28125_f64));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
