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
pub struct always_convertible_layout_mapping_std_extents_int__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__ {
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
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__ = ( self as *const always_convertible_layout_mapping_std_extents_int__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 unimplemented!() ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int(&self, mut idx: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__5__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__5__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int(&self, mut idx: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__5__ = ( self as *const always_convertible_layout_mapping_std_extents_int__5__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__5__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__5__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl
    always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__
{
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ = ( self as *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__5__7__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__5__7__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__5__7__ = ( self as *const always_convertible_layout_mapping_std_extents_int__5__7__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__5__7__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__5__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ { pub unsafe fn new_1 ( ext : *const () , mut offset : Option< () > , mut scaling : Option< () > , ) -> Self { let mut offset : ()  = offset.unwrap_or_else(|| unsafe { 0_()  }) ;
 let mut scaling : ()  = scaling.unwrap_or_else(|| unsafe { 1_()  }) ;
 let mut this = Self { extents_ : unimplemented!() , offset_ : offset , scaling_ : scaling , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn required_span_size ( &self , ) -> () { let mut size : () = 1_() ;
  ;
 ;
 let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { size  *= unimplemented!()   ;
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
 'loop_: while ( ( i ) < ( r ) ) { s  *= unimplemented!()   ;
 i .postfix_inc() ;
 } return ( ( s ) * ( self . scaling_  ) )  ;
 } pub unsafe fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : () , mut idx_1 : () , mut idx_2 : () , mut idx_3 : () , mut idx_4 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( ( self . offset_  ) + ( ( ( self . scaling_  ) * ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) ) ) )  ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut offset : Option< () > , mut scaling : Option< () > , ) -> Self { let mut offset : ()  = offset.unwrap_or_else(|| unsafe { 0_()  }) ;
 let mut scaling : ()  = scaling.unwrap_or_else(|| unsafe { 1_()  }) ;
 let mut this = Self { extents_ : unimplemented!() , offset_ : offset , scaling_ : scaling , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn required_span_size ( &self , ) -> () { let mut size : () = 1_() ;
  ;
 ;
 let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { size  *= unimplemented!()   ;
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
 'loop_: while ( ( i ) < ( r ) ) { s  *= unimplemented!()   ;
 i .postfix_inc() ;
 } return ( ( s ) * ( self . scaling_  ) )  ;
 } pub unsafe fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : () , mut idx_1 : () , mut idx_2 : () , mut idx_3 : () , mut idx_4 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( ( self . offset_  ) + ( ( ( self . scaling_  ) * ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) ) ) )  ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default
    for always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__
{
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__ {
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
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 unimplemented!() ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_unsigned_long__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ {
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
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
    }
}
impl Default
    for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__
{
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
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
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__5__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__5__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut offset : Option< () > , mut scaling : Option< () > , ) -> Self { let mut offset : ()  = offset.unwrap_or_else(|| unsafe { 0_()  }) ;
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
 return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ {
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
    pub unsafe fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
    }
}
impl Default
    for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__
{
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ {
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
    pub unsafe fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__5__7__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ { pub unsafe fn new_1 ( ext : *const () , mut offset : Option< () > , mut scaling : Option< () > , ) -> Self { let mut offset : ()  = offset.unwrap_or_else(|| unsafe { 0_()  }) ;
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
 } pub unsafe fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : () , mut idx_1 : () , mut idx_2 : () , mut idx_3 : () , mut idx_4 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut offset : Option< () > , mut scaling : Option< () > , ) -> Self { let mut offset : ()  = offset.unwrap_or_else(|| unsafe { 0_()  }) ;
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
 } pub unsafe fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : () , mut idx_1 : () , mut idx_2 : () , mut idx_3 : () , mut idx_4 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl
    always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__
{
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
    pub unsafe fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let mut idx_a: () = unimplemented!();
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ {
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
    pub unsafe fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let mut idx_a: () = unimplemented!();
        return ( self . offset_  ) . wrapping_add ( ( self . scaling_  ) . wrapping_mul ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( unimplemented!()  ) . wrapping_mul ( res ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) )  ;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__ {
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
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe!(
                        Generic,
                        {
                            let this_: *const always_convertible_layout_mapping_std_extents_long__ =
                                (self
                                    as *const always_convertible_layout_mapping_std_extents_long__);
                            let idx_a: *mut () = &mut idx_a;
                        },
                        |mut _a0: ()| -> () {
                            let mut res: () = 0_();
                            unimplemented!();
                            return res;
                        }
                    )
                    .spec::<fn(()) -> ()>(0)
                    .call(unsafe { std::mem::zeroed::<()>() })
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_long__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_long__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int(&self, mut idx: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_long__18446744073709551615__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__5__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__5__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int(&self, mut idx: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__5__ = ( self as *const always_convertible_layout_mapping_std_extents_long__5__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_long__5__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_long__5__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl
    always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__
{
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ = ( self as *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__5__7__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__5__7__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__5__7__ = ( self as *const always_convertible_layout_mapping_std_extents_long__5__7__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_long__5__7__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_long__5__7__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ { pub unsafe fn new_1 ( ext : *const () , mut offset : Option< () > , mut scaling : Option< () > , ) -> Self { let mut offset : ()  = offset.unwrap_or_else(|| unsafe { 0_()  }) ;
 let mut scaling : ()  = scaling.unwrap_or_else(|| unsafe { 1_()  }) ;
 let mut this = Self { extents_ : unimplemented!() , offset_ : offset , scaling_ : scaling , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn required_span_size ( &self , ) -> () { let mut size : () = 1_() ;
  ;
 ;
 let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { size  *= unimplemented!()   ;
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
 'loop_: while ( ( i ) < ( r ) ) { s  *= unimplemented!()   ;
 i .postfix_inc() ;
 } return ( ( s ) * ( self . scaling_  ) )  ;
 } pub unsafe fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : () , mut idx_1 : () , mut idx_2 : () , mut idx_3 : () , mut idx_4 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( ( self . offset_  ) + ( ( ( self . scaling_  ) * ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) ) ) )  ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut offset : Option< () > , mut scaling : Option< () > , ) -> Self { let mut offset : ()  = offset.unwrap_or_else(|| unsafe { 0_()  }) ;
 let mut scaling : ()  = scaling.unwrap_or_else(|| unsafe { 1_()  }) ;
 let mut this = Self { extents_ : unimplemented!() , offset_ : offset , scaling_ : scaling , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn required_span_size ( &self , ) -> () { let mut size : () = 1_() ;
  ;
 ;
 let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { size  *= unimplemented!()   ;
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
 'loop_: while ( ( i ) < ( r ) ) { s  *= unimplemented!()   ;
 i .postfix_inc() ;
 } return ( ( s ) * ( self . scaling_  ) )  ;
 } pub unsafe fn operator_call_9_int_int_int_int_int ( &self , mut idx_0 : () , mut idx_1 : () , mut idx_2 : () , mut idx_3 : () , mut idx_4 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( ( self . offset_  ) + ( ( ( self . scaling_  ) * ( ( ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ = ( self as *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } ) ) ) ) ) )  ;
 } }
impl Default for always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , offset_ : unimplemented!() , scaling_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default
    for always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__
{
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ {
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
            size *= unimplemented!();
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
            s *= unimplemented!();
            i.postfix_inc();
        }
        return ((s) * (self.scaling_));
    }
    pub unsafe fn operator_call_9_int_int_int_int_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
        mut idx_2: (),
        mut idx_3: (),
        mut idx_4: (),
    ) -> () {
        let mut idx_a: () = unimplemented!();
        return ((self.offset_)
            + ((self.scaling_)
                * (unsafe {
                    lambda_unsafe! ( Generic , { let this_ : *const always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ = ( self as *const always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( unimplemented!()  ) * ( res ) ) ) )  } } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
                })));
    }
}
impl Default for always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__ {
    pub unsafe fn is_always_unique() -> () {
        return true;
    }
    pub unsafe fn is_always_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_always_strided() -> () {
        return true;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__18446744073709551615__4__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__4__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__4__ {
    pub unsafe fn is_always_unique() -> () {
        return true;
    }
    pub unsafe fn is_always_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_always_strided() -> () {
        return true;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__4__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__4__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__ {
    extents_: (),
    offset_: (),
    scaling_: (),
}
impl always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__ {
    pub unsafe fn is_always_unique() -> () {
        return true;
    }
    pub unsafe fn is_always_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_always_strided() -> () {
        return true;
    }
}
impl Default for always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__ {
    fn default() -> Self {
        unsafe {
            always_convertible_layout_mapping_std_extents_int__4__18446744073709551615__ {
                extents_: unimplemented!(),
                offset_: unimplemented!(),
                scaling_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct always_convertible_layout {}
pub unsafe fn test_conversion_0(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_1(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_2(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_3(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_4(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_5(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_6(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_7(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_8(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_9(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_10(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_11(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_12(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_13(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_14(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_15(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_16(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_17(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_18(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_19(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_20(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_21(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_22(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_23(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_24(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_25(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_26(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_27(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_28(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_29(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_30(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_31(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_32(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_33(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_34(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_35(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_36(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_37(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_38(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_39(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_40(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_41(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_42(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_43(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_44(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_45(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_46(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_47(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_48(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_49(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_50(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_51(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_52(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_53(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_54(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_55(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_56(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_57(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_58(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_59(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_60(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_61(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_62(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_63(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_64(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_65(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_66(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_67(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_68(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_69(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_70(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_71(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_72(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_73(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_74(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_75(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_76(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_77(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_78(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_79(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_80(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_81(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_82(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_83(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_84(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_85(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_86(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_87(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_88(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_89(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_90(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_91(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_92(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_93(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_94(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_95(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_96(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_97(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_98(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_99(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_100(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_101(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_102(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_103(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_104(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_105(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_106(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_107(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_108(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_109(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_110(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_111(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_112(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_113(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_114(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_115(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_116(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_117(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_118(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_119(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_120(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_121(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_122(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_123(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_124(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_125(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_126(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_127(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_128(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_129(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_130(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_131(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_132(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_133(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_134(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_135(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_136(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_137(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_138(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_139(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_140(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_141(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_142(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_143(mut src_exts: ()) {
    let mut src: () = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> () {
                {
                    let mut strides: () = unimplemented!();
                    let mut stride: () = 2_();
                    let mut r: () = 0_();
                    'loop_: while ((r) < (unimplemented!())) {
                        (*unimplemented!()) = stride;
                        stride = (stride).wrapping_mul(unimplemented!());
                        r.postfix_inc();
                    }
                    return unimplemented!();
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        let mut dest_implicit: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_conversion_144(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_int__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_145(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_146(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_147(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__5__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_int__5__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__5__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_148(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_149(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_150(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_151(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__5__7__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_int__5__7__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__5__7__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_152(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_153(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_154(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_155(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ { { return always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_156(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_unsigned_long__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_unsigned_long__ {
                {
                    return always_convertible_layout_mapping_std_extents_unsigned_long__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_157(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_158(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_159(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_unsigned_long__5__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_unsigned_long__5__ {
                {
                    return always_convertible_layout_mapping_std_extents_unsigned_long__5__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_160(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_161(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_162(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_163(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__5__7__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_164(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_165(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_166(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__18446744073709551615__7__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_167(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ { { return always_convertible_layout_mapping_std_extents_unsigned_long__5__7__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_168(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_int__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_169(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_170(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_171(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__5__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_int__5__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__5__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_172(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_173(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_174(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_175(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__5__7__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_int__5__7__ {
                {
                    return always_convertible_layout_mapping_std_extents_int__5__7__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_176(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_177(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_178(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ { { return always_convertible_layout_mapping_std_extents_int__18446744073709551615__7__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_179(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ { { return always_convertible_layout_mapping_std_extents_int__5__7__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_180(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_long__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_long__ {
                {
                    return always_convertible_layout_mapping_std_extents_long__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_181(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_long__18446744073709551615__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_182(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_long__18446744073709551615__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_183(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_long__5__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_long__5__ {
                {
                    return always_convertible_layout_mapping_std_extents_long__5__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_184(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_185(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_186(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_187(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_long__5__7__ = (unsafe {
        lambda_unsafe!(
            {
                let src_exts: *mut () = &mut src_exts;
            },
            || -> always_convertible_layout_mapping_std_extents_long__5__7__ {
                {
                    return always_convertible_layout_mapping_std_extents_long__5__7__::new_1(
                        { src_exts },
                        None,
                        None,
                    );
                }
                panic!("ub: non-void function does not return a value")
            }
        )
        .call()
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_188(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_189(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_190(mut src_exts: ()) {
    let mut src : always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ = ( unsafe { lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ { { return always_convertible_layout_mapping_std_extents_long__18446744073709551615__7__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( ) } ) ;
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_191(mut src_exts: ()) {
    let mut src: always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ = (unsafe {
        lambda_unsafe! ( { let src_exts : *mut () = & mut src_exts ;
 } , | | -> always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ { { return always_convertible_layout_mapping_std_extents_long__5__7__8__9__1__ :: new_1 ( { src_exts } , None , None , )   ;
 } panic!("ub: non-void function does not return a value") } )  .call ( )
    });
    const _: () = assert!(true, "");;
    let mut dest: () = unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn test_conversion_192() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_194))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_195)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_196));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_0(unimplemented!()) });
    (unsafe { test_conversion_1(unimplemented!()) });
    (unsafe { test_conversion_1(unimplemented!()) });
    (unsafe { test_conversion_2(unimplemented!()) });
    (unsafe { test_conversion_3(unimplemented!()) });
    (unsafe { test_conversion_4(unimplemented!()) });
    (unsafe { test_conversion_5(unimplemented!()) });
    (unsafe { test_conversion_6(unimplemented!()) });
    (unsafe { test_conversion_7(unimplemented!()) });
    (unsafe { test_conversion_8(unimplemented!()) });
    (unsafe { test_conversion_9(unimplemented!()) });
    (unsafe { test_conversion_10(unimplemented!()) });
    (unsafe { test_conversion_11(unimplemented!()) });
}
pub unsafe fn test_conversion_197() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!()) >= (unimplemented!() as ()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_194))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_195)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_196));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_12(unimplemented!()) });
    (unsafe { test_conversion_13(unimplemented!()) });
    (unsafe { test_conversion_13(unimplemented!()) });
    (unsafe { test_conversion_14(unimplemented!()) });
    (unsafe { test_conversion_15(unimplemented!()) });
    (unsafe { test_conversion_16(unimplemented!()) });
    (unsafe { test_conversion_17(unimplemented!()) });
    (unsafe { test_conversion_18(unimplemented!()) });
    (unsafe { test_conversion_19(unimplemented!()) });
    (unsafe { test_conversion_20(unimplemented!()) });
    (unsafe { test_conversion_21(unimplemented!()) });
    (unsafe { test_conversion_22(unimplemented!()) });
    (unsafe { test_conversion_23(unimplemented!()) });
}
pub unsafe fn test_conversion_198() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!() as ()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_194))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_195)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_196));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_24(unimplemented!()) });
    (unsafe { test_conversion_25(unimplemented!()) });
    (unsafe { test_conversion_25(unimplemented!()) });
    (unsafe { test_conversion_26(unimplemented!()) });
    (unsafe { test_conversion_27(unimplemented!()) });
    (unsafe { test_conversion_28(unimplemented!()) });
    (unsafe { test_conversion_29(unimplemented!()) });
    (unsafe { test_conversion_30(unimplemented!()) });
    (unsafe { test_conversion_31(unimplemented!()) });
    (unsafe { test_conversion_32(unimplemented!()) });
    (unsafe { test_conversion_33(unimplemented!()) });
    (unsafe { test_conversion_34(unimplemented!()) });
    (unsafe { test_conversion_35(unimplemented!()) });
}
pub unsafe fn test_conversion_199() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!() as ()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_194))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_195)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_196));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_36(unimplemented!()) });
    (unsafe { test_conversion_37(unimplemented!()) });
    (unsafe { test_conversion_37(unimplemented!()) });
    (unsafe { test_conversion_38(unimplemented!()) });
    (unsafe { test_conversion_39(unimplemented!()) });
    (unsafe { test_conversion_40(unimplemented!()) });
    (unsafe { test_conversion_41(unimplemented!()) });
    (unsafe { test_conversion_42(unimplemented!()) });
    (unsafe { test_conversion_43(unimplemented!()) });
    (unsafe { test_conversion_44(unimplemented!()) });
    (unsafe { test_conversion_45(unimplemented!()) });
    (unsafe { test_conversion_46(unimplemented!()) });
    (unsafe { test_conversion_47(unimplemented!()) });
}
pub unsafe fn test_conversion_200() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_201))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_202)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_203));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_48(unimplemented!()) });
    (unsafe { test_conversion_49(unimplemented!()) });
    (unsafe { test_conversion_49(unimplemented!()) });
    (unsafe { test_conversion_50(unimplemented!()) });
    (unsafe { test_conversion_51(unimplemented!()) });
    (unsafe { test_conversion_52(unimplemented!()) });
    (unsafe { test_conversion_53(unimplemented!()) });
    (unsafe { test_conversion_54(unimplemented!()) });
    (unsafe { test_conversion_55(unimplemented!()) });
    (unsafe { test_conversion_56(unimplemented!()) });
    (unsafe { test_conversion_57(unimplemented!()) });
    (unsafe { test_conversion_58(unimplemented!()) });
    (unsafe { test_conversion_59(unimplemented!()) });
}
pub unsafe fn test_conversion_204() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!()) >= (unimplemented!() as ()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_201))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_202)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_203));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_60(unimplemented!()) });
    (unsafe { test_conversion_61(unimplemented!()) });
    (unsafe { test_conversion_61(unimplemented!()) });
    (unsafe { test_conversion_62(unimplemented!()) });
    (unsafe { test_conversion_63(unimplemented!()) });
    (unsafe { test_conversion_64(unimplemented!()) });
    (unsafe { test_conversion_65(unimplemented!()) });
    (unsafe { test_conversion_66(unimplemented!()) });
    (unsafe { test_conversion_67(unimplemented!()) });
    (unsafe { test_conversion_68(unimplemented!()) });
    (unsafe { test_conversion_69(unimplemented!()) });
    (unsafe { test_conversion_70(unimplemented!()) });
    (unsafe { test_conversion_71(unimplemented!()) });
}
pub unsafe fn test_conversion_205() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!() as ()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_201))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_202)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_203));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_72(unimplemented!()) });
    (unsafe { test_conversion_73(unimplemented!()) });
    (unsafe { test_conversion_73(unimplemented!()) });
    (unsafe { test_conversion_74(unimplemented!()) });
    (unsafe { test_conversion_75(unimplemented!()) });
    (unsafe { test_conversion_76(unimplemented!()) });
    (unsafe { test_conversion_77(unimplemented!()) });
    (unsafe { test_conversion_78(unimplemented!()) });
    (unsafe { test_conversion_79(unimplemented!()) });
    (unsafe { test_conversion_80(unimplemented!()) });
    (unsafe { test_conversion_81(unimplemented!()) });
    (unsafe { test_conversion_82(unimplemented!()) });
    (unsafe { test_conversion_83(unimplemented!()) });
}
pub unsafe fn test_conversion_206() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!() as ()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_201))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_202)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_203));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_84(unimplemented!()) });
    (unsafe { test_conversion_85(unimplemented!()) });
    (unsafe { test_conversion_85(unimplemented!()) });
    (unsafe { test_conversion_86(unimplemented!()) });
    (unsafe { test_conversion_87(unimplemented!()) });
    (unsafe { test_conversion_88(unimplemented!()) });
    (unsafe { test_conversion_89(unimplemented!()) });
    (unsafe { test_conversion_90(unimplemented!()) });
    (unsafe { test_conversion_91(unimplemented!()) });
    (unsafe { test_conversion_92(unimplemented!()) });
    (unsafe { test_conversion_93(unimplemented!()) });
    (unsafe { test_conversion_94(unimplemented!()) });
    (unsafe { test_conversion_95(unimplemented!()) });
}
pub unsafe fn test_conversion_207() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_208))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_209)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_210));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_96(unimplemented!()) });
    (unsafe { test_conversion_97(unimplemented!()) });
    (unsafe { test_conversion_97(unimplemented!()) });
    (unsafe { test_conversion_98(unimplemented!()) });
    (unsafe { test_conversion_99(unimplemented!()) });
    (unsafe { test_conversion_100(unimplemented!()) });
    (unsafe { test_conversion_101(unimplemented!()) });
    (unsafe { test_conversion_102(unimplemented!()) });
    (unsafe { test_conversion_103(unimplemented!()) });
    (unsafe { test_conversion_104(unimplemented!()) });
    (unsafe { test_conversion_105(unimplemented!()) });
    (unsafe { test_conversion_106(unimplemented!()) });
    (unsafe { test_conversion_107(unimplemented!()) });
}
pub unsafe fn test_conversion_211() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!()) >= (unimplemented!() as ()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_208))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_209)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_210));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_108(unimplemented!()) });
    (unsafe { test_conversion_109(unimplemented!()) });
    (unsafe { test_conversion_109(unimplemented!()) });
    (unsafe { test_conversion_110(unimplemented!()) });
    (unsafe { test_conversion_111(unimplemented!()) });
    (unsafe { test_conversion_112(unimplemented!()) });
    (unsafe { test_conversion_113(unimplemented!()) });
    (unsafe { test_conversion_114(unimplemented!()) });
    (unsafe { test_conversion_115(unimplemented!()) });
    (unsafe { test_conversion_116(unimplemented!()) });
    (unsafe { test_conversion_117(unimplemented!()) });
    (unsafe { test_conversion_118(unimplemented!()) });
    (unsafe { test_conversion_119(unimplemented!()) });
}
pub unsafe fn test_conversion_212() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!() as ()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_208))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_209)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_210));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_120(unimplemented!()) });
    (unsafe { test_conversion_121(unimplemented!()) });
    (unsafe { test_conversion_121(unimplemented!()) });
    (unsafe { test_conversion_122(unimplemented!()) });
    (unsafe { test_conversion_123(unimplemented!()) });
    (unsafe { test_conversion_124(unimplemented!()) });
    (unsafe { test_conversion_125(unimplemented!()) });
    (unsafe { test_conversion_126(unimplemented!()) });
    (unsafe { test_conversion_127(unimplemented!()) });
    (unsafe { test_conversion_128(unimplemented!()) });
    (unsafe { test_conversion_129(unimplemented!()) });
    (unsafe { test_conversion_130(unimplemented!()) });
    (unsafe { test_conversion_131(unimplemented!()) });
}
pub unsafe fn test_conversion_213() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!() as ()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_208))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_209)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_210));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_132(unimplemented!()) });
    (unsafe { test_conversion_133(unimplemented!()) });
    (unsafe { test_conversion_133(unimplemented!()) });
    (unsafe { test_conversion_134(unimplemented!()) });
    (unsafe { test_conversion_135(unimplemented!()) });
    (unsafe { test_conversion_136(unimplemented!()) });
    (unsafe { test_conversion_137(unimplemented!()) });
    (unsafe { test_conversion_138(unimplemented!()) });
    (unsafe { test_conversion_139(unimplemented!()) });
    (unsafe { test_conversion_140(unimplemented!()) });
    (unsafe { test_conversion_141(unimplemented!()) });
    (unsafe { test_conversion_142(unimplemented!()) });
    (unsafe { test_conversion_143(unimplemented!()) });
}
pub unsafe fn test_conversion_214() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_215))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_216)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_217));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_144(unimplemented!()) });
    (unsafe { test_conversion_145(unimplemented!()) });
    (unsafe { test_conversion_145(unimplemented!()) });
    (unsafe { test_conversion_146(unimplemented!()) });
    (unsafe { test_conversion_147(unimplemented!()) });
    (unsafe { test_conversion_148(unimplemented!()) });
    (unsafe { test_conversion_149(unimplemented!()) });
    (unsafe { test_conversion_150(unimplemented!()) });
    (unsafe { test_conversion_151(unimplemented!()) });
    (unsafe { test_conversion_152(unimplemented!()) });
    (unsafe { test_conversion_153(unimplemented!()) });
    (unsafe { test_conversion_154(unimplemented!()) });
    (unsafe { test_conversion_155(unimplemented!()) });
}
pub unsafe fn test_conversion_218() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!()) >= (unimplemented!() as ()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_215))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_216)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_217));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_156(unimplemented!()) });
    (unsafe { test_conversion_157(unimplemented!()) });
    (unsafe { test_conversion_157(unimplemented!()) });
    (unsafe { test_conversion_158(unimplemented!()) });
    (unsafe { test_conversion_159(unimplemented!()) });
    (unsafe { test_conversion_160(unimplemented!()) });
    (unsafe { test_conversion_161(unimplemented!()) });
    (unsafe { test_conversion_162(unimplemented!()) });
    (unsafe { test_conversion_163(unimplemented!()) });
    (unsafe { test_conversion_164(unimplemented!()) });
    (unsafe { test_conversion_165(unimplemented!()) });
    (unsafe { test_conversion_166(unimplemented!()) });
    (unsafe { test_conversion_167(unimplemented!()) });
}
pub unsafe fn test_conversion_219() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!() as ()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_215))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_216)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_217));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_168(unimplemented!()) });
    (unsafe { test_conversion_169(unimplemented!()) });
    (unsafe { test_conversion_169(unimplemented!()) });
    (unsafe { test_conversion_170(unimplemented!()) });
    (unsafe { test_conversion_171(unimplemented!()) });
    (unsafe { test_conversion_172(unimplemented!()) });
    (unsafe { test_conversion_173(unimplemented!()) });
    (unsafe { test_conversion_174(unimplemented!()) });
    (unsafe { test_conversion_175(unimplemented!()) });
    (unsafe { test_conversion_176(unimplemented!()) });
    (unsafe { test_conversion_177(unimplemented!()) });
    (unsafe { test_conversion_178(unimplemented!()) });
    (unsafe { test_conversion_179(unimplemented!()) });
}
pub unsafe fn test_conversion_220() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    let idx_convertible: () = ((unimplemented!() as ()) >= (unimplemented!()));
    let l_convertible: () = ((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_215))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_216)))
        || (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_217));
    let idx_l_convertible: () = (idx_convertible) && (l_convertible);
    (unsafe { test_conversion_180(unimplemented!()) });
    (unsafe { test_conversion_181(unimplemented!()) });
    (unsafe { test_conversion_181(unimplemented!()) });
    (unsafe { test_conversion_182(unimplemented!()) });
    (unsafe { test_conversion_183(unimplemented!()) });
    (unsafe { test_conversion_184(unimplemented!()) });
    (unsafe { test_conversion_185(unimplemented!()) });
    (unsafe { test_conversion_186(unimplemented!()) });
    (unsafe { test_conversion_187(unimplemented!()) });
    (unsafe { test_conversion_188(unimplemented!()) });
    (unsafe { test_conversion_189(unimplemented!()) });
    (unsafe { test_conversion_190(unimplemented!()) });
    (unsafe { test_conversion_191(unimplemented!()) });
}
pub unsafe fn test_no_implicit_conversion_221() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<int, D>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D>, ToM<int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D, 7>, FromM<FromL, int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5, 7>, ToM<int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5, 7>, FromM<FromL, int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D, 7>, ToM<int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<size_t, 5>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, size_t, 5>, ToM<int, 5>>"
    );;
}
pub unsafe fn test_no_implicit_conversion_222() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<int, D>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D>, ToM<int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D, 7>, FromM<FromL, int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5, 7>, ToM<int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5, 7>, FromM<FromL, int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D, 7>, ToM<int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<size_t, 5>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, size_t, 5>, ToM<int, 5>>"
    );;
}
pub unsafe fn test_no_implicit_conversion_223() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<int, D>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D>, ToM<int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, D, 7>, FromM<FromL, int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5, 7>, ToM<int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5, 7>, FromM<FromL, int, D, 7>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, int, D, 7>, ToM<int, 5, 7>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<size_t, 5>, FromM<FromL, int, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_convertible_v<FromM<FromL, int, 5>, ToM<size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<ToM<int, 5>, FromM<FromL, size_t, 5>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_convertible_v<FromM<FromL, size_t, 5>, ToM<int, 5>>"
    );;
}
pub unsafe fn test_rank_mismatch_224() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, D, D>, FromM<FromL, int, D, D>>"
    );;
}
pub unsafe fn test_rank_mismatch_225() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, D, D>, FromM<FromL, int, D, D>>"
    );;
}
pub unsafe fn test_rank_mismatch_226() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, D, D>, FromM<FromL, int, D, D>>"
    );;
}
pub unsafe fn test_rank_mismatch_227() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D>, FromM<FromL, int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, D, D>, FromM<FromL, int, D, D>>"
    );;
}
pub unsafe fn test_static_extent_mismatch_228() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, 5>, FromM<FromL, int, D, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5, D>, FromM<FromL, int, 4, D>>"
    );;
}
pub unsafe fn test_static_extent_mismatch_229() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, 5>, FromM<FromL, int, D, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5, D>, FromM<FromL, int, 4, D>>"
    );;
}
pub unsafe fn test_static_extent_mismatch_230() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, 5>, FromM<FromL, int, D, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5, D>, FromM<FromL, int, 4, D>>"
    );;
}
pub unsafe fn test_static_extent_mismatch_231() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_193));
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, D, 5>, FromM<FromL, int, D, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5>, FromM<FromL, int, 4>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<ToM<int, 5, D>, FromM<FromL, int, 4, D>>"
    );;
}
pub unsafe fn test_layout_232() {
    (unsafe { test_conversion_192() });
    (unsafe { test_conversion_197() });
    (unsafe { test_conversion_198() });
    (unsafe { test_conversion_199() });
    {
        (unsafe { test_no_implicit_conversion_221() });
    }
    (unsafe { test_rank_mismatch_224() });
    (unsafe { test_static_extent_mismatch_228() });
}
pub unsafe fn test_layout_233() {
    (unsafe { test_conversion_200() });
    (unsafe { test_conversion_204() });
    (unsafe { test_conversion_205() });
    (unsafe { test_conversion_206() });
    {
        (unsafe { test_no_implicit_conversion_222() });
    }
    (unsafe { test_rank_mismatch_225() });
    (unsafe { test_static_extent_mismatch_229() });
}
pub unsafe fn test_layout_234() {
    (unsafe { test_conversion_207() });
    (unsafe { test_conversion_211() });
    (unsafe { test_conversion_212() });
    (unsafe { test_conversion_213() });
    {
        (unsafe { test_no_implicit_conversion_223() });
    }
    (unsafe { test_rank_mismatch_226() });
    (unsafe { test_static_extent_mismatch_230() });
}
pub unsafe fn test_layout_235() {
    (unsafe { test_conversion_214() });
    (unsafe { test_conversion_218() });
    (unsafe { test_conversion_219() });
    (unsafe { test_conversion_220() });
    {}
    (unsafe { test_rank_mismatch_227() });
    (unsafe { test_static_extent_mismatch_231() });
}
pub unsafe fn test_236() -> () {
    (unsafe { test_layout_232() });
    (unsafe { test_layout_233() });
    (unsafe { test_layout_234() });
    (unsafe { test_layout_235() });
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
    (unsafe { test_236() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
