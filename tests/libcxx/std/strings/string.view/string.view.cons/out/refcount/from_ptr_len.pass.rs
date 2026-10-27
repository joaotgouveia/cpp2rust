extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut s: Ptr<()>, mut sz: ()) {
    {
        const _: () = assert!(true, "");;
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_1(mut s: Ptr<()>, mut sz: ()) {
    {
        const _: () = assert!(true, "");;
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_2(mut s: Ptr<()>, mut sz: ()) {
    {
        const _: () = assert!(true, "");;
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_3(mut s: Ptr<()>, mut sz: ()) {
    {
        const _: () = assert!(true, "");;
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_0(Ptr::<()>::from_string_literal(b"QBCDE"), 5_()) });
    ({ test_0(Ptr::<()>::from_string_literal(b"QBCDE"), 2_()) });
    ({ test_0(Ptr::<()>::from_string_literal(b""), 0_()) });
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(b"QBCDE");
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 2");;
        const _: () = assert!(true, "sv1.data() == s");;
    }
    ({
        test_1(
            Ptr::<()>::from_string_literal(&[
                81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            5_(),
        )
    });
    ({
        test_1(
            Ptr::<()>::from_string_literal(&[
                81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            2_(),
        )
    });
    ({ test_1(Ptr::<()>::from_string_literal(&[0 as ()]), 0_()) });
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(&[
            81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]);
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 2");;
        const _: () = assert!(true, "sv1.data() == s");;
    }
    ({
        test_2(
            Ptr::<()>::from_string_literal(&[
                81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            5_(),
        )
    });
    ({
        test_2(
            Ptr::<()>::from_string_literal(&[
                81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            2_(),
        )
    });
    ({ test_2(Ptr::<()>::from_string_literal(&[0 as ()]), 0_()) });
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(&[
            81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]);
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 2");;
        const _: () = assert!(true, "sv1.data() == s");;
    }
    ({
        test_3(
            Ptr::<()>::from_string_literal(&[
                81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            5_(),
        )
    });
    ({
        test_3(
            Ptr::<()>::from_string_literal(&[
                81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            2_(),
        )
    });
    ({ test_3(Ptr::<()>::from_string_literal(&[0 as ()]), 0_()) });
    {
        let mut s: Ptr<()> = Ptr::<()>::from_string_literal(&[
            81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]);
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.size() == 2");;
        const _: () = assert!(true, "sv1.data() == s");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
