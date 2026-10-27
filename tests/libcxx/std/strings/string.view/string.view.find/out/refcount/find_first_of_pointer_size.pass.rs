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
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test_1(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test0_2() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfkap"),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ihqrfebgad"),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikcrq"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmajblfhsg"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oqftjhdmkgsblacenirp"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oebqi"),
            2_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kojhpmbsfe"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acbsjqogpltdkhinfrem"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aobjd"),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbahntsje"),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fhepcrntkoagbmldqijs"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kocgb"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fbslrjiqkm"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jeidpcmalhfnqbgtrsko"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qngpd"),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rodhqklgmb"),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thdjgafrlbkoiqcspmne"),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ashjd"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mgojkldsqh"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"imqnaghkfrdtlopbjesc"),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nadkh"),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ofdrqmkebl"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdfjqgatlksriohemnpc"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prqgn"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pejafmnokr"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odnqkgijrhabfmcestlp"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtjpa"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ktsrmnqagd"),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtdhgcisbnmoaqkfpjle"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dplqa"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dkacjoptns"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqfimtrgnbekpdcsjalo"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lofbc"),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"astoegbfpn"),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pdgreqomsncafklhtibj"),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbtqd"),
            0_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tboimldpjh"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"slcerthdaiqjfnobgkpm"),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mkqpbtdalgniorhfescj"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pblas"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"djkqc"),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lgokshjtpb"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nocfa"),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bgtajmiedc"),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsckfnqgdahejiopbtmr"),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsocbhjtm"),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflmdnoicjgkberhqsa"),
            21_(),
            unimplemented!(),
        )
    });
}
pub fn test1_3() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"irkhs"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kantesmpgj"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oknlrstdpiqmjbaghcfe"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bnrpe"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oselktgbcapndfjihrmq"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
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
        ({ test0_2() });
        ({ test1_3() });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.find_first_of(\"\", 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_first_of(\"irkhs\", 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\"\", 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\"gfsrt\", 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\"lecar\", 0) == 0");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
