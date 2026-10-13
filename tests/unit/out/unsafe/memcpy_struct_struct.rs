extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Entry {
    pub bits: u8,
    pub value: u16,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut table: [Entry; 8] = [
        Entry {
            bits: 1_u8,
            value: 4369_u16,
        },
        Entry {
            bits: 2_u8,
            value: 8738_u16,
        },
        Entry {
            bits: 3_u8,
            value: 13107_u16,
        },
        Entry {
            bits: 4_u8,
            value: 17476_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
    ];
    let mut table_size: usize = 4_usize;
    (unsafe {
        let ___dest: *mut ::libc::c_void =
            ((&mut table[(table_size)] as *mut Entry) as *mut ::libc::c_void);
        let ___src: *const ::libc::c_void =
            ((&mut table[(0) as usize] as *mut Entry) as *const ::libc::c_void);
        let ___n: usize =
            ((table_size as u64).wrapping_mul((::std::mem::size_of::<Entry>() as u64)) as usize);
        memcpy_0(___dest, ___src, ___n)
    });
    (if ((table[(4) as usize].bits as i32) == (1)) && ((table[(4) as usize].value as i32) == (4369))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"table[4].bits == 1 && table[4].value == 0x1111".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((table[(5) as usize].bits as i32) == (2)) && ((table[(5) as usize].value as i32) == (8738))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"table[5].bits == 2 && table[5].value == 0x2222".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((table[(6) as usize].bits as i32) == (3))
        && ((table[(6) as usize].value as i32) == (13107))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"table[6].bits == 3 && table[6].value == 0x3333".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((table[(7) as usize].bits as i32) == (4))
        && ((table[(7) as usize].value as i32) == (17476))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"table[7].bits == 4 && table[7].value == 0x4444".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
pub unsafe fn memcpy_0(
    mut __dest: *mut ::libc::c_void,
    mut __src: *const ::libc::c_void,
    mut __n: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
