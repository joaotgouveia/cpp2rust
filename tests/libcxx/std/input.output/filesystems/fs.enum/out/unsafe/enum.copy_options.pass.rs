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
pub struct test_macros_detail_is_same_std_filesystem_copy_options__std_filesystem_copy_options_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_filesystem_copy_options_ref__std_filesystem_copy_options_ref_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct check_bitmask_type_std_filesystem_copy_options__std_filesystem_copy_options_skip_existing__std_filesystem_copy_options_update_existing__unsigned_short__1__4__0__std_filesystem_copy_options_none_
{}
impl check_bitmask_type_std_filesystem_copy_options__std_filesystem_copy_options_skip_existing__std_filesystem_copy_options_update_existing__unsigned_short__1__4__0__std_filesystem_copy_options_none_ { pub unsafe fn dcast ( mut e : () , ) -> u16 { return ( e as u16 )  ;
 } pub unsafe fn unpromote ( mut promoted : i32 , ) -> u16 { return ( promoted as u16 )  ;
 } pub unsafe fn check ( ) -> bool { { let mut ValRef : () = ( ( 1_u16 ) as () ) ;
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
 { let mut e : () = ( ( 1_u16 ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( ( unsafe { check_bitmask_type_std_filesystem_copy_options__std_filesystem_copy_options_skip_existing__std_filesystem_copy_options_update_existing__unsigned_short__1__4__0__std_filesystem_copy_options_none_::dcast ( ( * eref ) , ) } )  as i32 ) ) == ( ( ( ( ( 1_u16 as i32 ) ) & ( ( 4_u16 as i32 ) ) ) ) ) ) )  ;
 } { let mut e : () = ( ( 1_u16 ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( ( unsafe { check_bitmask_type_std_filesystem_copy_options__std_filesystem_copy_options_skip_existing__std_filesystem_copy_options_update_existing__unsigned_short__1__4__0__std_filesystem_copy_options_none_::dcast ( ( * eref ) , ) } )  as i32 ) ) == ( ( ( ( ( 1_u16 as i32 ) ) | ( ( 4_u16 as i32 ) ) ) ) ) ) )  ;
 } { let mut e : () = ( ( 1_u16 ) as () ) ;
  ;
 ;
 let eref : *mut () = & mut ( ( * unimplemented!()  )  ) ;
  ;
 ;
 assert!(( ( ( eref ) ) == ( ( & mut e as *mut () ) ) ) )  ;
 assert!(( ( ( ( unsafe { check_bitmask_type_std_filesystem_copy_options__std_filesystem_copy_options_skip_existing__std_filesystem_copy_options_update_existing__unsigned_short__1__4__0__std_filesystem_copy_options_none_::dcast ( ( * eref ) , ) } )  as i32 ) ) == ( ( ( ( ( 1_u16 as i32 ) ) ^ ( ( 4_u16 as i32 ) ) ) ) ) ) )  ;
 } return true  ;
 } }
pub unsafe fn ME_2(mut val: i32) -> () {
    return ((val) as ());
}
pub unsafe fn test_bitmask_binary_operations_3() -> bool {
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
        bitmask_elems.as_ptr().offset((9_i64) as isize),
    ) {
        let mut elem: () = (*elem);
        assert!(((unimplemented!()) == (elem)));
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (elem)));
        assert!(((unimplemented!()) == (elem)));
        assert!(((unimplemented!()) == (elem)));
        assert!(((unimplemented!()) == (unimplemented!())));
        {
            {
                let mut e: () = unimplemented!();
                assert!(((&mut (*unimplemented!()) as *mut ()) == (&mut e as *mut ())));
                assert!(((e) == (elem)));
            }
            {
                let mut e: () = unimplemented!();
                assert!(((&mut (*unimplemented!()) as *mut ()) == (&mut e as *mut ())));
                assert!(((e) == (unimplemented!())));
            }
            {
                let mut e: () = unimplemented!();
                assert!(((&mut (*unimplemented!()) as *mut ()) == (&mut e as *mut ())));
                assert!(((e) == (elem)));
            }
            {
                let mut e: () = elem;
                assert!(((&mut (*unimplemented!()) as *mut ()) == (&mut e as *mut ())));
                assert!(((e) == (elem)));
            }
            {
                let mut e: () = elem;
                assert!(((&mut (*unimplemented!()) as *mut ()) == (&mut e as *mut ())));
                assert!(((e) == (elem)));
            }
            {
                let mut e: () = elem;
                assert!(((&mut (*unimplemented!()) as *mut ()) == (&mut e as *mut ())));
                assert!(((e) == (unimplemented!())));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    const _: () = assert!(true, "std::is_enum<E>::value");;
    const _: () = assert!(true, "!std::is_convertible<E, UT>::value");;
    const _: () = assert!(true, "");;
    assert!(
        (unsafe {
            check_bitmask_type_std_filesystem_copy_options__std_filesystem_copy_options_skip_existing__std_filesystem_copy_options_update_existing__unsigned_short__1__4__0__std_filesystem_copy_options_none_::check ( )
        })
    );
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    (unsafe { test_bitmask_binary_operations_3() });
    const _: () = assert!(true, "test_bitmask_binary_operations()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
