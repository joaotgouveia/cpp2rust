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
pub struct test_macros_detail_is_same_std_filesystem_perms__std_filesystem_perms_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_filesystem_perms_ref__std_filesystem_perms_ref_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct check_bitmask_type_std_filesystem_perms__std_filesystem_perms_group_all__std_filesystem_perms_owner_all__unsigned_int__56__448__0__std_filesystem_perms_none_
{}
impl check_bitmask_type_std_filesystem_perms__std_filesystem_perms_group_all__std_filesystem_perms_owner_all__unsigned_int__56__448__0__std_filesystem_perms_none_ { pub unsafe fn dcast ( mut e : () , ) -> u32 { return ( e as u32 )  ;
 } pub unsafe fn unpromote ( mut promoted : u32 , ) -> u32 { return ( promoted as u32 )  ;
 } pub unsafe fn check ( ) -> bool { { let mut ValRef : () = ( ( 56_u32 ) as () ) ;
  ;
 ;
 const _: () = assert!(true, ""); ;
 ;
 const _: () = assert!(true, ""); ;
 ;
 const _: () = assert!(true, ""); ;
 ;
 const _: () = assert!(true, ""); ;
 ;
 const _: () = assert!(true, ""); ;
 ;
 const _: () = assert!(true, ""); ;
 ;
 const _: () = assert!(true, ""); ;
 ;
 } const _: () = assert!(true, "(Val1 & Zero) == Zero"); ;
 ;
 const _: () = assert!(true, "(Val1 & Val1) == Val1"); ;
 ;
 const _: () = assert!(true, "dcast(Val1 & Val2) == (UVal1 & UVal2)"); ;
 ;
 const _: () = assert!(true, "(Val1 | Zero) == Val1"); ;
 ;
 const _: () = assert!(true, "dcast(Val1 | Val2) == (UVal1 | UVal2)"); ;
 ;
 const _: () = assert!(true, "(Val1 ^ Zero) == Val1"); ;
 ;
 const _: () = assert!(true, "dcast(Val1 ^ Val2) == (UVal1 ^ UVal2)"); ;
 ;
 const _: () = assert!(true, "dcast(~Zero) == unpromote(~UZero)"); ;
 ;
 const _: () = assert!(true, "dcast(~Val1) == unpromote(~UVal1)"); ;
 ;
 { let mut e : () = ( ( 56_u32 ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( unsafe { check_bitmask_type_std_filesystem_perms__std_filesystem_perms_group_all__std_filesystem_perms_owner_all__unsigned_int__56__448__0__std_filesystem_perms_none_::dcast ( ( * eref ) , ) } )  ) == ( ( ( ( 56_u32 ) & ( 448_u32 ) ) ) ) ) )  ;
 } { let mut e : () = ( ( 56_u32 ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( unsafe { check_bitmask_type_std_filesystem_perms__std_filesystem_perms_group_all__std_filesystem_perms_owner_all__unsigned_int__56__448__0__std_filesystem_perms_none_::dcast ( ( * eref ) , ) } )  ) == ( ( ( ( 56_u32 ) | ( 448_u32 ) ) ) ) ) )  ;
 } { let mut e : () = ( ( 56_u32 ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( unsafe { check_bitmask_type_std_filesystem_perms__std_filesystem_perms_group_all__std_filesystem_perms_owner_all__unsigned_int__56__448__0__std_filesystem_perms_none_::dcast ( ( * eref ) , ) } )  ) == ( ( ( ( 56_u32 ) ^ ( 448_u32 ) ) ) ) ) )  ;
 } return true  ;
 } }
pub unsafe fn ME_2(mut val: i32) -> () {
    return ((val) as ());
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
    const _: () = assert!(true, "std::is_enum<E>::value");;
    const _: () = assert!(true, "!std::is_convertible<E, UT>::value");;
    const _: () = assert!(true, "");;
    assert!(
        (unsafe {
            check_bitmask_type_std_filesystem_perms__std_filesystem_perms_group_all__std_filesystem_perms_owner_all__unsigned_int__56__448__0__std_filesystem_perms_none_::check ( )
        })
    );
    const _: () = assert!(
        true,
        "E::none         == ME(0) &&\n\n        E::owner_read   == ME(0400) &&\n        E::owner_write  == ME(0200) &&\n        E::owner_exec   == ME(0100) &&\n        E::owner_all    == ME(0700) &&\n\n        E::group_read   == ME(040) &&\n        E::group_write  == ME(020) &&\n        E::group_exec   == ME(010) &&\n        E::group_all    == ME(070) &&\n\n        E::others_read  == ME(04) &&\n        E::others_write == ME(02) &&\n        E::others_exec  == ME(01) &&\n        E::others_all   == ME(07) &&\n        E::all          == ME(0777) &&\n        E::set_uid      == ME(04000) &&\n        E::set_gid      == ME(02000) &&\n        E::sticky_bit   == ME(01000) &&\n        E::mask         == ME(07777) &&\n        E::unknown      == ME(0xFFFF)"
    );;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
