extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
    pub unsafe fn find(mut s: *const (), mut n: (), a: *const ()) -> *const () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s);
                let _c2: () = (*a);
                constexpr_char_traits_char_::eq(_c1, _c2)
            }) {
                return s;
            }
            s.prefix_inc();
            n.prefix_dec();
        }
        return std::ptr::null();
    }
}
pub unsafe fn test_0(s: *const (), mut str: *const (), mut pos: (), mut n: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1))) {
        unimplemented!();
    }
}
pub unsafe fn test0_2() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"irkhs".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kante".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"oknlr".as_ptr(),
            0_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pcdro".as_ptr(),
            0_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bnrpe".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jtdaefblso".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"oselktgbca".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"eqgaplhckj".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bjahtcmnlp".as_ptr(),
            0_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"hjlcmgpket".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"htaobedqikfplcgjsmrn".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"hpqiarojkcdlsgnmfetb".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"dfkaprhjloqetcsimnbg".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ihqrfebgadntlpmjksoc".as_ptr(),
            0_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ngtjfcalbseiqrphmkdo".as_ptr(),
            0_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"lbtqd".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"tboim".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"slcer".as_ptr(),
            1_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"cbjfs".as_ptr(),
            1_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"aqibs".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"gtfblmqinc".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"mkqpbtdalg".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kphatlimcd".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pblasqogic".as_ptr(),
            1_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"arosdhcfme".as_ptr(),
            1_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"blkhjeogicatqfnpdmsr".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bmhineprjcoadgstflqk".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"djkqcmetslnghpbarfoi".as_ptr(),
            1_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"lgokshjtpbemarcdqnfi".as_ptr(),
            1_(),
            19_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bqjhtkfepimcnsgrlado".as_ptr(),
            1_(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"gfsrt".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pfsoc".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"tpflm".as_ptr(),
            0_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"sgkec".as_ptr(), 0_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"romds".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"qhjistlgmr".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pedfirsglo".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"aqcoslgrmk".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dabckmepqj".as_ptr(), 0_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pqscrjthli".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kfphdcsjqmobliagtren".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"rokpefncljibsdhqtagm".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"afionmkphlebtcjqsgrd".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"aenmqplidhkofrjbctsg".as_ptr(), 0_(), 19_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"osjmbtcadhiklegrpqnf".as_ptr(), 0_(), 20_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"osmia".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ckonl".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ilcaj".as_ptr(),
            1_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"lasiq".as_ptr(),
            1_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kfqmr".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"klnitfaobg".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"gjhmdlqikp".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"skbgtahqej".as_ptr(), 1_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"bjsdgtlpkf".as_ptr(), 1_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"bjgfmnlkio".as_ptr(), 1_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"lbhepotfsjdqigcnamkr".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"tebangckmpsrqdlfojhi".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"joflqbdkhtegimscpanr".as_ptr(), 1_(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"adpmcohetfbsrjinlqkg".as_ptr(), 1_(), 19_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"iacldqjpfnogbsrhmetk".as_ptr(), 1_(), 20_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            2_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"otkgb".as_ptr(),
            2_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"cqsjl".as_ptr(),
            2_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"dpifl".as_ptr(),
            2_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"oapht".as_ptr(), 2_(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"cifts".as_ptr(), 2_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"nmsckbgalo".as_ptr(),
            2_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"tpksqhamle".as_ptr(),
            2_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"tpdrchmkji".as_ptr(), 2_(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ijagfkblst".as_ptr(), 2_(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"kpocsignjb".as_ptr(), 2_(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pecqtkjsnbdrialgmohf".as_ptr(),
            2_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"aiortphfcmkjebgsndql".as_ptr(),
            2_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"sdbkeamglhipojqftrcn".as_ptr(), 2_(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ljqncehgmfktroapidbs".as_ptr(), 2_(), 19_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"rtcfodilamkbenjghqps".as_ptr(), 2_(), 20_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            4_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"mabns".as_ptr(),
            4_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bdnrp".as_ptr(),
            4_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"scidp".as_ptr(),
            4_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"agbjl".as_ptr(),
            4_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"jfmpr".as_ptr(), 4_(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"rbpefghsmj".as_ptr(),
            4_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"apsfntdoqc".as_ptr(),
            4_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ndkjeisgcl".as_ptr(), 4_(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"rnfpqatdeo".as_ptr(), 4_(), 9_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"bntjlqrfik".as_ptr(), 4_(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kcrtmpolnaqejghsfdbi".as_ptr(),
            4_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"lobheanpkmqidsrtcfgj".as_ptr(),
            4_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"athdkljcnreqbgpmisof".as_ptr(), 4_(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"qkdmjialrscpbhefgont".as_ptr(), 4_(), 19_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dmasojntqleribkgfchp".as_ptr(), 4_(), 20_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"psthd".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"rpmjd".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"dfsmk".as_ptr(),
            5_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
}
pub unsafe fn test1_3() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"skqne".as_ptr(), 5_(), 4_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"kipnf".as_ptr(), 5_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"hmrnqdgifl".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"fsmjcdairn".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"pcdgltbrfj".as_ptr(), 5_(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"aekfctpirg".as_ptr(), 5_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ledihrsgpf".as_ptr(), 5_(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"mqcklahsbtirgopefndj".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kmlthaoqgecrnpdbjfis".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"sfhbamcdptojlkrenqgi".as_ptr(), 5_(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"pbniofmcedrkhlstgaqj".as_ptr(), 5_(), 19_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"mongjratcskbhqiepfdl".as_ptr(), 5_(), 20_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"hrnat".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"gsqdt".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bspkd".as_ptr(),
            6_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ohcmb".as_ptr(), 6_(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"heatr".as_ptr(), 6_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pmblckedfn".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"aceqmsrbik".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lmbtdehjrn".as_ptr(), 6_(), 5_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"teqmcrlgib".as_ptr(), 6_(), 9_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"njolbmspac".as_ptr(), 6_(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pofnhidklamecrbqjgst".as_ptr(),
            6_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jbhckmtgrqnosafedpli".as_ptr(),
            6_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dobntpmqklicsahgjerf".as_ptr(), 6_(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"tpdshainjkbfoemlrgcq".as_ptr(), 6_(), 19_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"oldpfgeakrnitscbjmqh".as_ptr(), 6_(), 20_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"rqegt".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"dashm".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jqirk".as_ptr(),
            0_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"rckeg".as_ptr(),
            0_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jscie".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"efsphndliq".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"gdicosleja".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"qcpjibosfl".as_ptr(), 0_(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lrhmefnjcq".as_ptr(), 0_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"dtablcrseo".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"apckjsftedbhgomrnilq".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pcbrgflehjtiadnsokqm".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"nsiadegjklhobrmtqcpf".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"cpmajdqnolikhgsbretf".as_ptr(), 0_(), 19_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"jcflkntmgiqrphdosaeb".as_ptr(), 0_(), 20_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ontrs".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pfkna".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ekosa".as_ptr(), 1_(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"anqhk".as_ptr(),
            1_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"jekca".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ikemsjgacf".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"arolgsjkhm".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"oftkbldhre".as_ptr(), 1_(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"gbkqdoeftl".as_ptr(), 1_(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"sqcflrgtim".as_ptr(), 1_(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"fmhbkislrjdpanogqcet".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"rnioadktqlgpbcjsmhef".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"oakgtnldpsefihqmjcbr".as_ptr(), 1_(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"gbnaelosidmcjqktfhpr".as_ptr(), 1_(), 19_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"akbripjhlosndcmqgfet".as_ptr(), 1_(), 20_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pijag".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jrckd".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"qcloh".as_ptr(),
            5_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"thlmp".as_ptr(), 5_(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"qidmo".as_ptr(), 5_(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"lnegpsjqrd".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"rjqdablmfs".as_ptr(), 5_(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"enkgpbsjaq".as_ptr(),
            5_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"kdsgoaijfh".as_ptr(), 5_(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"trfqgmckbe".as_ptr(), 5_(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"igetsracjfkdnpoblhqm".as_ptr(),
            5_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"nqctfaogirshlekbdjpm".as_ptr(),
            5_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"csehfgomljdqinbartkp".as_ptr(), 5_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"qahoegcmplkfsjbdnitr".as_ptr(), 5_(), 19_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dpteiajrqmsognhlfbkc".as_ptr(), 5_(), 20_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            9_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"tqbnh".as_ptr(),
            9_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"akmle".as_ptr(),
            9_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"iqfkm".as_ptr(), 9_(), 2_(), 6_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"tqjsr".as_ptr(), 9_(), 4_(), 8_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"jplqg".as_ptr(), 9_(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"oilnrbcgtj".as_ptr(),
            9_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"morkglpesn".as_ptr(), 9_(), 1_(), 7_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dmicerngat".as_ptr(), 9_(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"radgeskbtc".as_ptr(), 9_(), 9_(), 6_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ljikprsmqo".as_ptr(), 9_(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"trqihkcgsjamfdbolnpe".as_ptr(),
            9_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lqmthbsrekajgnofcipd".as_ptr(), 9_(), 1_(), 6_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"jtalmedribkgqsopcnfh".as_ptr(), 9_(), 10_(), 7_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"spqfoiclmtagejbndkrh".as_ptr(), 9_(), 19_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"nmotklspigjrdhcfaebq".as_ptr(), 9_(), 20_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"hpmsd".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"qnpor".as_ptr(), 10_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"otdma".as_ptr(), 10_(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"efhjg".as_ptr(), 10_(), 4_(), 7_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"odpte".as_ptr(), 10_(), 5_(), 7_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bctdgfmolr".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"oaklidrbqg".as_ptr(), 10_(), 1_(), 1_())
    });
}
pub unsafe fn test2_4() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dnjfsagktr".as_ptr(), 10_(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"nejaktmiqg".as_ptr(), 10_(), 9_(), 8_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"pjqonlebsf".as_ptr(), 10_(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"dshmnbtolcjepgaikfqr".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"iogfhpabtjkqlrnemcds".as_ptr(), 10_(), 1_(), 8_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ngridfabjsecpqltkmoh".as_ptr(),
            10_(),
            10_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"athmknplcgofrqejsdib".as_ptr(),
            10_(),
            19_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ldobhmqcafnjtkeisgrp".as_ptr(),
            10_(),
            20_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"aocjb".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"jbrnk".as_ptr(), 11_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"tqedg".as_ptr(), 11_(), 2_(), 7_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"nqskp".as_ptr(), 11_(), 4_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"eaqkl".as_ptr(), 11_(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"reaoicljqm".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lsftgajqpm".as_ptr(), 11_(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"rlpfogmits".as_ptr(), 11_(), 5_(), 7_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"shkncmiaqj".as_ptr(), 11_(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"fpnatrhqgs".as_ptr(), 11_(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"sjclemqhnpdbgikarfot".as_ptr(),
            11_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"otcmedjikgsfnqbrhpla".as_ptr(),
            11_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bonsaefdqiprkhlgtjcm".as_ptr(),
            11_(),
            10_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"egpscmahijlfnkrodqtb".as_ptr(),
            11_(),
            19_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kmqbfepjthgilscrndoa".as_ptr(),
            11_(),
            20_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"prboq".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"fjcqh".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"fmosa".as_ptr(),
            0_(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"qdbok".as_ptr(),
            0_(),
            4_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"amslg".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"smpltjneqb".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"flitskrnge".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pgqihmlbef".as_ptr(),
            0_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"cfpdqjtgsb".as_ptr(),
            0_(),
            9_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"htpsiaflom".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kpjfiaceghsrdtlbnomq".as_ptr(),
            0_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"qhtbomidljgafneksprc".as_ptr(),
            0_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"nhtjobkcefldimpsaqgr".as_ptr(),
            0_(),
            10_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"prabcjfqnoeskilmtgdh".as_ptr(), 0_(), 19_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dtrgmchilkasqoebfpjn".as_ptr(), 0_(), 20_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"sqome".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"smfte".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ciboh".as_ptr(), 1_(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"haois".as_ptr(), 1_(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"abfki".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"frdkocntmq".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"oasbpedlnr".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"kltqmhgand".as_ptr(),
            1_(),
            5_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"gdtfjchpmr".as_ptr(), 1_(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ponmcqblet".as_ptr(), 1_(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"sgphqdnofeiklatbcmjr".as_ptr(),
            1_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ljqprsmigtfoneadckbh".as_ptr(),
            1_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ligeojhafnkmrcsqtbdp".as_ptr(), 1_(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lsimqfnjarbopedkhcgt".as_ptr(), 1_(), 19_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"abedmfjlghniorcqptks".as_ptr(), 1_(), 20_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"hqtoa".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"cahif".as_ptr(),
            10_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"kehis".as_ptr(), 10_(), 2_(), 7_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"kdlmh".as_ptr(), 10_(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"paeql".as_ptr(), 10_(), 5_(), 6_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"aghoqiefnb".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"jrbqaikpdo".as_ptr(), 10_(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"smjonaeqcl".as_ptr(), 10_(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"eqbdrkcfah".as_ptr(), 10_(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"kapmsienhf".as_ptr(), 10_(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jpqotrlenfcsbhkaimdg".as_ptr(),
            10_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jlbmhnfgtcqprikeados".as_ptr(),
            10_(),
            1_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"stgbhfmdaljnpqoicker".as_ptr(),
            10_(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"oihcetflbjagdsrkmqpn".as_ptr(),
            10_(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"adtclebmnpjsrqfkigoh".as_ptr(),
            10_(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            19_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"beafg".as_ptr(),
            19_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"iclat".as_ptr(), 19_(), 1_(), 16_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"rkhnf".as_ptr(), 19_(), 2_(), 7_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"clshq".as_ptr(), 19_(), 4_(), 16_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dtcoj".as_ptr(), 19_(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"rqosnjmfth".as_ptr(),
            19_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"siatdfqglh".as_ptr(), 19_(), 1_(), 15_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"mrlshtpgjq".as_ptr(), 19_(), 5_(), 17_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"adlcskgqjt".as_ptr(), 19_(), 9_(), 16_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"drshcjknaf".as_ptr(), 19_(), 10_(), 16_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"etsaqroinghpkjdlfcbm".as_ptr(),
            19_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"sgepdnkqliambtrocfhj".as_ptr(),
            19_(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"nlmcjaqgbsortfdihkpe".as_ptr(),
            19_(),
            10_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"racfnpmosldibqkghjet".as_ptr(),
            19_(),
            19_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"fjhdsctkqeiolagrnmbp".as_ptr(),
            19_(),
            20_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ejanp".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"odife".as_ptr(), 20_(), 1_(), 15_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"okaqd".as_ptr(), 20_(), 2_(), 12_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lcdbi".as_ptr(), 20_(), 4_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"fsqbj".as_ptr(), 20_(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"bigdomnplq".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"apiblotgcd".as_ptr(), 20_(), 1_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"acfhdenops".as_ptr(), 20_(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"jopdeamcrk".as_ptr(), 20_(), 9_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"trqncbkgmh".as_ptr(), 20_(), 10_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"tomglrkencbsfjqpihda".as_ptr(),
            20_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
}
pub unsafe fn test3_5() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"gbkhdnpoietfcmrslajq".as_ptr(), 20_(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"rtfnmbsglkjaichoqedp".as_ptr(),
            20_(),
            10_(),
            17_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ohkmdpfqbsacrtjnlgei".as_ptr(),
            20_(),
            19_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"dlbrteoisgphmkncajfq".as_ptr(),
            20_(),
            20_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"sjrlo".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"qjpor".as_ptr(), 21_(), 1_(), 6_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"odhfn".as_ptr(), 21_(), 2_(), 13_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"qtfin".as_ptr(), 21_(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"hpqfo".as_ptr(), 21_(), 5_(), 17_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"fabmertkos".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"brqtgkmaej".as_ptr(), 21_(), 1_(), 14_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"nfrdeihsgl".as_ptr(), 21_(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"hlfrosekpi".as_ptr(), 21_(), 9_(), 14_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"atgbkrjdsm".as_ptr(), 21_(), 10_(), 16_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"blnrptjgqmaifsdkhoec".as_ptr(),
            21_(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_1)),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"ctpmdahebfqjgknloris".as_ptr(),
            21_(),
            1_(),
            17_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"apnkeqthrmlbfodiscgj".as_ptr(),
            21_(),
            10_(),
            17_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jdgictpframeoqlsbknh".as_ptr(),
            21_(),
            19_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"qprlsfojamgndekthibc".as_ptr(),
            21_(),
            20_(),
            19_(),
        )
    });
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        (unsafe { test0_2() });
        (unsafe { test1_3() });
        (unsafe { test2_4() });
        (unsafe { test3_5() });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.find_last_of(\"\", 0, 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_last_of(\"irkhs\", 0, 5) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_of(\"\", 0, 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_of(\"gfsrt\", 5, 5) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_of(\"lecar\", 5, 5) == 4");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
