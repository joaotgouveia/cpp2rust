extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut fmt: Ptr<i8>) {
    {
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
    {
        let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
}
pub fn test_1(mut fmt: Ptr<i32>) {
    {
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
    {
        let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
}
pub fn test_2(mut fmt: Ptr<u8>) {
    {
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
    {
        let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
}
pub fn test_3(mut fmt: Ptr<u16>) {
    {
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
    {
        let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
}
pub fn test_4(mut fmt: Ptr<u32>) {
    {
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
    {
        let view: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let context: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((1) as isize) }));
        unimplemented!();
        assert!(({ unimplemented!() } == { fmt.offset((2) as isize) }));
        unimplemented!();
        assert!((unimplemented!() == unimplemented!()));
    }
}
pub fn test_5() -> bool {
    ({ test_0(Ptr::<i8>::from_string_literal(b"abc")) });
    ({
        test_1(Ptr::<i32>::from_string_literal(&[
            97 as i32, 98 as i32, 99 as i32, 0 as i32,
        ]))
    });
    ({
        test_2(Ptr::<u8>::from_string_literal(&[
            97 as u8, 98 as u8, 99 as u8, 0 as u8,
        ]))
    });
    ({
        test_3(Ptr::<u16>::from_string_literal(&[
            97 as u16, 98 as u16, 99 as u16, 0 as u16,
        ]))
    });
    ({
        test_4(Ptr::<u32>::from_string_literal(&[
            97 as u32, 98 as u32, 99 as u32, 0 as u32,
        ]))
    });
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
    ({ test_5() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
