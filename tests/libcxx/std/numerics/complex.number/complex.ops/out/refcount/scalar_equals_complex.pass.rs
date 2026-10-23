extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_0() {
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "lhs == rhs");;
    }
}
pub fn test_constexpr_1() {
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "lhs == rhs");;
    }
}
pub fn test_constexpr_2() {
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!(lhs == rhs)");;
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "lhs == rhs");;
    }
}
pub fn test_3() -> () {
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    ({ test_constexpr_0() });
    return true;
}
pub fn test_4() -> () {
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    ({ test_constexpr_1() });
    return true;
}
pub fn test_5() -> () {
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(-2.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let lhs: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let rhs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    ({ test_constexpr_2() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_3() });
    ({ test_4() });
    ({ test_5() });
    const _: () = assert!(true, "test<float>()");;
    const _: () = assert!(true, "test<double>()");;
    const _: () = assert!(true, "test<long double>()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
