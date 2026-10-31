extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_perm_options__std_filesystem_perm_options_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_perm_options_ref__std_filesystem_perm_options_ref_
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct check_bitmask_type_std_filesystem_perm_options__std_filesystem_perm_options_replace__std_filesystem_perm_options_nofollow__unsigned_int__1__8__0__0_
{}
impl check_bitmask_type_std_filesystem_perm_options__std_filesystem_perm_options_replace__std_filesystem_perm_options_nofollow__unsigned_int__1__8__0__0_ { pub fn dcast ( mut e : ()  , ) -> u32  { return ( e as u32 )   ;
 } pub fn unpromote ( mut promoted : u32  , ) -> u32  { return ( promoted as u32 )   ;
 } pub fn check ( ) -> bool  { { let mut ValRef : ()  = ( ( 1_u32 ) as () )  ;
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
 { let e : Value<() > = Rc::new(RefCell::new(( ( 1_u32 ) as () ) )) ;
  ;
 ;
 let eref : Ptr< ()  > = ( unimplemented!()  )  ;
  ;
 ;
 assert!(({ (( eref  ) ).clone() } == { ( e .as_pointer()  )  }) )  ;
 assert!(( ( { check_bitmask_type_std_filesystem_perm_options__std_filesystem_perm_options_replace__std_filesystem_perm_options_nofollow__unsigned_int__1__8__0__0_::dcast ( (eref.read())  , ) } )  == ( ( 1_u32 & 8_u32 ) ) ) )  ;
 } { let e : Value<() > = Rc::new(RefCell::new(( ( 1_u32 ) as () ) )) ;
  ;
 ;
 let eref : Ptr< ()  > = ( unimplemented!()  )  ;
  ;
 ;
 assert!(({ (( eref  ) ).clone() } == { ( e .as_pointer()  )  }) )  ;
 assert!(( ( { check_bitmask_type_std_filesystem_perm_options__std_filesystem_perm_options_replace__std_filesystem_perm_options_nofollow__unsigned_int__1__8__0__0_::dcast ( (eref.read())  , ) } )  == ( ( 1_u32 | 8_u32 ) ) ) )  ;
 } { let e : Value<() > = Rc::new(RefCell::new(( ( 1_u32 ) as () ) )) ;
  ;
 ;
 let eref : Ptr< ()  > = ( unimplemented!()  )  ;
  ;
 ;
 assert!(({ (( eref  ) ).clone() } == { ( e .as_pointer()  )  }) )  ;
 assert!(( ( { check_bitmask_type_std_filesystem_perm_options__std_filesystem_perm_options_replace__std_filesystem_perm_options_nofollow__unsigned_int__1__8__0__0_::dcast ( (eref.read())  , ) } )  == ( ( 1_u32 ^ 8_u32 ) ) ) )  ;
 } return true   ;
 } }
pub fn ME_2(mut val: i32) -> () {
    return ((val) as ());
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    const _: () = assert!(true, "std::is_enum<E>::value");;
    const _: () = assert!(true, "!std::is_convertible<E, UT>::value");;
    const _: () = assert!(true, "");;
    assert!(
        ({
            check_bitmask_type_std_filesystem_perm_options__std_filesystem_perm_options_replace__std_filesystem_perm_options_nofollow__unsigned_int__1__8__0__0_::check ( )
        })
    );
    const _: () = assert!(
        true,
        "E::replace  == ME(1) &&\n        E::add      == ME(2) &&\n        E::remove   == ME(4) &&\n        E::nofollow == ME(8)"
    );;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
