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
    let mut rlen: () = (unimplemented!().read());
    let mut dest1: Ptr<()> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    elem!(dest1, rlen).write(0_());
    let mut dest2: Ptr<()> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    elem!(dest2, rlen).write(0_());
    if (pos > unimplemented!()) {
    } else {
        unimplemented!();
        ({
            copy_n_1(
                unimplemented!().offset((pos) as isize),
                rlen,
                (dest2).clone(),
            )
        });
        let mut i: () = 0_();
        'loop_: while (i <= rlen) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    dest1.delete();
    dest2.delete();
}
pub fn test1_2(sv: (), mut n: (), mut pos: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let mut rlen: () = (unimplemented!().read());
    let mut dest1: Ptr<()> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    elem!(dest1, rlen).write(0_());
    let mut dest2: Ptr<()> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    elem!(dest2, rlen).write(0_());
    if (pos > unimplemented!()) {
    } else {
        unimplemented!();
        ({
            copy_n_3(
                unimplemented!().offset((pos) as isize),
                rlen,
                (dest2).clone(),
            )
        });
        let mut i: () = 0_();
        'loop_: while (i <= rlen) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    dest1.delete();
    dest2.delete();
}
pub fn test1_4(sv: (), mut n: (), mut pos: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let mut rlen: () = (unimplemented!().read());
    let mut dest1: Ptr<()> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    elem!(dest1, rlen).write(0_());
    let mut dest2: Ptr<()> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    elem!(dest2, rlen).write(0_());
    if (pos > unimplemented!()) {
    } else {
        unimplemented!();
        ({
            copy_n_5(
                unimplemented!().offset((pos) as isize),
                rlen,
                (dest2).clone(),
            )
        });
        let mut i: () = 0_();
        'loop_: while (i <= rlen) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    dest1.delete();
    dest2.delete();
}
pub fn test1_6(sv: (), mut n: (), mut pos: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let mut rlen: () = (unimplemented!().read());
    let mut dest1: Ptr<()> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    elem!(dest1, rlen).write(0_());
    let mut dest2: Ptr<()> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    );
    elem!(dest2, rlen).write(0_());
    if (pos > unimplemented!()) {
    } else {
        unimplemented!();
        ({
            copy_n_7(
                unimplemented!().offset((pos) as isize),
                rlen,
                (dest2).clone(),
            )
        });
        let mut i: () = 0_();
        'loop_: while (i <= rlen) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    dest1.delete();
    dest2.delete();
}
pub fn test_8(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_0(unimplemented!(), 0_(), 0_()) });
    ({ test1_0(unimplemented!(), 1_(), 0_()) });
    ({ test1_0(unimplemented!(), 20_(), 0_()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_0(_sv, _n, 0_())
    });
    ({ test1_0(unimplemented!(), 20_(), npos_9.with(|rc| *rc.borrow())) });
    ({ test1_0(unimplemented!(), 0_(), 3_()) });
    ({ test1_0(unimplemented!(), 2_(), 3_()) });
    ({ test1_0(unimplemented!(), 100_(), 3_()) });
    ({ test1_0(unimplemented!(), 100_(), npos_9.with(|rc| *rc.borrow())) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_0(_sv, _n, npos_9.with(|rc| *rc.borrow()))
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
        test1_0(_sv, _n, npos_9.with(|rc| *rc.borrow()))
    });
}
pub fn test_10(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_2(unimplemented!(), 0_(), 0_()) });
    ({ test1_2(unimplemented!(), 1_(), 0_()) });
    ({ test1_2(unimplemented!(), 20_(), 0_()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_2(_sv, _n, 0_())
    });
    ({ test1_2(unimplemented!(), 20_(), npos_11.with(|rc| *rc.borrow())) });
    ({ test1_2(unimplemented!(), 0_(), 3_()) });
    ({ test1_2(unimplemented!(), 2_(), 3_()) });
    ({ test1_2(unimplemented!(), 100_(), 3_()) });
    ({ test1_2(unimplemented!(), 100_(), npos_11.with(|rc| *rc.borrow())) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_2(_sv, _n, npos_11.with(|rc| *rc.borrow()))
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
        test1_2(_sv, _n, npos_11.with(|rc| *rc.borrow()))
    });
}
pub fn test_12(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_4(unimplemented!(), 0_(), 0_()) });
    ({ test1_4(unimplemented!(), 1_(), 0_()) });
    ({ test1_4(unimplemented!(), 20_(), 0_()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_4(_sv, _n, 0_())
    });
    ({ test1_4(unimplemented!(), 20_(), npos_13.with(|rc| *rc.borrow())) });
    ({ test1_4(unimplemented!(), 0_(), 3_()) });
    ({ test1_4(unimplemented!(), 2_(), 3_()) });
    ({ test1_4(unimplemented!(), 100_(), 3_()) });
    ({ test1_4(unimplemented!(), 100_(), npos_13.with(|rc| *rc.borrow())) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_4(_sv, _n, npos_13.with(|rc| *rc.borrow()))
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_4(_sv, _n, 0_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_4(_sv, _n, 1_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_4(_sv, _n, npos_13.with(|rc| *rc.borrow()))
    });
}
pub fn test_14(mut s: Ptr<()>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_6(unimplemented!(), 0_(), 0_()) });
    ({ test1_6(unimplemented!(), 1_(), 0_()) });
    ({ test1_6(unimplemented!(), 20_(), 0_()) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_6(_sv, _n, 0_())
    });
    ({ test1_6(unimplemented!(), 20_(), npos_15.with(|rc| *rc.borrow())) });
    ({ test1_6(unimplemented!(), 0_(), 3_()) });
    ({ test1_6(unimplemented!(), 2_(), 3_()) });
    ({ test1_6(unimplemented!(), 100_(), 3_()) });
    ({ test1_6(unimplemented!(), 100_(), npos_15.with(|rc| *rc.borrow())) });
    ({
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_6(_sv, _n, npos_15.with(|rc| *rc.borrow()))
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_6(_sv, _n, 0_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_6(_sv, _n, 1_())
    });
    ({
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_6(_sv, _n, npos_15.with(|rc| *rc.borrow()))
    });
}
pub fn test_constexpr_copy_16(mut abcde: Ptr<()>, mut ghijk: Ptr<()>, mut bcdjk: Ptr<()>) -> () {
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_constexpr_copy_17(mut abcde: Ptr<()>, mut ghijk: Ptr<()>, mut bcdjk: Ptr<()>) -> () {
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_constexpr_copy_18(mut abcde: Ptr<()>, mut ghijk: Ptr<()>, mut bcdjk: Ptr<()>) -> () {
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_constexpr_copy_19(mut abcde: Ptr<()>, mut ghijk: Ptr<()>, mut bcdjk: Ptr<()>) -> () {
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_constexpr_copy_20(mut abcde: Ptr<()>, mut ghijk: Ptr<()>, mut bcdjk: Ptr<()>) -> () {
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({
        test_8 ( Ptr::<() >::from_string_literal(b"ABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDE" )  , )
    });
    ({ test_8(Ptr::<()>::from_string_literal(b"ABCDE")) });
    ({ test_8(Ptr::<()>::from_string_literal(b"a")) });
    ({ test_8(Ptr::<()>::from_string_literal(b"")) });
    ({
        test_10(Ptr::<()>::from_string_literal(&[
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
        test_10(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_10(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()])) });
    ({ test_10(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({
        test_12(Ptr::<()>::from_string_literal(&[
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
        test_12(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_12(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()])) });
    ({ test_12(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({
        test_14(Ptr::<()>::from_string_literal(&[
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
        test_14(Ptr::<()>::from_string_literal(&[
            65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
        ]))
    });
    ({ test_14(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()])) });
    ({ test_14(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({
        test_constexpr_copy_16(
            Ptr::<()>::from_string_literal(b"ABCDE"),
            Ptr::<()>::from_string_literal(b"GHIJK"),
            Ptr::<()>::from_string_literal(b"BCDJK"),
        )
    });
    ({
        test_constexpr_copy_17(
            Ptr::<()>::from_string_literal(&[
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            Ptr::<()>::from_string_literal(&[
                71 as (), 72 as (), 73 as (), 74 as (), 75 as (), 0 as (),
            ]),
            Ptr::<()>::from_string_literal(&[
                66 as (), 67 as (), 68 as (), 74 as (), 75 as (), 0 as (),
            ]),
        )
    });
    ({
        test_constexpr_copy_18(
            Ptr::<()>::from_string_literal(&[
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            Ptr::<()>::from_string_literal(&[
                71 as (), 72 as (), 73 as (), 74 as (), 75 as (), 0 as (),
            ]),
            Ptr::<()>::from_string_literal(&[
                66 as (), 67 as (), 68 as (), 74 as (), 75 as (), 0 as (),
            ]),
        )
    });
    ({
        test_constexpr_copy_19(
            Ptr::<()>::from_string_literal(&[
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            Ptr::<()>::from_string_literal(&[
                71 as (), 72 as (), 73 as (), 74 as (), 75 as (), 0 as (),
            ]),
            Ptr::<()>::from_string_literal(&[
                66 as (), 67 as (), 68 as (), 74 as (), 75 as (), 0 as (),
            ]),
        )
    });
    ({
        test_constexpr_copy_20(
            Ptr::<()>::from_string_literal(&[
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            Ptr::<()>::from_string_literal(&[
                71 as (), 72 as (), 73 as (), 74 as (), 75 as (), 0 as (),
            ]),
            Ptr::<()>::from_string_literal(&[
                66 as (), 67 as (), 68 as (), 74 as (), 75 as (), 0 as (),
            ]),
        )
    });
    const _: () = assert!(true, "test_constexpr_copy(\"ABCDE\", \"GHIJK\", \"BCDJK\")");;
    const _: () = assert!(
        true,
        "test_constexpr_copy(L\"ABCDE\", L\"GHIJK\", L\"BCDJK\")"
    );;
    const _: () = assert!(
        true,
        "test_constexpr_copy(u\"ABCDE\", u\"GHIJK\", u\"BCDJK\")"
    );;
    const _: () = assert!(
        true,
        "test_constexpr_copy(U\"ABCDE\", U\"GHIJK\", U\"BCDJK\")"
    );;
    const _: () = assert!(
        true,
        "test_constexpr_copy(u8\"ABCDE\", u8\"GHIJK\", u8\"BCDJK\")"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
