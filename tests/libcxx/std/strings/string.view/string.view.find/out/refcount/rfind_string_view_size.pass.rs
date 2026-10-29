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
}
pub fn test_0(s: Ptr<()>, str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test_1(s: Ptr<()>, str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test0_2() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 2_(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 2_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 2_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 2_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 4_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 4_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 4_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 4_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 5_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 5_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 5_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 5_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 6_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 6_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 6_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 6_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 5_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 5_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 5_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 5_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 9_(), 9_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 9_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 9_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 9_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 10_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 10_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 10_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 10_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 11_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 11_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 11_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 11_(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 1_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 10_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 10_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 10_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 10_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 19_(), 19_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 19_(), 15_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 19_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 19_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 20_(), 20_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 20_(), 15_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 20_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 20_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 21_(), 20_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 21_(), 15_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 21_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(_s.as_pointer(), _str.as_pointer(), 21_(), 0_())
    });
}
pub fn test1_3() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), unimplemented!())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 20_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 15_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(_s.as_pointer(), _str.as_pointer(), 0_())
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
        ({ test0_2() });
        ({ test1_3() });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.rfind(sv1) == 0");;
        const _: () = assert!(true, "sv1.rfind(sv2) == SV::npos");;
        const _: () = assert!(true, "sv2.rfind(sv1) == 5");;
        const _: () = assert!(true, "sv2.rfind(sv2) == 0");;
        const _: () = assert!(true, "sv2.rfind(sv2, 1) == 0");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
