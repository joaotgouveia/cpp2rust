extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(*const libc::c_char , to_pconstlibcc_char)]
#[cpp_into(*const i32 , to_pconsti32)]
#[cpp_into(*const u8 , to_pconstu8)]
#[cpp_into(*const u16 , to_pconstu16)]
#[cpp_into(*const u32 , to_pconstu32)]
pub struct MultiStringType {
    pub w_: *const i32,
    pub wn_: usize,
    pub u8_: *const u8,
    pub u8n_: usize,
    pub u16_: *const u16,
    pub u16n_: usize,
    pub u32_: *const u32,
    pub u32n_: usize,
    pub s_: *const libc::c_char,
    pub sn_: usize,
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const i32,
        mut wn: usize,
        mut u8: *const u8,
        mut u8n: usize,
        mut u16: *const u16,
        mut u16n: usize,
        mut u32: *const u32,
        mut u32n: usize,
        mut s: *const libc::c_char,
        mut sn: usize,
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
    pub unsafe fn as_ptr_2(&self, mut _a0: *const libc::c_char) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const i32) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const u8) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const u16) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const u32) -> *const u32 {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const libc::c_char) -> usize {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const i32) -> usize {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const u8) -> usize {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const u16) -> usize {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const u32) -> usize {
        return self.u32n_;
    }
    pub unsafe fn to_pconstlibcc_char(&self) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn to_pconsti32(&self) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn to_pconstu8(&self) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn to_pconstu16(&self) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn to_pconstu32(&self) -> *const u32 {
        return self.u32_;
    }
}
pub static mut InStr_0: std::cell::LazyCell<MultiStringType> =
    std::cell::LazyCell::new(|| unsafe {
        MultiStringType::new(
            {
                [
                    97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                    103 as i32, 47 as i32, 34 as i32, 104 as i32, 105 as i32, 106 as i32,
                    107 as i32, 108 as i32, 109 as i32, 110 as i32, 111 as i32, 112 as i32,
                    34 as i32, 47 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32,
                    117 as i32, 118 as i32, 119 as i32, 120 as i32, 121 as i32, 122 as i32,
                    47 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32, 53 as i32, 54 as i32,
                    55 as i32, 56 as i32, 57 as i32, 0 as i32,
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[i32; 41]>() as usize)
                    .wrapping_div((::std::mem::size_of::<i32>() as usize))
                    as usize)
                    .wrapping_sub(1_usize)
            },
            {
                [
                    97 as u8, 98 as u8, 99 as u8, 100 as u8, 101 as u8, 102 as u8, 103 as u8,
                    47 as u8, 34 as u8, 104 as u8, 105 as u8, 106 as u8, 107 as u8, 108 as u8,
                    109 as u8, 110 as u8, 111 as u8, 112 as u8, 34 as u8, 47 as u8, 113 as u8,
                    114 as u8, 115 as u8, 116 as u8, 117 as u8, 118 as u8, 119 as u8, 120 as u8,
                    121 as u8, 122 as u8, 47 as u8, 49 as u8, 50 as u8, 51 as u8, 52 as u8,
                    53 as u8, 54 as u8, 55 as u8, 56 as u8, 57 as u8, 0 as u8,
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[u8; 41]>() as usize)
                    .wrapping_div((::std::mem::size_of::<u8>() as usize)) as usize)
                    .wrapping_sub(1_usize)
            },
            {
                [
                    97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16,
                    103 as u16, 47 as u16, 34 as u16, 104 as u16, 105 as u16, 106 as u16,
                    107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16, 112 as u16,
                    34 as u16, 47 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16,
                    117 as u16, 118 as u16, 119 as u16, 120 as u16, 121 as u16, 122 as u16,
                    47 as u16, 49 as u16, 50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16,
                    55 as u16, 56 as u16, 57 as u16, 0 as u16,
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[u16; 41]>() as usize)
                    .wrapping_div((::std::mem::size_of::<u16>() as usize))
                    as usize)
                    .wrapping_sub(1_usize)
            },
            {
                [
                    97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 47 as u32, 34 as u32, 104 as u32, 105 as u32, 106 as u32,
                    107 as u32, 108 as u32, 109 as u32, 110 as u32, 111 as u32, 112 as u32,
                    34 as u32, 47 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32,
                    117 as u32, 118 as u32, 119 as u32, 120 as u32, 121 as u32, 122 as u32,
                    47 as u32, 49 as u32, 50 as u32, 51 as u32, 52 as u32, 53 as u32, 54 as u32,
                    55 as u32, 56 as u32, 57 as u32, 0 as u32,
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[u32; 41]>() as usize)
                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                    as usize)
                    .wrapping_sub(1_usize)
            },
            { c"abcdefg/\"hijklmnop\"/qrstuvwxyz/123456789".as_ptr() },
            { (::std::mem::size_of::<[libc::c_char; 41]>() as usize).wrapping_sub(1_usize) },
        )
    });
