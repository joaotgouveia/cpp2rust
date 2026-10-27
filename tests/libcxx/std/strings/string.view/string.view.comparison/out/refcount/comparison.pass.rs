extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_strong_ordering__std_strong_ordering_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_weak_ordering__std_weak_ordering_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_partial_ordering__std_partial_ordering_ {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_wchar_t_ {}
impl constexpr_char_traits_wchar_t_ {
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_wchar_t_::lt(_c1, _c2)
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char8_t_ {}
impl constexpr_char_traits_char8_t_ {
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char8_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char8_t_::lt(_c1, _c2)
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char16_t_ {}
impl constexpr_char_traits_char16_t_ {
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char16_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char16_t_::lt(_c1, _c2)
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char32_t_ {}
impl constexpr_char_traits_char32_t_ {
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char32_t_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char32_t_::lt(_c1, _c2)
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
pub fn testComparisonsComplete_4(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_5(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_6(
    t1: Ptr<()>,
    t2: Ptr<Ptr<()>>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_7(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_8(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_9(
    t1: Ptr<()>,
    t2: Ptr<Ptr<()>>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_10(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_11(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_12(
    t1: Ptr<()>,
    t2: Ptr<Ptr<()>>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_13(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_14(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_15(
    t1: Ptr<()>,
    t2: Ptr<Ptr<()>>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_16(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_17(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_18(
    t1: Ptr<()>,
    t2: Ptr<Ptr<()>>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn AssertComparisonsAreNoexcept_19() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_20() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_21() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_22() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_23() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_24() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_25() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_26() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_27() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_28() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_29() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_30() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_31() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_32() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_33() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_34() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_35() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_36() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_37() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsAreNoexcept_38() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_39() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_40() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_41() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_42() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_43() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_44() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_45() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_46() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_47() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_48() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_49() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_50() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_51() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_52() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_53() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_54() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_55() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_56() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_57() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertComparisonsReturnBool_58() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_59() {
    ({ AssertComparisonsAreNoexcept_19() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_60() {
    ({ AssertComparisonsAreNoexcept_20() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_61() {
    ({ AssertComparisonsAreNoexcept_21() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_62() {
    ({ AssertComparisonsAreNoexcept_22() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_63() {
    ({ AssertComparisonsAreNoexcept_23() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_64() {
    ({ AssertComparisonsAreNoexcept_24() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_65() {
    ({ AssertComparisonsAreNoexcept_25() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_66() {
    ({ AssertComparisonsAreNoexcept_26() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_67() {
    ({ AssertComparisonsAreNoexcept_27() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_68() {
    ({ AssertComparisonsAreNoexcept_28() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_69() {
    ({ AssertComparisonsAreNoexcept_29() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_70() {
    ({ AssertComparisonsAreNoexcept_30() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_71() {
    ({ AssertComparisonsAreNoexcept_31() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_72() {
    ({ AssertComparisonsAreNoexcept_32() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_73() {
    ({ AssertComparisonsAreNoexcept_33() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_74() {
    ({ AssertComparisonsAreNoexcept_34() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_75() {
    ({ AssertComparisonsAreNoexcept_35() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_76() {
    ({ AssertComparisonsAreNoexcept_36() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_77() {
    ({ AssertComparisonsAreNoexcept_37() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderAreNoexcept_78() {
    ({ AssertComparisonsAreNoexcept_38() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_79() {
    ({ AssertComparisonsReturnBool_39() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_80() {
    ({ AssertComparisonsReturnBool_40() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_81() {
    ({ AssertComparisonsReturnBool_41() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_82() {
    ({ AssertComparisonsReturnBool_42() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_83() {
    ({ AssertComparisonsReturnBool_43() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_84() {
    ({ AssertComparisonsReturnBool_44() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_85() {
    ({ AssertComparisonsReturnBool_45() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_86() {
    ({ AssertComparisonsReturnBool_46() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_87() {
    ({ AssertComparisonsReturnBool_47() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_88() {
    ({ AssertComparisonsReturnBool_48() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_89() {
    ({ AssertComparisonsReturnBool_49() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_90() {
    ({ AssertComparisonsReturnBool_50() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_91() {
    ({ AssertComparisonsReturnBool_51() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_92() {
    ({ AssertComparisonsReturnBool_52() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_93() {
    ({ AssertComparisonsReturnBool_53() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_94() {
    ({ AssertComparisonsReturnBool_54() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_95() {
    ({ AssertComparisonsReturnBool_55() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_96() {
    ({ AssertComparisonsReturnBool_56() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_97() {
    ({ AssertComparisonsReturnBool_57() });
    const _: () = assert!(true, "");;
}
pub fn AssertOrderReturn_98() {
    ({ AssertComparisonsReturnBool_58() });
    const _: () = assert!(true, "");;
}
pub fn testOrder_99(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_100(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_5(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_101(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_6(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_102(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_103(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_5(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_104(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_6(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_105(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_106(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_5(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_107(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_6(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_108(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_7(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_109(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_8(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_110(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_9(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_111(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_7(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_112(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_8(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_113(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_9(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_114(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_7(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_115(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_8(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_116(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_9(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_117(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_10(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_118(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_11(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_119(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_12(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_120(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_10(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_121(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_11(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_122(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_12(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_123(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_10(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_124(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_11(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_125(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_12(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_126(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_13(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_127(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_14(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_128(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_15(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_129(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_13(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_130(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_14(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_131(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_15(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_132(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_13(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_133(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_14(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_134(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_15(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_135(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_16(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_136(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_17(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_137(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_18(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_138(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_16(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_139(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_17(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_140(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_18(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_141(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_16(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_142(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_17(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_143(t1: Ptr<()>, t2: Ptr<Ptr<()>>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<Ptr<()>> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_18(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn operator_lt_144(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_145(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } == { rhs.with(|__s| __s.value) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct LessAndEqComp {
    #[offset(0)]
    pub value: (),
}
impl LessAndEqComp {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_144(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_144(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_145(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub fn operator_cmp_146(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(_a0.value), &(_a1.value))));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct StrongOrder {
    #[offset(0)]
    pub value: (),
}
impl StrongOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_146(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_146(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub fn operator_cmp_147(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct WeakOrder {
    #[offset(0)]
    pub value: (),
}
impl WeakOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_147(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_147(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub fn operator_cmp_148(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == unimplemented!()) || (rhs.value == unimplemented!()) {
        return unimplemented!();
    }
    if (lhs.value == unimplemented!()) || (rhs.value == unimplemented!()) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_149(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct PartialOrder {
    #[offset(0)]
    pub value: (),
}
impl PartialOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_148(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_149(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub fn operator_eq_150(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct EqualityComparable {
    #[offset(0)]
    value_: (),
}
impl EqualityComparable {
    pub fn new(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_150(
                Rc::new(RefCell::new(EqualityComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(EqualityComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub fn operator_eq_151(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_152(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.value_)),
            &(_a1.with(|__s| __s.value_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ThreeWayComparable {
    #[offset(0)]
    value_: (),
}
impl ThreeWayComparable {
    pub fn new(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_152(
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_151(
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E1 {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E2 {
    #[offset(0)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ImplicitBool {
    #[offset(0)]
    pub val: (),
}
pub fn operator_eq_153(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_char__std_weak_ordering_ {}
impl char_traits_char__std_weak_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_char__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_char__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_char__std_partial_ordering_ {}
impl char_traits_char__std_partial_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_char__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_char__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_wchar_t__std_weak_ordering_ {}
impl char_traits_wchar_t__std_weak_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_wchar_t__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_wchar_t__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_wchar_t__std_partial_ordering_ {}
impl char_traits_wchar_t__std_partial_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_wchar_t__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_wchar_t__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_char8_t__std_weak_ordering_ {}
impl char_traits_char8_t__std_weak_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_char8_t__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_char8_t__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_char8_t__std_partial_ordering_ {}
impl char_traits_char8_t__std_partial_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_char8_t__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_char8_t__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_char16_t__std_weak_ordering_ {}
impl char_traits_char16_t__std_weak_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_char16_t__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_char16_t__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_char16_t__std_partial_ordering_ {}
impl char_traits_char16_t__std_partial_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_char16_t__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_char16_t__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_char32_t__std_weak_ordering_ {}
impl char_traits_char32_t__std_weak_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_char32_t__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_char32_t__std_weak_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct char_traits_char32_t__std_partial_ordering_ {}
impl char_traits_char32_t__std_partial_ordering_ {
    pub fn lt(mut __c1: (), mut __c2: ()) -> () {
        return (__c1 < __c2);
    }
    pub fn compare(mut __s1: Ptr<()>, mut __s2: Ptr<()>, mut __n: ()) -> () {
        'loop_: while (__n != 0) {
            if ({
                let ___c1: () = (__s1.read());
                let ___c2: () = (__s2.read());
                char_traits_char32_t__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return -1_();
            }
            if ({
                let ___c1: () = (__s2.read());
                let ___c2: () = (__s1.read());
                char_traits_char32_t__std_partial_ordering_::lt(___c1, ___c2)
            }) {
                return 1_();
            }
            {
                {
                    __n.prefix_dec();
                    __s1.prefix_inc()
                };
                __s2.prefix_inc()
            };
        }
        return 0_();
    }
}
pub fn test_154() {
    ({ AssertOrderAreNoexcept_59() });
    ({ AssertOrderReturn_79() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_155() {
    ({ AssertOrderAreNoexcept_60() });
    ({ AssertOrderReturn_80() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_156() {
    ({ AssertOrderAreNoexcept_61() });
    ({ AssertOrderReturn_81() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_157() {
    ({ AssertOrderAreNoexcept_62() });
    ({ AssertOrderReturn_82() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_158() {
    ({ AssertOrderAreNoexcept_63() });
    ({ AssertOrderReturn_83() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_159() {
    ({ AssertOrderAreNoexcept_64() });
    ({ AssertOrderReturn_84() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_160() {
    ({ AssertOrderAreNoexcept_65() });
    ({ AssertOrderReturn_85() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_161() {
    ({ AssertOrderAreNoexcept_66() });
    ({ AssertOrderReturn_86() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_162() {
    ({ AssertOrderAreNoexcept_67() });
    ({ AssertOrderReturn_87() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_163() {
    ({ AssertOrderAreNoexcept_68() });
    ({ AssertOrderReturn_88() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_164() {
    ({ AssertOrderAreNoexcept_69() });
    ({ AssertOrderReturn_89() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_165() {
    ({ AssertOrderAreNoexcept_70() });
    ({ AssertOrderReturn_90() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_166() {
    ({ AssertOrderAreNoexcept_71() });
    ({ AssertOrderReturn_91() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_167() {
    ({ AssertOrderAreNoexcept_72() });
    ({ AssertOrderReturn_92() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_168() {
    ({ AssertOrderAreNoexcept_73() });
    ({ AssertOrderReturn_93() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_169() {
    ({ AssertOrderAreNoexcept_74() });
    ({ AssertOrderReturn_94() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_170() {
    ({ AssertOrderAreNoexcept_75() });
    ({ AssertOrderReturn_95() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_171() {
    ({ AssertOrderAreNoexcept_76() });
    ({ AssertOrderReturn_96() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_172() {
    ({ AssertOrderAreNoexcept_77() });
    ({ AssertOrderReturn_97() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_173() {
    ({ AssertOrderAreNoexcept_78() });
    ({ AssertOrderReturn_98() });
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vn: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "v.size() == vn.size()");;
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let mut j: () = 0_();
        'loop_: while (j < unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
}
pub fn test_all_orderings_174() {
    ({ test_154() });
    ({ test_155() });
    ({ test_156() });
    ({ test_157() });
}
pub fn test_all_orderings_175() {
    ({ test_158() });
    ({ test_159() });
    ({ test_160() });
    ({ test_161() });
}
pub fn test_all_orderings_176() {
    ({ test_162() });
    ({ test_163() });
    ({ test_164() });
    ({ test_165() });
}
pub fn test_all_orderings_177() {
    ({ test_166() });
    ({ test_167() });
    ({ test_168() });
    ({ test_169() });
}
pub fn test_all_orderings_178() {
    ({ test_170() });
    ({ test_171() });
    ({ test_172() });
    ({ test_173() });
}
pub fn test_all_types_179() -> () {
    ({ test_all_orderings_174() });
    ({ test_all_orderings_175() });
    ({ test_all_orderings_176() });
    ({ test_all_orderings_177() });
    ({ test_all_orderings_178() });
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
    ({ test_all_types_179() });
    const _: () = assert!(true, "test_all_types()");;
    return 0_();
}
pub trait ImplicitBoolImpl {
    fn to__1(&self) -> ();
}
impl ImplicitBoolImpl for Ptr<ImplicitBool> {
    fn to__1(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub fn __cpp2rust_init_globals() {}
