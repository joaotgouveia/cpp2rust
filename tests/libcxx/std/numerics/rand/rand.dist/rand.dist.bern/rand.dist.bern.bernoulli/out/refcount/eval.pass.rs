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
        let mut N: i32 = 100000;
        let u: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut mean: f64 = (unimplemented!() / ((&(*u.borrow())).len() as f64));
        let mut var: f64 = 0_f64;
        let mut skew: f64 = 0_f64;
        let mut kurtosis: f64 = 0_f64;
        let mut i: usize = 0_usize;
        'loop_: while (i < (&(*u.borrow())).len()) {
            let mut dbl: f64 = (((u.decay().offset(i as isize).read() as i32) as f64) - mean);
            let mut d2: f64 = ({ sqr_0(dbl) });
            var += d2;
            skew += (dbl * d2);
            kurtosis += (d2 * d2);
            i.prefix_inc();
        }
        var /= ((&(*u.borrow())).len() as f64);
        let mut dev: f64 = unimplemented!();
        skew /= ((((&(*u.borrow())).len() as f64) * dev) * var);
        kurtosis /= ((((&(*u.borrow())).len() as f64) * var) * var);
        kurtosis -= 3_f64;
        let mut x_mean: f64 = unimplemented!();
        let mut x_var: f64 = (unimplemented!() * (1_f64 - unimplemented!()));
        let mut x_skew: f64 = ((1_f64 - (2_f64 * unimplemented!())) / unimplemented!());
        let mut x_kurtosis: f64 =
            ((((6_f64 * ({ sqr_0(unimplemented!()) })) - (6_f64 * unimplemented!())) + 1_f64)
                / x_var);
        assert!((unimplemented!() < 0.01_f64));
        assert!((unimplemented!() < 0.01_f64));
        assert!((unimplemented!() < 0.02_f64));
        assert!((unimplemented!() < 0.05_f64));
    }
    {
        let g: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut N: i32 = 100000;
        let u: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut mean: f64 = (unimplemented!() / ((&(*u.borrow())).len() as f64));
        let mut var: f64 = 0_f64;
        let mut skew: f64 = 0_f64;
        let mut kurtosis: f64 = 0_f64;
        let mut i: usize = 0_usize;
        'loop_: while (i < (&(*u.borrow())).len()) {
            let mut dbl: f64 = (((u.decay().offset(i as isize).read() as i32) as f64) - mean);
            let mut d2: f64 = ({ sqr_0(dbl) });
            var += d2;
            skew += (dbl * d2);
            kurtosis += (d2 * d2);
            i.prefix_inc();
        }
        var /= ((&(*u.borrow())).len() as f64);
        let mut dev: f64 = unimplemented!();
        skew /= ((((&(*u.borrow())).len() as f64) * dev) * var);
        kurtosis /= ((((&(*u.borrow())).len() as f64) * var) * var);
        kurtosis -= 3_f64;
        let mut x_mean: f64 = unimplemented!();
        let mut x_var: f64 = (unimplemented!() * (1_f64 - unimplemented!()));
        let mut x_skew: f64 = ((1_f64 - (2_f64 * unimplemented!())) / unimplemented!());
        let mut x_kurtosis: f64 =
            ((((6_f64 * ({ sqr_0(unimplemented!()) })) - (6_f64 * unimplemented!())) + 1_f64)
                / x_var);
        assert!((unimplemented!() < 0.01_f64));
        assert!((unimplemented!() < 0.01_f64));
        assert!((unimplemented!() < 0.02_f64));
        assert!((unimplemented!() < 0.05_f64));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
