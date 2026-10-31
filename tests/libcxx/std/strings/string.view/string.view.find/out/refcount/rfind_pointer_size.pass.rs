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
    pub fn eq(mut c1: i8, mut c2: i8) -> bool {
        return ((c1 as i32) == (c2 as i32));
    }
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
    pub fn length(mut s: Ptr<i8>) -> usize {
        let len: Value<usize> = Rc::new(RefCell::new(0_usize));
        'loop_: while !({
            let _c1: i8 = (s.read());
            let _c2: i8 = 0_i8;
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
}
pub fn test_0(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(({ unimplemented!() } == { x }));
    if (x != unimplemented!()) {
        let mut n: () = (unimplemented!() as ());
        assert!((x <= pos) && ({ (x).wrapping_add(n) } <= { unimplemented!() }));
    }
}
pub fn test_1(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(({ unimplemented!() } == { x }));
    if (x != unimplemented!()) {
        let mut pos: () = unimplemented!();
        let mut n: () = (unimplemented!() as ());
        assert!((x <= pos) && ({ (x).wrapping_add(n) } <= { unimplemented!() }));
    }
}
pub fn test0_2() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            6_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            6_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            9_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            11_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            11_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            11_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            19_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            19_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            20_(),
            20_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            20_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            20_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            21_(),
            20_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            21_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            21_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            21_(),
            0_(),
        )
    });
}
pub fn test1_3() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            20_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcde")).reinterpret_cast::<()>(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcde")).reinterpret_cast::<()>(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abcdeabcdeabcdeabcde")).reinterpret_cast::<()>(),
            0_(),
        )
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
    {
        ({ test0_2() });
        ({ test1_3() });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.rfind(\"\") == 0");;
        const _: () = assert!(true, "sv1.rfind(\"abcde\") == SV::npos");;
        const _: () = assert!(true, "sv2.rfind(\"\") == 5");;
        const _: () = assert!(true, "sv2.rfind(\"abcde\") == 0");;
        const _: () = assert!(true, "sv2.rfind(\"abcde\", 1) == 0");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
