extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_insert_iterator_NoIteratorAlias___std_insert_iterator_NoIteratorAlias__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoIteratorAlias_value_type {
    pub x: (),
}
impl NoIteratorAlias_value_type {
    pub unsafe fn new(mut d: ()) -> Self {
        let mut this = Self { x: d };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.x;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NoIteratorAlias {
    pub data_: [(); 3],
}
impl NoIteratorAlias {
    pub unsafe fn insert_2_struct_NoIteratorAlias_value_type(
        &mut self,
        mut pos: *mut (),
        mut value: NoIteratorAlias_value_type,
    ) -> *mut () {
        const _: () = assert!(true, "std::is_same_v<T, value_type>");;
        (*pos) = (unsafe { NoIteratorAlias_value_type::to_(&value) });
        return pos;
    }
}
impl Default for NoIteratorAlias {
    fn default() -> Self {
        unsafe {
            NoIteratorAlias {
                data_: [unimplemented!(), unimplemented!(), unimplemented!()],
            }
        }
    }
}
const _: () = assert!(
    true,
    "std::is_constructible_v<std::insert_iterator<NoIteratorAlias>, NoIteratorAlias&, double*>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::insert_iterator<NoIteratorAlias>, NoIteratorAlias&, NoIteratorAlias::value_type*>"
);
pub unsafe fn test_1() -> () {
    let mut c: NoIteratorAlias = <NoIteratorAlias>::default();
    let mut half: () = 0.5_f64;
    let mut it: () =
        std_insert_iterator_NoIteratorAlias_::new({ &mut c }, { c.data_.as_mut_ptr() });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
