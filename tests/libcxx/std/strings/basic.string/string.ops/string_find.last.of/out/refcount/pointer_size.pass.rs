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
    if (x != npos_5.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_6(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_7.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_8(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_5.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_9(s: Ptr<()>, mut str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_7.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test0_10() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fodgq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qanej") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfkap"),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"clbao") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ihqrfebgad"),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mekdn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"srdfq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oemth") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikcrq"),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cdaih") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmajblfhsg"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qohtk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oqftjhdmkgsblacenirp"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cshmd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lhcdo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oebqi"),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnsoh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kojhpmbsfe"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pkrof") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acbsjqogpltdkhinfrem"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fmtsp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"khbpm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aobjd"),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pbsji") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbahntsje"),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mprdj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fhepcrntkoagbmldqijs"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqmpa") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"omigs") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kocgb"),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"onmje") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fbslrjiqkm"),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oqmrj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jeidpcmalhfnqbgtrsko"),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"schfa") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"igdsc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qngpd"),
            6_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"brqgo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rodhqklgmb"),
            6_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tnrph") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thdjgafrlbkoiqcspmne"),
            6_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hcjitbfapl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"daiprenocl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ashjd"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"litpcfdghe") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mgojkldsqh"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aidjksrolc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"imqnaghkfrdtlopbjesc"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qpghtfbaji") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gfshlcmdjr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nadkh"),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nkodajteqp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ofdrqmkebl"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gbmetiprqd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdfjqgatlksriohemnpc"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"crnklpmegd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jsbtafedoc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prqgn"),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnmodrtkeb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pejafmnokr"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cpebqsfmnj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odnqkgijrhabfmcestlp"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lmofqdhpki") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnefkqimca") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtjpa"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"drtasbgmfp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ktsrmnqagd"),
            9_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lsaijeqhtr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtdhgcisbnmoaqkfpjle"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"elgofjmbrq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mjqdgalkpc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dplqa"),
            10_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kthqnfcerm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dkacjoptns"),
            10_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dfsjhanorc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqfimtrgnbekpdcsjalo"),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqsgalomhb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"akiteljmoh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lofbc"),
            11_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hlbdfreqjo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"astoegbfpn"),
            11_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"taqobhlerg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pdgreqomsncafklhtibj"),
            11_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"snafbdlghrjkpqtoceim") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aemtbrgcklhndjisfpoq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbtqd"),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pnracgfkjdiholtbqsem") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tboimldpjh"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dicfltehbsgrmojnpkaq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"slcerthdaiqjfnobgkpm"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jlnkraeodhcspfgbqitm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lhosrngtmfjikbqpcade") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rbtaqjhgkneisldpmfoc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gpifsqlrdkbonjtmheca") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mkqpbtdalgniorhfescj"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hdpkobnsalmcfijregtq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jtlshdgqaiprkbcoenfm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pblas"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fkdrbqltsgmcoiphneaj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"crsplifgtqedjohnabmk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"niptglfbosehkamrdqcj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"copqdhstbingamjfkler") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"djkqc"),
            19_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mrtaefilpdsgocnhqbjk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lgokshjtpb"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kojatdhlcmigpbfrqnes") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eaintpchlqsbdgrkjofm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gjnhidfsepkrtaqbmclo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nocfa"),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"spocfaktqdbiejlhngmr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bgtajmiedc"),
            20_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rphmlekgfscndtaobiqj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsckfnqgdahejiopbtmr"),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"liatsqdoegkmfcnbhrpj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"binjagtfldkrspcomqeh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            21_(),
            12_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"latkmisecnorjbfhqpdg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsocbhjtm"),
            21_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lecfratdjkhnsmqpoigb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflmdnoicjgkberhqsa"),
            21_(),
            19_(),
        )
    });
}
pub fn test0_11() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fodgq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qanej") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfkap"),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"clbao") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ihqrfebgad"),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mekdn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"srdfq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oemth") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikcrq"),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cdaih") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmajblfhsg"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qohtk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oqftjhdmkgsblacenirp"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cshmd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lhcdo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oebqi"),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnsoh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kojhpmbsfe"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pkrof") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acbsjqogpltdkhinfrem"),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fmtsp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"khbpm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aobjd"),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pbsji") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbahntsje"),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mprdj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fhepcrntkoagbmldqijs"),
            4_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqmpa") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"omigs") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kocgb"),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"onmje") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fbslrjiqkm"),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oqmrj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jeidpcmalhfnqbgtrsko"),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"schfa") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"igdsc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qngpd"),
            6_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"brqgo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rodhqklgmb"),
            6_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tnrph") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thdjgafrlbkoiqcspmne"),
            6_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hcjitbfapl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"daiprenocl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ashjd"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"litpcfdghe") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mgojkldsqh"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aidjksrolc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"imqnaghkfrdtlopbjesc"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qpghtfbaji") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gfshlcmdjr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nadkh"),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nkodajteqp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ofdrqmkebl"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gbmetiprqd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdfjqgatlksriohemnpc"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"crnklpmegd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jsbtafedoc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prqgn"),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnmodrtkeb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pejafmnokr"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cpebqsfmnj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odnqkgijrhabfmcestlp"),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lmofqdhpki") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnefkqimca") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtjpa"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"drtasbgmfp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ktsrmnqagd"),
            9_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lsaijeqhtr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtdhgcisbnmoaqkfpjle"),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"elgofjmbrq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mjqdgalkpc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dplqa"),
            10_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kthqnfcerm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dkacjoptns"),
            10_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dfsjhanorc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqfimtrgnbekpdcsjalo"),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqsgalomhb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"akiteljmoh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lofbc"),
            11_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hlbdfreqjo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"astoegbfpn"),
            11_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"taqobhlerg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pdgreqomsncafklhtibj"),
            11_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"snafbdlghrjkpqtoceim") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aemtbrgcklhndjisfpoq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbtqd"),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pnracgfkjdiholtbqsem") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tboimldpjh"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dicfltehbsgrmojnpkaq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"slcerthdaiqjfnobgkpm"),
            0_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jlnkraeodhcspfgbqitm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lhosrngtmfjikbqpcade") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rbtaqjhgkneisldpmfoc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gpifsqlrdkbonjtmheca") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mkqpbtdalgniorhfescj"),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hdpkobnsalmcfijregtq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jtlshdgqaiprkbcoenfm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pblas"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fkdrbqltsgmcoiphneaj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"crsplifgtqedjohnabmk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"niptglfbosehkamrdqcj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"copqdhstbingamjfkler") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"djkqc"),
            19_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mrtaefilpdsgocnhqbjk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lgokshjtpb"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kojatdhlcmigpbfrqnes") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eaintpchlqsbdgrkjofm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gjnhidfsepkrtaqbmclo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nocfa"),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"spocfaktqdbiejlhngmr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bgtajmiedc"),
            20_(),
            18_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rphmlekgfscndtaobiqj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsckfnqgdahejiopbtmr"),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"liatsqdoegkmfcnbhrpj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"binjagtfldkrspcomqeh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            21_(),
            12_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"latkmisecnorjbfhqpdg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsocbhjtm"),
            21_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lecfratdjkhnsmqpoigb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflmdnoicjgkberhqsa"),
            21_(),
            19_(),
        )
    });
}
pub fn test1_12() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nhmko") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lahfb") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"irkhs"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gmfhd") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kantesmpgj"),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"odaft") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oknlrstdpiqmjbaghcfe"),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eolhfgpjqk") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nbatdlmekr") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bnrpe"),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jdmciepkaq") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hkbgspoflt") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oselktgbcapndfjihrmq"),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gprdcokbnjhlsfmtieqa") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qjghlnftcaismkropdeb") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pnalfrdtkqcmojiesbhg") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pniotcfrhqsmgdkjbael") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            19_(),
        )
    });
}
pub fn test1_13() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"laenf"),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqlnkmbdjo"),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti"),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nhmko") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lahfb") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"irkhs"),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gmfhd") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kantesmpgj"),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"odaft") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oknlrstdpiqmjbaghcfe"),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eolhfgpjqk") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nbatdlmekr") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bnrpe"),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jdmciepkaq") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hkbgspoflt") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oselktgbcapndfjihrmq"),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gprdcokbnjhlsfmtieqa") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qjghlnftcaismkropdeb") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjaht"),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pnalfrdtkqcmojiesbhg") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hjlcmgpket"),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pniotcfrhqsmgdkjbael") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn"),
            19_(),
        )
    });
}
pub fn test_string_14() {
    ({ test0_10() });
    ({ test1_12() });
}
pub fn test_string_15() {
    ({ test0_11() });
    ({ test1_13() });
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
