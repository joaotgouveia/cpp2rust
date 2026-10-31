extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(a: Ptr<i32>, b: Ptr<i32>, x: Ptr<i32>) {
    assert!(
        ({
            ({
                let __a = (a).clone();
                let __b = (b).clone();
                if CppLess::less(&__a, &__b) { __b } else { __a }
            })
        } == { (x).clone() })
    );
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
        let x: Value<i32> = Rc::new(RefCell::new(0));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        ({
            let _a: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            test_0(_a, y.as_pointer(), _x)
        });
        ({
            let _a: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            test_0(_a, x.as_pointer(), _x)
        });
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(0));
        let y: Value<i32> = Rc::new(RefCell::new(1));
        ({
            let _b: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            test_0(x.as_pointer(), _b, _x)
        });
        ({
            let _a: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            test_0(_a, x.as_pointer(), _x)
        });
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(1));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        ({
            let _a: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            test_0(_a, y.as_pointer(), _x)
        });
        ({
            let _b: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            test_0(y.as_pointer(), _b, _x)
        });
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(1));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        const _: () = assert!(true, "std::max(x, y) == x");;
        const _: () = assert!(true, "std::max(y, x) == x");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
