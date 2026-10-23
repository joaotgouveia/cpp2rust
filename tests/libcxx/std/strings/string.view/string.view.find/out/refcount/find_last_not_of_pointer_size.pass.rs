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
pub fn test_0(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_1.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_2(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    unimplemented!();
    if (x != npos_1.with(|rc| *rc.borrow())) {
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
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
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
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            1_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            1_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
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
            Ptr::<()>::from_string_literal(b"dfkap"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ihqrfebgad"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
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
            Ptr::<()>::from_string_literal(b"ikcrq"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmajblfhsg"),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oqftjhdmkgsblacenirp"),
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
            Ptr::<()>::from_string_literal(b"oebqi"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kojhpmbsfe"),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acbsjqogpltdkhinfrem"),
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
            Ptr::<()>::from_string_literal(b"aobjd"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbahntsje"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fhepcrntkoagbmldqijs"),
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
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kocgb"),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fbslrjiqkm"),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jeidpcmalhfnqbgtrsko"),
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
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qngpd"),
            6_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rodhqklgmb"),
            6_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thdjgafrlbkoiqcspmne"),
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
            Ptr::<()>::from_string_literal(b"ashjd"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mgojkldsqh"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"imqnaghkfrdtlopbjesc"),
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
            Ptr::<()>::from_string_literal(b"nadkh"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ofdrqmkebl"),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdfjqgatlksriohemnpc"),
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
            Ptr::<()>::from_string_literal(b"prqgn"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pejafmnokr"),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odnqkgijrhabfmcestlp"),
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
            Ptr::<()>::from_string_literal(b"rtjpa"),
            9_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ktsrmnqagd"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtdhgcisbnmoaqkfpjle"),
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
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dplqa"),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dkacjoptns"),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqfimtrgnbekpdcsjalo"),
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
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lofbc"),
            11_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"astoegbfpn"),
            11_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pdgreqomsncafklhtibj"),
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
            Ptr::<()>::from_string_literal(b"lbtqd"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tboimldpjh"),
            0_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"slcerthdaiqjfnobgkpm"),
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
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mkqpbtdalgniorhfescj"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"pblas"),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            10_(),
            npos_1.with(|rc| *rc.borrow()),
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
            Ptr::<()>::from_string_literal(b"djkqc"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lgokshjtpb"),
            19_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            19_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nocfa"),
            20_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bgtajmiedc"),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsckfnqgdahejiopbtmr"),
            20_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            21_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsocbhjtm"),
            21_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflmdnoicjgkberhqsa"),
            21_(),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
}
pub fn test1_4() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"irkhs"),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kantesmpgj"),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oknlrstdpiqmjbaghcfe"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 9_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bnrpe"),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oselktgbcapndfjihrmq"),
            npos_1.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 19_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_2(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            npos_1.with(|rc| *rc.borrow()),
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
        const _: () = assert!(true, "sv1.find_last_not_of(\"\", 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_last_not_of(\"irkhs\", 5) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_not_of(\"\", 0) == 0");;
        const _: () = assert!(true, "sv2.find_last_not_of(\"gfsrt\", 5) == 4");;
        const _: () = assert!(true, "sv2.find_last_not_of(\"lecar\", 5) == 3");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
