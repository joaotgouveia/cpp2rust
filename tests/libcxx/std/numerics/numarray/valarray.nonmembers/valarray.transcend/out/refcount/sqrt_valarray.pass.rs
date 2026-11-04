extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_about_0(mut x: f64, mut y: f64, mut significant_digits: i32) -> bool {
    let mut epsilon: f64 = ({ pow_1((1_f64 / 10_f64), significant_digits) });
    return (({ libcc2rs::abs_refcount((x - y)) }) < epsilon);
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
        let a1: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
            0.5_f64, 0.75_f64, 1_f64, 3_f64, 7_f64,
        ])));
        let mut a3: [f64; 5] = [
            0.7071067811865476_f64,
            0.8660254037844386_f64,
            1_f64,
            1.7320508075688772_f64,
            2.6457513110645907_f64,
        ];
        let mut N: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize)))
            as u32);
        let v1: Value<std_valarray_double_> = Rc::new(RefCell::new(std_valarray_double_::new_1(
            { (a1.as_pointer() as Ptr<f64>) },
            { (N as usize) },
        )));
        let v3: Value<std_valarray_double_> = Rc::new(RefCell::new({
            let __tmp_0: Value<
                std__Expr_std___detail__UnClos_std__Sqrt__std__ValArray__double___double_,
            > = Rc::new(RefCell::new(({ sqrt_2(v1.as_pointer()) })));
            std_valarray_double_::new_2({ __tmp_0.as_pointer() })
        }));
        assert!(({ (({ (*v3.borrow()).size() }) == ({ (*v1.borrow()).size() })) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v3.borrow()).size() })) {
            assert!(
                ({
                    ({
                        let _x: f64 = { (*v3.borrow())[i] };
                        let _y: f64 = a3[(i) as usize];
                        is_about_0(_x, _y, 10)
                    })
                })
            );
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
