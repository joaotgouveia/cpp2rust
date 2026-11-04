extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0(sv: (), mut n: usize, mut pos: usize) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let mut rlen: usize = ({
        let __tmp_1: Value<u64> = Rc::new(RefCell::new((n as u64)));
        (match (Rc::new(RefCell::new(
            (unimplemented!() as u64).wrapping_sub((pos as u64)),
        ))
        .as_pointer(),)
        {
            (__tmp_0,) => {
                if CppLess::less(&(__tmp_0).clone(), &__tmp_1.as_pointer()) {
                    (__tmp_0).clone()
                } else {
                    __tmp_1.as_pointer()
                }
            }
        }
        .read())
    } as usize);
    let mut dest1: Ptr<i8> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_i8)
            .collect::<Box<[i8]>>(),
    );
    elem!(dest1, rlen).write(0_i8);
    let mut dest2: Ptr<i8> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_i8)
            .collect::<Box<[i8]>>(),
    );
    elem!(dest2, rlen).write(0_i8);
    if (pos > (unimplemented!() as usize)) {
    } else {
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while (i <= rlen) {
            assert!(
                ({ ((elem!(dest1, i).read()) as i32) } == { ((elem!(dest2, i).read()) as i32) })
            );
            i.prefix_inc();
        }
    }
    dest1.delete();
    dest2.delete();
}
pub fn test1_1(sv: (), mut n: usize, mut pos: usize) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let mut rlen: usize = ({
        let __tmp_3: Value<u64> = Rc::new(RefCell::new((n as u64)));
        (match (Rc::new(RefCell::new(
            (unimplemented!() as u64).wrapping_sub((pos as u64)),
        ))
        .as_pointer(),)
        {
            (__tmp_2,) => {
                if CppLess::less(&(__tmp_2).clone(), &__tmp_3.as_pointer()) {
                    (__tmp_2).clone()
                } else {
                    __tmp_3.as_pointer()
                }
            }
        }
        .read())
    } as usize);
    let mut dest1: Ptr<i32> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_i32)
            .collect::<Box<[i32]>>(),
    );
    elem!(dest1, rlen).write(0_i32);
    let mut dest2: Ptr<i32> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_i32)
            .collect::<Box<[i32]>>(),
    );
    elem!(dest2, rlen).write(0_i32);
    if (pos > (unimplemented!() as usize)) {
    } else {
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while (i <= rlen) {
            assert!(({ (elem!(dest1, i).read()) } == { (elem!(dest2, i).read()) }));
            i.prefix_inc();
        }
    }
    dest1.delete();
    dest2.delete();
}
pub fn test1_2(sv: (), mut n: usize, mut pos: usize) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let mut rlen: usize = ({
        let __tmp_5: Value<u64> = Rc::new(RefCell::new((n as u64)));
        (match (Rc::new(RefCell::new(
            (unimplemented!() as u64).wrapping_sub((pos as u64)),
        ))
        .as_pointer(),)
        {
            (__tmp_4,) => {
                if CppLess::less(&(__tmp_4).clone(), &__tmp_5.as_pointer()) {
                    (__tmp_4).clone()
                } else {
                    __tmp_5.as_pointer()
                }
            }
        }
        .read())
    } as usize);
    let mut dest1: Ptr<u16> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_u16)
            .collect::<Box<[u16]>>(),
    );
    elem!(dest1, rlen).write(0_u16);
    let mut dest2: Ptr<u16> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_u16)
            .collect::<Box<[u16]>>(),
    );
    elem!(dest2, rlen).write(0_u16);
    if (pos > (unimplemented!() as usize)) {
    } else {
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while (i <= rlen) {
            assert!(
                ({ ((elem!(dest1, i).read()) as i32) } == { ((elem!(dest2, i).read()) as i32) })
            );
            i.prefix_inc();
        }
    }
    dest1.delete();
    dest2.delete();
}
pub fn test1_3(sv: (), mut n: usize, mut pos: usize) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let mut rlen: usize = ({
        let __tmp_7: Value<u64> = Rc::new(RefCell::new((n as u64)));
        (match (Rc::new(RefCell::new(
            (unimplemented!() as u64).wrapping_sub((pos as u64)),
        ))
        .as_pointer(),)
        {
            (__tmp_6,) => {
                if CppLess::less(&(__tmp_6).clone(), &__tmp_7.as_pointer()) {
                    (__tmp_6).clone()
                } else {
                    __tmp_7.as_pointer()
                }
            }
        }
        .read())
    } as usize);
    let mut dest1: Ptr<u32> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_u32)
            .collect::<Box<[u32]>>(),
    );
    elem!(dest1, rlen).write(0_u32);
    let mut dest2: Ptr<u32> = Ptr::alloc_array(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_u32)
            .collect::<Box<[u32]>>(),
    );
    elem!(dest2, rlen).write(0_u32);
    if (pos > (unimplemented!() as usize)) {
    } else {
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while (i <= rlen) {
            assert!(({ (elem!(dest1, i).read()) } == { (elem!(dest2, i).read()) }));
            i.prefix_inc();
        }
    }
    dest1.delete();
    dest2.delete();
}
pub fn test_4(mut s: Ptr<i8>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_0(unimplemented!(), 0_usize, 0_usize) });
    ({ test1_0(unimplemented!(), 1_usize, 0_usize) });
    ({ test1_0(unimplemented!(), 20_usize, 0_usize) });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_0(_sv, _n, 0_usize)
    });
    ({ test1_0(unimplemented!(), 20_usize, (unimplemented!() as usize)) });
    ({ test1_0(unimplemented!(), 0_usize, 3_usize) });
    ({ test1_0(unimplemented!(), 2_usize, 3_usize) });
    ({ test1_0(unimplemented!(), 100_usize, 3_usize) });
    ({ test1_0(unimplemented!(), 100_usize, (unimplemented!() as usize)) });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_0(_sv, _n, (unimplemented!() as usize))
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_0(_sv, _n, 0_usize)
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_0(_sv, _n, 1_usize)
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_0(_sv, _n, (unimplemented!() as usize))
    });
}
pub fn test_5(mut s: Ptr<i32>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_1(unimplemented!(), 0_usize, 0_usize) });
    ({ test1_1(unimplemented!(), 1_usize, 0_usize) });
    ({ test1_1(unimplemented!(), 20_usize, 0_usize) });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_1(_sv, _n, 0_usize)
    });
    ({ test1_1(unimplemented!(), 20_usize, (unimplemented!() as usize)) });
    ({ test1_1(unimplemented!(), 0_usize, 3_usize) });
    ({ test1_1(unimplemented!(), 2_usize, 3_usize) });
    ({ test1_1(unimplemented!(), 100_usize, 3_usize) });
    ({ test1_1(unimplemented!(), 100_usize, (unimplemented!() as usize)) });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_1(_sv, _n, (unimplemented!() as usize))
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_1(_sv, _n, 0_usize)
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_1(_sv, _n, 1_usize)
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_1(_sv, _n, (unimplemented!() as usize))
    });
}
pub fn test_6(mut s: Ptr<u16>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_2(unimplemented!(), 0_usize, 0_usize) });
    ({ test1_2(unimplemented!(), 1_usize, 0_usize) });
    ({ test1_2(unimplemented!(), 20_usize, 0_usize) });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_2(_sv, _n, 0_usize)
    });
    ({ test1_2(unimplemented!(), 20_usize, (unimplemented!() as usize)) });
    ({ test1_2(unimplemented!(), 0_usize, 3_usize) });
    ({ test1_2(unimplemented!(), 2_usize, 3_usize) });
    ({ test1_2(unimplemented!(), 100_usize, 3_usize) });
    ({ test1_2(unimplemented!(), 100_usize, (unimplemented!() as usize)) });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_2(_sv, _n, (unimplemented!() as usize))
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_2(_sv, _n, 0_usize)
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_2(_sv, _n, 1_usize)
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_2(_sv, _n, (unimplemented!() as usize))
    });
}
pub fn test_7(mut s: Ptr<u32>) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_3(unimplemented!(), 0_usize, 0_usize) });
    ({ test1_3(unimplemented!(), 1_usize, 0_usize) });
    ({ test1_3(unimplemented!(), 20_usize, 0_usize) });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_3(_sv, _n, 0_usize)
    });
    ({ test1_3(unimplemented!(), 20_usize, (unimplemented!() as usize)) });
    ({ test1_3(unimplemented!(), 0_usize, 3_usize) });
    ({ test1_3(unimplemented!(), 2_usize, 3_usize) });
    ({ test1_3(unimplemented!(), 100_usize, 3_usize) });
    ({ test1_3(unimplemented!(), 100_usize, (unimplemented!() as usize)) });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_3(_sv, _n, (unimplemented!() as usize))
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_3(_sv, _n, 0_usize)
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_3(_sv, _n, 1_usize)
    });
    ({
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_3(_sv, _n, (unimplemented!() as usize))
    });
}
pub fn test_constexpr_copy_8(mut abcde: Ptr<i8>, mut ghijk: Ptr<i8>, mut bcdjk: Ptr<i8>) -> bool {
    let buf: Value<Box<[i8]>> =
        Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
    return true;
}
pub fn test_constexpr_copy_9(
    mut abcde: Ptr<i32>,
    mut ghijk: Ptr<i32>,
    mut bcdjk: Ptr<i32>,
) -> bool {
    let buf: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
    return true;
}
pub fn test_constexpr_copy_10(
    mut abcde: Ptr<u16>,
    mut ghijk: Ptr<u16>,
    mut bcdjk: Ptr<u16>,
) -> bool {
    let buf: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16,
    ])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
    return true;
}
pub fn test_constexpr_copy_11(
    mut abcde: Ptr<u32>,
    mut ghijk: Ptr<u32>,
    mut bcdjk: Ptr<u32>,
) -> bool {
    let buf: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32,
    ])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
    return true;
}
pub fn test_constexpr_copy_12(mut abcde: Ptr<u8>, mut ghijk: Ptr<u8>, mut bcdjk: Ptr<u8>) -> bool {
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8])));
    let lval: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
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
    ({
        test_4 ( Ptr::<i8 >::from_string_literal(b"ABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDE" )  , )
    });
    ({ test_4(Ptr::<i8>::from_string_literal(b"ABCDE")) });
    ({ test_4(Ptr::<i8>::from_string_literal(b"a")) });
    ({ test_4(Ptr::<i8>::from_string_literal(b"")) });
    ({
        test_5(Ptr::<i32>::from_string_literal(&[
            65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
            68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
            66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
            69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32,
            67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32,
            65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
            68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
            66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
            69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32,
            67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32,
            65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
            68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
            66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
            69 as i32, 0 as i32,
        ]))
    });
    ({
        test_5(Ptr::<i32>::from_string_literal(&[
            65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
        ]))
    });
    ({ test_5(Ptr::<i32>::from_string_literal(&[97 as i32, 0 as i32])) });
    ({ test_5(Ptr::<i32>::from_string_literal(&[0 as i32])) });
    ({
        test_6(Ptr::<u16>::from_string_literal(&[
            65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
            68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
            66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
            69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16,
            67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16,
            65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
            68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
            66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
            69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16,
            67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16,
            65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
            68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
            66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
            69 as u16, 0 as u16,
        ]))
    });
    ({
        test_6(Ptr::<u16>::from_string_literal(&[
            65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
        ]))
    });
    ({ test_6(Ptr::<u16>::from_string_literal(&[97 as u16, 0 as u16])) });
    ({ test_6(Ptr::<u16>::from_string_literal(&[0 as u16])) });
    ({
        test_7(Ptr::<u32>::from_string_literal(&[
            65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
            68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
            66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
            69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32,
            67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32,
            65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
            68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
            66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
            69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32,
            67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32,
            65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
            68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
            66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
            69 as u32, 0 as u32,
        ]))
    });
    ({
        test_7(Ptr::<u32>::from_string_literal(&[
            65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
        ]))
    });
    ({ test_7(Ptr::<u32>::from_string_literal(&[97 as u32, 0 as u32])) });
    ({ test_7(Ptr::<u32>::from_string_literal(&[0 as u32])) });
    ({
        test_constexpr_copy_8(
            Ptr::<i8>::from_string_literal(b"ABCDE"),
            Ptr::<i8>::from_string_literal(b"GHIJK"),
            Ptr::<i8>::from_string_literal(b"BCDJK"),
        )
    });
    ({
        test_constexpr_copy_9(
            Ptr::<i32>::from_string_literal(&[
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                71 as i32, 72 as i32, 73 as i32, 74 as i32, 75 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                66 as i32, 67 as i32, 68 as i32, 74 as i32, 75 as i32, 0 as i32,
            ]),
        )
    });
    ({
        test_constexpr_copy_10(
            Ptr::<u16>::from_string_literal(&[
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                71 as u16, 72 as u16, 73 as u16, 74 as u16, 75 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                66 as u16, 67 as u16, 68 as u16, 74 as u16, 75 as u16, 0 as u16,
            ]),
        )
    });
    ({
        test_constexpr_copy_11(
            Ptr::<u32>::from_string_literal(&[
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                71 as u32, 72 as u32, 73 as u32, 74 as u32, 75 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                66 as u32, 67 as u32, 68 as u32, 74 as u32, 75 as u32, 0 as u32,
            ]),
        )
    });
    ({
        test_constexpr_copy_12(
            Ptr::<u8>::from_string_literal(&[
                65 as u8, 66 as u8, 67 as u8, 68 as u8, 69 as u8, 0 as u8,
            ]),
            Ptr::<u8>::from_string_literal(&[
                71 as u8, 72 as u8, 73 as u8, 74 as u8, 75 as u8, 0 as u8,
            ]),
            Ptr::<u8>::from_string_literal(&[
                66 as u8, 67 as u8, 68 as u8, 74 as u8, 75 as u8, 0 as u8,
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
