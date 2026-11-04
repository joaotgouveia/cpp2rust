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
        let b: Value<Box<[f64]>> =
            Rc::new(RefCell::new(Box::new([10_f64, 14_f64, 16_f64, 17_f64])));
        let p: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([25_f64, 62.5_f64, 12.5_f64])));
        let pa: Value<std_piecewise_constant_distribution___param_type> = Rc::new(RefCell::new(
            std_piecewise_constant_distribution___param_type::new_1(
                { (b.as_pointer() as Ptr<f64>) },
                { (b.as_pointer() as Ptr<f64>).offset((4) as isize) },
                { (p.as_pointer() as Ptr<f64>) },
            ),
        ));
        let d: Value<std_piecewise_constant_distribution_double_> = Rc::new(RefCell::new(
            std_piecewise_constant_distribution_double_::new_1({ pa.as_pointer() }),
        ));
        let iv: Value<Vec<f64>> = Rc::new(RefCell::new(({ (*d.borrow()).intervals() })));
        assert!(({ ((*iv.borrow()).len() == 4_usize) }));
        assert!(((iv.decay().offset(0_usize as isize).read()) == 10_f64));
        assert!(((iv.decay().offset(1_usize as isize).read()) == 14_f64));
        assert!(((iv.decay().offset(2_usize as isize).read()) == 16_f64));
        assert!(((iv.decay().offset(3_usize as isize).read()) == 17_f64));
        let dn: Value<Vec<f64>> = Rc::new(RefCell::new(({ (*d.borrow()).densities() })));
        assert!(({ ((*dn.borrow()).len() == 3_usize) }));
        assert!(((dn.decay().offset(0_usize as isize).read()) == 0.0625_f64));
        assert!(((dn.decay().offset(1_usize as isize).read()) == 0.3125_f64));
        assert!(((dn.decay().offset(2_usize as isize).read()) == 0.125_f64));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
