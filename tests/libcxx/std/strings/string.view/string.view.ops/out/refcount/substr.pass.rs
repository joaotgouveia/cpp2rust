extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0(sv: (), mut n: (), mut pos: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (pos > unimplemented!()) {
        return;
    }
    unimplemented!();
    let mut rlen: () = (unimplemented!().read());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < rlen) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test1_1(sv: (), mut n: (), mut pos: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (pos > unimplemented!()) {
        return;
    }
    unimplemented!();
    let mut rlen: () = (unimplemented!().read());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < rlen) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test1_2(sv: (), mut n: (), mut pos: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (pos > unimplemented!()) {
        return;
    }
    unimplemented!();
    let mut rlen: () = (unimplemented!().read());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < rlen) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test1_3(sv: (), mut n: (), mut pos: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (pos > unimplemented!()) {
        return;
    }
    unimplemented!();
    let mut rlen: () = (unimplemented!().read());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < rlen) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_4(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_0(unimplemented!(), 0_(), 0_()) });
    ({ test1_0(unimplemented!(), 1_(), 0_()) });
    ({ test1_0(unimplemented!(), 20_(), 0_()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_0(_sv, _n, 0_())
    });
    ({ test1_0(unimplemented!(), 0_(), 3_()) });
    ({ test1_0(unimplemented!(), 2_(), 3_()) });
    ({ test1_0(unimplemented!(), 100_(), 3_()) });
    ({ test1_0(unimplemented!(), 0_(), unimplemented!()) });
    ({ test1_0(unimplemented!(), 2_(), unimplemented!()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_0(_sv, _n, unimplemented!())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_0(_sv, _n, 0_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_0(_sv, _n, 1_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_0(_sv, _n, unimplemented!())
    });
}
pub fn test_5(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_1(unimplemented!(), 0_(), 0_()) });
    ({ test1_1(unimplemented!(), 1_(), 0_()) });
    ({ test1_1(unimplemented!(), 20_(), 0_()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_1(_sv, _n, 0_())
    });
    ({ test1_1(unimplemented!(), 0_(), 3_()) });
    ({ test1_1(unimplemented!(), 2_(), 3_()) });
    ({ test1_1(unimplemented!(), 100_(), 3_()) });
    ({ test1_1(unimplemented!(), 0_(), unimplemented!()) });
    ({ test1_1(unimplemented!(), 2_(), unimplemented!()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_1(_sv, _n, unimplemented!())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_1(_sv, _n, 0_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_1(_sv, _n, 1_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_1(_sv, _n, unimplemented!())
    });
}
pub fn test_6(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_2(unimplemented!(), 0_(), 0_()) });
    ({ test1_2(unimplemented!(), 1_(), 0_()) });
    ({ test1_2(unimplemented!(), 20_(), 0_()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_2(_sv, _n, 0_())
    });
    ({ test1_2(unimplemented!(), 0_(), 3_()) });
    ({ test1_2(unimplemented!(), 2_(), 3_()) });
    ({ test1_2(unimplemented!(), 100_(), 3_()) });
    ({ test1_2(unimplemented!(), 0_(), unimplemented!()) });
    ({ test1_2(unimplemented!(), 2_(), unimplemented!()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_2(_sv, _n, unimplemented!())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_2(_sv, _n, 0_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_2(_sv, _n, 1_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_2(_sv, _n, unimplemented!())
    });
}
pub fn test_7(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_3(unimplemented!(), 0_(), 0_()) });
    ({ test1_3(unimplemented!(), 1_(), 0_()) });
    ({ test1_3(unimplemented!(), 20_(), 0_()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_3(_sv, _n, 0_())
    });
    ({ test1_3(unimplemented!(), 0_(), 3_()) });
    ({ test1_3(unimplemented!(), 2_(), 3_()) });
    ({ test1_3(unimplemented!(), 100_(), 3_()) });
    ({ test1_3(unimplemented!(), 0_(), unimplemented!()) });
    ({ test1_3(unimplemented!(), 2_(), unimplemented!()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_3(_sv, _n, unimplemented!())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_3(_sv, _n, 0_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_3(_sv, _n, 1_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_3(_sv, _n, unimplemented!())
    });
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
    ({
        test_4 ( Ptr::<() >::from_string_literal(b"ABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDE" )  , )
    });
    ({ test_4(Ptr::<()>::from_string_literal(b"ABCDE")) });
    ({ test_4(Ptr::<()>::from_string_literal(b"a")) });
    ({ test_4(Ptr::<()>::from_string_literal(b"")) });
    ({
        test_5(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
            67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
            67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 0 as (),
        ]))
    });
    ({
        test_5(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_5(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()])) });
    ({ test_5(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({
        test_6(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
            67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
            67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 0 as (),
        ]))
    });
    ({
        test_6(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_6(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()])) });
    ({ test_6(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({
        test_7(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
            67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
            67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
            68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
            66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
            69 as (), 0 as (),
        ]))
    });
    ({
        test_7(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_7(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()])) });
    ({ test_7(Ptr::<()>::from_string_literal(&[0 as ()])) });
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(true, "sv2.size() == 3");;
            const _: () = assert!(true, "sv2[0] == \'A\'");;
            const _: () = assert!(true, "sv2[1] == \'B\'");;
            const _: () = assert!(true, "sv2[2] == \'C\'");;
        }
        {
            let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(true, "sv2.size() == 0");;
        }
        {
            let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(true, "sv2.size() == 2");;
            const _: () = assert!(true, "sv2[0] == \'D\'");;
            const _: () = assert!(true, "sv2[1] == \'E\'");;
        }
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
