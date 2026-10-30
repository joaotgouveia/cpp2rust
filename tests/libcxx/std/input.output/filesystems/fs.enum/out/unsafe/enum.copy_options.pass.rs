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
pub struct test_macros_detail_is_same_std_filesystem_copy_options__std_filesystem_copy_options_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_filesystem_copy_options_ref__std_filesystem_copy_options_ref_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct check_bitmask_type_std_filesystem_copy_options__std_filesystem_copy_options_skip_existing__std_filesystem_copy_options_update_existing__unsigned_short__1__4__0__std_filesystem_copy_options_none_
{}
impl check_bitmask_type_std_filesystem_copy_options__std_filesystem_copy_options_skip_existing__std_filesystem_copy_options_update_existing__unsigned_short__1__4__0__std_filesystem_copy_options_none_ { pub unsafe fn dcast ( mut e : () , ) -> () { return ( e as () )  ;
 } pub unsafe fn unpromote ( mut promoted : () , ) -> () { return promoted  ;
 } pub unsafe fn check ( ) -> () { { let mut ValRef : () = ( ( 1_() ) as () ) ;
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
 { let mut e : () = ( ( 1_() ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut e : () = ( ( 1_() ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut e : () = ( ( 1_() ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } return true  ;
 } }
pub unsafe fn ME_2(mut val: ()) -> () {
    return ((val) as ());
}
pub unsafe fn test_bitmask_binary_operations_3() -> () {
    let bitmask_elems: [(); 9] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    'loop_: for elem in IterRange::new(
        bitmask_elems.as_ptr(),
        bitmask_elems.as_ptr().offset((9_()) as isize),
    ) {
        let mut elem: () = (*elem);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            {
                let mut e: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut e: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut e: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut e: () = elem;
                unimplemented!();
                unimplemented!();
            }
            {
                let mut e: () = elem;
                unimplemented!();
                unimplemented!();
            }
            {
                let mut e: () = elem;
                unimplemented!();
                unimplemented!();
            }
        }
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
    const _: () = assert!(true, "std::is_enum<E>::value");;
    const _: () = assert!(true, "!std::is_convertible<E, UT>::value");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    (unsafe { test_bitmask_binary_operations_3() });
    const _: () = assert!(true, "test_bitmask_binary_operations()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
