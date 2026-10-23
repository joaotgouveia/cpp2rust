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
pub fn test_4(s: Ptr<()>, str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_5.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_6(s: Ptr<()>, str: Ptr<()>, mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_7.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_8(s: Ptr<()>, str: Ptr<()>, mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if (x != npos_5.with(|rc| *rc.borrow())) {
        unimplemented!();
    }
}
pub fn test_9(s: Ptr<()>, str: Ptr<()>, mut x: ()) {
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"laenf") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pqlnkmbdjo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bjaht") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hjlcmgpket") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qanej") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dfkap") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"clbao") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ihqrfebgad") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mekdn") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"srdfq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oemth") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ikcrq") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cdaih") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dmajblfhsg") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 3_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qohtk") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oqftjhdmkgsblacenirp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cshmd") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 2_(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lhcdo") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oebqi") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 2_(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnsoh") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kojhpmbsfe") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pkrof") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acbsjqogpltdkhinfrem") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            2_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fmtsp") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 4_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"khbpm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aobjd") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 4_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pbsji") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pcbahntsje") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 4_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mprdj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fhepcrntkoagbmldqijs") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            4_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqmpa") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kocgb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"onmje") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fbslrjiqkm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oqmrj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jeidpcmalhfnqbgtrsko") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"schfa") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qngpd") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            6_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"brqgo") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rodhqklgmb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            6_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tnrph") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"thdjgafrlbkoiqcspmne") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            6_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hcjitbfapl") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"daiprenocl") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ashjd") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"litpcfdghe") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mgojkldsqh") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aidjksrolc") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"imqnaghkfrdtlopbjesc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qpghtfbaji") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gfshlcmdjr") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nadkh") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nkodajteqp") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ofdrqmkebl") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gbmetiprqd") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bdfjqgatlksriohemnpc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"crnklpmegd") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 5_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jsbtafedoc") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"prqgn") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 5_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnmodrtkeb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pejafmnokr") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 5_(), 6_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cpebqsfmnj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"odnqkgijrhabfmcestlp") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            5_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lmofqdhpki") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 9_(), 9_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnefkqimca") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rtjpa") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            9_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"drtasbgmfp") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ktsrmnqagd") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 9_(), 9_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lsaijeqhtr") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rtdhgcisbnmoaqkfpjle") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            9_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"elgofjmbrq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dplqa") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kthqnfcerm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dkacjoptns") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dfsjhanorc") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hqfimtrgnbekpdcsjalo") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqsgalomhb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lofbc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            11_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hlbdfreqjo") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"astoegbfpn") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            11_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"taqobhlerg") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pdgreqomsncafklhtibj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            11_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"snafbdlghrjkpqtoceim") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aemtbrgcklhndjisfpoq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lbtqd") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pnracgfkjdiholtbqsem") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tboimldpjh") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 0_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dicfltehbsgrmojnpkaq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"slcerthdaiqjfnobgkpm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            0_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jlnkraeodhcspfgbqitm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lhosrngtmfjikbqpcade") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aqibs") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rbtaqjhgkneisldpmfoc") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gtfblmqinc") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 1_(), 3_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gpifsqlrdkbonjtmheca") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mkqpbtdalgniorhfescj") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            1_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hdpkobnsalmcfijregtq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 10_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jtlshdgqaiprkbcoenfm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pblas") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 10_(), 11_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fkdrbqltsgmcoiphneaj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"arosdhcfme") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 10_(), 13_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"crsplifgtqedjohnabmk") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            10_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"niptglfbosehkamrdqcj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 19_(), 19_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"copqdhstbingamjfkler") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"djkqc") },
                None,
            ),
        ));
        test_4(_s.as_pointer(), _str.as_pointer(), 19_(), 19_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mrtaefilpdsgocnhqbjk") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lgokshjtpb") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            19_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kojatdhlcmigpbfrqnes") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            19_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eaintpchlqsbdgrkjofm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nocfa") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            20_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"spocfaktqdbiejlhngmr") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bgtajmiedc") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            20_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rphmlekgfscndtaobiqj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lsckfnqgdahejiopbtmr") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            20_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"liatsqdoegkmfcnbhrpj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gfsrt") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            21_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"latkmisecnorjbfhqpdg") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pfsocbhjtm") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            21_(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lecfratdjkhnsmqpoigb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tpflmdnoicjgkberhqsa") },
                None,
            ),
        ));
        test_4(
            _s.as_pointer(),
            _str.as_pointer(),
            21_(),
            npos_5.with(|rc| *rc.borrow()),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"laenf") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pqlnkmbdjo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bjaht") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hjlcmgpket") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qanej") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dfkap") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"clbao") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ihqrfebgad") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mekdn") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ngtjfcalbseiqrphmkdo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"srdfq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oemth") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ikcrq") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cdaih") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dmajblfhsg") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 3_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qohtk") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oqftjhdmkgsblacenirp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cshmd") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 2_(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lhcdo") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oebqi") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 2_(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnsoh") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kojhpmbsfe") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pkrof") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"acbsjqogpltdkhinfrem") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            2_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fmtsp") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 4_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"khbpm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aobjd") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 4_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pbsji") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pcbahntsje") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 4_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mprdj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fhepcrntkoagbmldqijs") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            4_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqmpa") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kocgb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"onmje") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fbslrjiqkm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oqmrj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jeidpcmalhfnqbgtrsko") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"schfa") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qngpd") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            6_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"brqgo") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rodhqklgmb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            6_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tnrph") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"thdjgafrlbkoiqcspmne") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            6_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hcjitbfapl") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"daiprenocl") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ashjd") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"litpcfdghe") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mgojkldsqh") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aidjksrolc") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"imqnaghkfrdtlopbjesc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qpghtfbaji") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gfshlcmdjr") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nadkh") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nkodajteqp") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ofdrqmkebl") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 4_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gbmetiprqd") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bdfjqgatlksriohemnpc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"crnklpmegd") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 5_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jsbtafedoc") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"prqgn") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 5_(), 5_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qnmodrtkeb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pejafmnokr") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 5_(), 6_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"cpebqsfmnj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"odnqkgijrhabfmcestlp") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            5_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lmofqdhpki") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 9_(), 9_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hnefkqimca") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rtjpa") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            9_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"drtasbgmfp") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"ktsrmnqagd") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 9_(), 9_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lsaijeqhtr") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rtdhgcisbnmoaqkfpjle") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            9_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"elgofjmbrq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dplqa") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kthqnfcerm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dkacjoptns") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dfsjhanorc") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hqfimtrgnbekpdcsjalo") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eqsgalomhb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lofbc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            11_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hlbdfreqjo") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"astoegbfpn") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            11_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"taqobhlerg") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pdgreqomsncafklhtibj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            11_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"snafbdlghrjkpqtoceim") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aemtbrgcklhndjisfpoq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lbtqd") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pnracgfkjdiholtbqsem") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tboimldpjh") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 0_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"dicfltehbsgrmojnpkaq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"slcerthdaiqjfnobgkpm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            0_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jlnkraeodhcspfgbqitm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lhosrngtmfjikbqpcade") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"aqibs") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rbtaqjhgkneisldpmfoc") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gtfblmqinc") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 1_(), 3_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gpifsqlrdkbonjtmheca") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mkqpbtdalgniorhfescj") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            1_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hdpkobnsalmcfijregtq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 10_(), 10_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jtlshdgqaiprkbcoenfm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pblas") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 10_(), 11_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"fkdrbqltsgmcoiphneaj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"arosdhcfme") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 10_(), 13_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"crsplifgtqedjohnabmk") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"blkhjeogicatqfnpdmsr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            10_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"niptglfbosehkamrdqcj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 19_(), 19_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"copqdhstbingamjfkler") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"djkqc") },
                None,
            ),
        ));
        test_6(_s.as_pointer(), _str.as_pointer(), 19_(), 19_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"mrtaefilpdsgocnhqbjk") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lgokshjtpb") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            19_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kojatdhlcmigpbfrqnes") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bqjhtkfepimcnsgrlado") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            19_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eaintpchlqsbdgrkjofm") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nocfa") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            20_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"spocfaktqdbiejlhngmr") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bgtajmiedc") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            20_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"rphmlekgfscndtaobiqj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lsckfnqgdahejiopbtmr") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            20_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"liatsqdoegkmfcnbhrpj") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gfsrt") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            21_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"latkmisecnorjbfhqpdg") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pfsocbhjtm") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            21_(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lecfratdjkhnsmqpoigb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"tpflmdnoicjgkberhqsa") },
                None,
            ),
        ));
        test_6(
            _s.as_pointer(),
            _str.as_pointer(),
            21_(),
            npos_7.with(|rc| *rc.borrow()),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"laenf") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pqlnkmbdjo") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lahfb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"irkhs") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gmfhd") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kantesmpgj") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"odaft") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oknlrstdpiqmjbaghcfe") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            _str.as_pointer(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eolhfgpjqk") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nbatdlmekr") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bnrpe") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jdmciepkaq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jtdaefblso") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hkbgspoflt") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oselktgbcapndfjihrmq") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            _str.as_pointer(),
            npos_5.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gprdcokbnjhlsfmtieqa") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qjghlnftcaismkropdeb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bjaht") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pnalfrdtkqcmojiesbhg") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hjlcmgpket") },
                None,
            ),
        ));
        test_8(_s.as_pointer(), _str.as_pointer(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pniotcfrhqsmgdkjbael") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn") },
                None,
            ),
        ));
        test_8(
            _s.as_pointer(),
            _str.as_pointer(),
            npos_5.with(|rc| *rc.borrow()),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"laenf") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pqlnkmbdjo") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qkamfogpnljdcshbreti") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            _str.as_pointer(),
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
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"lahfb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"irkhs") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gmfhd") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"kantesmpgj") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"odaft") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oknlrstdpiqmjbaghcfe") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            _str.as_pointer(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"eolhfgpjqk") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"nbatdlmekr") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bnrpe") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jdmciepkaq") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"jtdaefblso") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 2_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hkbgspoflt") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"oselktgbcapndfjihrmq") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            _str.as_pointer(),
            npos_7.with(|rc| *rc.borrow()),
        )
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"gprdcokbnjhlsfmtieqa") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"qjghlnftcaismkropdeb") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"bjaht") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 0_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pnalfrdtkqcmojiesbhg") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"hjlcmgpket") },
                None,
            ),
        ));
        test_9(_s.as_pointer(), _str.as_pointer(), 1_())
    });
    ({
        let _s: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"pniotcfrhqsmgdkjbael") },
                None,
            ),
        ));
        let _str: Value<()> = Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"htaobedqikfplcgjsmrn") },
                None,
            ),
        ));
        test_9(
            _s.as_pointer(),
            _str.as_pointer(),
            npos_7.with(|rc| *rc.borrow()),
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
    {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b" !") },
            None,
        );
        unimplemented!();
    }
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
