extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct not_extents_constructible_tag {}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_int__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__ {
    pub unsafe fn new_1(ext: *const (), mut offset: Option<()>, mut scaling: Option<()>) -> Self {
        let mut offset: () = offset.unwrap_or_else(|| unsafe { 0_() });
        let mut scaling: () = scaling.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self {
            extents_: unimplemented!(),
            offset_: offset,
            scaling_: scaling,
        };
        this
    }
    pub unsafe fn is_always_unique() -> () {
        return true;
    }
    pub unsafe fn is_always_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_always_strided() -> () {
        return true;
    }
    pub unsafe fn operator_call_9(&self) -> () {
        let mut idx_a: () = unimplemented!();
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_int__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_int__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 unimplemented!() ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unimplemented!() , ) } ) ) ) )  ;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_int__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_unsigned_int__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_int__5__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__5__ {
    pub unsafe fn new_1(ext: *const (), mut offset: Option<()>, mut scaling: Option<()>) -> Self {
        let mut offset: () = offset.unwrap_or_else(|| unsafe { 0_() });
        let mut scaling: () = scaling.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self {
            extents_: unimplemented!(),
            offset_: offset,
            scaling_: scaling,
        };
        this
    }
    pub unsafe fn extents(&self) -> *const () {
        return &self.extents_;
    }
    pub unsafe fn required_span_size(&self) -> () {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        return (*unimplemented!());
    }
    pub unsafe fn is_always_unique() -> () {
        return true;
    }
    pub unsafe fn is_always_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_always_strided() -> () {
        return true;
    }
    pub unsafe fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = 0_();
        'loop_: while ((i) < (r)) {
            s = (s).wrapping_mul(unimplemented!());
            i.postfix_inc();
        }
        return (s).wrapping_mul(self.scaling_);
    }
    pub unsafe fn operator_call_9_int(&self, mut idx: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_int__5__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_int__5__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unimplemented!() , ) } ) ) ) )  ;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_int__5__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_unsigned_int__5__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut offset : Option< () > , mut scaling : Option< () > , ) -> Self { let mut offset : ()  = offset.unwrap_or_else(|| unsafe { 0_()  }) ;
 let mut scaling : ()  = scaling.unwrap_or_else(|| unsafe { 1_()  }) ;
 let mut this = Self { extents_ : unimplemented!() , offset_ : offset , scaling_ : scaling , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn required_span_size ( &self , ) -> () { let mut size : () = 1_() ;
  ;
 ;
 let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { size = ( size ) . wrapping_mul ( unimplemented!()  ) ;
 r .postfix_inc() ;
 } return ( * unimplemented!()  )   ;
 } pub unsafe fn is_always_unique ( ) -> () { return true  ;
 } pub unsafe fn is_always_exhaustive ( ) -> () { return true  ;
 } pub unsafe fn is_always_strided ( ) -> () { return true  ;
 } pub unsafe fn stride ( &self , mut r : () , ) -> () { let mut s : () = 1_() ;
  ;
 ;
 let mut i : () = 0_() ;
  ;
 ;
 'loop_: while ( ( i ) < ( r ) ) { s = ( s ) . wrapping_mul ( unimplemented!()  ) ;
 i .postfix_inc() ;
 } return ( s ) . wrapping_mul ( self . scaling_  )  ;
 } pub unsafe fn operator_call_9_int_int ( &self , mut idx_0 : () , mut idx_1 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unimplemented!() , ) } ) ) ) )  ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct always_convertible_layout {}
