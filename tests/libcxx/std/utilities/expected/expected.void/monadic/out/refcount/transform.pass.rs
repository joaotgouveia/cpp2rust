extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(
    !(false),
    "!has_transform<const std::expected<int, std::unique_ptr<int>>&, int()>"
);
const _: () = assert!(
    !(false),
    "!has_transform<const std::expected<int, std::unique_ptr<int>>&&, int()>"
);
pub fn test_val_types_0() {
    {
        let l: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    return 1;
                }
            })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let l: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    return 1;
                }
            })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let l: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    return 1;
                }
            })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let l: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    return 1;
                }
            })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
}
pub fn test_fail_1() {
    {
        let l: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    assert!(false);
                    return 0;
                }
            })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 5));
    }
    {
        let l: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    assert!(false);
                    return 0;
                }
            })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 5));
    }
    {
        let l: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    assert!(false);
                    return 0;
                }
            })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 5));
    }
    {
        let l: Value<FnPtr<fn() -> i32>> =
            Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
                {
                    assert!(false);
                    return 0;
                }
            })));
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let val: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 5));
    }
}
pub fn test_2() -> bool {
    ({ test_fail_1() });
    ({ test_val_types_0() });
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
