extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sqr_0(mut x: ()) -> () {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let g: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut N: () = 100000_();
        let mut u: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut mean: () = (unimplemented!() / unimplemented!());
        let mut var: () = 0_();
        let mut skew: () = 0_();
        let mut kurtosis: () = 0_();
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            let mut dbl: () = (unimplemented!() - mean);
            let mut d2: () = ({ sqr_0(dbl) });
            var += d2;
            skew += (dbl * d2);
            kurtosis += (d2 * d2);
            i.prefix_inc();
        }
        var /= unimplemented!();
        let mut dev: () = ({ sqrt_1(var) });
        skew /= ((unimplemented!() * dev) * var);
        kurtosis /= ((unimplemented!() * var) * var);
        kurtosis -= 3_();
        let mut x_mean: () = unimplemented!();
        let mut x_var: () = (unimplemented!() * (1_() - unimplemented!()));
        let mut x_skew: () = ((1_() - (2_() * unimplemented!())) / ({ sqrt_1(x_var) }));
        let mut x_kurtosis: () =
            ((((6_() * ({ sqr_0(unimplemented!()) })) - (6_() * unimplemented!())) + 1_()) / x_var);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let g: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut N: () = 100000_();
        let mut u: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < N) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut mean: () = (unimplemented!() / unimplemented!());
        let mut var: () = 0_();
        let mut skew: () = 0_();
        let mut kurtosis: () = 0_();
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            let mut dbl: () = (unimplemented!() - mean);
            let mut d2: () = ({ sqr_0(dbl) });
            var += d2;
            skew += (dbl * d2);
            kurtosis += (d2 * d2);
            i.prefix_inc();
        }
        var /= unimplemented!();
        let mut dev: () = ({ sqrt_1(var) });
        skew /= ((unimplemented!() * dev) * var);
        kurtosis /= ((unimplemented!() * var) * var);
        kurtosis -= 3_();
        let mut x_mean: () = unimplemented!();
        let mut x_var: () = (unimplemented!() * (1_() - unimplemented!()));
        let mut x_skew: () = ((1_() - (2_() * unimplemented!())) / ({ sqrt_1(x_var) }));
        let mut x_kurtosis: () =
            ((((6_() * ({ sqr_0(unimplemented!()) })) - (6_() * unimplemented!())) + 1_()) / x_var);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
