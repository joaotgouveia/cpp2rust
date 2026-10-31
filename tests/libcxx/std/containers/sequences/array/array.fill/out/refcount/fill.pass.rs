extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> bool {
    {
        let c: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([1_f64, 2_f64, 3.5_f64])));
        unimplemented!();
        assert!(((*c.borrow()).len() == 3_usize));
        assert!(((c.decay().offset(0_usize as isize).read()) == 5.5_f64));
        assert!(((c.decay().offset(1_usize as isize).read()) == 5.5_f64));
        assert!(((c.decay().offset(2_usize as isize).read()) == 5.5_f64));
    }
    {
        let c: Value<Vec<f64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_f64).to_vec(),
        ));
        unimplemented!();
        assert!(((*c.borrow()).len() == 0_usize));
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
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
