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
pub fn test_0(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_1.with(|rc| *rc.borrow())) {
        let mut n: () = unimplemented!();
        unimplemented!();
    }
}
pub fn test_2(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_1.with(|rc| *rc.borrow())) {
        let mut pos: () = unimplemented!();
        let mut n: () = unimplemented!();
        unimplemented!();
    }
}
pub fn test0_3() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
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
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            1_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            2_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            2_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            4_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            4_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            6_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            6_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            6_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
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
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            5_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            9_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            9_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            11_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            11_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            11_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
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
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            10_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            19_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            19_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            20_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            20_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            20_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            20_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            21_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            21_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            21_(),
            0_(),
        )
    });
}
pub fn test1_4() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 20_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcde"),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcde"),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abcdeabcdeabcdeabcde"),
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        ({ test0_3() });
        ({ test1_4() });
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
