extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        let mut N: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let v: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a.as_pointer() as Ptr<i32>) },
            { (N as usize) },
        )));
        let v2: Value<std_valarray_int_> = Rc::new(RefCell::new((*v.borrow()).clone()));
        assert!(({ (({ (*v2.borrow()).size() }) == ({ (*v.borrow()).size() })) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v2.borrow()).size() })) {
            assert!(({ ({ (*v2.borrow())[i] } == { (*v.borrow())[i] }) }));
            i.prefix_inc();
        }
    }
    {
        let a: Value<Box<[f64]>> = Rc::new(RefCell::new(Box::new([
            1_f64, 2.5_f64, 3_f64, 4.25_f64, 5_f64,
        ])));
        let mut N: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize)))
            as u32);
        let v: Value<std_valarray_double_> = Rc::new(RefCell::new(std_valarray_double_::new_1(
            { (a.as_pointer() as Ptr<f64>) },
            { (N as usize) },
        )));
        let v2: Value<std_valarray_double_> = Rc::new(RefCell::new((*v.borrow()).clone()));
        assert!(({ (({ (*v2.borrow()).size() }) == ({ (*v.borrow()).size() })) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v2.borrow()).size() })) {
            assert!(({ ({ (*v2.borrow())[i] } == { (*v.borrow())[i] }) }));
            i.prefix_inc();
        }
    }
    {
        let a: Value<Box<[std_valarray_double_]>> = Rc::new(RefCell::new(Box::new([
            std_valarray_double_::new_2({ 1_usize }),
            std_valarray_double_::new_2({ 2_usize }),
            std_valarray_double_::new_2({ 3_usize }),
            std_valarray_double_::new_2({ 4_usize }),
            std_valarray_double_::new_2({ 5_usize }),
        ])));
        let mut N: u32 = (((80usize as usize).wrapping_div((16usize as usize))) as u32);
        let v: Value<std_valarray_std_valarray_double__> =
            Rc::new(RefCell::new(std_valarray_std_valarray_double__::new_1(
                { (a.as_pointer() as Ptr<std_valarray_double_>) },
                { (N as usize) },
            )));
        let v2: Value<std_valarray_std_valarray_double__> =
            Rc::new(RefCell::new((*v.borrow()).clone()));
        assert!(({ (({ (*v2.borrow()).size() }) == ({ (*v.borrow()).size() })) }));
        let mut i: u32 = 0_u32;
        'loop_: while (i < N) {
            assert!(
                ({
                    (({ (*v2.borrow())[(i as usize)].size() })
                        == ({ (*v.borrow())[(i as usize)].size() }))
                })
            );
            let mut j: usize = 0_usize;
            'loop_: while (j < ({ (*v.borrow())[(i as usize)].size() })) {
                assert!(
                    ((elem!(v2.as_pointer().offset((i as usize)), j).read())
                        == (elem!(v.as_pointer().offset((i as usize)), j).read()))
                );
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
