extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sqr_0(mut x: f64) -> f64 {
    return (x * x);
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
        let g: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut N: i32 = 100000;
        let u: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            let mut v: () = unimplemented!();
            assert!((unimplemented!() <= v) && (v < unimplemented!()));
            {
                let a0_clone = v.clone();
                (*u.borrow_mut()).push(a0_clone)
            };
            i.prefix_inc();
        }
        let mut mean: () = (unimplemented!() / ((*u.borrow()).len() as f64));
        let mut var: () = 0_();
        let mut skew: () = 0_();
        let mut kurtosis: () = 0_();
        let mut i: usize = 0_usize;
        'loop_: while (i < (*u.borrow()).len()) {
            let mut dbl: () = ((u.decay().offset(i as isize).read()) - mean);
            let mut d2: () = ({ sqr_0(dbl) });
            var += d2;
            skew += (dbl * d2);
            kurtosis += (d2 * d2);
            i.prefix_inc();
        }
        var /= ((*u.borrow()).len() as ());
        let mut dev: () = unimplemented!();
        skew /= ((((*u.borrow()).len() as ()) * dev) * var);
        kurtosis /= ((((*u.borrow()).len() as ()) * var) * var);
        kurtosis -= 3_();
        let mut x_mean: () = ((unimplemented!() + unimplemented!()) / 2_());
        let mut x_var: () = (({ sqr_0((unimplemented!() - unimplemented!())) }) / 12_f64);
        let mut x_skew: () = 0_();
        let mut x_kurtosis: () = (-6_f64 / 5_f64);
        assert!((unimplemented!() < 0.01_f64));
        assert!((unimplemented!() < 0.01_f64));
        assert!((unimplemented!() < 0.01_f64));
        assert!((unimplemented!() < 0.01_f64));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
