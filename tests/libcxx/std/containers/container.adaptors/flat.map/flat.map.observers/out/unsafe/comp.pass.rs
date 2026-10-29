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
pub struct test_macros_detail_is_same_std_less_int___std_less_int__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_function_bool__int__int____std_function_bool__int__int___
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_less_void___std_less_void__ {}
pub unsafe fn test_4() -> () {
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!std::is_same_v<M::value_compare, Comp>");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut kc: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut vc : std__Flat_map_impl_int__char__std_less_int___std_vector_int___std_vector_char___false__value_compare = unimplemented!()  ;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut comp: () = unimplemented!();
        let mut m : () = std_flat_map_int__int__std_function_bool__int__int____std_vector_int__std_allocator_int____std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!() } , { & comp } , )  ;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut kc: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut vc : std__Flat_map_impl_int__int__std_function_bool__int__int____std_vector_int___std_vector_int___false__value_compare = unimplemented!()  ;
        let mut a: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "!noexcept(vc(a, a))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        let mut kc: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut vc : std__Flat_map_impl_int__int__std_less_void___std_vector_int___std_vector_int___false__value_compare = unimplemented!()  ;
        let mut a: () = unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut comp: () = unimplemented!();
        let mut m : () = std_flat_map_std_vector_int__std_allocator_int____int__std_function_bool__const_std_vector_int__std_allocator_int___ref__const_std_vector_int__std_allocator_int___ref____std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int______std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!() } , { & comp } , )  ;
        let mut vc : std__Flat_map_impl_std_vector_int___int__std_function_bool__const_std_vector_int__ref__const_std_vector_int__ref____std_vector_std_vector_int____std_vector_int___false__value_compare = unimplemented!()  ;
        const _: () = assert!(( ( ::std::mem::size_of::<std__Flat_map_impl_std_vector_int___int__std_function_bool__const_std_vector_int__ref__const_std_vector_int__ref____std_vector_std_vector_int____std_vector_int___false__value_compare>() ) >= ( ::std::mem::size_of::<()>() ) ) , "sizeof(vc) >= sizeof(Comp)");;
        unimplemented!();
        m = {
            let mut __tmp_0: () = unimplemented!();
            std_flat_map_std_vector_int__std_allocator_int____int__std_function_bool__const_std_vector_int__std_allocator_int___ref__const_std_vector_int__std_allocator_int___ref____std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int______std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!() } , { & mut __tmp_0 } , )
        };
        unimplemented!();
        let mut a: () = unimplemented!();
        let mut b: () = unimplemented!();
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_4() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
