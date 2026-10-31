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
pub struct test_macros_detail_is_same_std_filesystem_directory_options__std_filesystem_directory_options_
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_filesystem_directory_options_ref__std_filesystem_directory_options_ref_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_
{}
impl check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_ { pub unsafe fn dcast ( mut e : () , ) -> u8 { return ( e as u8 )  ;
 } pub unsafe fn unpromote ( mut promoted : i32 , ) -> u8 { return ( promoted as u8 )  ;
 } pub unsafe fn check ( ) -> bool { { let mut ValRef : () = ( ( ( '\x01' as u8  ) ) as () ) ;
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
 { let mut e : () = ( ( ( '\x01' as u8  ) ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( ( unsafe { check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_::dcast ( ( * eref ) , ) } )  as i32 ) ) == ( ( ( ( ( ( '\x01' as u8  ) as i32 ) ) & ( ( ( '\x02' as u8  ) as i32 ) ) ) ) ) ) )  ;
 } { let mut e : () = ( ( ( '\x01' as u8  ) ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( ( unsafe { check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_::dcast ( ( * eref ) , ) } )  as i32 ) ) == ( ( ( ( ( ( '\x01' as u8  ) as i32 ) ) | ( ( ( '\x02' as u8  ) as i32 ) ) ) ) ) ) )  ;
 } { let mut e : () = ( ( ( '\x01' as u8  ) ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( ( unsafe { check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_::dcast ( ( * eref ) , ) } )  as i32 ) ) == ( ( ( ( ( ( '\x01' as u8  ) as i32 ) ) ^ ( ( ( '\x02' as u8  ) as i32 ) ) ) ) ) ) )  ;
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
            check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_::check ( )
        })
    );
    const _: () = assert!(
        true,
        "E::none                     == ME(0) &&\n        E::follow_directory_symlink == ME(1) &&\n        E::skip_permission_denied   == ME(2)"
    );;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
