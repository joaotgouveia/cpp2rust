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
pub static mut InStr_0: std::cell::LazyCell<MultiStringType> =
    std::cell::LazyCell::new(|| unsafe {
        MultiStringType::new(
            {
                [
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    47 as (), 34 as (), 104 as (), 105 as (), 106 as (), 107 as (), 108 as (),
                    109 as (), 110 as (), 111 as (), 112 as (), 34 as (), 47 as (), 113 as (),
                    114 as (), 115 as (), 116 as (), 117 as (), 118 as (), 119 as (), 120 as (),
                    121 as (), 122 as (), 47 as (), 49 as (), 50 as (), 51 as (), 52 as (),
                    53 as (), 54 as (), 55 as (), 56 as (), 57 as (), 0 as (),
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[(); 41]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                [
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    47 as (), 34 as (), 104 as (), 105 as (), 106 as (), 107 as (), 108 as (),
                    109 as (), 110 as (), 111 as (), 112 as (), 34 as (), 47 as (), 113 as (),
                    114 as (), 115 as (), 116 as (), 117 as (), 118 as (), 119 as (), 120 as (),
                    121 as (), 122 as (), 47 as (), 49 as (), 50 as (), 51 as (), 52 as (),
                    53 as (), 54 as (), 55 as (), 56 as (), 57 as (), 0 as (),
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[(); 41]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                [
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    47 as (), 34 as (), 104 as (), 105 as (), 106 as (), 107 as (), 108 as (),
                    109 as (), 110 as (), 111 as (), 112 as (), 34 as (), 47 as (), 113 as (),
                    114 as (), 115 as (), 116 as (), 117 as (), 118 as (), 119 as (), 120 as (),
                    121 as (), 122 as (), 47 as (), 49 as (), 50 as (), 51 as (), 52 as (),
                    53 as (), 54 as (), 55 as (), 56 as (), 57 as (), 0 as (),
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[(); 41]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                [
                    97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                    47 as (), 34 as (), 104 as (), 105 as (), 106 as (), 107 as (), 108 as (),
                    109 as (), 110 as (), 111 as (), 112 as (), 34 as (), 47 as (), 113 as (),
                    114 as (), 115 as (), 116 as (), 117 as (), 118 as (), 119 as (), 120 as (),
                    121 as (), 122 as (), 47 as (), 49 as (), 50 as (), 51 as (), 52 as (),
                    53 as (), 54 as (), 55 as (), 56 as (), 57 as (), 0 as (),
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[(); 41]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { c"abcdefg/\"hijklmnop\"/qrstuvwxyz/123456789".as_ptr() },
            { (::std::mem::size_of::<[(); 41]>() as ()).wrapping_sub(1_()) },
        )
    });
pub static mut OutStr_1: std::cell::LazyCell<MultiStringType> =
    std::cell::LazyCell::new(|| unsafe {
        MultiStringType::new(
            {
                [
                    34 as (), 97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (),
                    103 as (), 47 as (), 92 as (), 34 as (), 104 as (), 105 as (), 106 as (),
                    107 as (), 108 as (), 109 as (), 110 as (), 111 as (), 112 as (), 92 as (),
                    34 as (), 47 as (), 113 as (), 114 as (), 115 as (), 116 as (), 117 as (),
                    118 as (), 119 as (), 120 as (), 121 as (), 122 as (), 47 as (), 49 as (),
                    50 as (), 51 as (), 52 as (), 53 as (), 54 as (), 55 as (), 56 as (), 57 as (),
                    34 as (), 0 as (),
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[(); 45]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                [
                    34 as (), 97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (),
                    103 as (), 47 as (), 92 as (), 34 as (), 104 as (), 105 as (), 106 as (),
                    107 as (), 108 as (), 109 as (), 110 as (), 111 as (), 112 as (), 92 as (),
                    34 as (), 47 as (), 113 as (), 114 as (), 115 as (), 116 as (), 117 as (),
                    118 as (), 119 as (), 120 as (), 121 as (), 122 as (), 47 as (), 49 as (),
                    50 as (), 51 as (), 52 as (), 53 as (), 54 as (), 55 as (), 56 as (), 57 as (),
                    34 as (), 0 as (),
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[(); 45]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                [
                    34 as (), 97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (),
                    103 as (), 47 as (), 92 as (), 34 as (), 104 as (), 105 as (), 106 as (),
                    107 as (), 108 as (), 109 as (), 110 as (), 111 as (), 112 as (), 92 as (),
                    34 as (), 47 as (), 113 as (), 114 as (), 115 as (), 116 as (), 117 as (),
                    118 as (), 119 as (), 120 as (), 121 as (), 122 as (), 47 as (), 49 as (),
                    50 as (), 51 as (), 52 as (), 53 as (), 54 as (), 55 as (), 56 as (), 57 as (),
                    34 as (), 0 as (),
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[(); 45]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            {
                [
                    34 as (), 97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (),
                    103 as (), 47 as (), 92 as (), 34 as (), 104 as (), 105 as (), 106 as (),
                    107 as (), 108 as (), 109 as (), 110 as (), 111 as (), 112 as (), 92 as (),
                    34 as (), 47 as (), 113 as (), 114 as (), 115 as (), 116 as (), 117 as (),
                    118 as (), 119 as (), 120 as (), 121 as (), 122 as (), 47 as (), 49 as (),
                    50 as (), 51 as (), 52 as (), 53 as (), 54 as (), 55 as (), 56 as (), 57 as (),
                    34 as (), 0 as (),
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[(); 45]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                    .wrapping_sub(1_())
            },
            { c"\"abcdefg/\\\"hijklmnop\\\"/qrstuvwxyz/123456789\"".as_ptr() },
            { (::std::mem::size_of::<[(); 45]>() as ()).wrapping_sub(1_()) },
        )
    });
pub unsafe fn TestOutput_2() {
    let mut input: *const () = (unsafe {
        MultiStringType::to_pconst_12(&(*std::cell::LazyCell::force_mut(&mut *&raw mut InStr_0)))
    });
    let mut expected_output: *const () = ((unsafe {
        MultiStringType::to_pconst_12(&(*std::cell::LazyCell::force_mut(&mut *&raw mut OutStr_1)))
    }) as *const ());
    let dir: () = unimplemented!();
    let mut stream: () = unimplemented!();
    let result: *mut () = &mut (*unimplemented!());
    unimplemented!();
    unimplemented!();
}
pub unsafe fn TestOutput_3() {
    let mut input: *const () = (unsafe {
        MultiStringType::to_pconst_12(&(*std::cell::LazyCell::force_mut(&mut *&raw mut InStr_0)))
    });
    let mut expected_output: *const () = ((unsafe {
        MultiStringType::to_pconst_13(&(*std::cell::LazyCell::force_mut(&mut *&raw mut OutStr_1)))
    }) as *const ());
    let dir: () = unimplemented!();
    let mut stream: () = unimplemented!();
    let result: *mut () = &mut (*unimplemented!());
    unimplemented!();
    unimplemented!();
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
    (unsafe { TestOutput_2() });
    (unsafe { TestOutput_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const InStr_0);
    std::cell::LazyCell::force(&*&raw const OutStr_1);
}
