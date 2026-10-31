extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long__long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct short_container {}
impl short_container {
    pub unsafe fn size(&self) -> u16 {
        return 60000_u16;
    }
}
pub unsafe fn test_container_1(c: *mut Vec<i32>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == ((*c).len() as i64)));
}
pub unsafe fn test_container_2(c: *mut ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == (unimplemented!() as i64)));
}
pub unsafe fn test_container_3(c: *mut Vec<i32>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == ((*c).len() as i64)));
}
pub unsafe fn test_container_4(c: *mut ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == (unimplemented!() as i64)));
}
pub unsafe fn test_const_container_5(c: *const Vec<i32>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == ((*c).len() as i64)));
}
pub unsafe fn test_const_container_6(c: *const ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == (unimplemented!() as i64)));
}
pub unsafe fn test_const_container_7(c: *const Vec<i32>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == ((*c).len() as i64)));
}
pub unsafe fn test_const_container_8(c: *const ()) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == (unimplemented!() as i64)));
}
pub unsafe fn test_const_container_9(c: *const Vec<i32>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == ((*c).len() as i64)));
}
pub unsafe fn test_container_10(c: *mut Vec<i32>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(((unimplemented!()) == ((*c).len() as i64)));
}
pub unsafe fn test_const_array_11(array: *const [i32; 3]) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(array))>");;
    assert!(((unimplemented!() as u64) == (3_u64)));
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
    let mut v: Vec<i32> = Vec::new();
    {
        let __a1 = 1;
        v.push(__a1)
    };
    let mut l: () = unimplemented!();
    unimplemented!();
    let mut a: Vec<i32> = std::array::from_fn::<_, 1, _>(|_| 0_i32).to_vec();
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut a;
        __v.as_mut_ptr().add(__idx)
    }) = 3;
    let mut il: Vec<i32> = Vec::from([4]);
    (unsafe { test_container_1(&mut v) });
    const _: () = assert!(true, "");;
    (unsafe { test_container_2(&mut l) });
    const _: () = assert!(true, "");;
    (unsafe { test_container_3(&mut a) });
    const _: () = assert!(true, "");;
    (unsafe { test_container_10(&mut il) });
    const _: () = assert!(true, "");;
    (unsafe { test_const_container_5(&v) });
    (unsafe { test_const_container_6(&l) });
    (unsafe { test_const_container_7(&a) });
    (unsafe { test_const_container_9(&il) });
    let mut sv: () = unimplemented!();
    (unsafe { test_container_4(&mut sv) });
    const _: () = assert!(true, "");;
    (unsafe { test_const_container_8(&sv) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(arrA))>");;
    (unsafe { test_const_array_11(&(*std::cell::LazyCell::force_mut(&mut *&raw mut arrA_12))) });
    let mut sc: short_container = <short_container>::default();
    const _: () = assert!(
        true,
        "std::is_signed_v<                      decltype(std::ssize(sc))>"
    );;
    const _: () = assert!(
        true,
        "std::numeric_limits<                   decltype(std::ssize(sc))>::max()  > 60000"
    );;
    const _: () = assert!(
        true,
        "std::numeric_limits<std::make_signed_t<decltype(std:: size(sc))>>::max() < 60000"
    );;
    assert!(((unimplemented!()) == (60000_i64)));
    const _: () = assert!(true, "");;
    return 0;
}
static mut arrA_12: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [1, 2, 3] });
pub unsafe fn __cpp2rust_init_globals() {}
