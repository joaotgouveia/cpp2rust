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
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([6, 7, 8, 9, 10])));
        let mut a3: [i32; 5] = [6, 14, 24, 36, 50];
        let mut N: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
            as u32);
        let v1: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a1.as_pointer() as Ptr<i32>) },
            { (N as usize) },
        )));
        let v2: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a2.as_pointer() as Ptr<i32>) },
            { (N as usize) },
        )));
        let v3: Value<std_valarray_int_> = Rc::new(RefCell::new({
            let __tmp_0 : Value<std__Expr_std___detail__BinClos_std___multiplies__std__ValArray__std__ValArray__int__int___int_ > = Rc::new(RefCell::new((( * (*v1.borrow()) .upgrade().deref() ) ).clone()));
            std_valarray_int_::new_2({ __tmp_0.as_pointer() })
        }));
        assert!(({ (({ (*v1.borrow()).size() }) == ({ (*v2.borrow()).size() })) }));
        assert!(({ (({ (*v1.borrow()).size() }) == ({ (*v3.borrow()).size() })) }));
        let mut i: usize = 0_usize;
        'loop_: while (i < ({ (*v1.borrow()).size() })) {
            assert!(({ ({ (*v3.borrow())[i] } == a3[(i) as usize]) }));
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
