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
    pub fn find(s: Ptr<()>, mut n: (), a: Ptr<()>) -> Ptr<()> {
        let s: Value<Ptr<()>> = Rc::new(RefCell::new(s));
        'loop_: while (n != 0) {
            if ({
                let _c1: () = ((*s.borrow()).read());
                let _c2: () = (a.read());
                constexpr_char_traits_char_::eq(_c1, _c2)
            }) {
                return (*s.borrow()).clone();
            }
            (*s.borrow_mut()).prefix_inc();
            n.prefix_dec();
        }
        return Ptr::<()>::null();
    }
}
pub fn test_0(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test0_1() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            2_(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            2_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            2_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            2_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            2_(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            2_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            2_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            2_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            2_(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            2_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            2_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            2_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            4_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            4_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            4_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            4_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            4_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            4_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            4_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            4_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            4_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            4_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            4_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            4_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            2_(),
            unimplemented!(),
        )
    });
}
pub fn test1_2() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            6_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            6_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            6_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            6_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            6_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            6_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            6_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            6_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            6_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            2_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            4_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            2_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            4_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            9_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            9_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            9_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            9_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            9_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            9_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            9_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            9_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            9_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            9_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            9_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            9_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            9_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            9_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            9_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
}
pub fn test2_3() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            11_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            11_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            11_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            11_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            11_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            11_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            11_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            11_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            11_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            11_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            11_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            11_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            2_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            4_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            9_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            2_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
            5_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            5_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            9_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            19_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            19_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            19_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            19_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            19_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            19_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            19_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            19_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            19_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            19_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            19_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            19_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            19_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            19_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            19_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            0_(),
            20_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            20_(),
            0_(),
            20_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            20_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            20_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            20_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            20_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            20_(),
            0_(),
            20_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            20_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            20_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            20_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            20_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            20_(),
            0_(),
            20_(),
        )
    });
}
pub fn test3_4() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            20_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            20_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            20_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            20_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            21_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            21_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            21_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            21_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            21_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            21_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            21_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            21_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            21_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            21_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            21_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            21_(),
            20_(),
            unimplemented!(),
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        ({ test0_1() });
        ({ test1_2() });
        ({ test2_3() });
        ({ test3_4() });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.find(\"\", 0, 0) == 0");;
        const _: () = assert!(true, "sv1.find(\"abcde\", 0, 0) == 0");;
        const _: () = assert!(true, "sv1.find(\"abcde\", 0, 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find(\"\", 0, 0) == 0");;
        const _: () = assert!(true, "sv2.find(\"abcde\", 0, 0) == 0");;
        const _: () = assert!(true, "sv2.find(\"abcde\", 0, 1) == 0");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
