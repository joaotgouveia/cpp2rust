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
    pub fn find(s: Ptr<i8>, mut n: usize, a: Ptr<i8>) -> Ptr<i8> {
        let s: Value<Ptr<i8>> = Rc::new(RefCell::new(s));
        'loop_: while (n != 0) {
            if ({
                let _c1: i8 = ((*s.borrow()).read());
                let _c2: i8 = (a.read());
                constexpr_char_traits_char_::eq(_c1, _c2)
            }) {
                return (*s.borrow()).clone();
            }
            (*s.borrow_mut()).prefix_inc();
            n.prefix_dec();
        }
        return Ptr::<i8>::null();
    }
}
pub fn test_0(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(({ unimplemented!() } == { x }));
    if (x != unimplemented!()) {
        assert!((pos <= x) && ({ x } < { unimplemented!() }));
    }
}
pub fn test_1(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    assert!(({ unimplemented!() } == { x }));
    if (x != unimplemented!()) {
        assert!(({ x } < { unimplemented!() }));
    }
}
pub fn test0_2() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"laenf")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pqlnkmbdjo")).reinterpret_cast::<()>(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qkamfogpnljdcshbreti")).reinterpret_cast::<()>(),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bjaht")).reinterpret_cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hjlcmgpket")).reinterpret_cast::<()>(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"htaobedqikfplcgjsmrn")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"dfkap")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ihqrfebgad")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ngtjfcalbseiqrphmkdo")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"ikcrq")).reinterpret_cast::<()>(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dmajblfhsg")).reinterpret_cast::<()>(),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oqftjhdmkgsblacenirp")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"oebqi")).reinterpret_cast::<()>(),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kojhpmbsfe")).reinterpret_cast::<()>(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"acbsjqogpltdkhinfrem")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"aobjd")).reinterpret_cast::<()>(),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pcbahntsje")).reinterpret_cast::<()>(),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fhepcrntkoagbmldqijs")).reinterpret_cast::<()>(),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kocgb")).reinterpret_cast::<()>(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fbslrjiqkm")).reinterpret_cast::<()>(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jeidpcmalhfnqbgtrsko")).reinterpret_cast::<()>(),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qngpd")).reinterpret_cast::<()>(),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rodhqklgmb")).reinterpret_cast::<()>(),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"thdjgafrlbkoiqcspmne")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"ashjd")).reinterpret_cast::<()>(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"mgojkldsqh")).reinterpret_cast::<()>(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"imqnaghkfrdtlopbjesc")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"nadkh")).reinterpret_cast::<()>(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ofdrqmkebl")).reinterpret_cast::<()>(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bdfjqgatlksriohemnpc")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"prqgn")).reinterpret_cast::<()>(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pejafmnokr")).reinterpret_cast::<()>(),
            5_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"odnqkgijrhabfmcestlp")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"rtjpa")).reinterpret_cast::<()>(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ktsrmnqagd")).reinterpret_cast::<()>(),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rtdhgcisbnmoaqkfpjle")).reinterpret_cast::<()>(),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dplqa")).reinterpret_cast::<()>(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dkacjoptns")).reinterpret_cast::<()>(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hqfimtrgnbekpdcsjalo")).reinterpret_cast::<()>(),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lofbc")).reinterpret_cast::<()>(),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"astoegbfpn")).reinterpret_cast::<()>(),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pdgreqomsncafklhtibj")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"lbtqd")).reinterpret_cast::<()>(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tboimldpjh")).reinterpret_cast::<()>(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"slcerthdaiqjfnobgkpm")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"aqibs")).reinterpret_cast::<()>(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gtfblmqinc")).reinterpret_cast::<()>(),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"mkqpbtdalgniorhfescj")).reinterpret_cast::<()>(),
            1_(),
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
            (Ptr::<i8>::from_string_literal(b"pblas")).reinterpret_cast::<()>(),
            10_(),
            11_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"arosdhcfme")).reinterpret_cast::<()>(),
            10_(),
            13_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"blkhjeogicatqfnpdmsr")).reinterpret_cast::<()>(),
            10_(),
            unimplemented!(),
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
            (Ptr::<i8>::from_string_literal(b"djkqc")).reinterpret_cast::<()>(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lgokshjtpb")).reinterpret_cast::<()>(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bqjhtkfepimcnsgrlado")).reinterpret_cast::<()>(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nocfa")).reinterpret_cast::<()>(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bgtajmiedc")).reinterpret_cast::<()>(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lsckfnqgdahejiopbtmr")).reinterpret_cast::<()>(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gfsrt")).reinterpret_cast::<()>(),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pfsocbhjtm")).reinterpret_cast::<()>(),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tpflmdnoicjgkberhqsa")).reinterpret_cast::<()>(),
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
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"laenf")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pqlnkmbdjo")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qkamfogpnljdcshbreti")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
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
            (Ptr::<i8>::from_string_literal(b"irkhs")).reinterpret_cast::<()>(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kantesmpgj")).reinterpret_cast::<()>(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oknlrstdpiqmjbaghcfe")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
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
            (Ptr::<i8>::from_string_literal(b"bnrpe")).reinterpret_cast::<()>(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jtdaefblso")).reinterpret_cast::<()>(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oselktgbcapndfjihrmq")).reinterpret_cast::<()>(),
            unimplemented!(),
        )
    });
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
            (Ptr::<i8>::from_string_literal(b"bjaht")).reinterpret_cast::<()>(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hjlcmgpket")).reinterpret_cast::<()>(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_1(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"htaobedqikfplcgjsmrn")).reinterpret_cast::<()>(),
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        ({ test0_2() });
        ({ test1_3() });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.find_first_not_of(\"\", 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_first_not_of(\"irkhs\", 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_not_of(\"\", 0) == 0");;
        const _: () = assert!(true, "sv2.find_first_not_of(\"gfsrt\", 0) == 0");;
        const _: () = assert!(true, "sv2.find_first_not_of(\"lecar\", 0) == 1");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
