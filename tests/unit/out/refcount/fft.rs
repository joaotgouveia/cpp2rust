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
pub fn fft_3(
    a: Ptr<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__>,
    mut N: (),
) -> std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ {
    let y: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4(N) })));
    if (N == 1_()) {
        let __rhs = Complex {
            re: elem!(a, 0_()).with(|__s| __s.re),
            img: elem!(a, 0_()).with(|__s| __s.img),
        };
        (*y.borrow_mut())[0_()] = __rhs;
        return (*y.borrow_mut()).clone();
    }
    let w: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4(N) })));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let mut alpha: () = (((-2_() * 3.141592653589793_f64) * i) / N);
        let __rhs = Complex {
            re: unimplemented!(),
            img: unimplemented!(),
        };
        (*w.borrow_mut())[i] = __rhs;
        i.postfix_inc();
    }
    let A0: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4((N / 2_())) })));
    let A1: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4((N / 2_())) })));
    let mut i: () = 0_();
    'loop_: while (i < (N / 2_())) {
        let __rhs = Complex {
            re: elem!(a, (i * 2_())).with(|__s| __s.re),
            img: elem!(a, (i * 2_())).with(|__s| __s.img),
        };
        (*A0.borrow_mut())[i] = __rhs;
        let __rhs = Complex {
            re: elem!(a, ((i * 2_()) + 1_())).with(|__s| __s.re),
            img: elem!(a, ((i * 2_()) + 1_())).with(|__s| __s.img),
        };
        (*A1.borrow_mut())[i] = __rhs;
        i.postfix_inc();
    }
    let y0: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ fft_3(A0.as_pointer(), (N / 2_())) })));
    let y1: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ fft_3(A1.as_pointer(), (N / 2_())) })));
    let mut k: () = 0_();
    'loop_: while (k < (N / 2_())) {
        let mut yk: Complex = ({
            let _z1: Complex = ((*y0.borrow())[k]).clone();
            let _z2: Complex = ({
                let _z1: Complex = ((*w.borrow())[k]).clone();
                let _z2: Complex = ((*y1.borrow())[k]).clone();
                Product_0(_z1, _z2)
            });
            Sum_1(_z1, _z2)
        });
        (*y.borrow_mut())[k] = Complex {
            re: yk.re,
            img: yk.img,
        };
        let mut yk_n2: Complex = ({
            let _z1: Complex = ((*y0.borrow())[k]).clone();
            let _z2: Complex = ({
                Neg_2(
                    ({
                        let _z1: Complex = ((*w.borrow())[k]).clone();
                        let _z2: Complex = ((*y1.borrow())[k]).clone();
                        Product_0(_z1, _z2)
                    }),
                )
            });
            Sum_1(_z1, _z2)
        });
        (*y.borrow_mut())[(k + (N / 2_()))] = Complex {
            re: yk_n2.re,
            img: yk_n2.img,
        };
        k.postfix_inc();
    }
    return (*y.borrow_mut()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut N: () = 4_();
    let a: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4(N) })));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let __rhs = Complex {
            re: (i + 1_()),
            img: 0_(),
        };
        (*a.borrow_mut())[i] = __rhs;
        i.postfix_inc();
    }
    let b: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ fft_3(a.as_pointer(), N) })));
    let reals: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_5(N) })));
    let imgs: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_5(N) })));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        let __rhs = unimplemented!();
        (*reals.borrow_mut())[i] = __rhs;
        let __rhs = unimplemented!();
        (*imgs.borrow_mut())[i] = __rhs;
        i.prefix_inc();
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
