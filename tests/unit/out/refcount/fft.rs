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
    pub re: f64,
    #[offset(8)]
    pub img: f64,
}
impl CppMove for Complex {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn Product_0(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: f64 = (z1.re * z2.re);
    let mut bd: f64 = (z1.img * z2.img);
    let mut ad: f64 = (z1.re * z2.img);
    let mut bc: f64 = (z1.img * z2.re);
    return Complex {
        re: (ac - bd),
        img: (ad + bc),
    };
}
pub fn Sum_1(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: f64 = (z1.re + z2.re);
    let mut bd: f64 = (z1.img + z2.img);
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
    mut N: i32,
) -> std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__ {
    let y: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4((N as u64)) })));
    if (N == 1) {
        let __rhs = Complex {
            re: elem!(a, 0_u64).with(|__s| __s.re),
            img: elem!(a, 0_u64).with(|__s| __s.img),
        };
        (*y.borrow_mut())[0_u64] = __rhs;
        return (*y.borrow_mut()).clone();
    }
    let w: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4((N as u64)) })));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let mut alpha: f64 =
            ((((-2_i32 as f64) * 3.141592653589793_f64) * (i as f64)) / (N as f64));
        let __rhs = Complex {
            re: ({ cos_5(alpha) }),
            img: ({ sin_6(alpha) }),
        };
        (*w.borrow_mut())[(i as u64)] = __rhs;
        i.postfix_inc();
    }
    let A0: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4(((N / 2) as u64)) })));
    let A1: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4(((N / 2) as u64)) })));
    let mut i: i32 = 0;
    'loop_: while (i < (N / 2)) {
        let __rhs = Complex {
            re: elem!(a, ((i * 2) as u64)).with(|__s| __s.re),
            img: elem!(a, ((i * 2) as u64)).with(|__s| __s.img),
        };
        (*A0.borrow_mut())[(i as u64)] = __rhs;
        let __rhs = Complex {
            re: elem!(a, (((i * 2) + 1) as u64)).with(|__s| __s.re),
            img: elem!(a, (((i * 2) + 1) as u64)).with(|__s| __s.img),
        };
        (*A1.borrow_mut())[(i as u64)] = __rhs;
        i.postfix_inc();
    }
    let y0: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ fft_3(A0.as_pointer(), (N / 2)) })));
    let y1: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ fft_3(A1.as_pointer(), (N / 2)) })));
    let mut k: i32 = 0;
    'loop_: while (k < (N / 2)) {
        let mut yk: Complex = ({
            let _z1: Complex = ((*y0.borrow())[(k as u64)]).clone();
            let _z2: Complex = ({
                let _z1: Complex = ((*w.borrow())[(k as u64)]).clone();
                let _z2: Complex = ((*y1.borrow())[(k as u64)]).clone();
                Product_0(_z1, _z2)
            });
            Sum_1(_z1, _z2)
        });
        (*y.borrow_mut())[(k as u64)] = Complex {
            re: yk.re,
            img: yk.img,
        };
        let mut yk_n2: Complex = ({
            let _z1: Complex = ((*y0.borrow())[(k as u64)]).clone();
            let _z2: Complex = ({
                Neg_2(
                    ({
                        let _z1: Complex = ((*w.borrow())[(k as u64)]).clone();
                        let _z2: Complex = ((*y1.borrow())[(k as u64)]).clone();
                        Product_0(_z1, _z2)
                    }),
                )
            });
            Sum_1(_z1, _z2)
        });
        (*y.borrow_mut())[((k + (N / 2)) as u64)] = Complex {
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
fn main_0() -> i32 {
    let mut N: i32 = 4;
    let a: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ make_unique_4((N as u64)) })));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let __rhs = Complex {
            re: ((i as f64) + 1_f64),
            img: 0_f64,
        };
        (*a.borrow_mut())[(i as u64)] = __rhs;
        i.postfix_inc();
    }
    let b: Value<std_unique_ptr_Complexarrarr__std_default_delete_Complexarrarr__> =
        Rc::new(RefCell::new(({ fft_3(a.as_pointer(), N) })));
    let reals: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_7((N as u64)) })));
    let imgs: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_7((N as u64)) })));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let __rhs = (({ round_8({ (*b.borrow())[(i as u64)].re }) }) as i32);
        (*reals.borrow_mut())[(i as u64)] = __rhs;
        let __rhs = (({ round_8({ (*b.borrow())[(i as u64)].img }) }) as i32);
        (*imgs.borrow_mut())[(i as u64)] = __rhs;
        i.prefix_inc();
    }
    ({
        cpp2rust_assert_fail_9(
            ((((((({ (*reals.borrow())[0_u64] } == 10) && ({ (*imgs.borrow())[0_u64] } == 0))
                && ({ (*reals.borrow())[1_u64] } == -2_i32))
                && ({ (*imgs.borrow())[1_u64] } == 2))
                && ({ (*reals.borrow())[2_u64] } == -2_i32))
                && ({ (*imgs.borrow())[2_u64] } == 0))
                && ({ (*reals.borrow())[3_u64] } == -2_i32))
                && ({ (*imgs.borrow())[3_u64] } == -2_i32),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
