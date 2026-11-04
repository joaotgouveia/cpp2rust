extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let c: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f32) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f32) }));
    let c2: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ ((({ (*c.borrow()).real_5() }) as f64) == 1.5_f64) }));
    assert!(({ ((({ (*c.borrow()).imag_6() }) as f64) == 2.5_f64) }));
    let c3: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ ((({ (*c.borrow()).real_5() }) as f64) == 3.5_f64) }));
    assert!(({ ((({ (*c.borrow()).imag_6() }) as f64) == -4.5_f64) }));
    return true;
}
pub fn test_1() -> bool {
    let c: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f32) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f32) }));
    let c2: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ ((({ (*c.borrow()).real_5() }) as f64) == 1.5_f64) }));
    assert!(({ ((({ (*c.borrow()).imag_6() }) as f64) == 2.5_f64) }));
    let c3: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ ((({ (*c.borrow()).real_5() }) as f64) == 3.5_f64) }));
    assert!(({ ((({ (*c.borrow()).imag_6() }) as f64) == -4.5_f64) }));
    return true;
}
pub fn test_2() -> bool {
    let c: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f32) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f32) }));
    let c2: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ ((({ (*c.borrow()).real_5() }) as f64) == 1.5_f64) }));
    assert!(({ ((({ (*c.borrow()).imag_6() }) as f64) == 2.5_f64) }));
    let c3: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ ((({ (*c.borrow()).real_5() }) as f64) == 3.5_f64) }));
    assert!(({ ((({ (*c.borrow()).imag_6() }) as f64) == -4.5_f64) }));
    return true;
}
pub fn test_3() -> bool {
    let c: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f64) }));
    let c2: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 1.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 2.5_f64) }));
    let c3: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 3.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == -4.5_f64) }));
    return true;
}
pub fn test_4() -> bool {
    let c: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f64) }));
    let c2: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 1.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 2.5_f64) }));
    let c3: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 3.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == -4.5_f64) }));
    return true;
}
pub fn test_5() -> bool {
    let c: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f64) }));
    let c2: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 1.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 2.5_f64) }));
    let c3: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 3.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == -4.5_f64) }));
    return true;
}
pub fn test_6() -> bool {
    let c: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f64) }));
    let c2: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 1.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 2.5_f64) }));
    let c3: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 3.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == -4.5_f64) }));
    return true;
}
pub fn test_7() -> bool {
    let c: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f64) }));
    let c2: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 1.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 2.5_f64) }));
    let c3: Value<std_complex_double_> = Rc::new(RefCell::new(<std_complex_double_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 3.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == -4.5_f64) }));
    return true;
}
pub fn test_8() -> bool {
    let c: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    assert!(({ (({ (*c.borrow()).real_5() }) == 0_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 0_f64) }));
    let c2: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    (*c.borrow_mut()) = (*c2.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 1.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == 2.5_f64) }));
    let c3: Value<std_complex_long_double_> =
        Rc::new(RefCell::new(<std_complex_long_double_>::default()));
    (*c.borrow_mut()) = (*c3.borrow()).clone();
    assert!(({ (({ (*c.borrow()).real_5() }) == 3.5_f64) }));
    assert!(({ (({ (*c.borrow()).imag_6() }) == -4.5_f64) }));
    return true;
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
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_8() });
    const _: () = assert!(true, "test<float, float>()");;
    const _: () = assert!(true, "test<float, double>()");;
    const _: () = assert!(true, "test<float, long double>()");;
    const _: () = assert!(true, "test<double, float>()");;
    const _: () = assert!(true, "test<double, double>()");;
    const _: () = assert!(true, "test<double, long double>()");;
    const _: () = assert!(true, "test<long double, float>()");;
    const _: () = assert!(true, "test<long double, double>()");;
    const _: () = assert!(true, "test<long double, long double>()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