pub unsafe fn test_comparison_different_rank_0() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 1>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, 1>, std::extents<T2, 5>>"
    );;
}
pub unsafe fn test_comparison_different_rank_2() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 1>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, 1>, std::extents<T2, 5>>"
    );;
}
pub unsafe fn test_comparison_different_rank_3() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 1>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, 1>, std::extents<T2, 5>>"
    );;
}
pub unsafe fn test_comparison_different_rank_4() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        true,
        "layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1>, std::extents<T2, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 1>, std::extents<T2>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D>, std::extents<T2, D, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5>, std::extents<T2, 5, 1>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, D, D>, std::extents<T2, D>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, D>, std::extents<T2, 5>>"
    );;
    const _: () = assert!(
        !(false),
        "!layout_mapping_comparable<std::extents<T1, 5, 1>, std::extents<T2, 5>>"
    );;
}
pub unsafe fn test_comparison_5(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_6(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_7(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_8(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_9(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_10(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_11(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_12(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_13(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_14(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_15(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_16(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_17(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_18(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_19(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_20(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_21(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_22(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_23(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_24(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_25(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_26(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_27(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_28(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_29(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_30(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_31(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_32(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_33(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_34(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_35(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_36(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_37(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_38(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_39(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_40(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_41(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_42(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_43(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_44(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_45(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_46(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_47(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_48(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_49(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_50(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_51(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_52(
    mut equal: (),
    mut dest_exts: (),
    mut src_exts: (),
    mut dest_strides: (),
    mut src_strides: (),
) {
    let mut dest: () = unimplemented!();
    let mut src: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_same_rank_53() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    (unsafe {
        test_comparison_5(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_6(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_6(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_7(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_8(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_9(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_7(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_6(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_6(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_7(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_10(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_11(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_12(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_13(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_14(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_14(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_12(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_15(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_16(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_comparison_same_rank_54() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    (unsafe {
        test_comparison_17(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_18(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_18(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_19(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_20(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_21(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_19(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_18(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_18(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_19(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_22(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_23(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_24(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_25(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_26(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_26(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_24(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_27(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_28(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_comparison_same_rank_55() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    (unsafe {
        test_comparison_29(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_30(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_30(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_31(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_32(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_33(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_31(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_30(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_30(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_31(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_34(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_35(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_36(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_37(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_38(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_38(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_36(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_39(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_40(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_comparison_same_rank_56() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    (unsafe {
        test_comparison_41(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_42(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_42(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_43(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_44(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_45(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_43(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_42(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_42(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_43(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_46(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_47(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_48(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_49(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_50(
            true,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_50(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_48(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_51(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_52(
            false,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_comparison_with_57(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_with_58(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_with_59(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_with_60(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_with_61(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_with_62(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_with_63(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_with_64(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparison_with_65(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: always_convertible_layout_mapping_std_extents_unsigned_int__ =
        always_convertible_layout_mapping_std_extents_unsigned_int__::new_1({ &e2 }, None, None);
    unimplemented!();
}
pub unsafe fn test_comparison_with_66(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: always_convertible_layout_mapping_std_extents_unsigned_int__5__ =
        always_convertible_layout_mapping_std_extents_unsigned_int__5__::new_1({ &e2 }, None, None);
    unimplemented!();
}
pub unsafe fn test_comparison_with_67(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map: always_convertible_layout_mapping_std_extents_unsigned_int__5__ =
        always_convertible_layout_mapping_std_extents_unsigned_int__5__::new_1({ &e2 }, None, None);
    unimplemented!();
}
pub unsafe fn test_comparison_with_68(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map : always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ = always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ :: new_1 ( { & e2 } , None , None , )  ;
    unimplemented!();
}
pub unsafe fn test_comparison_with_69(
    mut expect_equal: (),
    mut e1: (),
    mut strides: (),
    mut e2: (),
    mut other_args_4: (),
    mut other_args_5: (),
) {
    let mut map: () = unimplemented!();
    let mut other_map : always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ = always_convertible_layout_mapping_std_extents_unsigned_int__18446744073709551615__18446744073709551615__ :: new_1 ( { & e2 } , { Some ( other_args_4 ) } , { Some ( other_args_5 ) } , )  ;
    unimplemented!();
}
pub unsafe fn test_comparison_with_70() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    let mut is_left_based: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_71))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_72));
    (unsafe {
        test_comparison_with_57(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_58(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_59(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_59(false, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_60(
            is_left_based,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_with_60(
            !(is_left_based),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_with_60(false, unimplemented!(), unimplemented!(), unimplemented!())
    });
    {}
}
pub unsafe fn test_comparison_with_73() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    let mut is_left_based: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_74))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_75));
    (unsafe {
        test_comparison_with_61(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_62(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_63(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_63(false, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_64(
            is_left_based,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_with_64(
            !(is_left_based),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_with_64(false, unimplemented!(), unimplemented!(), unimplemented!())
    });
    {}
}
pub unsafe fn test_comparison_with_76() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_1));
    let mut is_left_based: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_77))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_78));
    (unsafe {
        test_comparison_with_65(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_66(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_67(true, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_67(false, unimplemented!(), unimplemented!(), unimplemented!())
    });
    (unsafe {
        test_comparison_with_68(
            is_left_based,
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_with_68(
            !(is_left_based),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_comparison_with_68(false, unimplemented!(), unimplemented!(), unimplemented!())
    });
    {
        (unsafe {
            test_comparison_with_69(
                true,
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                0_(),
                2_(),
            )
        });
        (unsafe {
            test_comparison_with_69(
                false,
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                1_(),
                2_(),
            )
        });
    }
}
pub unsafe fn test_comparison_index_type_79() {
    (unsafe { test_comparison_same_rank_53() });
    (unsafe { test_comparison_different_rank_0() });
    (unsafe { test_comparison_with_70() });
    (unsafe { test_comparison_with_73() });
    (unsafe { test_comparison_with_76() });
}
pub unsafe fn test_comparison_index_type_80() {
    (unsafe { test_comparison_same_rank_54() });
    (unsafe { test_comparison_different_rank_2() });
    (unsafe { test_comparison_with_70() });
    (unsafe { test_comparison_with_73() });
    (unsafe { test_comparison_with_76() });
}
pub unsafe fn test_comparison_index_type_81() {
    (unsafe { test_comparison_same_rank_55() });
    (unsafe { test_comparison_different_rank_3() });
    (unsafe { test_comparison_with_70() });
    (unsafe { test_comparison_with_73() });
    (unsafe { test_comparison_with_76() });
}
pub unsafe fn test_comparison_index_type_82() {
    (unsafe { test_comparison_same_rank_56() });
    (unsafe { test_comparison_different_rank_4() });
    (unsafe { test_comparison_with_70() });
    (unsafe { test_comparison_with_73() });
    (unsafe { test_comparison_with_76() });
}
pub unsafe fn test_83() -> () {
    (unsafe { test_comparison_index_type_79() });
    (unsafe { test_comparison_index_type_80() });
    (unsafe { test_comparison_index_type_81() });
    (unsafe { test_comparison_index_type_82() });
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
    (unsafe { test_83() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
