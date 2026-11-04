extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let z: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    let mut a: Ptr<f32> = (z.as_pointer()).reinterpret_cast::<f32>();
    assert!(({ (0_f32 == ({ (*z.borrow()).real_5() })) }));
    assert!(({ (0_f32 == ({ (*z.borrow()).imag_6() })) }));
    assert!(({ ({ (elem!(a, 0).read()) } == { ({ (*z.borrow()).real_5() }) }) }));
    assert!(({ ({ (elem!(a, 1).read()) } == { ({ (*z.borrow()).imag_6() }) }) }));
    elem!(a, 0).write(5_f32);
    elem!(a, 1).write(6_f32);
    assert!(({ ({ (elem!(a, 0).read()) } == { ({ (*z.borrow()).real_5() }) }) }));
    assert!(({ ({ (elem!(a, 1).read()) } == { ({ (*z.borrow()).imag_6() }) }) }));
}
pub fn test_1() {
    let z: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    let mut a: Ptr<f64> = (z.as_pointer()).reinterpret_cast::<f64>();
    assert!(({ (0_f64 == ({ (*z.borrow()).real_5() })) }));
    assert!(({ (0_f64 == ({ (*z.borrow()).imag_6() })) }));
    assert!(({ ({ (elem!(a, 0).read()) } == { ({ (*z.borrow()).real_5() }) }) }));
    assert!(({ ({ (elem!(a, 1).read()) } == { ({ (*z.borrow()).imag_6() }) }) }));
    elem!(a, 0).write(5_f64);
    elem!(a, 1).write(6_f64);
    assert!(({ ({ (elem!(a, 0).read()) } == { ({ (*z.borrow()).real_5() }) }) }));
    assert!(({ ({ (elem!(a, 1).read()) } == { ({ (*z.borrow()).imag_6() }) }) }));
}
pub fn test_2() {
    let z: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    let mut a: Ptr<f64> = (z.as_pointer()).reinterpret_cast::<f64>();
    assert!(({ (0_f64 == ({ (*z.borrow()).real_5() })) }));
    assert!(({ (0_f64 == ({ (*z.borrow()).imag_6() })) }));
    assert!(({ ({ (elem!(a, 0).read()) } == { ({ (*z.borrow()).real_5() }) }) }));
    assert!(({ ({ (elem!(a, 1).read()) } == { ({ (*z.borrow()).imag_6() }) }) }));
    elem!(a, 0).write(5_f64);
    elem!(a, 1).write(6_f64);
    assert!(({ ({ (elem!(a, 0).read()) } == { ({ (*z.borrow()).real_5() }) }) }));
    assert!(({ ({ (elem!(a, 1).read()) } == { ({ (*z.borrow()).imag_6() }) }) }));
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
