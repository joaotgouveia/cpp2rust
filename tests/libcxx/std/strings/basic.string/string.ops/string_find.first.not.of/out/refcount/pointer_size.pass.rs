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
pub fn test_4(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test_5(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test_6(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test_7(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    unimplemented!();
    if (x != unimplemented!()) {
        unimplemented!();
    }
}
pub fn test0_8() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            1_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfkap"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ihqrfebgad"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikcrq"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmajblfhsg"),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oqftjhdmkgsblacenirp"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oebqi"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kojhpmbsfe"),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acbsjqogpltdkhinfrem"),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aobjd"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbahntsje"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fhepcrntkoagbmldqijs"),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kocgb"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fbslrjiqkm"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jeidpcmalhfnqbgtrsko"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qngpd"),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rodhqklgmb"),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thdjgafrlbkoiqcspmne"),
            6_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ashjd"),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mgojkldsqh"),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"imqnaghkfrdtlopbjesc"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nadkh"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ofdrqmkebl"),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdfjqgatlksriohemnpc"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prqgn"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pejafmnokr"),
            5_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odnqkgijrhabfmcestlp"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtjpa"),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ktsrmnqagd"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtdhgcisbnmoaqkfpjle"),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dplqa"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dkacjoptns"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqfimtrgnbekpdcsjalo"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lofbc"),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"astoegbfpn"),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pdgreqomsncafklhtibj"),
            11_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbtqd"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tboimldpjh"),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"slcerthdaiqjfnobgkpm"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mkqpbtdalgniorhfescj"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pblas"),
            10_(),
            11_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            10_(),
            13_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"djkqc"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lgokshjtpb"),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nocfa"),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bgtajmiedc"),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsckfnqgdahejiopbtmr"),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsocbhjtm"),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflmdnoicjgkberhqsa"),
            21_(),
            unimplemented!(),
        )
    });
}
pub fn test0_9() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            1_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfkap"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ihqrfebgad"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikcrq"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmajblfhsg"),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oqftjhdmkgsblacenirp"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oebqi"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kojhpmbsfe"),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acbsjqogpltdkhinfrem"),
            2_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aobjd"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbahntsje"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fhepcrntkoagbmldqijs"),
            4_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kocgb"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fbslrjiqkm"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jeidpcmalhfnqbgtrsko"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qngpd"),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rodhqklgmb"),
            6_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thdjgafrlbkoiqcspmne"),
            6_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ashjd"),
            0_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mgojkldsqh"),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"imqnaghkfrdtlopbjesc"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nadkh"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ofdrqmkebl"),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdfjqgatlksriohemnpc"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prqgn"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pejafmnokr"),
            5_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odnqkgijrhabfmcestlp"),
            5_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtjpa"),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ktsrmnqagd"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtdhgcisbnmoaqkfpjle"),
            9_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dplqa"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dkacjoptns"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqfimtrgnbekpdcsjalo"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lofbc"),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"astoegbfpn"),
            11_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pdgreqomsncafklhtibj"),
            11_(),
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
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbtqd"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tboimldpjh"),
            0_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"slcerthdaiqjfnobgkpm"),
            0_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mkqpbtdalgniorhfescj"),
            1_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pblas"),
            10_(),
            11_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            10_(),
            13_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            10_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"djkqc"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lgokshjtpb"),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            19_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nocfa"),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bgtajmiedc"),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsckfnqgdahejiopbtmr"),
            20_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsocbhjtm"),
            21_(),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_5(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflmdnoicjgkberhqsa"),
            21_(),
            unimplemented!(),
        )
    });
}
pub fn test1_10() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"irkhs"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kantesmpgj"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oknlrstdpiqmjbaghcfe"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bnrpe"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oselktgbcapndfjihrmq"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            unimplemented!(),
        )
    });
}
pub fn test1_11() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"irkhs"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kantesmpgj"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oknlrstdpiqmjbaghcfe"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bnrpe"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oselktgbcapndfjihrmq"),
            unimplemented!(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(_s.as_pointer(), Ptr::<()>::from_string_literal(b""), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_7(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            unimplemented!(),
        )
    });
}
pub fn test_string_12() {
    ({ test0_8() });
    ({ test1_10() });
}
pub fn test_string_13() {
    ({ test0_9() });
    ({ test1_11() });
}
pub fn test_14() -> () {
    ({ test_string_12() });
    ({ test_string_13() });
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
    ({ test_14() });
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
