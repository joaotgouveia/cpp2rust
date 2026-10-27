extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
}
pub fn sign_0(mut x: ()) -> () {
    return if (x > 0_()) {
        1_()
    } else {
        (if (x < 0_()) { -1_() } else { 0_() })
    };
}
pub fn test1_1(sv1: (), mut s: Ptr<()>, mut expected: ()) {
    let sv1: Value<()> = Rc::new(RefCell::new(sv1));
    unimplemented!();
}
pub fn test1_2(sv1: (), mut s: Ptr<()>, mut expected: ()) {
    let sv1: Value<()> = Rc::new(RefCell::new(sv1));
    unimplemented!();
}
pub fn test1_3(sv1: (), mut s: Ptr<()>, mut expected: ()) {
    let sv1: Value<()> = Rc::new(RefCell::new(sv1));
    unimplemented!();
}
pub fn test1_4(sv1: (), mut s: Ptr<()>, mut expected: ()) {
    let sv1: Value<()> = Rc::new(RefCell::new(sv1));
    unimplemented!();
}
pub fn test_5(mut s1: Ptr<()>, mut s2: Ptr<()>, mut expected: ()) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_1(unimplemented!(), (s2).clone(), expected) });
}
pub fn test_6(mut s1: Ptr<()>, mut s2: Ptr<()>, mut expected: ()) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_2(unimplemented!(), (s2).clone(), expected) });
}
pub fn test_7(mut s1: Ptr<()>, mut s2: Ptr<()>, mut expected: ()) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_3(unimplemented!(), (s2).clone(), expected) });
}
pub fn test_8(mut s1: Ptr<()>, mut s2: Ptr<()>, mut expected: ()) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_4(unimplemented!(), (s2).clone(), expected) });
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
    {
        ({
            test_5(
                Ptr::<()>::from_string_literal(b""),
                Ptr::<()>::from_string_literal(b""),
                0_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b""),
                Ptr::<()>::from_string_literal(b"abcde"),
                -5_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b""),
                Ptr::<()>::from_string_literal(b"abcdefghij"),
                -10_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b""),
                Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst"),
                -20_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcde"),
                Ptr::<()>::from_string_literal(b""),
                5_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcde"),
                Ptr::<()>::from_string_literal(b"abcde"),
                0_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcde"),
                Ptr::<()>::from_string_literal(b"abcdefghij"),
                -5_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcde"),
                Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst"),
                -15_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcdefghij"),
                Ptr::<()>::from_string_literal(b""),
                10_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcdefghij"),
                Ptr::<()>::from_string_literal(b"abcde"),
                5_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcdefghij"),
                Ptr::<()>::from_string_literal(b"abcdefghij"),
                0_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcdefghij"),
                Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst"),
                -10_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst"),
                Ptr::<()>::from_string_literal(b""),
                20_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst"),
                Ptr::<()>::from_string_literal(b"abcde"),
                15_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst"),
                Ptr::<()>::from_string_literal(b"abcdefghij"),
                10_(),
            )
        });
        ({
            test_5(
                Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst"),
                Ptr::<()>::from_string_literal(b"abcdefghijklmnopqrst"),
                0_(),
            )
        });
    }
    {
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                0_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                -5_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                -10_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -20_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                5_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                0_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                -5_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -15_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                10_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                5_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                0_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -10_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                20_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                15_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                10_(),
            )
        });
        ({
            test_6(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                0_(),
            )
        });
    }
    {
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                0_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                -5_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                -10_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -20_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                5_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                0_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                -5_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -15_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                10_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                5_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                0_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -10_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                20_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                15_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                10_(),
            )
        });
        ({
            test_7(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                0_(),
            )
        });
    }
    {
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                0_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                -5_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                -10_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[0 as ()]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -20_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                5_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                0_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                -5_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -15_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                10_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                5_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                0_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                -10_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[0 as ()]),
                20_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 0 as (),
                ]),
                15_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 0 as (),
                ]),
                10_(),
            )
        });
        ({
            test_8(
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                Ptr::<()>::from_string_literal(&[
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (), 110 as (),
                    111 as (), 112 as (), 113 as (), 114 as (), 115 as (), 116 as (), 0 as (),
                ]),
                0_(),
            )
        });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.compare(\"\") == 0");;
        const _: () = assert!(true, "sv1.compare(\"abcde\") < 0");;
        const _: () = assert!(true, "sv2.compare(\"\") > 0");;
        const _: () = assert!(true, "sv2.compare(\"abcde\") == 0");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
