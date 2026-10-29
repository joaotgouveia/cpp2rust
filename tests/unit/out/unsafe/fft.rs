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
pub unsafe fn fft_3(
    a: *mut std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__,
    mut N: (),
) -> std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ {
    let mut y: std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ =
        (unsafe { make_unique_4(N) });
    if ((N) == (1_())) {
        y[(0_()) as usize] = Complex {
            re: (&(*a))[(0_()) as usize].re,
            img: (&(*a))[(0_()) as usize].img,
        };
        return y;
    }
    let mut w: std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ =
        (unsafe { make_unique_4(N) });
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut alpha: () = ((((-2_()) * (3.141592653589793_f64)) * (i)) / (N));
        w[(i) as usize] = Complex {
            re: unimplemented!(),
            img: unimplemented!(),
        };
        i.postfix_inc();
    }
    let mut A0: std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ =
        (unsafe { make_unique_4(((N) / (2_()))) });
    let mut A1: std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ =
        (unsafe { make_unique_4(((N) / (2_()))) });
    let mut i: () = 0_();
    'loop_: while ((i) < ((N) / (2_()))) {
        A0[(i) as usize] = Complex {
            re: (&(*a))[((i) * (2_())) as usize].re,
            img: (&(*a))[((i) * (2_())) as usize].img,
        };
        A1[(i) as usize] = Complex {
            re: (&(*a))[(((i) * (2_())) + (1_())) as usize].re,
            img: (&(*a))[(((i) * (2_())) + (1_())) as usize].img,
        };
        i.postfix_inc();
    }
    let mut y0: std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ =
        (unsafe { fft_3(&mut A0, ((N) / (2_()))) });
    let mut y1: std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ =
        (unsafe { fft_3(&mut A1, ((N) / (2_()))) });
    let mut k: () = 0_();
    'loop_: while ((k) < ((N) / (2_()))) {
        let mut yk: Complex = (unsafe {
            let _z1: Complex = y0[(k) as usize];
            let _z2: Complex = (unsafe {
                let _z1: Complex = w[(k) as usize];
                let _z2: Complex = y1[(k) as usize];
                Product_0(_z1, _z2)
            });
            Sum_1(_z1, _z2)
        });
        y[(k) as usize] = Complex {
            re: yk.re,
            img: yk.img,
        };
        let mut yk_n2: Complex = (unsafe {
            let _z1: Complex = y0[(k) as usize];
            let _z2: Complex = (unsafe {
                Neg_2(
                    (unsafe {
                        let _z1: Complex = w[(k) as usize];
                        let _z2: Complex = y1[(k) as usize];
                        Product_0(_z1, _z2)
                    }),
                )
            });
            Sum_1(_z1, _z2)
        });
        y[((k) + ((N) / (2_()))) as usize] = Complex {
            re: yk_n2.re,
            img: yk_n2.img,
        };
        k.postfix_inc();
    }
    return y;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut N: () = 4_();
    let mut a: std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ =
        (unsafe { make_unique_4(N) });
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        a[(i) as usize] = Complex {
            re: ((i) + (1_())),
            img: 0_(),
        };
        i.postfix_inc();
    }
    let mut b: std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ =
        (unsafe { fft_3(&mut a, N) });
    let mut reals: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        (unsafe { make_unique_5(N) });
    let mut imgs: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        (unsafe { make_unique_5(N) });
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        reals[(i) as usize] = unimplemented!();
        imgs[(i) as usize] = unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
