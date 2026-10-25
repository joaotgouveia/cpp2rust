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
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
    }
}
pub unsafe fn PathReserve_0(p: *mut (), mut N: ()) {
    let native_ref: *const () = unimplemented!();
    unimplemented!();
}
pub unsafe fn PathEq_1(LHS: *const (), RHS: *const ()) -> () {
    return unimplemented!();
}
pub unsafe fn PathEqIgnoreSep_2(mut LHS: (), mut RHS: ()) -> () {
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn IterEnd_3(mut B: *const ()) -> *const () {
    'loop_: while ((*B) != (unimplemented!())) {
        B.prefix_inc();
    }
    return B;
}
pub unsafe fn IterEnd_4(mut B: *const ()) -> *const () {
    'loop_: while ((*B) != (unimplemented!())) {
        B.prefix_inc();
    }
    return B;
}
pub unsafe fn IterEnd_5(mut B: *const ()) -> *const () {
    'loop_: while ((*B) != (unimplemented!())) {
        B.prefix_inc();
    }
    return B;
}
pub unsafe fn IterEnd_6(mut B: *const ()) -> *const () {
    'loop_: while ((*B) != (unimplemented!())) {
        B.prefix_inc();
    }
    return B;
}
pub unsafe fn IterEnd_7(mut B: *const ()) -> *const () {
    'loop_: while ((*B) != (unimplemented!())) {
        B.prefix_inc();
    }
    return B;
}
pub unsafe fn StrEnd_8(mut P: *const ()) -> *const () {
    return (unsafe { IterEnd_3(P) });
}
pub unsafe fn StrEnd_9(mut P: *const ()) -> *const () {
    return (unsafe { IterEnd_4(P) });
}
pub unsafe fn StrEnd_10(mut P: *const ()) -> *const () {
    return (unsafe { IterEnd_5(P) });
}
pub unsafe fn StrEnd_11(mut P: *const ()) -> *const () {
    return (unsafe { IterEnd_6(P) });
}
pub unsafe fn StrEnd_12(mut P: *const ()) -> *const () {
    return (unsafe { IterEnd_7(P) });
}
pub static mut PathList_13: std::cell::LazyCell<[MultiStringType; 42]> = std::cell::LazyCell::new(
    || unsafe {
        [
            MultiStringType::new(
                { [0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 1]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 1]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 1]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 1]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"".as_ptr() },
                { (::std::mem::size_of::<[(); 1]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [32 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [32 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [32 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [32 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c" ".as_ptr() },
                { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"//".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c".".as_ptr() },
                { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"..".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 2]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/".as_ptr() },
                { (::std::mem::size_of::<[(); 2]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/foo".as_ptr() },
                { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (), 114 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/bar".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                        114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 9]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                        114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 9]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                        114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 9]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 111 as (), 111 as (), 47 as (), 98 as (), 97 as (),
                        114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 9]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/foo/bar".as_ptr() },
                { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"//net".as_ptr() },
                { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                        111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                        111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                        111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 110 as (), 101 as (), 116 as (), 47 as (), 102 as (),
                        111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"//net/foo".as_ptr() },
                { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"///foo///".as_ptr() },
                { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 13]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 13]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 13]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 47 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        47 as (), 47 as (), 98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 13]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"///foo///bar".as_ptr() },
                { (::std::mem::size_of::<[(); 13]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/.".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"./".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 46 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"/..".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [46 as (), 46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [46 as (), 46 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"../".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/.".as_ptr() },
                { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/..".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/./".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                        97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                        97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                        97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 47 as (), 98 as (),
                        97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 10]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/./bar".as_ptr() },
                { (::std::mem::size_of::<[(); 10]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/../".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        102 as (), 111 as (), 111 as (), 47 as (), 46 as (), 46 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"foo/../bar".as_ptr() },
                { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [99 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [99 as (), 58 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 6]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:foo".as_ptr() },
                { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/foo".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 47 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/foo/bar".as_ptr() },
                { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [112 as (), 114 as (), 110 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [112 as (), 114 as (), 110 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [112 as (), 114 as (), 110 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [112 as (), 114 as (), 110 as (), 58 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 5]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"prn:".as_ptr() },
                { (::std::mem::size_of::<[(); 5]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [99 as (), 58 as (), 92 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 92 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 92 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [99 as (), 58 as (), 92 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 4]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:\\".as_ptr() },
                { (::std::mem::size_of::<[(); 4]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:\\foo".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 102 as (), 111 as (), 111 as (), 92 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 7]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:foo\\".as_ptr() },
                { (::std::mem::size_of::<[(); 7]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:\\foo\\".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 92 as (), 102 as (), 111 as (), 111 as (), 47 as (),
                        0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 8]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:\\foo/".as_ptr() },
                { (::std::mem::size_of::<[(); 8]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        99 as (), 58 as (), 47 as (), 102 as (), 111 as (), 111 as (), 92 as (),
                        98 as (), 97 as (), 114 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 11]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"c:/foo\\bar".as_ptr() },
                { (::std::mem::size_of::<[(); 11]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { [47 as (), 47 as (), 0 as ()].as_ptr() },
                {
                    ((::std::mem::size_of::<[(); 3]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                { c"//".as_ptr() },
                { (::std::mem::size_of::<[(); 3]>() as ()).wrapping_sub(1_()) },
            ),
            MultiStringType::new(
                {
                    [
                        47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                        121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                        101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 82]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                        121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                        101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 82]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                        121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                        101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 82]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    [
                        47 as (), 102 as (), 105 as (), 110 as (), 97 as (), 108 as (), 108 as (),
                        121 as (), 47 as (), 119 as (), 101 as (), 47 as (), 110 as (), 101 as (),
                        101 as (), 100 as (), 47 as (), 111 as (), 110 as (), 101 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        114 as (), 101 as (), 97 as (), 108 as (), 108 as (), 121 as (), 47 as (),
                        108 as (), 111 as (), 110 as (), 103 as (), 47 as (), 115 as (), 116 as (),
                        114 as (), 105 as (), 110 as (), 103 as (), 0 as (),
                    ]
                    .as_ptr()
                },
                {
                    ((::std::mem::size_of::<[(); 82]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ()))
                        as ())
                        .wrapping_sub(1_())
                },
                {
                    c"/finally/we/need/one/really/really/really/really/really/really/really/long/string" .as_ptr()
                },
                { (::std::mem::size_of::<[(); 82]>() as ()).wrapping_sub(1_()) },
            ),
        ]
    },
);
pub static mut PathListSize_14: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (::std::mem::size_of::<[MultiStringType; 42]>() as ())
        .wrapping_div((::std::mem::size_of::<MultiStringType>() as ()))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_16))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_17))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_18)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_15)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_16)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_17)) = 0_();
    }
}
pub static mut outstanding_bytes_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: (),
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        unimplemented!();
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        unimplemented!();
        (*other).moved_from_ = true;
        self.moved_from_ = false;
        return &mut (*(self as *mut support_double_move_tracker));
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        unsafe { support_double_move_tracker::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_output_iterator_const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_const_char_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
pub unsafe fn operator_ne_19(
    x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    y: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_const_char_ptr__Traits_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__Traits_ {}
pub unsafe fn operator_eq_20(
    x: *const cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_,
    y: *const cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_21(
    x: *const cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_,
    y: *const cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_20(
                self as *const cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_,
                other as *const cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_char8_t_ptr__const_char8_t_ptr_ {}
pub unsafe fn operator_eq_22(
    x: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    y: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_23(
    x: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    y: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_22(
                self as *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
                other as *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {}
pub unsafe fn operator_eq_24(
    x: *const cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_,
    y: *const cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_25(
    x: *const cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_,
    y: *const cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_24(
                self as *const cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_,
                other as *const cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_char16_t_ptr__const_char16_t_ptr_ {}
pub unsafe fn operator_eq_26(
    x: *const cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_,
    y: *const cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_27(
    x: *const cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_,
    y: *const cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_26(
                self as *const cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_,
                other as *const cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_char32_t_ptr__const_char32_t_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.it_.offset((n) as isize));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_int_ptr_,
    ) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_int_ptr_,
    ) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IteratorOpCounts {
    pub increments: (),
    pub decrements: (),
    pub zero_moves: (),
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub unsafe fn operator_eq_28(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> () {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut (),
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_28(
                self as *const forward_sized_iterator_int_ptr_,
                other as *const forward_sized_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            forward_sized_iterator_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub unsafe fn operator_add_29(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_30(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_29(_i, n)
    });
}
pub unsafe fn iter_swap_31(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_32(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_33(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_34(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut (),
    iter_moves_: *mut (),
    iter_swaps_: *mut (),
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut (), mut iter_moves: *mut (), mut iter_swaps: *mut ()) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut (), iter_moves: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut (), iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> () {
        unimplemented!();
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> () {
        unimplemented!();
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ()
            as ());
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_34(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_33(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        unsafe {
            adl_Iterator {
                ptr_: std::ptr::null_mut(),
                iter_moves_: std::ptr::null_mut(),
                iter_swaps_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rvalue_iterator_int_ptr_ {
    it_: *mut *mut (),
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        return (((*(self as *const rvalue_iterator_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn RunTestCaseImpl_35(MS: *const MultiStringType) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_8(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_36(MS: *const MultiStringType, mut args: ()) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_8(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_37(MS: *const MultiStringType) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_14(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_9(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_38(MS: *const MultiStringType, mut args: ()) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_14(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_9(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_39(MS: *const MultiStringType) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_13(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_10(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_40(MS: *const MultiStringType, mut args: ()) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_13(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_10(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_41(MS: *const MultiStringType) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_15(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_11(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_42(MS: *const MultiStringType, mut args: ()) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_15(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_11(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_43(MS: *const MultiStringType) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_16(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_12(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCaseImpl_44(MS: *const MultiStringType, mut args: ()) {
    let mut Expect: *const () = (unsafe { MultiStringType::to_pconst_12(&(*MS)) });
    let mut TestPath: *const () = (unsafe { MultiStringType::to_pconst_16(&(*MS)) });
    let mut TestPathEnd: *const () = (unsafe { StrEnd_12(TestPath) });
    let Size: () = ((TestPathEnd as usize - TestPath as usize) / ::std::mem::size_of::<()>()) as ();
    let SSize: () = (((unsafe { StrEnd_8(Expect) }) as usize - Expect as usize)
        / ::std::mem::size_of::<()>()) as ();
    unimplemented!();
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let S: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn RunTestCase_45(MS: *const MultiStringType) {
    (unsafe { RunTestCaseImpl_35(MS) });
    (unsafe { RunTestCaseImpl_36(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_36(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_36(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_36(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_36(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_36(MS, unimplemented!()) });
}
pub unsafe fn RunTestCase_46(MS: *const MultiStringType) {
    (unsafe { RunTestCaseImpl_37(MS) });
    (unsafe { RunTestCaseImpl_38(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_38(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_38(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_38(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_38(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_38(MS, unimplemented!()) });
}
pub unsafe fn RunTestCase_47(MS: *const MultiStringType) {
    (unsafe { RunTestCaseImpl_39(MS) });
    (unsafe { RunTestCaseImpl_40(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_40(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_40(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_40(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_40(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_40(MS, unimplemented!()) });
}
pub unsafe fn RunTestCase_48(MS: *const MultiStringType) {
    (unsafe { RunTestCaseImpl_41(MS) });
    (unsafe { RunTestCaseImpl_42(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_42(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_42(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_42(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_42(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_42(MS, unimplemented!()) });
}
pub unsafe fn RunTestCase_49(MS: *const MultiStringType) {
    (unsafe { RunTestCaseImpl_43(MS) });
    (unsafe { RunTestCaseImpl_44(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_44(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_44(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_44(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_44(MS, unimplemented!()) });
    (unsafe { RunTestCaseImpl_44(MS, unimplemented!()) });
}
pub unsafe fn test_sfinae_50() {
    {
        const _: () = assert!(true, "std::is_constructible<path, It>::value");;
    }
    {
        const _: () = assert!(true, "std::is_constructible<path, It>::value");;
    }
    {
        const _: () = assert!(true, "std::is_constructible<path, It>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<path, It>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_constructible<path, int*>::value");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Traits_51 {}
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
    'loop_: for MS in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut PathList_13)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut PathList_13))
            .as_ptr()
            .offset((42_()) as isize),
    ) {
        let MS: *const MultiStringType = &(*MS);
        (unsafe { RunTestCase_45(MS) });
        (unsafe { RunTestCase_46(MS) });
        (unsafe { RunTestCase_47(MS) });
        (unsafe { RunTestCase_48(MS) });
        (unsafe { RunTestCase_49(MS) });
    }
    (unsafe { test_sfinae_50() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const PathList_13);
    std::cell::LazyCell::force(&*&raw const PathListSize_14);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_15);
    std::cell::LazyCell::force(&*&raw const alloc_count_16);
    std::cell::LazyCell::force(&*&raw const dealloc_count_17);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_18);
}
