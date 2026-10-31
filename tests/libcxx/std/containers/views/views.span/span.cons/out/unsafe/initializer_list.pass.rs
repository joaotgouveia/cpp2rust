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
#[cpp_from(*mut Sink , new_1, value)]
pub struct Sink {}
impl Sink {
    pub unsafe fn new_1(mut _a0: *mut Sink) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn count_0(mut sp: ()) -> usize {
    return (unimplemented!() as usize);
}
pub unsafe fn count_n_1(mut sp: ()) -> usize {
    return (unimplemented!() as usize);
}
pub unsafe fn test_2() -> bool {
    let mut a: [Sink; 10] = [<Sink>::default(); 10];
    assert!(((unsafe { count_0(unimplemented!(),) }) == (10_usize)));
    assert!(((unsafe { count_0(unimplemented!(),) }) == (10_usize)));
    assert!(((unsafe { count_n_1(unimplemented!(),) }) == (10_usize)));
    return true;
}
pub unsafe fn three_3(mut sp: ()) -> i32 {
    return (unimplemented!() as i32);
}
pub unsafe fn four_4(mut sp: ()) -> i32 {
    return (unimplemented!() as i32);
}
pub unsafe fn test_P2447R4_annex_c_examples_5() -> bool {
    {
        let mut a: [*mut ::libc::c_void; 10] = [std::ptr::null_mut(); 10];
        assert!(((unsafe { three_3(unimplemented!(),) }) == (0)));
    }
    {
        let mut a: [(); 10] = unimplemented!();
        assert!(((unsafe { four_4(unimplemented!(),) }) == (10)));
    }
    return true;
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
    assert!((unsafe { test_2() }));
    const _: () = assert!(true, "test()");;
    assert!((unsafe { test_P2447R4_annex_c_examples_5() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
