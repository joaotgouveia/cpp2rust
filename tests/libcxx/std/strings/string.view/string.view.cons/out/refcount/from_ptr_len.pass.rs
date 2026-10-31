extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut s: Ptr<i8>, mut sz: usize) {
    {
        const _: () = assert!(true, "");;
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as usize) == sz));
        assert!(({ unimplemented!() } == { (s).clone() }));
    }
}
pub fn test_1(mut s: Ptr<i32>, mut sz: usize) {
    {
        const _: () = assert!(true, "");;
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as usize) == sz));
        assert!(({ unimplemented!() } == { (s).clone() }));
    }
}
pub fn test_2(mut s: Ptr<u16>, mut sz: usize) {
    {
        const _: () = assert!(true, "");;
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as usize) == sz));
        assert!(({ unimplemented!() } == { (s).clone() }));
    }
}
pub fn test_3(mut s: Ptr<u32>, mut sz: usize) {
    {
        const _: () = assert!(true, "");;
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as usize) == sz));
        assert!(({ unimplemented!() } == { (s).clone() }));
    }
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
    ({ test_0(Ptr::<i8>::from_string_literal(b"QBCDE"), 5_usize) });
    ({ test_0(Ptr::<i8>::from_string_literal(b"QBCDE"), 2_usize) });
    ({ test_0(Ptr::<i8>::from_string_literal(b""), 0_usize) });
    {
        let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"QBCDE");
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 2");;
        const _: () = assert!(true, "sv1.data() == s");;
    }
    ({
        test_1(
            Ptr::<i32>::from_string_literal(&[
                81 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]),
            5_usize,
        )
    });
    ({
        test_1(
            Ptr::<i32>::from_string_literal(&[
                81 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]),
            2_usize,
        )
    });
    ({ test_1(Ptr::<i32>::from_string_literal(&[0 as i32]), 0_usize) });
    {
        let mut s: Ptr<i32> = Ptr::<i32>::from_string_literal(&[
            81 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
        ]);
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 2");;
        const _: () = assert!(true, "sv1.data() == s");;
    }
    ({
        test_2(
            Ptr::<u16>::from_string_literal(&[
                81 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]),
            5_usize,
        )
    });
    ({
        test_2(
            Ptr::<u16>::from_string_literal(&[
                81 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]),
            2_usize,
        )
    });
    ({ test_2(Ptr::<u16>::from_string_literal(&[0 as u16]), 0_usize) });
    {
        let mut s: Ptr<u16> = Ptr::<u16>::from_string_literal(&[
            81 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
        ]);
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 2");;
        const _: () = assert!(true, "sv1.data() == s");;
    }
    ({
        test_3(
            Ptr::<u32>::from_string_literal(&[
                81 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]),
            5_usize,
        )
    });
    ({
        test_3(
            Ptr::<u32>::from_string_literal(&[
                81 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]),
            2_usize,
        )
    });
    ({ test_3(Ptr::<u32>::from_string_literal(&[0 as u32]), 0_usize) });
    {
        let mut s: Ptr<u32> = Ptr::<u32>::from_string_literal(&[
            81 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
        ]);
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 2");;
        const _: () = assert!(true, "sv1.data() == s");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
