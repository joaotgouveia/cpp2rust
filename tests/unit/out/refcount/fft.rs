extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Complex {
    #[offset(0)]
    pub re: (),
    #[offset(8)]
    pub img: (),
}
pub fn Product_0(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: () = (z1.re * z2.re);
    let mut bd: () = (z1.img * z2.img);
    let mut ad: () = (z1.re * z2.img);
    let mut bc: () = (z1.img * z2.re);
    return Complex {
        re: (ac - bd),
        img: (ad + bc),
    };
}
pub fn Sum_1(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: () = (z1.re + z2.re);
    let mut bd: () = (z1.img + z2.img);
    return Complex { re: ac, img: bd };
}
pub fn Neg_2(mut z1: Complex) -> Complex {
    return Complex {
        re: -z1.re,
        img: -z1.img,
    };
}
pub fn fft_3(a: Ptr<()>, mut N: ()) -> () {
    let mut y: () = unimplemented!();
    if (N == 1_()) {
        let __rhs = Complex {
            re: unimplemented!().with(|__s| __s.re),
            img: unimplemented!().with(|__s| __s.img),
        };
        unimplemented!().write(__rhs);
        return unimplemented!();
    }
    let mut w: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let mut alpha: () = (((-2_() * 3.141592653589793_f64) * i) / N);
        let __rhs = Complex {
            re: unimplemented!(),
            img: unimplemented!(),
        };
        unimplemented!().write(__rhs);
        i.postfix_inc();
    }
    let A0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let A1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < (N / 2_())) {
        let __rhs = Complex {
            re: unimplemented!().with(|__s| __s.re),
            img: unimplemented!().with(|__s| __s.img),
        };
        unimplemented!().write(__rhs);
        let __rhs = Complex {
            re: unimplemented!().with(|__s| __s.re),
            img: unimplemented!().with(|__s| __s.img),
        };
        unimplemented!().write(__rhs);
        i.postfix_inc();
    }
    let mut y0: () = ({ fft_3(A0.as_pointer(), (N / 2_())) });
    let mut y1: () = ({ fft_3(A1.as_pointer(), (N / 2_())) });
    let mut k: () = 0_();
    'loop_: while (k < (N / 2_())) {
        let mut yk: Complex = ({
            let _z1: Complex = (*unimplemented!().upgrade().deref()).clone();
            let _z2: Complex = ({
                let _z1: Complex = (*unimplemented!().upgrade().deref()).clone();
                let _z2: Complex = (*unimplemented!().upgrade().deref()).clone();
                Product_0(_z1, _z2)
            });
            Sum_1(_z1, _z2)
        });
        unimplemented!().write(Complex {
            re: yk.re,
            img: yk.img,
        });
        let mut yk_n2: Complex = ({
            let _z1: Complex = (*unimplemented!().upgrade().deref()).clone();
            let _z2: Complex = ({
                Neg_2(
                    ({
                        let _z1: Complex = (*unimplemented!().upgrade().deref()).clone();
                        let _z2: Complex = (*unimplemented!().upgrade().deref()).clone();
                        Product_0(_z1, _z2)
                    }),
                )
            });
            Sum_1(_z1, _z2)
        });
        unimplemented!().write(Complex {
            re: yk_n2.re,
            img: yk_n2.img,
        });
        k.postfix_inc();
    }
    return unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut N: () = 4_();
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let __rhs = Complex {
            re: (i + 1_()),
            img: 0_(),
        };
        unimplemented!().write(__rhs);
        i.postfix_inc();
    }
    let mut b: () = ({ fft_3(a.as_pointer(), N) });
    let mut reals: () = unimplemented!();
    let mut imgs: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let __rhs = unimplemented!();
        unimplemented!().write(__rhs);
        let __rhs = unimplemented!();
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
