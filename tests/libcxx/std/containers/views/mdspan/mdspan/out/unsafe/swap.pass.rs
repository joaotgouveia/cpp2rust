extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MinimalElementType {
    pub val: (),
}
impl MinimalElementType {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct not_extents_constructible_tag {}
static mut Wrap_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
pub unsafe fn operator_eq_1(
    lhs: *const layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__,
    rhs: *const layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__,
) -> () {
    return unimplemented!();
}
pub unsafe fn swap_2(
    x: *mut layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__,
    y: *mut layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__,
) {
    unimplemented!();
    {
        ( * ( unsafe { layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__::swap_counter ( ) } )  )  .postfix_inc() ;
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
    pub unsafe fn copy_from(
        other : *const layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__,
    ) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_1(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn extents(&self) -> *const () {
        return &self.extents_;
    }
    pub unsafe fn swap_counter() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut value_3));
    }
}
impl std::cmp::PartialEq
    for layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(self as *const layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__, other as *const layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__)
        }
    }
}
impl std::cmp::Eq
    for layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__
{
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__)
        }
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct layout_wrapping_integral_4_ {}
static mut value_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct always_convertible_layout {}
pub unsafe fn test_swap_4(mut a: (), mut b: ()) {
    let mut org_a: () = unimplemented!();
    let mut org_b: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!();
        unimplemented!();
    }
    { {} }
}
pub unsafe fn test_swap_5(mut a: (), mut b: ()) {
    let mut org_a: () = unimplemented!();
    let mut org_b: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!();
        unimplemented!();
    }
    {
        {
            unimplemented!();
        }
    }
}
pub unsafe fn test_6() -> () {
    let mut data_a: [(); 1024] = [unimplemented!(); 1024];
    let mut data_b: [(); 1024] = [unimplemented!(); 1024];
    {
        let mut a: () = unimplemented!();
        let mut b: () = unimplemented!();
        (unsafe { test_swap_4(unimplemented!(), unimplemented!()) });
    }
    {
        let mut map_a : layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ = { let mut __tmp_0 : ()  = unimplemented!() ; layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ :: new_1 ( { & mut __tmp_0 } , { <not_extents_constructible_tag >::default() } , )  } ;
        let mut map_b : layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ = { let mut __tmp_1 : ()  = unimplemented!() ; layout_wrapping_integral_4__mapping_std_extents_int__4__18446744073709551615__ :: new_1 ( { & mut __tmp_1 } , { <not_extents_constructible_tag >::default() } , )  } ;
        let mut a: () = unimplemented!();
        let mut b: () = unimplemented!();
        (unsafe { test_swap_5(unimplemented!(), unimplemented!()) });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Wrap_0);
}