pub static mut OutStr_1: std::cell::LazyCell<MultiStringType> =
    std::cell::LazyCell::new(|| unsafe {
        MultiStringType::new(
            {
                [
                    34 as i32, 97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                    103 as i32, 47 as i32, 92 as i32, 34 as i32, 104 as i32, 105 as i32,
                    106 as i32, 107 as i32, 108 as i32, 109 as i32, 110 as i32, 111 as i32,
                    112 as i32, 92 as i32, 34 as i32, 47 as i32, 113 as i32, 114 as i32,
                    115 as i32, 116 as i32, 117 as i32, 118 as i32, 119 as i32, 120 as i32,
                    121 as i32, 122 as i32, 47 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32,
                    53 as i32, 54 as i32, 55 as i32, 56 as i32, 57 as i32, 34 as i32, 0 as i32,
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[i32; 45]>() as usize)
                    .wrapping_div((::std::mem::size_of::<i32>() as usize))
                    as usize)
                    .wrapping_sub(1_usize)
            },
            {
                [
                    34 as u8, 97 as u8, 98 as u8, 99 as u8, 100 as u8, 101 as u8, 102 as u8,
                    103 as u8, 47 as u8, 92 as u8, 34 as u8, 104 as u8, 105 as u8, 106 as u8,
                    107 as u8, 108 as u8, 109 as u8, 110 as u8, 111 as u8, 112 as u8, 92 as u8,
                    34 as u8, 47 as u8, 113 as u8, 114 as u8, 115 as u8, 116 as u8, 117 as u8,
                    118 as u8, 119 as u8, 120 as u8, 121 as u8, 122 as u8, 47 as u8, 49 as u8,
                    50 as u8, 51 as u8, 52 as u8, 53 as u8, 54 as u8, 55 as u8, 56 as u8, 57 as u8,
                    34 as u8, 0 as u8,
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[u8; 45]>() as usize)
                    .wrapping_div((::std::mem::size_of::<u8>() as usize)) as usize)
                    .wrapping_sub(1_usize)
            },
            {
                [
                    34 as u16, 97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16,
                    103 as u16, 47 as u16, 92 as u16, 34 as u16, 104 as u16, 105 as u16,
                    106 as u16, 107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16,
                    112 as u16, 92 as u16, 34 as u16, 47 as u16, 113 as u16, 114 as u16,
                    115 as u16, 116 as u16, 117 as u16, 118 as u16, 119 as u16, 120 as u16,
                    121 as u16, 122 as u16, 47 as u16, 49 as u16, 50 as u16, 51 as u16, 52 as u16,
                    53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16, 34 as u16, 0 as u16,
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[u16; 45]>() as usize)
                    .wrapping_div((::std::mem::size_of::<u16>() as usize))
                    as usize)
                    .wrapping_sub(1_usize)
            },
            {
                [
                    34 as u32, 97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                    103 as u32, 47 as u32, 92 as u32, 34 as u32, 104 as u32, 105 as u32,
                    106 as u32, 107 as u32, 108 as u32, 109 as u32, 110 as u32, 111 as u32,
                    112 as u32, 92 as u32, 34 as u32, 47 as u32, 113 as u32, 114 as u32,
                    115 as u32, 116 as u32, 117 as u32, 118 as u32, 119 as u32, 120 as u32,
                    121 as u32, 122 as u32, 47 as u32, 49 as u32, 50 as u32, 51 as u32, 52 as u32,
                    53 as u32, 54 as u32, 55 as u32, 56 as u32, 57 as u32, 34 as u32, 0 as u32,
                ]
                .as_ptr()
            },
            {
                ((::std::mem::size_of::<[u32; 45]>() as usize)
                    .wrapping_div((::std::mem::size_of::<u32>() as usize))
                    as usize)
                    .wrapping_sub(1_usize)
            },
            { c"\"abcdefg/\\\"hijklmnop\\\"/qrstuvwxyz/123456789\"".as_ptr() },
            { (::std::mem::size_of::<[libc::c_char; 45]>() as usize).wrapping_sub(1_usize) },
        )
    });
pub unsafe fn TestOutput_2() {
    let mut input: *const libc::c_char = (unsafe {
        MultiStringType::to_pconstlibcc_char(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut InStr_0)),
        )
    });
    let mut expected_output: *const libc::c_char = ((unsafe {
        MultiStringType::to_pconstlibcc_char(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut OutStr_1)),
        )
    }) as *const libc::c_char);
    let dir: () = unimplemented!();
    let mut stream: () = unimplemented!();
    let result: *mut ::std::fs::File = &mut (*unimplemented!());
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(
                expected_output,
                libcc2rs::c_str_len(expected_output) + 1
            )
            .to_vec()
    );
    assert!(((result) == (&mut stream as *mut ())));
}
pub unsafe fn TestOutput_3() {
    let mut input: *const libc::c_char = (unsafe {
        MultiStringType::to_pconstlibcc_char(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut InStr_0)),
        )
    });
    let mut expected_output: *const i32 = ((unsafe {
        MultiStringType::to_pconsti32(&(*std::cell::LazyCell::force_mut(&mut *&raw mut OutStr_1)))
    }) as *const i32);
    let dir: () = unimplemented!();
    let mut stream: () = unimplemented!();
    let result: *mut ::std::fs::File = &mut (*unimplemented!());
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(
                expected_output,
                libcc2rs::c_str_len(expected_output) + 1
            )
            .to_vec()
    );
    assert!(((result) == (&mut stream as *mut ())));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { TestOutput_2() });
    (unsafe { TestOutput_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const InStr_0);
    std::cell::LazyCell::force(&*&raw const OutStr_1);
}
