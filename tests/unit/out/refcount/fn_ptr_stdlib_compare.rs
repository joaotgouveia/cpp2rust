extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_alternative_fread_0(mut p: Ptr<i8>, mut n: u64, mut m: u64, mut f: AnyPtr) -> u64 {
    return 22_u64;
}
pub fn my_alternative_fwrite_1(mut p: Ptr<i8>, mut n: u64, mut m: u64, mut f: AnyPtr) -> u64 {
    return 33_u64;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fn1: FnPtr<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64> =
        FnPtr::<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64>::new(fread_2);
    ({
        cpp2rust_assert_fail_3(
            ({ (fn1).clone() } == {
                FnPtr::<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64>::new(fread_2)
            }),
        )
    });
    ({ cpp2rust_assert_fail_3(!((fn1).is_null())) });
    let mut fn2: FnPtr<fn(Ptr<i8>, u64, u64, AnyPtr) -> u64> = FnPtr::<
        fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64,
    >::new(fread_2)
    .cast::<fn(Ptr<i8>, u64, u64, AnyPtr) -> u64>();
    ({
        cpp2rust_assert_fail_3(
            ({ (fn1).clone() } == { fn2.cast::<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64>() }),
        )
    });
    let mut f3: FnPtr<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64> = FnPtr::<
        fn(Ptr<i8>, u64, u64, AnyPtr) -> u64,
    >::new(
        my_alternative_fread_0
    )
    .cast::<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64>();
    ({
        cpp2rust_assert_fail_3(
            (({ (f3).call(AnyPtr::default(), 0_u64, 0_u64, Ptr::<_IO_FILE>::null()) }) == 22_u64),
        )
    });
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: Ptr<_IO_FILE> = ({
            fopen_4(
                Ptr::<i8>::from_string_literal(b"/dev/zero"),
                Ptr::<i8>::from_string_literal(b"rb"),
            )
        });
        ({ cpp2rust_assert_fail_3(!((stream).is_null())) });
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..16).map(|_| 0_i8).collect::<Box<[i8]>>()));
        ({
            let ___s: AnyPtr = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
            let ___n: u64 = (::std::mem::size_of::<[i8; 16]>() as u64);
            memset_5(___s, (('X' as i8) as i32), ___n)
        });
        let mut n: u64 = ({
            (fread_2)(
                ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                1_u64,
                10_u64,
                (stream).clone(),
            )
        });
        ({ cpp2rust_assert_fail_3((n == 10_u64)) });
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_3((((*buf.borrow())[(i) as usize] as i32) == 0)) });
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while (i < 16) {
            ({
                cpp2rust_assert_fail_3(
                    (((*buf.borrow())[(i) as usize] as i32) == (('X' as i8) as i32)),
                )
            });
            i.prefix_inc();
        }
        ({ fclose_6((stream).clone()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: Ptr<_IO_FILE> = ({
            fopen_4(
                Ptr::<i8>::from_string_literal(b"/dev/zero"),
                Ptr::<i8>::from_string_literal(b"rb"),
            )
        });
        ({ cpp2rust_assert_fail_3(!((stream).is_null())) });
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..16).map(|_| 0_i8).collect::<Box<[i8]>>()));
        ({
            let ___s: AnyPtr = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
            let ___n: u64 = (::std::mem::size_of::<[i8; 16]>() as u64);
            memset_5(___s, (('X' as i8) as i32), ___n)
        });
        let mut n: u64 = ({
            (fn1).call(
                ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                1_u64,
                10_u64,
                (stream).clone(),
            )
        });
        ({ cpp2rust_assert_fail_3((n == 10_u64)) });
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            ({ cpp2rust_assert_fail_3((((*buf.borrow())[(i) as usize] as i32) == 0)) });
            i.prefix_inc();
        }
        let mut i: i32 = 10;
        'loop_: while (i < 16) {
            ({
                cpp2rust_assert_fail_3(
                    (((*buf.borrow())[(i) as usize] as i32) == (('X' as i8) as i32)),
                )
            });
            i.prefix_inc();
        }
        ({ fclose_6((stream).clone()) });
    }
    let mut gn1: FnPtr<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64> =
        FnPtr::<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64>::new(fwrite_7);
    ({
        cpp2rust_assert_fail_3(
            ({ (gn1).clone() } == {
                FnPtr::<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64>::new(fwrite_7)
            }),
        )
    });
    ({ cpp2rust_assert_fail_3(!((gn1).is_null())) });
    let mut gn2: FnPtr<fn(Ptr<i8>, u64, u64, AnyPtr) -> u64> = FnPtr::<
        fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64,
    >::new(fwrite_7)
    .cast::<fn(Ptr<i8>, u64, u64, AnyPtr) -> u64>();
    ({
        cpp2rust_assert_fail_3(
            ({ (gn1).clone() } == { gn2.cast::<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64>() }),
        )
    });
    let mut g3: FnPtr<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64> = FnPtr::<
        fn(Ptr<i8>, u64, u64, AnyPtr) -> u64,
    >::new(
        my_alternative_fwrite_1
    )
    .cast::<fn(AnyPtr, u64, u64, Ptr<_IO_FILE>) -> u64>();
    ({
        cpp2rust_assert_fail_3(
            (({ (g3).call(AnyPtr::default(), 0_u64, 0_u64, Ptr::<_IO_FILE>::null()) }) == 33_u64),
        )
    });
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: Ptr<_IO_FILE> = ({
            fopen_4(
                Ptr::<i8>::from_string_literal(b"/dev/null"),
                Ptr::<i8>::from_string_literal(b"wb"),
            )
        });
        ({ cpp2rust_assert_fail_3(!((stream).is_null())) });
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_i8).collect::<Box<[i8]>>()));
        ({
            let ___s: AnyPtr = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
            let ___n: u64 = (::std::mem::size_of::<[i8; 10]>() as u64);
            memset_5(___s, (('Y' as i8) as i32), ___n)
        });
        let mut n: u64 = ({
            (fwrite_7)(
                ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                1_u64,
                10_u64,
                (stream).clone(),
            )
        });
        ({ cpp2rust_assert_fail_3((n == 10_u64)) });
        ({ fclose_6((stream).clone()) });
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0 != 0) {
        __do_while = false;
        let mut stream: Ptr<_IO_FILE> = ({
            fopen_4(
                Ptr::<i8>::from_string_literal(b"/dev/null"),
                Ptr::<i8>::from_string_literal(b"wb"),
            )
        });
        ({ cpp2rust_assert_fail_3(!((stream).is_null())) });
        let buf: Value<Box<[i8]>> =
            Rc::new(RefCell::new((0..10).map(|_| 0_i8).collect::<Box<[i8]>>()));
        ({
            let ___s: AnyPtr = ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
            let ___n: u64 = (::std::mem::size_of::<[i8; 10]>() as u64);
            memset_5(___s, (('Y' as i8) as i32), ___n)
        });
        let mut n: u64 = ({
            (gn1).call(
                ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                1_u64,
                10_u64,
                (stream).clone(),
            )
        });
        ({ cpp2rust_assert_fail_3((n == 10_u64)) });
        ({ fclose_6((stream).clone()) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
