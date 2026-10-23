extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_directory_options__std_filesystem_directory_options_
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_filesystem_directory_options_ref__std_filesystem_directory_options_ref_
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_
{}
impl check_bitmask_type_std_filesystem_directory_options__std_filesystem_directory_options_follow_directory_symlink__std_filesystem_directory_options_skip_permission_denied__unsigned_char____x01_____x02_____x00___std_filesystem_directory_options_none_ { pub fn dcast ( mut e : ()  , ) -> ()  { return ( e as () )   ;
 } pub fn unpromote ( mut promoted : ()  , ) -> ()  { return promoted   ;
 } pub fn check ( ) -> ()  { { let mut ValRef : ()  = ( ( ( '\x01' as ()  ) ) as () )  ;
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
 { let e : Value<() > = Rc::new(RefCell::new(( ( ( '\x01' as ()  ) ) as () ) )) ;
  ;
 ;
 let eref : Ptr< ()  > = ( unimplemented!()  )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let e : Value<() > = Rc::new(RefCell::new(( ( ( '\x01' as ()  ) ) as () ) )) ;
  ;
 ;
 let eref : Ptr< ()  > = ( unimplemented!()  )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let e : Value<() > = Rc::new(RefCell::new(( ( ( '\x01' as ()  ) ) as () ) )) ;
  ;
 ;
 let eref : Ptr< ()  > = ( unimplemented!()  )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } return true   ;
 } }
pub fn ME_2(mut val: ()) -> () {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    const _: () = assert!(true, "std::is_enum<E>::value");;
    const _: () = assert!(true, "!std::is_convertible<E, UT>::value");;
    const _: () = assert!(true, "std::is_same<UT, unsigned char>::value");;
    unimplemented!();
    const _: () = assert!(
        true,
        "E::none                     == ME(0) &&\n        E::follow_directory_symlink == ME(1) &&\n        E::skip_permission_denied   == ME(2)"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
