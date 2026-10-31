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
pub fn test_0(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    assert!(({ unimplemented!() } == { x }));
    if (x != unimplemented!()) {
        assert!((x <= pos) && ({ x } < { unimplemented!() }));
    }
}
pub fn test0_1() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"irkhs")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kante")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oknlr")).reinterpret_cast::<()>(),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pcdro")).reinterpret_cast::<()>(),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bnrpe")).reinterpret_cast::<()>(),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jtdaefblso")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oselktgbca")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"eqgaplhckj")).reinterpret_cast::<()>(),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bjahtcmnlp")).reinterpret_cast::<()>(),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hjlcmgpket")).reinterpret_cast::<()>(),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"htaobedqikfplcgjsmrn")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hpqiarojkcdlsgnmfetb")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dfkaprhjloqetcsimnbg")).reinterpret_cast::<()>(),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ihqrfebgadntlpmjksoc")).reinterpret_cast::<()>(),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ngtjfcalbseiqrphmkdo")).reinterpret_cast::<()>(),
            0_(),
            20_(),
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
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lbtqd")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tboim")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"slcer")).reinterpret_cast::<()>(),
            1_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"cbjfs")).reinterpret_cast::<()>(),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"aqibs")).reinterpret_cast::<()>(),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gtfblmqinc")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"mkqpbtdalg")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kphatlimcd")).reinterpret_cast::<()>(),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pblasqogic")).reinterpret_cast::<()>(),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"arosdhcfme")).reinterpret_cast::<()>(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"blkhjeogicatqfnpdmsr")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bmhineprjcoadgstflqk")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"djkqcmetslnghpbarfoi")).reinterpret_cast::<()>(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lgokshjtpbemarcdqnfi")).reinterpret_cast::<()>(),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bqjhtkfepimcnsgrlado")).reinterpret_cast::<()>(),
            1_(),
            20_(),
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
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gfsrt")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pfsoc")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tpflm")).reinterpret_cast::<()>(),
            0_(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sgkec")).reinterpret_cast::<()>(),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"romds")).reinterpret_cast::<()>(),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qhjistlgmr")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pedfirsglo")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"aqcoslgrmk")).reinterpret_cast::<()>(),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dabckmepqj")).reinterpret_cast::<()>(),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pqscrjthli")).reinterpret_cast::<()>(),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kfphdcsjqmobliagtren")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rokpefncljibsdhqtagm")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"afionmkphlebtcjqsgrd")).reinterpret_cast::<()>(),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"aenmqplidhkofrjbctsg")).reinterpret_cast::<()>(),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"osjmbtcadhiklegrpqnf")).reinterpret_cast::<()>(),
            0_(),
            20_(),
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
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"osmia")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ckonl")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ilcaj")).reinterpret_cast::<()>(),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lasiq")).reinterpret_cast::<()>(),
            1_(),
            4_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kfqmr")).reinterpret_cast::<()>(),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"klnitfaobg")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gjhmdlqikp")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"skbgtahqej")).reinterpret_cast::<()>(),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bjsdgtlpkf")).reinterpret_cast::<()>(),
            1_(),
            9_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bjgfmnlkio")).reinterpret_cast::<()>(),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lbhepotfsjdqigcnamkr")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tebangckmpsrqdlfojhi")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"joflqbdkhtegimscpanr")).reinterpret_cast::<()>(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"adpmcohetfbsrjinlqkg")).reinterpret_cast::<()>(),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"iacldqjpfnogbsrhmetk")).reinterpret_cast::<()>(),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            2_(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"otkgb")).reinterpret_cast::<()>(),
            2_(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"cqsjl")).reinterpret_cast::<()>(),
            2_(),
            1_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dpifl")).reinterpret_cast::<()>(),
            2_(),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oapht")).reinterpret_cast::<()>(),
            2_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"cifts")).reinterpret_cast::<()>(),
            2_(),
            5_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nmsckbgalo")).reinterpret_cast::<()>(),
            2_(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tpksqhamle")).reinterpret_cast::<()>(),
            2_(),
            1_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tpdrchmkji")).reinterpret_cast::<()>(),
            2_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ijagfkblst")).reinterpret_cast::<()>(),
            2_(),
            9_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kpocsignjb")).reinterpret_cast::<()>(),
            2_(),
            10_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pecqtkjsnbdrialgmohf")).reinterpret_cast::<()>(),
            2_(),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"aiortphfcmkjebgsndql")).reinterpret_cast::<()>(),
            2_(),
            1_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sdbkeamglhipojqftrcn")).reinterpret_cast::<()>(),
            2_(),
            10_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ljqncehgmfktroapidbs")).reinterpret_cast::<()>(),
            2_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rtcfodilamkbenjghqps")).reinterpret_cast::<()>(),
            2_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            4_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"mabns")).reinterpret_cast::<()>(),
            4_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bdnrp")).reinterpret_cast::<()>(),
            4_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"scidp")).reinterpret_cast::<()>(),
            4_(),
            2_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"agbjl")).reinterpret_cast::<()>(),
            4_(),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jfmpr")).reinterpret_cast::<()>(),
            4_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rbpefghsmj")).reinterpret_cast::<()>(),
            4_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"apsfntdoqc")).reinterpret_cast::<()>(),
            4_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ndkjeisgcl")).reinterpret_cast::<()>(),
            4_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rnfpqatdeo")).reinterpret_cast::<()>(),
            4_(),
            9_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bntjlqrfik")).reinterpret_cast::<()>(),
            4_(),
            10_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kcrtmpolnaqejghsfdbi")).reinterpret_cast::<()>(),
            4_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lobheanpkmqidsrtcfgj")).reinterpret_cast::<()>(),
            4_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"athdkljcnreqbgpmisof")).reinterpret_cast::<()>(),
            4_(),
            10_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qkdmjialrscpbhefgont")).reinterpret_cast::<()>(),
            4_(),
            19_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dmasojntqleribkgfchp")).reinterpret_cast::<()>(),
            4_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            5_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"psthd")).reinterpret_cast::<()>(),
            5_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rpmjd")).reinterpret_cast::<()>(),
            5_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dfsmk")).reinterpret_cast::<()>(),
            5_(),
            2_(),
            4_(),
        )
    });
}
pub fn test1_2() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"skqne")).reinterpret_cast::<()>(),
            5_(),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kipnf")).reinterpret_cast::<()>(),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hmrnqdgifl")).reinterpret_cast::<()>(),
            5_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fsmjcdairn")).reinterpret_cast::<()>(),
            5_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pcdgltbrfj")).reinterpret_cast::<()>(),
            5_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"aekfctpirg")).reinterpret_cast::<()>(),
            5_(),
            9_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ledihrsgpf")).reinterpret_cast::<()>(),
            5_(),
            10_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"mqcklahsbtirgopefndj")).reinterpret_cast::<()>(),
            5_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kmlthaoqgecrnpdbjfis")).reinterpret_cast::<()>(),
            5_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sfhbamcdptojlkrenqgi")).reinterpret_cast::<()>(),
            5_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pbniofmcedrkhlstgaqj")).reinterpret_cast::<()>(),
            5_(),
            19_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"mongjratcskbhqiepfdl")).reinterpret_cast::<()>(),
            5_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            6_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hrnat")).reinterpret_cast::<()>(),
            6_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gsqdt")).reinterpret_cast::<()>(),
            6_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bspkd")).reinterpret_cast::<()>(),
            6_(),
            2_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ohcmb")).reinterpret_cast::<()>(),
            6_(),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"heatr")).reinterpret_cast::<()>(),
            6_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pmblckedfn")).reinterpret_cast::<()>(),
            6_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"aceqmsrbik")).reinterpret_cast::<()>(),
            6_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lmbtdehjrn")).reinterpret_cast::<()>(),
            6_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"teqmcrlgib")).reinterpret_cast::<()>(),
            6_(),
            9_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"njolbmspac")).reinterpret_cast::<()>(),
            6_(),
            10_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pofnhidklamecrbqjgst")).reinterpret_cast::<()>(),
            6_(),
            0_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jbhckmtgrqnosafedpli")).reinterpret_cast::<()>(),
            6_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dobntpmqklicsahgjerf")).reinterpret_cast::<()>(),
            6_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tpdshainjkbfoemlrgcq")).reinterpret_cast::<()>(),
            6_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oldpfgeakrnitscbjmqh")).reinterpret_cast::<()>(),
            6_(),
            20_(),
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
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rqegt")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dashm")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jqirk")).reinterpret_cast::<()>(),
            0_(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rckeg")).reinterpret_cast::<()>(),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jscie")).reinterpret_cast::<()>(),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"efsphndliq")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gdicosleja")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qcpjibosfl")).reinterpret_cast::<()>(),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lrhmefnjcq")).reinterpret_cast::<()>(),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dtablcrseo")).reinterpret_cast::<()>(),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"apckjsftedbhgomrnilq")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pcbrgflehjtiadnsokqm")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nsiadegjklhobrmtqcpf")).reinterpret_cast::<()>(),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"cpmajdqnolikhgsbretf")).reinterpret_cast::<()>(),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jcflkntmgiqrphdosaeb")).reinterpret_cast::<()>(),
            0_(),
            20_(),
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
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ontrs")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pfkna")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ekosa")).reinterpret_cast::<()>(),
            1_(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"anqhk")).reinterpret_cast::<()>(),
            1_(),
            4_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jekca")).reinterpret_cast::<()>(),
            1_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ikemsjgacf")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"arolgsjkhm")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oftkbldhre")).reinterpret_cast::<()>(),
            1_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gbkqdoeftl")).reinterpret_cast::<()>(),
            1_(),
            9_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sqcflrgtim")).reinterpret_cast::<()>(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fmhbkislrjdpanogqcet")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rnioadktqlgpbcjsmhef")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oakgtnldpsefihqmjcbr")).reinterpret_cast::<()>(),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gbnaelosidmcjqktfhpr")).reinterpret_cast::<()>(),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"akbripjhlosndcmqgfet")).reinterpret_cast::<()>(),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pijag")).reinterpret_cast::<()>(),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jrckd")).reinterpret_cast::<()>(),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qcloh")).reinterpret_cast::<()>(),
            5_(),
            2_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"thlmp")).reinterpret_cast::<()>(),
            5_(),
            4_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qidmo")).reinterpret_cast::<()>(),
            5_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lnegpsjqrd")).reinterpret_cast::<()>(),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rjqdablmfs")).reinterpret_cast::<()>(),
            5_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"enkgpbsjaq")).reinterpret_cast::<()>(),
            5_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kdsgoaijfh")).reinterpret_cast::<()>(),
            5_(),
            9_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"trfqgmckbe")).reinterpret_cast::<()>(),
            5_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"igetsracjfkdnpoblhqm")).reinterpret_cast::<()>(),
            5_(),
            0_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nqctfaogirshlekbdjpm")).reinterpret_cast::<()>(),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"csehfgomljdqinbartkp")).reinterpret_cast::<()>(),
            5_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qahoegcmplkfsjbdnitr")).reinterpret_cast::<()>(),
            5_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dpteiajrqmsognhlfbkc")).reinterpret_cast::<()>(),
            5_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            9_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tqbnh")).reinterpret_cast::<()>(),
            9_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"akmle")).reinterpret_cast::<()>(),
            9_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"iqfkm")).reinterpret_cast::<()>(),
            9_(),
            2_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tqjsr")).reinterpret_cast::<()>(),
            9_(),
            4_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jplqg")).reinterpret_cast::<()>(),
            9_(),
            5_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oilnrbcgtj")).reinterpret_cast::<()>(),
            9_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"morkglpesn")).reinterpret_cast::<()>(),
            9_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dmicerngat")).reinterpret_cast::<()>(),
            9_(),
            5_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"radgeskbtc")).reinterpret_cast::<()>(),
            9_(),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ljikprsmqo")).reinterpret_cast::<()>(),
            9_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"trqihkcgsjamfdbolnpe")).reinterpret_cast::<()>(),
            9_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lqmthbsrekajgnofcipd")).reinterpret_cast::<()>(),
            9_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jtalmedribkgqsopcnfh")).reinterpret_cast::<()>(),
            9_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"spqfoiclmtagejbndkrh")).reinterpret_cast::<()>(),
            9_(),
            19_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nmotklspigjrdhcfaebq")).reinterpret_cast::<()>(),
            9_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            10_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hpmsd")).reinterpret_cast::<()>(),
            10_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qnpor")).reinterpret_cast::<()>(),
            10_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"otdma")).reinterpret_cast::<()>(),
            10_(),
            2_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"efhjg")).reinterpret_cast::<()>(),
            10_(),
            4_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"odpte")).reinterpret_cast::<()>(),
            10_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bctdgfmolr")).reinterpret_cast::<()>(),
            10_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oaklidrbqg")).reinterpret_cast::<()>(),
            10_(),
            1_(),
            9_(),
        )
    });
}
pub fn test2_3() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dnjfsagktr")).reinterpret_cast::<()>(),
            10_(),
            5_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nejaktmiqg")).reinterpret_cast::<()>(),
            10_(),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pjqonlebsf")).reinterpret_cast::<()>(),
            10_(),
            10_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dshmnbtolcjepgaikfqr")).reinterpret_cast::<()>(),
            10_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"iogfhpabtjkqlrnemcds")).reinterpret_cast::<()>(),
            10_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ngridfabjsecpqltkmoh")).reinterpret_cast::<()>(),
            10_(),
            10_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"athmknplcgofrqejsdib")).reinterpret_cast::<()>(),
            10_(),
            19_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ldobhmqcafnjtkeisgrp")).reinterpret_cast::<()>(),
            10_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            11_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"aocjb")).reinterpret_cast::<()>(),
            11_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jbrnk")).reinterpret_cast::<()>(),
            11_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tqedg")).reinterpret_cast::<()>(),
            11_(),
            2_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nqskp")).reinterpret_cast::<()>(),
            11_(),
            4_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"eaqkl")).reinterpret_cast::<()>(),
            11_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"reaoicljqm")).reinterpret_cast::<()>(),
            11_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lsftgajqpm")).reinterpret_cast::<()>(),
            11_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rlpfogmits")).reinterpret_cast::<()>(),
            11_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"shkncmiaqj")).reinterpret_cast::<()>(),
            11_(),
            9_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fpnatrhqgs")).reinterpret_cast::<()>(),
            11_(),
            10_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sjclemqhnpdbgikarfot")).reinterpret_cast::<()>(),
            11_(),
            0_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"otcmedjikgsfnqbrhpla")).reinterpret_cast::<()>(),
            11_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bonsaefdqiprkhlgtjcm")).reinterpret_cast::<()>(),
            11_(),
            10_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"egpscmahijlfnkrodqtb")).reinterpret_cast::<()>(),
            11_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kmqbfepjthgilscrndoa")).reinterpret_cast::<()>(),
            11_(),
            20_(),
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
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"prboq")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fjcqh")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fmosa")).reinterpret_cast::<()>(),
            0_(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qdbok")).reinterpret_cast::<()>(),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"amslg")).reinterpret_cast::<()>(),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"smpltjneqb")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"flitskrnge")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"pgqihmlbef")).reinterpret_cast::<()>(),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"cfpdqjtgsb")).reinterpret_cast::<()>(),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"htpsiaflom")).reinterpret_cast::<()>(),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kpjfiaceghsrdtlbnomq")).reinterpret_cast::<()>(),
            0_(),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qhtbomidljgafneksprc")).reinterpret_cast::<()>(),
            0_(),
            1_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nhtjobkcefldimpsaqgr")).reinterpret_cast::<()>(),
            0_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"prabcjfqnoeskilmtgdh")).reinterpret_cast::<()>(),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dtrgmchilkasqoebfpjn")).reinterpret_cast::<()>(),
            0_(),
            20_(),
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
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sqome")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"smfte")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ciboh")).reinterpret_cast::<()>(),
            1_(),
            2_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"haois")).reinterpret_cast::<()>(),
            1_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abfki")).reinterpret_cast::<()>(),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"frdkocntmq")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oasbpedlnr")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kltqmhgand")).reinterpret_cast::<()>(),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gdtfjchpmr")).reinterpret_cast::<()>(),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ponmcqblet")).reinterpret_cast::<()>(),
            1_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sgphqdnofeiklatbcmjr")).reinterpret_cast::<()>(),
            1_(),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ljqprsmigtfoneadckbh")).reinterpret_cast::<()>(),
            1_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ligeojhafnkmrcsqtbdp")).reinterpret_cast::<()>(),
            1_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lsimqfnjarbopedkhcgt")).reinterpret_cast::<()>(),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"abedmfjlghniorcqptks")).reinterpret_cast::<()>(),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hqtoa")).reinterpret_cast::<()>(),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"cahif")).reinterpret_cast::<()>(),
            10_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kehis")).reinterpret_cast::<()>(),
            10_(),
            2_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kdlmh")).reinterpret_cast::<()>(),
            10_(),
            4_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"paeql")).reinterpret_cast::<()>(),
            10_(),
            5_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"aghoqiefnb")).reinterpret_cast::<()>(),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jrbqaikpdo")).reinterpret_cast::<()>(),
            10_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"smjonaeqcl")).reinterpret_cast::<()>(),
            10_(),
            5_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"eqbdrkcfah")).reinterpret_cast::<()>(),
            10_(),
            9_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"kapmsienhf")).reinterpret_cast::<()>(),
            10_(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jpqotrlenfcsbhkaimdg")).reinterpret_cast::<()>(),
            10_(),
            0_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jlbmhnfgtcqprikeados")).reinterpret_cast::<()>(),
            10_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"stgbhfmdaljnpqoicker")).reinterpret_cast::<()>(),
            10_(),
            10_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"oihcetflbjagdsrkmqpn")).reinterpret_cast::<()>(),
            10_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"adtclebmnpjsrqfkigoh")).reinterpret_cast::<()>(),
            10_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            19_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"beafg")).reinterpret_cast::<()>(),
            19_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"iclat")).reinterpret_cast::<()>(),
            19_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rkhnf")).reinterpret_cast::<()>(),
            19_(),
            2_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"clshq")).reinterpret_cast::<()>(),
            19_(),
            4_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dtcoj")).reinterpret_cast::<()>(),
            19_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rqosnjmfth")).reinterpret_cast::<()>(),
            19_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"siatdfqglh")).reinterpret_cast::<()>(),
            19_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"mrlshtpgjq")).reinterpret_cast::<()>(),
            19_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"adlcskgqjt")).reinterpret_cast::<()>(),
            19_(),
            9_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"drshcjknaf")).reinterpret_cast::<()>(),
            19_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"etsaqroinghpkjdlfcbm")).reinterpret_cast::<()>(),
            19_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sgepdnkqliambtrocfhj")).reinterpret_cast::<()>(),
            19_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nlmcjaqgbsortfdihkpe")).reinterpret_cast::<()>(),
            19_(),
            10_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"racfnpmosldibqkghjet")).reinterpret_cast::<()>(),
            19_(),
            19_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fjhdsctkqeiolagrnmbp")).reinterpret_cast::<()>(),
            19_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"")).reinterpret_cast::<()>(),
            20_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ejanp")).reinterpret_cast::<()>(),
            20_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"odife")).reinterpret_cast::<()>(),
            20_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"okaqd")).reinterpret_cast::<()>(),
            20_(),
            2_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"lcdbi")).reinterpret_cast::<()>(),
            20_(),
            4_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fsqbj")).reinterpret_cast::<()>(),
            20_(),
            5_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"bigdomnplq")).reinterpret_cast::<()>(),
            20_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"apiblotgcd")).reinterpret_cast::<()>(),
            20_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"acfhdenops")).reinterpret_cast::<()>(),
            20_(),
            5_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jopdeamcrk")).reinterpret_cast::<()>(),
            20_(),
            9_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"trqncbkgmh")).reinterpret_cast::<()>(),
            20_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"tomglrkencbsfjqpihda")).reinterpret_cast::<()>(),
            20_(),
            0_(),
            19_(),
        )
    });
}
pub fn test3_4() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"gbkhdnpoietfcmrslajq")).reinterpret_cast::<()>(),
            20_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"rtfnmbsglkjaichoqedp")).reinterpret_cast::<()>(),
            20_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ohkmdpfqbsacrtjnlgei")).reinterpret_cast::<()>(),
            20_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"dlbrteoisgphmkncajfq")).reinterpret_cast::<()>(),
            20_(),
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
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"sjrlo")).reinterpret_cast::<()>(),
            21_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qjpor")).reinterpret_cast::<()>(),
            21_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"odhfn")).reinterpret_cast::<()>(),
            21_(),
            2_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qtfin")).reinterpret_cast::<()>(),
            21_(),
            4_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hpqfo")).reinterpret_cast::<()>(),
            21_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"fabmertkos")).reinterpret_cast::<()>(),
            21_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"brqtgkmaej")).reinterpret_cast::<()>(),
            21_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"nfrdeihsgl")).reinterpret_cast::<()>(),
            21_(),
            5_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"hlfrosekpi")).reinterpret_cast::<()>(),
            21_(),
            9_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"atgbkrjdsm")).reinterpret_cast::<()>(),
            21_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"blnrptjgqmaifsdkhoec")).reinterpret_cast::<()>(),
            21_(),
            0_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"ctpmdahebfqjgknloris")).reinterpret_cast::<()>(),
            21_(),
            1_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"apnkeqthrmlbfodiscgj")).reinterpret_cast::<()>(),
            21_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"jdgictpframeoqlsbknh")).reinterpret_cast::<()>(),
            21_(),
            19_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_0(
            _s.as_pointer(),
            (Ptr::<i8>::from_string_literal(b"qprlsfojamgndekthibc")).reinterpret_cast::<()>(),
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        ({ test0_1() });
        ({ test1_2() });
        ({ test2_3() });
        ({ test3_4() });
    }
    {
        let sv1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sv2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv1.find_last_not_of(\"\", 0, 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_last_not_of(\"irkhs\", 0, 5) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_not_of(\"\", 0, 0) == 0");;
        const _: () = assert!(true, "sv2.find_last_not_of(\"gfsrt\", 5, 0) == 4");;
        const _: () = assert!(true, "sv2.find_last_not_of(\"lecar\", 5, 0) == 4");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
