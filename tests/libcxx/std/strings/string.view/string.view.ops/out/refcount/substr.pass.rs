extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test1_0(sv: (), n: usize, mut pos: usize) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (pos > (unimplemented!() as usize)) {
        return;
    }
    unimplemented!();
    let mut rlen: usize = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*n.borrow()) as u64)));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new(
            (unimplemented!() as u64).wrapping_sub((pos as u64)),
        ));
        ({
            let __a = __tmp_0.as_pointer();
            let __b = __tmp_1.as_pointer();
            if CppLess::less(&__b, &__a) { __b } else { __a }
        }
        .read())
    } as usize);
    assert!(((unimplemented!() as usize) == rlen));
    let mut i: usize = 0_usize;
    'loop_: while (i < rlen) {
        assert!((((unimplemented!().read()) as i32) == ((unimplemented!().read()) as i32)));
        i.prefix_inc();
    }
}
pub fn test1_1(sv: (), n: usize, mut pos: usize) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (pos > (unimplemented!() as usize)) {
        return;
    }
    unimplemented!();
    let mut rlen: usize = ({
        let __tmp_2: Value<u64> = Rc::new(RefCell::new(((*n.borrow()) as u64)));
        let __tmp_3: Value<u64> = Rc::new(RefCell::new(
            (unimplemented!() as u64).wrapping_sub((pos as u64)),
        ));
        ({
            let __a = __tmp_2.as_pointer();
            let __b = __tmp_3.as_pointer();
            if CppLess::less(&__b, &__a) { __b } else { __a }
        }
        .read())
    } as usize);
    assert!(((unimplemented!() as usize) == rlen));
    let mut i: usize = 0_usize;
    'loop_: while (i < rlen) {
        assert!((((unimplemented!().read()) as i32) == ((unimplemented!().read()) as i32)));
        i.prefix_inc();
    }
}
pub fn test1_2(sv: (), n: usize, mut pos: usize) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (pos > (unimplemented!() as usize)) {
        return;
    }
    unimplemented!();
    let mut rlen: usize = ({
        let __tmp_4: Value<u64> = Rc::new(RefCell::new(((*n.borrow()) as u64)));
        let __tmp_5: Value<u64> = Rc::new(RefCell::new(
            (unimplemented!() as u64).wrapping_sub((pos as u64)),
        ));
        ({
            let __a = __tmp_4.as_pointer();
            let __b = __tmp_5.as_pointer();
            if CppLess::less(&__b, &__a) { __b } else { __a }
        }
        .read())
    } as usize);
    assert!(((unimplemented!() as usize) == rlen));
    let mut i: usize = 0_usize;
    'loop_: while (i < rlen) {
        assert!((((unimplemented!().read()) as i32) == ((unimplemented!().read()) as i32)));
        i.prefix_inc();
    }
}
pub fn test1_3(sv: (), n: usize, mut pos: usize) {
    let sv: Value<()> = Rc::new(RefCell::new(sv));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if (pos > (unimplemented!() as usize)) {
        return;
    }
    unimplemented!();
    let mut rlen: usize = ({
        let __tmp_6: Value<u64> = Rc::new(RefCell::new(((*n.borrow()) as u64)));
        let __tmp_7: Value<u64> = Rc::new(RefCell::new(
            (unimplemented!() as u64).wrapping_sub((pos as u64)),
        ));
        ({
            let __a = __tmp_6.as_pointer();
            let __b = __tmp_7.as_pointer();
            if CppLess::less(&__b, &__a) { __b } else { __a }
        }
        .read())
    } as usize);
    assert!(((unimplemented!() as usize) == rlen));
    let mut i: usize = 0_usize;
    'loop_: while (i < rlen) {
        assert!((((unimplemented!().read()) as u32) == ((unimplemented!().read()) as u32)));
        i.prefix_inc();
    }
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
    ({ test1_0(unimplemented!(), 0_usize, 3_usize) });
    ({ test1_0(unimplemented!(), 2_usize, 3_usize) });
    ({ test1_0(unimplemented!(), 100_usize, 3_usize) });
    ({ test1_0(unimplemented!(), 0_usize, (unimplemented!() as usize)) });
    ({ test1_0(unimplemented!(), 2_usize, (unimplemented!() as usize)) });
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
    ({ test1_1(unimplemented!(), 0_usize, 3_usize) });
    ({ test1_1(unimplemented!(), 2_usize, 3_usize) });
    ({ test1_1(unimplemented!(), 100_usize, 3_usize) });
    ({ test1_1(unimplemented!(), 0_usize, (unimplemented!() as usize)) });
    ({ test1_1(unimplemented!(), 2_usize, (unimplemented!() as usize)) });
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
    ({ test1_2(unimplemented!(), 0_usize, 3_usize) });
    ({ test1_2(unimplemented!(), 2_usize, 3_usize) });
    ({ test1_2(unimplemented!(), 100_usize, 3_usize) });
    ({ test1_2(unimplemented!(), 0_usize, (unimplemented!() as usize)) });
    ({ test1_2(unimplemented!(), 2_usize, (unimplemented!() as usize)) });
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
    ({ test1_3(unimplemented!(), 0_usize, 3_usize) });
    ({ test1_3(unimplemented!(), 2_usize, 3_usize) });
    ({ test1_3(unimplemented!(), 100_usize, 3_usize) });
    ({ test1_3(unimplemented!(), 0_usize, (unimplemented!() as usize)) });
    ({ test1_3(unimplemented!(), 2_usize, (unimplemented!() as usize)) });
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
