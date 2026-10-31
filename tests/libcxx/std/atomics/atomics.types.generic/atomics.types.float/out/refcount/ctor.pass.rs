extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static a1_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static a1_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static a2_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static a2_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn testOne_4() {
    const _: () = assert!(true, "std::is_nothrow_constructible_v<std::atomic<T>>");;
    const _: () = assert!(true, "std::is_nothrow_constructible_v<std::atomic<T>, T>");;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            assert!((unimplemented!() == 0_f32));
        }
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            assert!((unimplemented!() == (5.2_f64 as f32)));
        }
    }
    {
        assert!((unimplemented!() == (0_f64 as f32)));
        assert!((unimplemented!() == (5.2_f64 as f32)));
    }
}
pub fn testOne_5() {
    const _: () = assert!(true, "std::is_nothrow_constructible_v<std::atomic<T>>");;
    const _: () = assert!(true, "std::is_nothrow_constructible_v<std::atomic<T>, T>");;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            assert!((unimplemented!() == 0_f64));
        }
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            assert!((unimplemented!() == 5.2_f64));
        }
    }
    {
        assert!((unimplemented!() == 0_f64));
        assert!((unimplemented!() == 5.2_f64));
    }
}
pub fn test_6() -> bool {
    ({ testOne_4() });
    ({ testOne_5() });
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
    ({ test_6() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = a1_0.with(|_| ());
    let _ = a1_1.with(|_| ());
    let _ = a2_2.with(|_| ());
    let _ = a2_3.with(|_| ());
}
