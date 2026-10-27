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
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
}
pub unsafe fn test_0(s: *const (), mut str: *const (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_1(s: *const (), mut str: *const (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test0_2() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"laenf".as_ptr(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"pqlnkmbdjo".as_ptr(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"qkamfogpnljdcshbreti".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"bjaht".as_ptr(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"hjlcmgpket".as_ptr(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"htaobedqikfplcgjsmrn".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dfkap".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ihqrfebgad".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ngtjfcalbseiqrphmkdo".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ikcrq".as_ptr(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dmajblfhsg".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"oqftjhdmkgsblacenirp".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 2_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"oebqi".as_ptr(), 2_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"kojhpmbsfe".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"acbsjqogpltdkhinfrem".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"aobjd".as_ptr(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"pcbahntsje".as_ptr(), 4_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"fhepcrntkoagbmldqijs".as_ptr(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"kocgb".as_ptr(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"fbslrjiqkm".as_ptr(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"jeidpcmalhfnqbgtrsko".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 6_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"qngpd".as_ptr(), 6_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"rodhqklgmb".as_ptr(), 6_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"thdjgafrlbkoiqcspmne".as_ptr(),
            6_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ashjd".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"mgojkldsqh".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"imqnaghkfrdtlopbjesc".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"nadkh".as_ptr(), 1_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ofdrqmkebl".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"bdfjqgatlksriohemnpc".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"prqgn".as_ptr(), 5_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"pejafmnokr".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"odnqkgijrhabfmcestlp".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 9_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"rtjpa".as_ptr(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"ktsrmnqagd".as_ptr(), 9_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"rtdhgcisbnmoaqkfpjle".as_ptr(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 10_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dplqa".as_ptr(), 10_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"dkacjoptns".as_ptr(), 10_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"hqfimtrgnbekpdcsjalo".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 11_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lofbc".as_ptr(), 11_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"astoegbfpn".as_ptr(), 11_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"pdgreqomsncafklhtibj".as_ptr(),
            11_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 0_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lbtqd".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"tboimldpjh".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"slcerthdaiqjfnobgkpm".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 1_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"aqibs".as_ptr(), 1_(), 3_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"gtfblmqinc".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"mkqpbtdalgniorhfescj".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 10_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"pblas".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"arosdhcfme".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"blkhjeogicatqfnpdmsr".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 19_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"djkqc".as_ptr(), 19_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"lgokshjtpb".as_ptr(), 19_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"bqjhtkfepimcnsgrlado".as_ptr(), 19_(), 19_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 20_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"nocfa".as_ptr(), 20_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"bgtajmiedc".as_ptr(), 20_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"lsckfnqgdahejiopbtmr".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"".as_ptr(), 21_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"gfsrt".as_ptr(), 21_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(&mut _s, c"pfsocbhjtm".as_ptr(), 21_(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_0(
            &mut _s,
            c"tpflmdnoicjgkberhqsa".as_ptr(),
            21_(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test1_3() {
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"".as_ptr(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"laenf".as_ptr(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"pqlnkmbdjo".as_ptr(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"qkamfogpnljdcshbreti".as_ptr(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"".as_ptr(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"irkhs".as_ptr(), 2_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"kantesmpgj".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"oknlrstdpiqmjbaghcfe".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"".as_ptr(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"bnrpe".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"jtdaefblso".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"oselktgbcapndfjihrmq".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"".as_ptr(), unimplemented!())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"bjaht".as_ptr(), 1_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"hjlcmgpket".as_ptr(), 0_())
    });
    (unsafe {
        let mut _s: () = unimplemented!();
        test_1(&mut _s, c"htaobedqikfplcgjsmrn".as_ptr(), 0_())
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
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.find_first_of(\"\", 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_first_of(\"irkhs\", 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\"\", 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\"gfsrt\", 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\"lecar\", 0) == 0");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
