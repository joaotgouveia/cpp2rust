extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Complex {
    pub re: (),
    pub img: (),
}
pub unsafe fn Product_0(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: () = ((z1.re) * (z2.re));
    let mut bd: () = ((z1.img) * (z2.img));
    let mut ad: () = ((z1.re) * (z2.img));
    let mut bc: () = ((z1.img) * (z2.re));
    return Complex {
        re: ((ac) - (bd)),
        img: ((ad) + (bc)),
    };
}
pub unsafe fn Sum_1(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: () = ((z1.re) + (z2.re));
    let mut bd: () = ((z1.img) + (z2.img));
    return Complex { re: ac, img: bd };
}
pub unsafe fn Neg_2(mut z1: Complex) -> Complex {
    return Complex {
        re: -z1.re,
        img: -z1.img,
    };
}
pub unsafe fn fft_3(a: *mut (), mut N: ()) -> () {
    let mut y: () = unimplemented!();
    if ((N) == (1_())) {
        (*unimplemented!()) = Complex {
            re: (*unimplemented!()).re,
            img: (*unimplemented!()).img,
        };
        return unimplemented!();
    }
    let mut w: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut alpha: () = ((((-2_()) * (3.141592653589793_f64)) * (i)) / (N));
        (*unimplemented!()) = Complex {
            re: unimplemented!(),
            img: unimplemented!(),
        };
        i.postfix_inc();
    }
    let mut A0: () = unimplemented!();
    let mut A1: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < ((N) / (2_()))) {
        (*unimplemented!()) = Complex {
            re: (*unimplemented!()).re,
            img: (*unimplemented!()).img,
        };
        (*unimplemented!()) = Complex {
            re: (*unimplemented!()).re,
            img: (*unimplemented!()).img,
        };
        i.postfix_inc();
    }
    let mut y0: () = (unsafe { fft_3(&mut A0, ((N) / (2_()))) });
    let mut y1: () = (unsafe { fft_3(&mut A1, ((N) / (2_()))) });
    let mut k: () = 0_();
    'loop_: while ((k) < ((N) / (2_()))) {
        let mut yk: Complex = (unsafe {
            let _z1: Complex = (*unimplemented!());
            let _z2: Complex = (unsafe {
                let _z1: Complex = (*unimplemented!());
                let _z2: Complex = (*unimplemented!());
                Product_0(_z1, _z2)
            });
            Sum_1(_z1, _z2)
        });
        (*unimplemented!()) = Complex {
            re: yk.re,
            img: yk.img,
        };
        let mut yk_n2: Complex = (unsafe {
            let _z1: Complex = (*unimplemented!());
            let _z2: Complex = (unsafe {
                Neg_2(
                    (unsafe {
                        let _z1: Complex = (*unimplemented!());
                        let _z2: Complex = (*unimplemented!());
                        Product_0(_z1, _z2)
                    }),
                )
            });
            Sum_1(_z1, _z2)
        });
        (*unimplemented!()) = Complex {
            re: yk_n2.re,
            img: yk_n2.img,
        };
        k.postfix_inc();
    }
    return unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut N: () = 4_();
    let mut a: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*unimplemented!()) = Complex {
            re: ((i) + (1_())),
            img: 0_(),
        };
        i.postfix_inc();
    }
    let mut b: () = (unsafe { fft_3(&mut a, N) });
    let mut reals: () = unimplemented!();
    let mut imgs: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*unimplemented!()) = unimplemented!();
        (*unimplemented!()) = unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
