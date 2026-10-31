extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_const_container_0(c: *const Vec<i32>) {
    assert!(((unimplemented!()) == ((*c).as_ptr())));
}
pub unsafe fn test_const_container_1(c: *const Vec<i32>) {
    assert!(((unimplemented!()) == ((*c).as_ptr())));
}
pub unsafe fn test_const_container_2(c: *const ()) {
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_const_container_3(c: *const Vec<i32>) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_container_4(c: *mut Vec<i32>) {
    assert!(((unimplemented!()) == ((*c).as_mut_ptr())));
}
pub unsafe fn test_container_5(c: *mut Vec<i32>) {
    assert!(((unimplemented!()) == ((*c).as_mut_ptr())));
}
pub unsafe fn test_container_6(c: *mut ()) {
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_container_7(c: *mut Vec<i32>) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_const_array_8(array: *const [i32; 3]) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (&(*array)[(0) as usize] as *const i32)));
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
    let mut a: Vec<i32> = std::array::from_fn::<_, 1, _>(|_| 0_i32).to_vec();
    (*{
        let __idx = 0_usize as usize;
        let __v: &mut Vec<i32> = &mut a;
        __v.as_mut_ptr().add(__idx)
    }) = 3;
    let mut il: Vec<i32> = Vec::from([4]);
    (unsafe { test_container_4(&mut v) });
    (unsafe { test_container_5(&mut a) });
    (unsafe { test_container_7(&mut il) });
    (unsafe { test_const_container_0(&v) });
    (unsafe { test_const_container_1(&a) });
    (unsafe { test_const_container_3(&il) });
    let mut sv: () = unimplemented!();
    (unsafe { test_container_6(&mut sv) });
    (unsafe { test_const_container_2(&sv) });
    (unsafe { test_const_array_8(&(*std::cell::LazyCell::force_mut(&mut *&raw mut arrA_9))) });
    return 0;
}
static mut arrA_9: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [1, 2, 3] });
pub unsafe fn __cpp2rust_init_globals() {}
