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
    if (x != npos_5.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_6(s: Ptr<()>, mut str: Ptr<()>, mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_7.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test0_8() {
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
            Ptr::<()>::from_string_literal(b"irkhs"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"kante"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"oknlr"),
            0_(),
            2_(),
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
            Ptr::<()>::from_string_literal(b"pcdro"),
            0_(),
            4_(),
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
            Ptr::<()>::from_string_literal(b"bnrpe"),
            0_(),
            5_(),
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
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"oselktgbca"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"eqgaplhckj"),
            0_(),
            5_(),
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
            Ptr::<()>::from_string_literal(b"bjahtcmnlp"),
            0_(),
            9_(),
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
            0_(),
            10_(),
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
            0_(),
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
            Ptr::<()>::from_string_literal(b"hpqiarojkcdlsgnmfetb"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"dfkaprhjloqetcsimnbg"),
            0_(),
            10_(),
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
            Ptr::<()>::from_string_literal(b"ihqrfebgadntlpmjksoc"),
            0_(),
            19_(),
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
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
            0_(),
            20_(),
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
            Ptr::<()>::from_string_literal(b"lbtqd"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"tboim"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"slcer"),
            1_(),
            2_(),
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
            Ptr::<()>::from_string_literal(b"cbjfs"),
            1_(),
            4_(),
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
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            5_(),
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
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"mkqpbtdalg"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"kphatlimcd"),
            1_(),
            5_(),
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
            Ptr::<()>::from_string_literal(b"pblasqogic"),
            1_(),
            9_(),
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
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            1_(),
            10_(),
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
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"bmhineprjcoadgstflqk"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"djkqcmetslnghpbarfoi"),
            1_(),
            10_(),
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
            Ptr::<()>::from_string_literal(b"lgokshjtpbemarcdqnfi"),
            1_(),
            19_(),
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
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            1_(),
            20_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eaint") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"binja") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"latkm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsoc"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lecfr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflm"),
            0_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqkst") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgkec"),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cdafr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"romds"),
            0_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"prbhe") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qhjistlgmr"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lbisk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pedfirsglo"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hrlpd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqcoslgrmk"),
            0_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ehmja") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dabckmepqj"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mhqgd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqscrjthli"),
            0_(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tgklq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kfphdcsjqmobliagtren"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bocjs") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rokpefncljibsdhqtagm"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"grbsd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"afionmkphlebtcjqsgrd"),
            0_(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ofjqr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aenmqplidhkofrjbctsg"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"btlfi") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"osjmbtcadhiklegrpqnf"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"clrgb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tjmek") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"osmia"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bgstp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ckonl"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hstrk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ilcaj"),
            1_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kmspj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lasiq"),
            1_(),
            4_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tjboh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kfqmr"),
            1_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ilbcj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"klnitfaobg"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jkngf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gjhmdlqikp"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gfcql") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"skbgtahqej"),
            1_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dqtlg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjsdgtlpkf"),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bthpg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjgfmnlkio"),
            1_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dgsnq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbhepotfsjdqigcnamkr"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rmfhp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tebangckmpsrqdlfojhi"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jfdam") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"joflqbdkhtegimscpanr"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"edapb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adpmcohetfbsrjinlqkg"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"brfsm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iacldqjpfnogbsrhmetk"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ndrhl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mrecp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otkgb"),
            2_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qlasf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cqsjl"),
            2_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"smaqd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dpifl"),
            2_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hjeni") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oapht"),
            2_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ocmfj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cifts"),
            2_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hmftq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nmsckbgalo"),
            2_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fklad") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpksqhamle"),
            2_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dirnm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpdrchmkji"),
            2_(),
            5_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hrgdc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ijagfkblst"),
            2_(),
            9_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ifakg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kpocsignjb"),
            2_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ebrgd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pecqtkjsnbdrialgmohf"),
            2_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rcjml") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aiortphfcmkjebgsndql"),
            2_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"peqmt") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sdbkeamglhipojqftrcn"),
            2_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"frehn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljqncehgmfktroapidbs"),
            2_(),
            19_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tqolf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtcfodilamkbenjghqps"),
            2_(),
            20_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cjgao") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kjplq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mabns"),
            4_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"herni") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdnrp"),
            4_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tadrb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"scidp"),
            4_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pkfeo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"agbjl"),
            4_(),
            4_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hoser") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jfmpr"),
            4_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kgrsp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rbpefghsmj"),
            4_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pgejb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apsfntdoqc"),
            4_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"thlnq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ndkjeisgcl"),
            4_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nbmit") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rnfpqatdeo"),
            4_(),
            9_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jgmib") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bntjlqrfik"),
            4_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ncrfj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kcrtmpolnaqejghsfdbi"),
            4_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ncsik") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lobheanpkmqidsrtcfgj"),
            4_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sgbfh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"athdkljcnreqbgpmisof"),
            4_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dktbn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkdmjialrscpbhefgont"),
            4_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fthqm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmasojntqleribkgfchp"),
            4_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klopi") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dajhn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"psthd"),
            5_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jbgno") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rpmjd"),
            5_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hkjae") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfsmk"),
            5_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
}
pub fn test0_9() {
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
            Ptr::<()>::from_string_literal(b"irkhs"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"kante"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"oknlr"),
            0_(),
            2_(),
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
            Ptr::<()>::from_string_literal(b"pcdro"),
            0_(),
            4_(),
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
            Ptr::<()>::from_string_literal(b"bnrpe"),
            0_(),
            5_(),
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
            Ptr::<()>::from_string_literal(b"jtdaefblso"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"oselktgbca"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"eqgaplhckj"),
            0_(),
            5_(),
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
            Ptr::<()>::from_string_literal(b"bjahtcmnlp"),
            0_(),
            9_(),
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
            0_(),
            10_(),
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
            0_(),
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
            Ptr::<()>::from_string_literal(b"hpqiarojkcdlsgnmfetb"),
            0_(),
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
            Ptr::<()>::from_string_literal(b"dfkaprhjloqetcsimnbg"),
            0_(),
            10_(),
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
            Ptr::<()>::from_string_literal(b"ihqrfebgadntlpmjksoc"),
            0_(),
            19_(),
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
            Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo"),
            0_(),
            20_(),
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
            Ptr::<()>::from_string_literal(b"lbtqd"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"tboim"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"slcer"),
            1_(),
            2_(),
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
            Ptr::<()>::from_string_literal(b"cbjfs"),
            1_(),
            4_(),
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
            Ptr::<()>::from_string_literal(b"aqibs"),
            1_(),
            5_(),
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
            Ptr::<()>::from_string_literal(b"gtfblmqinc"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"mkqpbtdalg"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"kphatlimcd"),
            1_(),
            5_(),
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
            Ptr::<()>::from_string_literal(b"pblasqogic"),
            1_(),
            9_(),
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
            Ptr::<()>::from_string_literal(b"arosdhcfme"),
            1_(),
            10_(),
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
            Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"bmhineprjcoadgstflqk"),
            1_(),
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
            Ptr::<()>::from_string_literal(b"djkqcmetslnghpbarfoi"),
            1_(),
            10_(),
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
            Ptr::<()>::from_string_literal(b"lgokshjtpbemarcdqnfi"),
            1_(),
            19_(),
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
            Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado"),
            1_(),
            20_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eaint") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"binja") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gfsrt"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"latkm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfsoc"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lecfr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpflm"),
            0_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqkst") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgkec"),
            0_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cdafr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"romds"),
            0_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"prbhe") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qhjistlgmr"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lbisk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pedfirsglo"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hrlpd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aqcoslgrmk"),
            0_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ehmja") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dabckmepqj"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mhqgd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pqscrjthli"),
            0_(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tgklq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kfphdcsjqmobliagtren"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bocjs") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rokpefncljibsdhqtagm"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"grbsd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"afionmkphlebtcjqsgrd"),
            0_(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ofjqr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aenmqplidhkofrjbctsg"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"btlfi") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"osjmbtcadhiklegrpqnf"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"clrgb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tjmek") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"osmia"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bgstp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ckonl"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hstrk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ilcaj"),
            1_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kmspj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lasiq"),
            1_(),
            4_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tjboh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kfqmr"),
            1_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ilbcj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"klnitfaobg"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jkngf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gjhmdlqikp"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gfcql") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"skbgtahqej"),
            1_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dqtlg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjsdgtlpkf"),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bthpg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bjgfmnlkio"),
            1_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dgsnq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lbhepotfsjdqigcnamkr"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rmfhp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tebangckmpsrqdlfojhi"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jfdam") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"joflqbdkhtegimscpanr"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"edapb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adpmcohetfbsrjinlqkg"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"brfsm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iacldqjpfnogbsrhmetk"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ndrhl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            2_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mrecp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otkgb"),
            2_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qlasf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cqsjl"),
            2_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"smaqd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dpifl"),
            2_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hjeni") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oapht"),
            2_(),
            4_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ocmfj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cifts"),
            2_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hmftq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nmsckbgalo"),
            2_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fklad") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpksqhamle"),
            2_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dirnm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpdrchmkji"),
            2_(),
            5_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hrgdc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ijagfkblst"),
            2_(),
            9_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ifakg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kpocsignjb"),
            2_(),
            10_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ebrgd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pecqtkjsnbdrialgmohf"),
            2_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rcjml") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aiortphfcmkjebgsndql"),
            2_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"peqmt") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sdbkeamglhipojqftrcn"),
            2_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"frehn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljqncehgmfktroapidbs"),
            2_(),
            19_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tqolf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtcfodilamkbenjghqps"),
            2_(),
            20_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cjgao") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            4_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kjplq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mabns"),
            4_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"herni") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bdnrp"),
            4_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tadrb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"scidp"),
            4_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pkfeo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"agbjl"),
            4_(),
            4_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hoser") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jfmpr"),
            4_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kgrsp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rbpefghsmj"),
            4_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pgejb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apsfntdoqc"),
            4_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"thlnq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ndkjeisgcl"),
            4_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nbmit") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rnfpqatdeo"),
            4_(),
            9_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jgmib") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bntjlqrfik"),
            4_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ncrfj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kcrtmpolnaqejghsfdbi"),
            4_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ncsik") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lobheanpkmqidsrtcfgj"),
            4_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sgbfh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"athdkljcnreqbgpmisof"),
            4_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dktbn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qkdmjialrscpbhefgont"),
            4_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fthqm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmasojntqleribkgfchp"),
            4_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klopi") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dajhn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"psthd"),
            5_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jbgno") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rpmjd"),
            5_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hkjae") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dfsmk"),
            5_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
}
pub fn test1_10() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gbhqo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"skqne"),
            5_(),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ktdor") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kipnf"),
            5_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ldprn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hmrnqdgifl"),
            5_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"egmjk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fsmjcdairn"),
            5_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"armql") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcdgltbrfj"),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cdhjo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aekfctpirg"),
            5_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jcons") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ledihrsgpf"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cbrkp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mqcklahsbtirgopefndj"),
            5_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fhgna") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kmlthaoqgecrnpdbjfis"),
            5_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ejfcd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sfhbamcdptojlkrenqgi"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kqjhe") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pbniofmcedrkhlstgaqj"),
            5_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pbdjl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mongjratcskbhqiepfdl"),
            5_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gajqn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"stedk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hrnat"),
            6_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tjkaf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gsqdt"),
            6_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dthpe") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bspkd"),
            6_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klhde") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ohcmb"),
            6_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bhlki") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"heatr"),
            6_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lqmoh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pmblckedfn"),
            6_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mtqin") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aceqmsrbik"),
            6_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dpqbr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lmbtdehjrn"),
            6_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kdhmo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"teqmcrlgib"),
            6_(),
            9_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jblqp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"njolbmspac"),
            6_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qmjgl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pofnhidklamecrbqjgst"),
            6_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rothp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jbhckmtgrqnosafedpli"),
            6_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ghknq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dobntpmqklicsahgjerf"),
            6_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eopfi") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpdshainjkbfoemlrgcq"),
            6_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dsnmg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oldpfgeakrnitscbjmqh"),
            6_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jnkrfhotgl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dltjfngbko") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rqegt"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bmjlpkiqde") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dashm"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"skrflobnqm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jqirk"),
            0_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jkpldtshrm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rckeg"),
            0_(),
            4_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ghasdbnjqo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jscie"),
            0_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"igrkhpbqjt") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"efsphndliq"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ikthdgcamf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gdicosleja"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pcofgeniam") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qcpjibosfl"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rlfjgesqhc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lrhmefnjcq"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"itphbqsker") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtablcrseo"),
            0_(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"skjafcirqm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apckjsftedbhgomrnilq"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tcqomarsfd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbrgflehjtiadnsokqm"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rocfeldqpk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nsiadegjklhobrmtqcpf"),
            0_(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cfpegndlkt") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cpmajdqnolikhgsbretf"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fqbtnkeasj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jcflkntmgiqrphdosaeb"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"shbcqnmoar") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bdoshlmfin") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ontrs"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"khfrebnsgq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfkna"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"getcrsaoji") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ekosa"),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fjiknedcpq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"anqhk"),
            1_(),
            4_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tkejgnafrm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jekca"),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jnakolqrde") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikemsjgacf"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lcjptsmgbe") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arolgsjkhm"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"itfsmcjorl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oftkbldhre"),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"omchkfrjea") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbkqdoeftl"),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cigfqkated") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sqcflrgtim"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tscenjikml") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fmhbkislrjdpanogqcet"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qcpaemsinf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rnioadktqlgpbcjsmhef"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gltkojeipd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oakgtnldpsefihqmjcbr"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qistfrgnmp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbnaelosidmcjqktfhpr"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bdnpfcqaem") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"akbripjhlosndcmqgfet"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ectnhskflp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fgtianblpq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pijag"),
            5_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mfeqklirnh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jrckd"),
            5_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"astedncjhk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qcloh"),
            5_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fhlqgcajbr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thlmp"),
            5_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"epfhocmdng") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qidmo"),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"apcnsibger") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lnegpsjqrd"),
            5_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aqkocrbign") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rjqdablmfs"),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ijsmdtqgce") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"enkgpbsjaq"),
            5_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"clobgsrken") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kdsgoaijfh"),
            5_(),
            9_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jbhcfposld") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trfqgmckbe"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oqnpblhide") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"igetsracjfkdnpoblhqm"),
            5_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lroeasctif") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nqctfaogirshlekbdjpm"),
            5_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bpjlgmiedh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"csehfgomljdqinbartkp"),
            5_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pamkeoidrj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qahoegcmplkfsjbdnitr"),
            5_(),
            19_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"espogqbthk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dpteiajrqmsognhlfbkc"),
            5_(),
            20_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"shoiedtcjb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ebcinjgads") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqbnh"),
            9_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dqmregkcfl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"akmle"),
            9_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ngcrieqajf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iqfkm"),
            9_(),
            2_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qosmilgnjb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqjsr"),
            9_(),
            4_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ikabsjtdfl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jplqg"),
            9_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ersmicafdh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oilnrbcgtj"),
            9_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fdnplotmgh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"morkglpesn"),
            9_(),
            1_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fdbicojerm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmicerngat"),
            9_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mbtafndjcq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"radgeskbtc"),
            9_(),
            9_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mlenkpfdtc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljikprsmqo"),
            9_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ahlcifdqgs") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trqihkcgsjamfdbolnpe"),
            9_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bgjemaltks") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lqmthbsrekajgnofcipd"),
            9_(),
            1_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pdhslbqrfc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtalmedribkgqsopcnfh"),
            9_(),
            10_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dirhtsnjkc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"spqfoiclmtagejbndkrh"),
            9_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dlroktbcja") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nmotklspigjrdhcfaebq"),
            9_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ncjpmaekbs") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hlbosgmrak") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpmsd"),
            10_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pqfhsgilen") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qnpor"),
            10_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gqtjsbdckh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otdma"),
            10_(),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cfkqpjlegi") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"efhjg"),
            10_(),
            4_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"beanrfodgj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odpte"),
            10_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"adtkqpbjfi") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bctdgfmolr"),
            10_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"iomkfthagj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oaklidrbqg"),
            10_(),
            1_(),
            1_(),
        )
    });
}
pub fn test1_11() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gbhqo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"skqne"),
            5_(),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ktdor") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kipnf"),
            5_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ldprn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hmrnqdgifl"),
            5_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"egmjk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fsmjcdairn"),
            5_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"armql") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcdgltbrfj"),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cdhjo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aekfctpirg"),
            5_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jcons") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ledihrsgpf"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cbrkp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mqcklahsbtirgopefndj"),
            5_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fhgna") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kmlthaoqgecrnpdbjfis"),
            5_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ejfcd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sfhbamcdptojlkrenqgi"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kqjhe") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pbniofmcedrkhlstgaqj"),
            5_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pbdjl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mongjratcskbhqiepfdl"),
            5_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gajqn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            6_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"stedk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hrnat"),
            6_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tjkaf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gsqdt"),
            6_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dthpe") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bspkd"),
            6_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klhde") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ohcmb"),
            6_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bhlki") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"heatr"),
            6_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lqmoh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pmblckedfn"),
            6_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mtqin") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aceqmsrbik"),
            6_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dpqbr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lmbtdehjrn"),
            6_(),
            5_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kdhmo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"teqmcrlgib"),
            6_(),
            9_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jblqp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"njolbmspac"),
            6_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qmjgl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pofnhidklamecrbqjgst"),
            6_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rothp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jbhckmtgrqnosafedpli"),
            6_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ghknq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dobntpmqklicsahgjerf"),
            6_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eopfi") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tpdshainjkbfoemlrgcq"),
            6_(),
            19_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dsnmg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oldpfgeakrnitscbjmqh"),
            6_(),
            20_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jnkrfhotgl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dltjfngbko") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rqegt"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bmjlpkiqde") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dashm"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"skrflobnqm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jqirk"),
            0_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jkpldtshrm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rckeg"),
            0_(),
            4_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ghasdbnjqo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jscie"),
            0_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"igrkhpbqjt") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"efsphndliq"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ikthdgcamf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gdicosleja"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pcofgeniam") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qcpjibosfl"),
            0_(),
            5_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rlfjgesqhc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lrhmefnjcq"),
            0_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"itphbqsker") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtablcrseo"),
            0_(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"skjafcirqm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apckjsftedbhgomrnilq"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tcqomarsfd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pcbrgflehjtiadnsokqm"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rocfeldqpk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nsiadegjklhobrmtqcpf"),
            0_(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cfpegndlkt") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cpmajdqnolikhgsbretf"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fqbtnkeasj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jcflkntmgiqrphdosaeb"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"shbcqnmoar") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bdoshlmfin") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ontrs"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"khfrebnsgq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pfkna"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"getcrsaoji") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ekosa"),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fjiknedcpq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"anqhk"),
            1_(),
            4_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tkejgnafrm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jekca"),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jnakolqrde") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ikemsjgacf"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lcjptsmgbe") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"arolgsjkhm"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"itfsmcjorl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oftkbldhre"),
            1_(),
            5_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"omchkfrjea") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbkqdoeftl"),
            1_(),
            9_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cigfqkated") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sqcflrgtim"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tscenjikml") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fmhbkislrjdpanogqcet"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qcpaemsinf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rnioadktqlgpbcjsmhef"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gltkojeipd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oakgtnldpsefihqmjcbr"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qistfrgnmp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbnaelosidmcjqktfhpr"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bdnpfcqaem") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"akbripjhlosndcmqgfet"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ectnhskflp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            5_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fgtianblpq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pijag"),
            5_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mfeqklirnh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jrckd"),
            5_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"astedncjhk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qcloh"),
            5_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fhlqgcajbr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"thlmp"),
            5_(),
            4_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"epfhocmdng") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qidmo"),
            5_(),
            5_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"apcnsibger") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lnegpsjqrd"),
            5_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aqkocrbign") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rjqdablmfs"),
            5_(),
            1_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ijsmdtqgce") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"enkgpbsjaq"),
            5_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"clobgsrken") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kdsgoaijfh"),
            5_(),
            9_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jbhcfposld") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trfqgmckbe"),
            5_(),
            10_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oqnpblhide") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"igetsracjfkdnpoblhqm"),
            5_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lroeasctif") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nqctfaogirshlekbdjpm"),
            5_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bpjlgmiedh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"csehfgomljdqinbartkp"),
            5_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pamkeoidrj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qahoegcmplkfsjbdnitr"),
            5_(),
            19_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"espogqbthk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dpteiajrqmsognhlfbkc"),
            5_(),
            20_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"shoiedtcjb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            9_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ebcinjgads") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqbnh"),
            9_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dqmregkcfl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"akmle"),
            9_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ngcrieqajf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iqfkm"),
            9_(),
            2_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qosmilgnjb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqjsr"),
            9_(),
            4_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ikabsjtdfl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jplqg"),
            9_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ersmicafdh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oilnrbcgtj"),
            9_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fdnplotmgh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"morkglpesn"),
            9_(),
            1_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fdbicojerm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dmicerngat"),
            9_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mbtafndjcq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"radgeskbtc"),
            9_(),
            9_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mlenkpfdtc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljikprsmqo"),
            9_(),
            10_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ahlcifdqgs") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trqihkcgsjamfdbolnpe"),
            9_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bgjemaltks") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lqmthbsrekajgnofcipd"),
            9_(),
            1_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pdhslbqrfc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jtalmedribkgqsopcnfh"),
            9_(),
            10_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dirhtsnjkc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"spqfoiclmtagejbndkrh"),
            9_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dlroktbcja") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nmotklspigjrdhcfaebq"),
            9_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ncjpmaekbs") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hlbosgmrak") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpmsd"),
            10_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pqfhsgilen") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qnpor"),
            10_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gqtjsbdckh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otdma"),
            10_(),
            2_(),
            2_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cfkqpjlegi") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"efhjg"),
            10_(),
            4_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"beanrfodgj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odpte"),
            10_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"adtkqpbjfi") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bctdgfmolr"),
            10_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"iomkfthagj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oaklidrbqg"),
            10_(),
            1_(),
            1_(),
        )
    });
}
pub fn test2_12() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sdpcilonqj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dnjfsagktr"),
            10_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gtfbdkqeml") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nejaktmiqg"),
            10_(),
            9_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bmeqgcdorj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pjqonlebsf"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"etqlcanmob") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dshmnbtolcjepgaikfqr"),
            10_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"roqmkbdtia") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iogfhpabtjkqlrnemcds"),
            10_(),
            1_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kadsithljf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngridfabjsecpqltkmoh"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sgtkpbfdmh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"athmknplcgofrqejsdib"),
            10_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qgmetnabkl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ldobhmqcafnjtkeisgrp"),
            10_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cqjohampgd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hobitmpsan") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aocjb"),
            11_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tjehkpsalm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jbrnk"),
            11_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ngfbojitcl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqedg"),
            11_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rcfkdbhgjo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nqskp"),
            11_(),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qghptonrea") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eaqkl"),
            11_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnprfgqjdl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"reaoicljqm"),
            11_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hlmgabenti") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsftgajqpm"),
            11_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ofcjanmrbs") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rlpfogmits"),
            11_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jqedtkornm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"shkncmiaqj"),
            11_(),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rfedlasjmg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fpnatrhqgs"),
            11_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"talpqjsgkm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sjclemqhnpdbgikarfot"),
            11_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lrkcbtqpie") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otcmedjikgsfnqbrhpla"),
            11_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cipogdskjf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bonsaefdqiprkhlgtjcm"),
            11_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nqedcojahi") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"egpscmahijlfnkrodqtb"),
            11_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hefnrkmctj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kmqbfepjthgilscrndoa"),
            11_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"atqirnmekfjolhpdsgcb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"echfkmlpribjnqsaogtd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prboq"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnhiftdgcleajbpkrosm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fjcqh"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"chamfknorbedjitgslpq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fmosa"),
            0_(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"njhqpibfmtlkaecdrgso") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qdbok"),
            0_(),
            4_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ebnghfsqkprmdcljoiat") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"amslg"),
            0_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"letjomsgihfrpqbkancd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smpltjneqb"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nblgoipcrqeaktshjdmf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"flitskrnge"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cehkbngtjoiflqapsmrd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pgqihmlbef"),
            0_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mignapfoklbhcqjetdrs") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cfpdqjtgsb"),
            0_(),
            9_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ceatbhlsqjgpnokfrmdi") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htpsiaflom"),
            0_(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ocihkjgrdelpfnmastqb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kpjfiaceghsrdtlbnomq"),
            0_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"noelgschdtbrjfmiqkap") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qhtbomidljgafneksprc"),
            0_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dkclqfombepritjnghas") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nhtjobkcefldimpsaqgr"),
            0_(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"miklnresdgbhqcojftap") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prabcjfqnoeskilmtgdh"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"htbcigojaqmdkfrnlsep") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtrgmchilkasqoebfpjn"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"febhmqtjanokscdirpgl") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"loakbsqjpcrdhftniegm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sqome"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"reagphsqflbitdcjmkno") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smfte"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jitlfrqemsdhkopncabg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ciboh"),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mhtaepscdnrjqgbkifol") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"haois"),
            1_(),
            4_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tocesrfmnglpbjihqadk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abfki"),
            1_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lpfmctjrhdagneskbqoi") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"frdkocntmq"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lsmqaepkdhncirbtjfgo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oasbpedlnr"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"epoiqmtldrabnkjhcfsg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kltqmhgand"),
            1_(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"emgasrilpknqojhtbdcf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gdtfjchpmr"),
            1_(),
            9_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnfiagdpcklrjetqbsom") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ponmcqblet"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nsdfebgajhmtricpoklq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgphqdnofeiklatbcmjr"),
            1_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"atjgfsdlpobmeiqhncrk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljqprsmigtfoneadckbh"),
            1_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sitodfgnrejlahcbmqkp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ligeojhafnkmrcsqtbdp"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fraghmbiceknltjpqosd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsimqfnjarbopedkhcgt"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pmafenlhqtdbkirjsogc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abedmfjlghniorcqptks"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pihgmoeqtnakrjslcbfd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gjdkeprctqblnhiafsom") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqtoa"),
            10_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mkpnblfdsahrcqijteog") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cahif"),
            10_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gckarqnelodfjhmbptis") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kehis"),
            10_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gqpskidtbclomahnrjfe") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kdlmh"),
            10_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pkldjsqrfgitbhmaecno") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"paeql"),
            10_(),
            5_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aftsijrbeklnmcdqhgop") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aghoqiefnb"),
            10_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mtlgdrhafjkbiepqnsoc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jrbqaikpdo"),
            10_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pqgirnaefthokdmbsclj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smjonaeqcl"),
            10_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kpdbgjmtherlsfcqoina") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eqbdrkcfah"),
            10_(),
            9_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jrlbothiknqmdgcfasep") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kapmsienhf"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mjogldqferckabinptsh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jpqotrlenfcsbhkaimdg"),
            10_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"apoklnefbhmgqcdrisjt") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jlbmhnfgtcqprikeados"),
            10_(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ifeopcnrjbhkdgatmqls") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"stgbhfmdaljnpqoicker"),
            10_(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ckqhaiesmjdnrgolbtpf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oihcetflbjagdsrkmqpn"),
            10_(),
            19_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bnlgapfimcoterskqdjh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adtclebmnpjsrqfkigoh"),
            10_(),
            20_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kgdlrobpmjcthqsafeni") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dfkechomjapgnslbtqir") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"beafg"),
            19_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rloadknfbqtgmhcsipje") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iclat"),
            19_(),
            1_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mgjhkolrnadqbpetcifs") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rkhnf"),
            19_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cmlfakiojdrgtbsphqen") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"clshq"),
            19_(),
            4_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kghbfipeomsntdalrqjc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtcoj"),
            19_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eldiqckrnmtasbghjfpo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rqosnjmfth"),
            19_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abqjcfedgotihlnspkrm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"siatdfqglh"),
            19_(),
            1_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qfbadrtjsimkolcenhpg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mrlshtpgjq"),
            19_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abseghclkjqifmtodrnp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adlcskgqjt"),
            19_(),
            9_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ibmsnlrjefhtdokacqpg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"drshcjknaf"),
            19_(),
            10_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mrkfciqjebaponsthldg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"etsaqroinghpkjdlfcbm"),
            19_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mjkticdeoqshpalrfbgn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgepdnkqliambtrocfhj"),
            19_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rqnoclbdejgiphtfsakm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nlmcjaqgbsortfdihkpe"),
            19_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"plkqbhmtfaeodjcrsing") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"racfnpmosldibqkghjet"),
            19_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oegalhmstjrfickpbndq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fjhdsctkqeiolagrnmbp"),
            19_(),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rdtgjcaohpblniekmsfq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ofkqbnjetrmsaidphglc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ejanp"),
            20_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"grkpahljcftesdmonqib") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odife"),
            20_(),
            1_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jimlgbhfqkteospardcn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"okaqd"),
            20_(),
            2_(),
            12_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gftenihpmslrjkqadcob") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lcdbi"),
            20_(),
            4_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bmhldogtckrfsanijepq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fsqbj"),
            20_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nfqkrpjdesabgtlcmoih") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bigdomnplq"),
            20_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"focalnrpiqmdkstehbjg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apiblotgcd"),
            20_(),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rhqdspkmebiflcotnjga") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acfhdenops"),
            20_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rahdtmsckfboqlpniegj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jopdeamcrk"),
            20_(),
            9_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fbkeiopclstmdqranjhg") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trqncbkgmh"),
            20_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lifhpdgmbconstjeqark") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tomglrkencbsfjqpihda"),
            20_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
}
pub fn test2_13() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sdpcilonqj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dnjfsagktr"),
            10_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gtfbdkqeml") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nejaktmiqg"),
            10_(),
            9_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bmeqgcdorj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pjqonlebsf"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"etqlcanmob") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dshmnbtolcjepgaikfqr"),
            10_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"roqmkbdtia") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iogfhpabtjkqlrnemcds"),
            10_(),
            1_(),
            8_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kadsithljf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ngridfabjsecpqltkmoh"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sgtkpbfdmh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"athmknplcgofrqejsdib"),
            10_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qgmetnabkl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ldobhmqcafnjtkeisgrp"),
            10_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cqjohampgd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            11_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hobitmpsan") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aocjb"),
            11_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tjehkpsalm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jbrnk"),
            11_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ngfbojitcl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tqedg"),
            11_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rcfkdbhgjo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nqskp"),
            11_(),
            4_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qghptonrea") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eaqkl"),
            11_(),
            5_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnprfgqjdl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"reaoicljqm"),
            11_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hlmgabenti") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsftgajqpm"),
            11_(),
            1_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ofcjanmrbs") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rlpfogmits"),
            11_(),
            5_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jqedtkornm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"shkncmiaqj"),
            11_(),
            9_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rfedlasjmg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fpnatrhqgs"),
            11_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"talpqjsgkm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sjclemqhnpdbgikarfot"),
            11_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lrkcbtqpie") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"otcmedjikgsfnqbrhpla"),
            11_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cipogdskjf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bonsaefdqiprkhlgtjcm"),
            11_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nqedcojahi") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"egpscmahijlfnkrodqtb"),
            11_(),
            19_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hefnrkmctj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kmqbfepjthgilscrndoa"),
            11_(),
            20_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"atqirnmekfjolhpdsgcb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"echfkmlpribjnqsaogtd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prboq"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnhiftdgcleajbpkrosm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fjcqh"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"chamfknorbedjitgslpq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fmosa"),
            0_(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"njhqpibfmtlkaecdrgso") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qdbok"),
            0_(),
            4_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ebnghfsqkprmdcljoiat") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"amslg"),
            0_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"letjomsgihfrpqbkancd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smpltjneqb"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nblgoipcrqeaktshjdmf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"flitskrnge"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cehkbngtjoiflqapsmrd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"pgqihmlbef"),
            0_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mignapfoklbhcqjetdrs") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cfpdqjtgsb"),
            0_(),
            9_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ceatbhlsqjgpnokfrmdi") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"htpsiaflom"),
            0_(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ocihkjgrdelpfnmastqb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kpjfiaceghsrdtlbnomq"),
            0_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"noelgschdtbrjfmiqkap") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qhtbomidljgafneksprc"),
            0_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dkclqfombepritjnghas") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nhtjobkcefldimpsaqgr"),
            0_(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"miklnresdgbhqcojftap") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"prabcjfqnoeskilmtgdh"),
            0_(),
            19_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"htbcigojaqmdkfrnlsep") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtrgmchilkasqoebfpjn"),
            0_(),
            20_(),
            0_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"febhmqtjanokscdirpgl") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"loakbsqjpcrdhftniegm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sqome"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"reagphsqflbitdcjmkno") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smfte"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jitlfrqemsdhkopncabg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ciboh"),
            1_(),
            2_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mhtaepscdnrjqgbkifol") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"haois"),
            1_(),
            4_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tocesrfmnglpbjihqadk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abfki"),
            1_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lpfmctjrhdagneskbqoi") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"frdkocntmq"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lsmqaepkdhncirbtjfgo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oasbpedlnr"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"epoiqmtldrabnkjhcfsg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kltqmhgand"),
            1_(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"emgasrilpknqojhtbdcf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gdtfjchpmr"),
            1_(),
            9_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnfiagdpcklrjetqbsom") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ponmcqblet"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nsdfebgajhmtricpoklq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgphqdnofeiklatbcmjr"),
            1_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"atjgfsdlpobmeiqhncrk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ljqprsmigtfoneadckbh"),
            1_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sitodfgnrejlahcbmqkp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ligeojhafnkmrcsqtbdp"),
            1_(),
            10_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fraghmbiceknltjpqosd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lsimqfnjarbopedkhcgt"),
            1_(),
            19_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pmafenlhqtdbkirjsogc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"abedmfjlghniorcqptks"),
            1_(),
            20_(),
            1_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pihgmoeqtnakrjslcbfd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            10_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gjdkeprctqblnhiafsom") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hqtoa"),
            10_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mkpnblfdsahrcqijteog") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"cahif"),
            10_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gckarqnelodfjhmbptis") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kehis"),
            10_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gqpskidtbclomahnrjfe") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kdlmh"),
            10_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pkldjsqrfgitbhmaecno") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"paeql"),
            10_(),
            5_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aftsijrbeklnmcdqhgop") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"aghoqiefnb"),
            10_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mtlgdrhafjkbiepqnsoc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jrbqaikpdo"),
            10_(),
            1_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pqgirnaefthokdmbsclj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"smjonaeqcl"),
            10_(),
            5_(),
            5_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kpdbgjmtherlsfcqoina") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"eqbdrkcfah"),
            10_(),
            9_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jrlbothiknqmdgcfasep") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"kapmsienhf"),
            10_(),
            10_(),
            9_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mjogldqferckabinptsh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jpqotrlenfcsbhkaimdg"),
            10_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"apoklnefbhmgqcdrisjt") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jlbmhnfgtcqprikeados"),
            10_(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ifeopcnrjbhkdgatmqls") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"stgbhfmdaljnpqoicker"),
            10_(),
            10_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ckqhaiesmjdnrgolbtpf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"oihcetflbjagdsrkmqpn"),
            10_(),
            19_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bnlgapfimcoterskqdjh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adtclebmnpjsrqfkigoh"),
            10_(),
            20_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kgdlrobpmjcthqsafeni") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            19_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dfkechomjapgnslbtqir") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"beafg"),
            19_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rloadknfbqtgmhcsipje") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"iclat"),
            19_(),
            1_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mgjhkolrnadqbpetcifs") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rkhnf"),
            19_(),
            2_(),
            7_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cmlfakiojdrgtbsphqen") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"clshq"),
            19_(),
            4_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kghbfipeomsntdalrqjc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dtcoj"),
            19_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eldiqckrnmtasbghjfpo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rqosnjmfth"),
            19_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abqjcfedgotihlnspkrm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"siatdfqglh"),
            19_(),
            1_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qfbadrtjsimkolcenhpg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"mrlshtpgjq"),
            19_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abseghclkjqifmtodrnp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"adlcskgqjt"),
            19_(),
            9_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ibmsnlrjefhtdokacqpg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"drshcjknaf"),
            19_(),
            10_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mrkfciqjebaponsthldg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"etsaqroinghpkjdlfcbm"),
            19_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mjkticdeoqshpalrfbgn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sgepdnkqliambtrocfhj"),
            19_(),
            1_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rqnoclbdejgiphtfsakm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nlmcjaqgbsortfdihkpe"),
            19_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"plkqbhmtfaeodjcrsing") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"racfnpmosldibqkghjet"),
            19_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oegalhmstjrfickpbndq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fjhdsctkqeiolagrnmbp"),
            19_(),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rdtgjcaohpblniekmsfq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            20_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ofkqbnjetrmsaidphglc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ejanp"),
            20_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"grkpahljcftesdmonqib") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odife"),
            20_(),
            1_(),
            15_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jimlgbhfqkteospardcn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"okaqd"),
            20_(),
            2_(),
            12_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gftenihpmslrjkqadcob") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"lcdbi"),
            20_(),
            4_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bmhldogtckrfsanijepq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fsqbj"),
            20_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nfqkrpjdesabgtlcmoih") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"bigdomnplq"),
            20_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"focalnrpiqmdkstehbjg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apiblotgcd"),
            20_(),
            1_(),
            3_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rhqdspkmebiflcotnjga") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"acfhdenops"),
            20_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rahdtmsckfboqlpniegj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jopdeamcrk"),
            20_(),
            9_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fbkeiopclstmdqranjhg") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"trqncbkgmh"),
            20_(),
            10_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lifhpdgmbconstjeqark") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"tomglrkencbsfjqpihda"),
            20_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
}
pub fn test3_14() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pboqganrhedjmltsicfk") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbkhdnpoietfcmrslajq"),
            20_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klchabsimetjnqgorfpd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtfnmbsglkjaichoqedp"),
            20_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sirfgmjqhctndbklaepo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ohkmdpfqbsacrtjnlgei"),
            20_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rlbdsiceaonqjtfpghkm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dlbrteoisgphmkncajfq"),
            20_(),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ecgdanriptblhjfqskom") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fdmiarlpgcskbhoteqjn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sjrlo"),
            21_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rlbstjqopignecmfadkh") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qjpor"),
            21_(),
            1_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"grjpqmbshektdolcafni") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odhfn"),
            21_(),
            2_(),
            13_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sakfcohtqnibprjmlged") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qtfin"),
            21_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mjtdglasihqpocebrfkn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpqfo"),
            21_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"okaplfrntghqbmeicsdj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fabmertkos"),
            21_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sahngemrtcjidqbklfpo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"brqtgkmaej"),
            21_(),
            1_(),
            14_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dlmsipcnekhbgoaftqjr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nfrdeihsgl"),
            21_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ahegrmqnoiklpfsdbcjt") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hlfrosekpi"),
            21_(),
            9_(),
            14_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hdsjbnmlegtkqripacof") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"atgbkrjdsm"),
            21_(),
            10_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pcnedrfjihqbalkgtoms") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blnrptjgqmaifsdkhoec"),
            21_(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qjidealmtpskrbfhocng") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ctpmdahebfqjgknloris"),
            21_(),
            1_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qeindtagmokpfhsclrbj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apnkeqthrmlbfodiscgj"),
            21_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kpfegbjhsrnodltqciam") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jdgictpframeoqlsbknh"),
            21_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnbrcplsjfgiktoedmaq") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qprlsfojamgndekthibc"),
            21_(),
            20_(),
            19_(),
        )
    });
}
pub fn test3_15() {
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pboqganrhedjmltsicfk") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"gbkhdnpoietfcmrslajq"),
            20_(),
            1_(),
            4_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"klchabsimetjnqgorfpd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"rtfnmbsglkjaichoqedp"),
            20_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sirfgmjqhctndbklaepo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ohkmdpfqbsacrtjnlgei"),
            20_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rlbdsiceaonqjtfpghkm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"dlbrteoisgphmkncajfq"),
            20_(),
            20_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ecgdanriptblhjfqskom") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b""),
            21_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fdmiarlpgcskbhoteqjn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"sjrlo"),
            21_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rlbstjqopignecmfadkh") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qjpor"),
            21_(),
            1_(),
            6_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"grjpqmbshektdolcafni") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"odhfn"),
            21_(),
            2_(),
            13_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sakfcohtqnibprjmlged") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qtfin"),
            21_(),
            4_(),
            10_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mjtdglasihqpocebrfkn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hpqfo"),
            21_(),
            5_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"okaplfrntghqbmeicsdj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"fabmertkos"),
            21_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"sahngemrtcjidqbklfpo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"brqtgkmaej"),
            21_(),
            1_(),
            14_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dlmsipcnekhbgoaftqjr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"nfrdeihsgl"),
            21_(),
            5_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ahegrmqnoiklpfsdbcjt") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"hlfrosekpi"),
            21_(),
            9_(),
            14_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hdsjbnmlegtkqripacof") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"atgbkrjdsm"),
            21_(),
            10_(),
            16_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pcnedrfjihqbalkgtoms") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"blnrptjgqmaifsdkhoec"),
            21_(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qjidealmtpskrbfhocng") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"ctpmdahebfqjgknloris"),
            21_(),
            1_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qeindtagmokpfhsclrbj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"apnkeqthrmlbfodiscgj"),
            21_(),
            10_(),
            17_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kpfegbjhsrnodltqciam") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"jdgictpframeoqlsbknh"),
            21_(),
            19_(),
            19_(),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnbrcplsjfgiktoedmaq") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            Ptr::<()>::from_string_literal(b"qprlsfojamgndekthibc"),
            21_(),
            20_(),
            19_(),
        )
    });
}
pub fn test_string_16() {
    ({ test0_8() });
    ({ test1_10() });
    ({ test2_12() });
    ({ test3_14() });
}
pub fn test_string_17() {
    ({ test0_9() });
    ({ test1_11() });
    ({ test2_13() });
    ({ test3_15() });
}
pub fn test_18() -> () {
    ({ test_string_16() });
    ({ test_string_17() });
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
    ({ test_18() });
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
