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
    pub fn lt(mut c1: i8, mut c2: i8) -> bool {
        return ((c1 as i32) < (c2 as i32));
    }
    pub fn compare(mut s1: Ptr<i8>, mut s2: Ptr<i8>, mut n: usize) -> i32 {
        'loop_: while (n != 0) {
            if ({
                let _c1: i8 = (s1.read());
                let _c2: i8 = (s2.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_i32;
            }
            if ({
                let _c1: i8 = (s2.read());
                let _c2: i8 = (s1.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1;
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0;
    }
}
pub fn sign_0(mut x: i32) -> i32 {
    return if (x > 0) {
        1
    } else {
        (if (x < 0) { -1_i32 } else { 0 })
    };
}
pub fn test1_1(sv1: (), sv2: (), mut expected: i32) {
    let sv1: Value<()> = Rc::new(RefCell::new(sv1));
    let sv2: Value<()> = Rc::new(RefCell::new(sv2));
    assert!((({ sign_0(unimplemented!(),) }) == ({ sign_0(expected,) })));
}
pub fn test1_2(sv1: (), sv2: (), mut expected: i32) {
    let sv1: Value<()> = Rc::new(RefCell::new(sv1));
    let sv2: Value<()> = Rc::new(RefCell::new(sv2));
    assert!((({ sign_0(unimplemented!(),) }) == ({ sign_0(expected,) })));
}
pub fn test1_3(sv1: (), sv2: (), mut expected: i32) {
    let sv1: Value<()> = Rc::new(RefCell::new(sv1));
    let sv2: Value<()> = Rc::new(RefCell::new(sv2));
    assert!((({ sign_0(unimplemented!(),) }) == ({ sign_0(expected,) })));
}
pub fn test1_4(sv1: (), sv2: (), mut expected: i32) {
    let sv1: Value<()> = Rc::new(RefCell::new(sv1));
    let sv2: Value<()> = Rc::new(RefCell::new(sv2));
    assert!((({ sign_0(unimplemented!(),) }) == ({ sign_0(expected,) })));
}
pub fn test_5(mut s1: Ptr<i8>, mut s2: Ptr<i8>, mut expected: i32) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_1(unimplemented!(), unimplemented!(), expected) });
}
pub fn test_6(mut s1: Ptr<i32>, mut s2: Ptr<i32>, mut expected: i32) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_2(unimplemented!(), unimplemented!(), expected) });
}
pub fn test_7(mut s1: Ptr<u16>, mut s2: Ptr<u16>, mut expected: i32) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_3(unimplemented!(), unimplemented!(), expected) });
}
pub fn test_8(mut s1: Ptr<u32>, mut s2: Ptr<u32>, mut expected: i32) {
    let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_4(unimplemented!(), unimplemented!(), expected) });
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
        test_5(
            Ptr::<i8>::from_string_literal(b""),
            Ptr::<i8>::from_string_literal(b""),
            0,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b""),
            Ptr::<i8>::from_string_literal(b"abcde"),
            -5_i32,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b""),
            Ptr::<i8>::from_string_literal(b"abcdefghij"),
            -10_i32,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b""),
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrst"),
            -20_i32,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcde"),
            Ptr::<i8>::from_string_literal(b""),
            5,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcde"),
            Ptr::<i8>::from_string_literal(b"abcde"),
            0,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcde"),
            Ptr::<i8>::from_string_literal(b"abcdefghij"),
            -5_i32,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcde"),
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrst"),
            -15_i32,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcdefghij"),
            Ptr::<i8>::from_string_literal(b""),
            10,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcdefghij"),
            Ptr::<i8>::from_string_literal(b"abcde"),
            5,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcdefghij"),
            Ptr::<i8>::from_string_literal(b"abcdefghij"),
            0,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcdefghij"),
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrst"),
            -10_i32,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrst"),
            Ptr::<i8>::from_string_literal(b""),
            20,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrst"),
            Ptr::<i8>::from_string_literal(b"abcde"),
            15,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrst"),
            Ptr::<i8>::from_string_literal(b"abcdefghij"),
            10,
        )
    });
    ({
        test_5(
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrst"),
            Ptr::<i8>::from_string_literal(b"abcdefghijklmnopqrst"),
            0,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[0 as i32]),
            Ptr::<i32>::from_string_literal(&[0 as i32]),
            0,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[0 as i32]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            -5_i32,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[0 as i32]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]),
            -10_i32,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[0 as i32]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]),
            -20_i32,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[0 as i32]),
            5,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            0,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]),
            -5_i32,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]),
            -15_i32,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[0 as i32]),
            10,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            5,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]),
            0,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]),
            -10_i32,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[0 as i32]),
            20,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 0 as i32,
            ]),
            15,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 0 as i32,
            ]),
            10,
        )
    });
    ({
        test_6(
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]),
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32,
                111 as i32, 112 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 0 as i32,
            ]),
            0,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[0 as u16]),
            Ptr::<u16>::from_string_literal(&[0 as u16]),
            0,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[0 as u16]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]),
            -5_i32,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[0 as u16]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]),
            -10_i32,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[0 as u16]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]),
            -20_i32,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[0 as u16]),
            5,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]),
            0,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]),
            -5_i32,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]),
            -15_i32,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[0 as u16]),
            10,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]),
            5,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]),
            0,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]),
            -10_i32,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[0 as u16]),
            20,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 0 as u16,
            ]),
            15,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 0 as u16,
            ]),
            10,
        )
    });
    ({
        test_7(
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]),
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16,
                111 as u16, 112 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 0 as u16,
            ]),
            0,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[0 as u32]),
            Ptr::<u32>::from_string_literal(&[0 as u32]),
            0,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[0 as u32]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]),
            -5_i32,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[0 as u32]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]),
            -10_i32,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[0 as u32]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]),
            -20_i32,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[0 as u32]),
            5,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]),
            0,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]),
            -5_i32,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]),
            -15_i32,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[0 as u32]),
            10,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]),
            5,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]),
            0,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]),
            -10_i32,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[0 as u32]),
            20,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 0 as u32,
            ]),
            15,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 0 as u32,
            ]),
            10,
        )
    });
    ({
        test_8(
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]),
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32,
                111 as u32, 112 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 0 as u32,
            ]),
            0,
        )
    });
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.compare(sv2) == 0");;
        const _: () = assert!(true, "sv2.compare(sv1) == 0");;
        const _: () = assert!(true, "sv3.compare(sv2) > 0");;
        const _: () = assert!(true, "sv2.compare(sv3) < 0");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
