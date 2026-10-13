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
pub type std_byte = u8;
pub unsafe fn my_foo_0(mut p: *mut ::libc::c_void) -> i32 {
    return (*(p as *mut i32));
}
pub unsafe fn foo_1(
    mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    mut pi: *mut i32,
) -> i32 {
    return (unsafe { (fn_).unwrap()((pi as *mut ::libc::c_void)) });
}
pub unsafe fn twice_2(mut x: u64) -> u64 {
    return (x).wrapping_mul(2_u64);
}
pub unsafe fn twice_in_place_3(x: *mut u64) -> u64 {
    (*x) = (*x).wrapping_mul(2_u64);
    return (*x);
}
pub unsafe fn ret_size_4(mut v: i32) -> usize {
    return (((v) + (1)) as usize);
}
pub unsafe fn call_fn_5(mut f: Option<unsafe fn(i32) -> usize>, mut v: i32) -> usize {
    return (unsafe { (f).unwrap()(v) }).wrapping_mul(2_usize);
}
pub unsafe fn identity_hash_6(mut v: bool) -> usize {
    return (v as usize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HashHolder_7 {
    pub h: Option<unsafe fn(bool) -> u64>,
}
impl HashHolder_7 {
    pub unsafe fn new(h: *const Option<unsafe fn(bool) -> u64>) -> Self {
        let mut this = Self { h: (*h) };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HashHolder_8 {
    pub h: Option<unsafe fn(u64) -> u64>,
}
impl HashHolder_8 {
    pub unsafe fn new(h: *const Option<unsafe fn(u64) -> u64>) -> Self {
        let mut this = Self { h: (*h) };
        this
    }
}
pub unsafe fn hash_identity_9(mut v: u64) -> usize {
    return (v as usize);
}
pub unsafe fn test_hash_10(mut v: u64) {
    let mut hh: HashHolder_8 = {
        let mut __tmp_0: Option<unsafe fn(u64) -> u64> = std::mem::transmute::<
            Option<unsafe fn(u64) -> usize>,
            Option<unsafe fn(u64) -> u64>,
        >(Some(hash_identity_9));
        HashHolder_8::new({ &mut __tmp_0 })
    };
    (if ((unsafe { (hh.h).unwrap()(v) }) == (7_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"hh.h(v) == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"void test_hash(T) [T = unsigned long]".as_ptr(),
            )
        });
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32> = None;
    (if (fn_).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"fn == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((fn_) != (Some(my_foo_0))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"fn != my_foo".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    fn_ = Some(my_foo_0);
    (if !((fn_).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"fn != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((fn_) == (Some(my_foo_0))) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"fn == my_foo".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut a: i32 = 10;
    (if ((unsafe { foo_1(fn_, (&mut a as *mut i32)) }) == (a)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"foo(fn, &a) == a".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ul_fn: Option<unsafe fn(u64) -> u64> = (Some(twice_2));
    let mut n: usize = 21_usize;
    let mut r: usize = ((unsafe { (ul_fn).unwrap()((n as u64)) }) as usize);
    (if ((r) == (42_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"r == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ul_ref_fn: Option<unsafe fn(*mut u64) -> u64> = (Some(twice_in_place_3));
    let mut m: usize = 21_usize;
    let mut q: usize =
        ((unsafe { (ul_ref_fn).unwrap()(&mut *(&raw mut m).cast::<u64>()) }) as usize);
    (if ((q) == (42_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"q == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((m) == (42_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"m == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { call_fn_5(Some(ret_size_4), 3) }) == (8_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"call_fn(ret_size<int>, 3) == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut hh: HashHolder_7 = {
        let mut __tmp_1: Option<unsafe fn(bool) -> u64> = std::mem::transmute::<
            Option<unsafe fn(bool) -> usize>,
            Option<unsafe fn(bool) -> u64>,
        >(Some(identity_hash_6));
        HashHolder_7::new({ &mut __tmp_1 })
    };
    (if ((unsafe { (hh.h).unwrap()(true) }) == (1_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_11(
                c"hh.h(true) == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { test_hash_10(7_u64) });
    return 0;
}
pub unsafe fn __assert_fail_11(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
