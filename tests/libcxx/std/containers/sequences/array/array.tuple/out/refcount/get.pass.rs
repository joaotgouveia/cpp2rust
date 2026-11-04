extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tempArray_0(mut args_0: i32, mut args_1: i32, mut args_2: i32) -> Vec<i32> {
    return Vec::from([args_0, args_1, args_2]);
}
pub fn tests_1() -> bool {
    {
        let array: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([3.3_f64])));
        assert!(((array.decay().offset(0 as isize).read()) == 3.3_f64));
        array.decay().offset(0 as isize).write(99.1_f64);
        assert!(((array.decay().offset(0 as isize).read()) == 99.1_f64));
    }
    {
        let array: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([3.3_f64, 4.4_f64])));
        assert!(((array.decay().offset(0 as isize).read()) == 3.3_f64));
        assert!(((array.decay().offset(1 as isize).read()) == 4.4_f64));
        array.decay().offset(0 as isize).write(99.1_f64);
        array.decay().offset(1 as isize).write(99.2_f64);
        assert!(((array.decay().offset(0 as isize).read()) == 99.1_f64));
        assert!(((array.decay().offset(1 as isize).read()) == 99.2_f64));
    }
    {
        let array: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([3.3_f64, 4.4_f64, 5.5_f64])));
        assert!(((array.decay().offset(0 as isize).read()) == 3.3_f64));
        assert!(((array.decay().offset(1 as isize).read()) == 4.4_f64));
        assert!(((array.decay().offset(2 as isize).read()) == 5.5_f64));
        array.decay().offset(1 as isize).write(99.2_f64);
        assert!(((array.decay().offset(0 as isize).read()) == 3.3_f64));
        assert!(((array.decay().offset(1 as isize).read()) == 99.2_f64));
        assert!(((array.decay().offset(2 as isize).read()) == 5.5_f64));
    }
    {
        let array: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([3.3_f64])));
        const _: () = assert!(
            true,
            "std::is_same<double&, decltype(std::get<0>(array))>::value"
        );;
    }
    {
        assert!(
            ((match (Rc::new(RefCell::new(({ tempArray_0(1, 2, 3,) }))).as_pointer(),) {
                (__tmp_0,) => {
                    __tmp_0.decay().offset(0 as isize)
                }
            }
            .read())
                == 1)
        );
        assert!(
            ((match (Rc::new(RefCell::new(({ tempArray_0(1, 2, 3,) }))).as_pointer(),) {
                (__tmp_1,) => {
                    __tmp_1.decay().offset(1 as isize)
                }
            }
            .read())
                == 2)
        );
        assert!(
            ((match (Rc::new(RefCell::new(({ tempArray_0(1, 2, 3,) }))).as_pointer(),) {
                (__tmp_2,) => {
                    __tmp_2.decay().offset(2 as isize)
                }
            }
            .read())
                == 3)
        );
    }
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
    ({ tests_1() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
