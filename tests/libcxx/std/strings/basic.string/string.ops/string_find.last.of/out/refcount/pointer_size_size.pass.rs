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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(t: Ptr<()>) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_char_ {}
pub fn test_4(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test_5(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test0_6() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"irkhs"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kante"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oknlr"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcdro"),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bnrpe"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oselktgbca"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eqgaplhckj"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjahtcmnlp"),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpqiarojkcdlsgnmfetb"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfkaprhjloqetcsimnbg"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ihqrfebgadntlpmjksoc"),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
            0_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbtqd"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tboim"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"slcer"),
            1_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cbjfs"),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mkqpbtdalg"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kphatlimcd"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pblasqogic"),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bmhineprjcoadgstflqk"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"djkqcmetslnghpbarfoi"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lgokshjtpbemarcdqnfi"),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsoc"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflm"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgkec"),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"romds"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qhjistlgmr"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pedfirsglo"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqcoslgrmk"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dabckmepqj"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqscrjthli"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kfphdcsjqmobliagtren"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rokpefncljibsdhqtagm"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"afionmkphlebtcjqsgrd"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aenmqplidhkofrjbctsg"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"osjmbtcadhiklegrpqnf"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"osmia"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ckonl"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ilcaj"),
            1_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lasiq"),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kfqmr"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"klnitfaobg"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gjhmdlqikp"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"skbgtahqej"),
            1_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjsdgtlpkf"),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjgfmnlkio"),
            1_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbhepotfsjdqigcnamkr"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tebangckmpsrqdlfojhi"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"joflqbdkhtegimscpanr"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adpmcohetfbsrjinlqkg"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iacldqjpfnogbsrhmetk"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otkgb"),
            2_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cqsjl"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dpifl"),
            2_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oapht"),
            2_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cifts"),
            2_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nmsckbgalo"),
            2_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpksqhamle"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpdrchmkji"),
            2_(),
            5_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ijagfkblst"),
            2_(),
            9_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kpocsignjb"),
            2_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pecqtkjsnbdrialgmohf"),
            2_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aiortphfcmkjebgsndql"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sdbkeamglhipojqftrcn"),
            2_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljqncehgmfktroapidbs"),
            2_(),
            19_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtcfodilamkbenjghqps"),
            2_(),
            20_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mabns"),
            4_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdnrp"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"scidp"),
            4_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"agbjl"),
            4_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jfmpr"),
            4_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rbpefghsmj"),
            4_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apsfntdoqc"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ndkjeisgcl"),
            4_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rnfpqatdeo"),
            4_(),
            9_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bntjlqrfik"),
            4_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kcrtmpolnaqejghsfdbi"),
            4_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lobheanpkmqidsrtcfgj"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"athdkljcnreqbgpmisof"),
            4_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkdmjialrscpbhefgont"),
            4_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmasojntqleribkgfchp"),
            4_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"psthd"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rpmjd"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfsmk"),
            5_(),
            2_(),
            unimplemented!(),
        )
    });
}
pub fn test0_7() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"irkhs"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kante"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oknlr"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcdro"),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bnrpe"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oselktgbca"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eqgaplhckj"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjahtcmnlp"),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpqiarojkcdlsgnmfetb"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfkaprhjloqetcsimnbg"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ihqrfebgadntlpmjksoc"),
            0_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
            0_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbtqd"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tboim"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"slcer"),
            1_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cbjfs"),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mkqpbtdalg"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kphatlimcd"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pblasqogic"),
            1_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bmhineprjcoadgstflqk"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"djkqcmetslnghpbarfoi"),
            1_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lgokshjtpbemarcdqnfi"),
            1_(),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            1_(),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsoc"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflm"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgkec"),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"romds"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qhjistlgmr"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pedfirsglo"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqcoslgrmk"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dabckmepqj"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqscrjthli"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kfphdcsjqmobliagtren"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rokpefncljibsdhqtagm"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"afionmkphlebtcjqsgrd"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aenmqplidhkofrjbctsg"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"osjmbtcadhiklegrpqnf"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"osmia"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ckonl"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ilcaj"),
            1_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lasiq"),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kfqmr"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"klnitfaobg"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gjhmdlqikp"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"skbgtahqej"),
            1_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjsdgtlpkf"),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjgfmnlkio"),
            1_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbhepotfsjdqigcnamkr"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tebangckmpsrqdlfojhi"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"joflqbdkhtegimscpanr"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adpmcohetfbsrjinlqkg"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iacldqjpfnogbsrhmetk"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otkgb"),
            2_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cqsjl"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dpifl"),
            2_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oapht"),
            2_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cifts"),
            2_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nmsckbgalo"),
            2_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpksqhamle"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpdrchmkji"),
            2_(),
            5_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ijagfkblst"),
            2_(),
            9_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kpocsignjb"),
            2_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pecqtkjsnbdrialgmohf"),
            2_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aiortphfcmkjebgsndql"),
            2_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sdbkeamglhipojqftrcn"),
            2_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljqncehgmfktroapidbs"),
            2_(),
            19_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtcfodilamkbenjghqps"),
            2_(),
            20_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mabns"),
            4_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdnrp"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"scidp"),
            4_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"agbjl"),
            4_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jfmpr"),
            4_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rbpefghsmj"),
            4_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apsfntdoqc"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ndkjeisgcl"),
            4_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rnfpqatdeo"),
            4_(),
            9_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bntjlqrfik"),
            4_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kcrtmpolnaqejghsfdbi"),
            4_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lobheanpkmqidsrtcfgj"),
            4_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"athdkljcnreqbgpmisof"),
            4_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkdmjialrscpbhefgont"),
            4_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmasojntqleribkgfchp"),
            4_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"psthd"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rpmjd"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfsmk"),
            5_(),
            2_(),
            unimplemented!(),
        )
    });
}
pub fn test1_8() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"skqne"),
            5_(),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kipnf"),
            5_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hmrnqdgifl"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fsmjcdairn"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcdgltbrfj"),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aekfctpirg"),
            5_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ledihrsgpf"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mqcklahsbtirgopefndj"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kmlthaoqgecrnpdbjfis"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sfhbamcdptojlkrenqgi"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pbniofmcedrkhlstgaqj"),
            5_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mongjratcskbhqiepfdl"),
            5_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hrnat"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gsqdt"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bspkd"),
            6_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ohcmb"),
            6_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"heatr"),
            6_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pmblckedfn"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aceqmsrbik"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lmbtdehjrn"),
            6_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"teqmcrlgib"),
            6_(),
            9_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"njolbmspac"),
            6_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pofnhidklamecrbqjgst"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jbhckmtgrqnosafedpli"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dobntpmqklicsahgjerf"),
            6_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpdshainjkbfoemlrgcq"),
            6_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oldpfgeakrnitscbjmqh"),
            6_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rqegt"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dashm"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jqirk"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rckeg"),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jscie"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"efsphndliq"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gdicosleja"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qcpjibosfl"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lrhmefnjcq"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtablcrseo"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apckjsftedbhgomrnilq"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbrgflehjtiadnsokqm"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nsiadegjklhobrmtqcpf"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cpmajdqnolikhgsbretf"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jcflkntmgiqrphdosaeb"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ontrs"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfkna"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ekosa"),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"anqhk"),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jekca"),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikemsjgacf"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arolgsjkhm"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oftkbldhre"),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbkqdoeftl"),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sqcflrgtim"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fmhbkislrjdpanogqcet"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rnioadktqlgpbcjsmhef"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oakgtnldpsefihqmjcbr"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbnaelosidmcjqktfhpr"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"akbripjhlosndcmqgfet"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pijag"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jrckd"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qcloh"),
            5_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thlmp"),
            5_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qidmo"),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lnegpsjqrd"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rjqdablmfs"),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"enkgpbsjaq"),
            5_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kdsgoaijfh"),
            5_(),
            9_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trfqgmckbe"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"igetsracjfkdnpoblhqm"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nqctfaogirshlekbdjpm"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"csehfgomljdqinbartkp"),
            5_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qahoegcmplkfsjbdnitr"),
            5_(),
            19_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dpteiajrqmsognhlfbkc"),
            5_(),
            20_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqbnh"),
            9_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"akmle"),
            9_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iqfkm"),
            9_(),
            2_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqjsr"),
            9_(),
            4_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jplqg"),
            9_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oilnrbcgtj"),
            9_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"morkglpesn"),
            9_(),
            1_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmicerngat"),
            9_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"radgeskbtc"),
            9_(),
            9_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljikprsmqo"),
            9_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trqihkcgsjamfdbolnpe"),
            9_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lqmthbsrekajgnofcipd"),
            9_(),
            1_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtalmedribkgqsopcnfh"),
            9_(),
            10_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"spqfoiclmtagejbndkrh"),
            9_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nmotklspigjrdhcfaebq"),
            9_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpmsd"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qnpor"),
            10_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otdma"),
            10_(),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"efhjg"),
            10_(),
            4_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odpte"),
            10_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bctdgfmolr"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oaklidrbqg"),
            10_(),
            1_(),
            1_(),
        )
    });
}
pub fn test1_9() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"skqne"),
            5_(),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kipnf"),
            5_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hmrnqdgifl"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fsmjcdairn"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcdgltbrfj"),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aekfctpirg"),
            5_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ledihrsgpf"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mqcklahsbtirgopefndj"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kmlthaoqgecrnpdbjfis"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sfhbamcdptojlkrenqgi"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pbniofmcedrkhlstgaqj"),
            5_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mongjratcskbhqiepfdl"),
            5_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hrnat"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gsqdt"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bspkd"),
            6_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ohcmb"),
            6_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"heatr"),
            6_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pmblckedfn"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aceqmsrbik"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lmbtdehjrn"),
            6_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"teqmcrlgib"),
            6_(),
            9_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"njolbmspac"),
            6_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pofnhidklamecrbqjgst"),
            6_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jbhckmtgrqnosafedpli"),
            6_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dobntpmqklicsahgjerf"),
            6_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpdshainjkbfoemlrgcq"),
            6_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oldpfgeakrnitscbjmqh"),
            6_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rqegt"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dashm"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jqirk"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rckeg"),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jscie"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"efsphndliq"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gdicosleja"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qcpjibosfl"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lrhmefnjcq"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtablcrseo"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apckjsftedbhgomrnilq"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbrgflehjtiadnsokqm"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nsiadegjklhobrmtqcpf"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cpmajdqnolikhgsbretf"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jcflkntmgiqrphdosaeb"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ontrs"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfkna"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ekosa"),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"anqhk"),
            1_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jekca"),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikemsjgacf"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arolgsjkhm"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oftkbldhre"),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbkqdoeftl"),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sqcflrgtim"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fmhbkislrjdpanogqcet"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rnioadktqlgpbcjsmhef"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oakgtnldpsefihqmjcbr"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbnaelosidmcjqktfhpr"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"akbripjhlosndcmqgfet"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pijag"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jrckd"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qcloh"),
            5_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thlmp"),
            5_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qidmo"),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lnegpsjqrd"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rjqdablmfs"),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"enkgpbsjaq"),
            5_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kdsgoaijfh"),
            5_(),
            9_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trfqgmckbe"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"igetsracjfkdnpoblhqm"),
            5_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nqctfaogirshlekbdjpm"),
            5_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"csehfgomljdqinbartkp"),
            5_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qahoegcmplkfsjbdnitr"),
            5_(),
            19_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dpteiajrqmsognhlfbkc"),
            5_(),
            20_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqbnh"),
            9_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"akmle"),
            9_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iqfkm"),
            9_(),
            2_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqjsr"),
            9_(),
            4_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jplqg"),
            9_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oilnrbcgtj"),
            9_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"morkglpesn"),
            9_(),
            1_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmicerngat"),
            9_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"radgeskbtc"),
            9_(),
            9_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljikprsmqo"),
            9_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trqihkcgsjamfdbolnpe"),
            9_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lqmthbsrekajgnofcipd"),
            9_(),
            1_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtalmedribkgqsopcnfh"),
            9_(),
            10_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"spqfoiclmtagejbndkrh"),
            9_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nmotklspigjrdhcfaebq"),
            9_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpmsd"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qnpor"),
            10_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otdma"),
            10_(),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"efhjg"),
            10_(),
            4_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odpte"),
            10_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bctdgfmolr"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oaklidrbqg"),
            10_(),
            1_(),
            1_(),
        )
    });
}
pub fn test2_10() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dnjfsagktr"),
            10_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nejaktmiqg"),
            10_(),
            9_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pjqonlebsf"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dshmnbtolcjepgaikfqr"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iogfhpabtjkqlrnemcds"),
            10_(),
            1_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngridfabjsecpqltkmoh"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"athmknplcgofrqejsdib"),
            10_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ldobhmqcafnjtkeisgrp"),
            10_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aocjb"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jbrnk"),
            11_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqedg"),
            11_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nqskp"),
            11_(),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eaqkl"),
            11_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"reaoicljqm"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsftgajqpm"),
            11_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rlpfogmits"),
            11_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"shkncmiaqj"),
            11_(),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fpnatrhqgs"),
            11_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sjclemqhnpdbgikarfot"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otcmedjikgsfnqbrhpla"),
            11_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bonsaefdqiprkhlgtjcm"),
            11_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"egpscmahijlfnkrodqtb"),
            11_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kmqbfepjthgilscrndoa"),
            11_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prboq"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fjcqh"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fmosa"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qdbok"),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"amslg"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smpltjneqb"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"flitskrnge"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pgqihmlbef"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cfpdqjtgsb"),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htpsiaflom"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kpjfiaceghsrdtlbnomq"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qhtbomidljgafneksprc"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nhtjobkcefldimpsaqgr"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prabcjfqnoeskilmtgdh"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtrgmchilkasqoebfpjn"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sqome"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smfte"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ciboh"),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"haois"),
            1_(),
            4_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abfki"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"frdkocntmq"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oasbpedlnr"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kltqmhgand"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gdtfjchpmr"),
            1_(),
            9_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ponmcqblet"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgphqdnofeiklatbcmjr"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljqprsmigtfoneadckbh"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ligeojhafnkmrcsqtbdp"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsimqfnjarbopedkhcgt"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abedmfjlghniorcqptks"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqtoa"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cahif"),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kehis"),
            10_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kdlmh"),
            10_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"paeql"),
            10_(),
            5_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aghoqiefnb"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jrbqaikpdo"),
            10_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smjonaeqcl"),
            10_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eqbdrkcfah"),
            10_(),
            9_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kapmsienhf"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jpqotrlenfcsbhkaimdg"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jlbmhnfgtcqprikeados"),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"stgbhfmdaljnpqoicker"),
            10_(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oihcetflbjagdsrkmqpn"),
            10_(),
            19_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adtclebmnpjsrqfkigoh"),
            10_(),
            20_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"beafg"),
            19_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iclat"),
            19_(),
            1_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rkhnf"),
            19_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"clshq"),
            19_(),
            4_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtcoj"),
            19_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rqosnjmfth"),
            19_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"siatdfqglh"),
            19_(),
            1_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mrlshtpgjq"),
            19_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adlcskgqjt"),
            19_(),
            9_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"drshcjknaf"),
            19_(),
            10_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"etsaqroinghpkjdlfcbm"),
            19_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgepdnkqliambtrocfhj"),
            19_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nlmcjaqgbsortfdihkpe"),
            19_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"racfnpmosldibqkghjet"),
            19_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fjhdsctkqeiolagrnmbp"),
            19_(),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ejanp"),
            20_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odife"),
            20_(),
            1_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"okaqd"),
            20_(),
            2_(),
            12_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lcdbi"),
            20_(),
            4_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fsqbj"),
            20_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bigdomnplq"),
            20_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apiblotgcd"),
            20_(),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acfhdenops"),
            20_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jopdeamcrk"),
            20_(),
            9_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trqncbkgmh"),
            20_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tomglrkencbsfjqpihda"),
            20_(),
            0_(),
            unimplemented!(),
        )
    });
}
pub fn test2_11() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dnjfsagktr"),
            10_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nejaktmiqg"),
            10_(),
            9_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pjqonlebsf"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dshmnbtolcjepgaikfqr"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iogfhpabtjkqlrnemcds"),
            10_(),
            1_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngridfabjsecpqltkmoh"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"athmknplcgofrqejsdib"),
            10_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ldobhmqcafnjtkeisgrp"),
            10_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aocjb"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jbrnk"),
            11_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqedg"),
            11_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nqskp"),
            11_(),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eaqkl"),
            11_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"reaoicljqm"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsftgajqpm"),
            11_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rlpfogmits"),
            11_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"shkncmiaqj"),
            11_(),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fpnatrhqgs"),
            11_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sjclemqhnpdbgikarfot"),
            11_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otcmedjikgsfnqbrhpla"),
            11_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bonsaefdqiprkhlgtjcm"),
            11_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"egpscmahijlfnkrodqtb"),
            11_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kmqbfepjthgilscrndoa"),
            11_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prboq"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fjcqh"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fmosa"),
            0_(),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qdbok"),
            0_(),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"amslg"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smpltjneqb"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"flitskrnge"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pgqihmlbef"),
            0_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cfpdqjtgsb"),
            0_(),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htpsiaflom"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kpjfiaceghsrdtlbnomq"),
            0_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qhtbomidljgafneksprc"),
            0_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nhtjobkcefldimpsaqgr"),
            0_(),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prabcjfqnoeskilmtgdh"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtrgmchilkasqoebfpjn"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sqome"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smfte"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ciboh"),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"haois"),
            1_(),
            4_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abfki"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"frdkocntmq"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oasbpedlnr"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kltqmhgand"),
            1_(),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gdtfjchpmr"),
            1_(),
            9_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ponmcqblet"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgphqdnofeiklatbcmjr"),
            1_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljqprsmigtfoneadckbh"),
            1_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ligeojhafnkmrcsqtbdp"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsimqfnjarbopedkhcgt"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abedmfjlghniorcqptks"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqtoa"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cahif"),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kehis"),
            10_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kdlmh"),
            10_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"paeql"),
            10_(),
            5_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aghoqiefnb"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jrbqaikpdo"),
            10_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smjonaeqcl"),
            10_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eqbdrkcfah"),
            10_(),
            9_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kapmsienhf"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jpqotrlenfcsbhkaimdg"),
            10_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jlbmhnfgtcqprikeados"),
            10_(),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"stgbhfmdaljnpqoicker"),
            10_(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oihcetflbjagdsrkmqpn"),
            10_(),
            19_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adtclebmnpjsrqfkigoh"),
            10_(),
            20_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"beafg"),
            19_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iclat"),
            19_(),
            1_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rkhnf"),
            19_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"clshq"),
            19_(),
            4_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtcoj"),
            19_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rqosnjmfth"),
            19_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"siatdfqglh"),
            19_(),
            1_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mrlshtpgjq"),
            19_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adlcskgqjt"),
            19_(),
            9_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"drshcjknaf"),
            19_(),
            10_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"etsaqroinghpkjdlfcbm"),
            19_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgepdnkqliambtrocfhj"),
            19_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nlmcjaqgbsortfdihkpe"),
            19_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"racfnpmosldibqkghjet"),
            19_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fjhdsctkqeiolagrnmbp"),
            19_(),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ejanp"),
            20_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odife"),
            20_(),
            1_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"okaqd"),
            20_(),
            2_(),
            12_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lcdbi"),
            20_(),
            4_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fsqbj"),
            20_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bigdomnplq"),
            20_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apiblotgcd"),
            20_(),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acfhdenops"),
            20_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jopdeamcrk"),
            20_(),
            9_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trqncbkgmh"),
            20_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tomglrkencbsfjqpihda"),
            20_(),
            0_(),
            unimplemented!(),
        )
    });
}
pub fn test3_12() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbkhdnpoietfcmrslajq"),
            20_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtfnmbsglkjaichoqedp"),
            20_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ohkmdpfqbsacrtjnlgei"),
            20_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dlbrteoisgphmkncajfq"),
            20_(),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sjrlo"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qjpor"),
            21_(),
            1_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odhfn"),
            21_(),
            2_(),
            13_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qtfin"),
            21_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpqfo"),
            21_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fabmertkos"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"brqtgkmaej"),
            21_(),
            1_(),
            14_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nfrdeihsgl"),
            21_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hlfrosekpi"),
            21_(),
            9_(),
            14_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"atgbkrjdsm"),
            21_(),
            10_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blnrptjgqmaifsdkhoec"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ctpmdahebfqjgknloris"),
            21_(),
            1_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apnkeqthrmlbfodiscgj"),
            21_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jdgictpframeoqlsbknh"),
            21_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qprlsfojamgndekthibc"),
            21_(),
            20_(),
            19_(),
        )
    });
}
pub fn test3_13() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbkhdnpoietfcmrslajq"),
            20_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtfnmbsglkjaichoqedp"),
            20_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ohkmdpfqbsacrtjnlgei"),
            20_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dlbrteoisgphmkncajfq"),
            20_(),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sjrlo"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qjpor"),
            21_(),
            1_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odhfn"),
            21_(),
            2_(),
            13_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qtfin"),
            21_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpqfo"),
            21_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fabmertkos"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"brqtgkmaej"),
            21_(),
            1_(),
            14_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nfrdeihsgl"),
            21_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hlfrosekpi"),
            21_(),
            9_(),
            14_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"atgbkrjdsm"),
            21_(),
            10_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blnrptjgqmaifsdkhoec"),
            21_(),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ctpmdahebfqjgknloris"),
            21_(),
            1_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apnkeqthrmlbfodiscgj"),
            21_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jdgictpframeoqlsbknh"),
            21_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qprlsfojamgndekthibc"),
            21_(),
            20_(),
            19_(),
        )
    });
}
pub fn test_string_14() {
    ({ test0_6() });
    ({ test1_8() });
    ({ test2_10() });
    ({ test3_12() });
}
pub fn test_string_15() {
    ({ test0_7() });
    ({ test1_9() });
    ({ test2_11() });
    ({ test3_13() });
}
pub fn test_16() -> () {
    ({ test_string_14() });
    ({ test_string_15() });
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
    ({ test_16() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait min_allocator_char_Impl {
    fn allocate(&self, n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_char__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_char_Impl for Ptr<min_allocator_char_> {
    fn allocate(&self, mut n: ()) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_pointer_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, n: ()) -> Ptr<()>;
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_char__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
