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
pub struct IntType {
    pub val: (),
}
impl IntType {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const IntType) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn to__4(&self) -> () {
        return self.val;
    }
    pub unsafe fn to__5(&self) -> () {
        return self.val;
    }
    pub unsafe fn to__6(&self) -> () {
        return self.val;
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { IntType::operator_eq(self, other as *const IntType) }
    }
}
impl std::cmp::Eq for IntType {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntTypeNC {
    pub val: (),
}
impl IntTypeNC {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const IntType) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn to__4(&mut self) -> () {
        return self.val;
    }
    pub unsafe fn to__5(&mut self) -> () {
        return self.val;
    }
    pub unsafe fn to__6(&mut self) -> () {
        return self.val;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntConfig_false__false__true__true_ {
    pub val: (),
}
impl IntConfig_false__false__true__true_ {
    pub unsafe fn new(mut val_: ()) -> Self {
        let mut this = Self { val: val_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntConfig_false__true__true__true_ {
    pub val: (),
}
impl IntConfig_false__true__true__true_ {
    pub unsafe fn new(mut val_: ()) -> Self {
        let mut this = Self { val: val_ };
        this
    }
    pub unsafe fn to__2(&mut self) -> () {
        return self.val;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntConfig_false__true__false__true_ {
    pub val: (),
}
impl IntConfig_false__true__false__true_ {
    pub unsafe fn new(mut val_: ()) -> Self {
        let mut this = Self { val: val_ };
        this
    }
    pub unsafe fn to__2(&mut self) -> () {
        return self.val;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntConfig_true__false__false__true_ {
    pub val: (),
}
impl IntConfig_true__false__false__true_ {
    pub unsafe fn new(mut val_: ()) -> Self {
        let mut this = Self { val: val_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntConfig_true__false__true__false_ {
    pub val: (),
}
impl IntConfig_true__false__true__false_ {
    pub unsafe fn new(mut val_: ()) -> Self {
        let mut this = Self { val: val_ };
        this
    }
    pub unsafe fn to__3(&self) -> () {
        return self.val;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntConfig_true__true__false__true_ {
    pub val: (),
}
impl IntConfig_true__true__false__true_ {
    pub unsafe fn new(mut val_: ()) -> Self {
        let mut this = Self { val: val_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntConfig_true__true__true__true_ {
    pub val: (),
}
impl IntConfig_true__true__true__true_ {
    pub unsafe fn new(mut val_: ()) -> Self {
        let mut this = Self { val: val_ };
        this
    }
    pub unsafe fn to__3(&self) -> () {
        return self.val;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct not_extents_constructible_tag {}
static mut Wrap_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__ {
    pub unsafe fn copy_from(
        other: *const layout_wrapping_integral_4__mapping_std_extents_int__,
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
    pub unsafe fn operator_call_11(&self) -> () {
        let mut idx_a: () = unimplemented!();
        return (unsafe {
            lambda_unsafe!(
                Generic,
                {
                    let this_: *const layout_wrapping_integral_4__mapping_std_extents_int__ =
                        (self as *const layout_wrapping_integral_4__mapping_std_extents_int__);
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
        });
    }
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_int__::copy_from(
                self as *const layout_wrapping_integral_4__mapping_std_extents_int__,
            )
        }
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_int__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
    pub unsafe fn copy_from(
        other : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__,
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
    pub unsafe fn operator_call_11_unsigned_int(&self, mut idx: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return (unsafe {
            lambda_unsafe! ( Generic , { let this_ : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ = ( self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_1)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_1)) } ) ) . wrapping_mul ( res ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
        });
    }
}
impl Clone
    for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__
{
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__)
        }
    }
}
impl Default
    for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__
{
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    pub unsafe fn copy_from(
        other: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
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
    pub unsafe fn operator_call_11_unsigned_int(&self, mut idx: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return (unsafe {
            lambda_unsafe! ( Generic , { let this_ : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ = ( self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_2)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_2)) } ) ) . wrapping_mul ( res ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
        });
    }
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::copy_from(
                self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
            )
        }
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    pub unsafe fn copy_from(
        other: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__,
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
    pub unsafe fn operator_call_11_unsigned_int_unsigned_int(
        &self,
        mut idx_0: (),
        mut idx_1: (),
    ) -> () {
        let mut idx_a: () = unimplemented!();
        return (unsafe {
            lambda_unsafe! ( Generic , { let this_ : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ = ( self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_3)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_3)) } ) ) . wrapping_mul ( res ) )  ;
 res  = ( ( * unimplemented!()  )  ) . wrapping_add ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_3)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_3)) } ) ) . wrapping_mul ( res ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
        });
    }
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__::copy_from(
                self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__,
            )
        }
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn operator_call_11_signed_char_signed_char_signed_char_signed_char ( &self , mut idx_0 : () , mut idx_1 : () , mut idx_2 : () , mut idx_3 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ = ( self as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_4)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_4)) } ) ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_4)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_4)) } ) ) * ( res ) ) ) )  ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_4)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_4)) } ) ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_4)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_4)) } ) ) * ( res ) ) ) )  } } } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } )  ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    pub unsafe fn copy_from(
        other: *const layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__,
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
    pub unsafe fn operator_call_11_int(&self, mut idx: ()) -> () {
        let mut idx_a: () = unimplemented!();
        return (unsafe {
            lambda_unsafe! ( Generic , { let this_ : *const layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ = ( self as *const layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_5)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_5)) } ) ) * ( res ) ) ) )  ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , )
        });
    }
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__)
        }
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
    pub unsafe fn copy_from(
        other : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__,
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
}
impl Clone
    for layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__
{
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__)
        }
    }
}
impl Default
    for layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__
{
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn operator_call_11_unsigned_char_unsigned_char ( &self , mut idx_0 : () , mut idx_1 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ = ( self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_7)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_7)) } ) ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_7)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_7)) } ) ) * ( res ) ) ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } )  ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn operator_call_11_int_int ( &self , mut idx_0 : () , mut idx_1 : () , ) -> () { let mut idx_a : () = unimplemented!() ;
  ;
 ;
 return ( unsafe { lambda_unsafe! ( Generic , { let this_ : *const layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ = ( self as *const layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__  ) ;
 let idx_a : *mut () = & mut idx_a ;
 } , | mut _a0 : () , | -> () { let mut res : () = 0_() ;
  ;
 ;
 { res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_8)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_8)) } ) ) * ( res ) ) ) )  ;
 res  = ( ( ( * unimplemented!()  )  ) + ( ( ( ( if ( ( unimplemented!()  ) < ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_8)) ) ) { unimplemented!()  } else { (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_8)) } ) ) * ( res ) ) ) )  } ;
 return res  ;
 } )  .spec::<fn(() ,) -> () >(0) .call ( unsafe { std::mem::zeroed::<() >() } , ) } )  ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct layout_wrapping_integral_4_ {}
pub unsafe fn construct_mapping_9(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_10(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_11(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_12(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_13(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_14(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_15(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_16(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_17(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_18(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_19(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_20(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_21(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_22(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_23(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_24(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_25(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_26(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_27(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_28(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_29(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_30(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_31(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}
pub unsafe fn construct_mapping_32(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_33(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}
pub unsafe fn construct_mapping_34(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__::new_1(
        { &exts },
        { <not_extents_constructible_tag>::default() },
    );
}pub unsafe fn construct_mapping_35 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_36(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    return layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__::new_1(
        { &exts },
        { <not_extents_constructible_tag>::default() },
    );
}
pub unsafe fn construct_mapping_37(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_38 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_39(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__
{
    return layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct always_convertible_layout {}
pub unsafe fn check_operator_constraints_40(mut m: (), mut idxs: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_41(mut m: (), mut idxs: IntType) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_42(mut m: (), mut idxs_1: (), mut idxs_2: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_43(mut m: (), mut idxs_1: (), mut idxs_2: IntType) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_44(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__true__true_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_45(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__false__true_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_46(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_true__false__true__false_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_47(mut m: (), mut idxs: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_48(mut m: (), mut idxs: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_49(mut m: (), mut idxs: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_50(mut m: (), mut idxs: IntType) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_51(mut m: (), mut idxs_1: (), mut idxs_2: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_52(mut m: (), mut idxs_1: (), mut idxs_2: IntType) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_53(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__true__true_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_54(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__false__true_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_55(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_true__false__true__false_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_56(mut m: (), mut idxs: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_57(mut m: (), mut idxs: IntType) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_58(mut m: (), mut idxs_1: (), mut idxs_2: ()) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_59(mut m: (), mut idxs_1: (), mut idxs_2: IntType) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_60(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__true__true_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_61(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__false__true_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_62(
    mut m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_true__false__true__false_,
) -> () {
    &(unimplemented!());
    return true;
}
pub unsafe fn check_operator_constraints_63(mut _a0: (), mut _a1: (), mut _a2: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_64(mut _a0: (), mut _a1: IntType) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_65(mut _a0: (), mut _a1: IntType) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_66(
    mut _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_false__false__true__true_,
) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_67(
    mut _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_true__false__false__true_,
) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_68(mut _a0: (), mut _a1: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_69(mut _a0: (), mut _a1: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_70(mut _a0: (), mut _a1: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_71(mut _a0: (), mut _a1: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_72(mut _a0: (), mut _a1: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_73(mut _a0: (), mut _a1: (), mut _a2: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_74(mut _a0: (), mut _a1: IntType) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_75(mut _a0: (), mut _a1: IntType) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_76(
    mut _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_false__false__true__true_,
) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_77(
    mut _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_true__false__false__true_,
) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_78(mut _a0: (), mut _a1: (), mut _a2: ()) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_79(mut _a0: (), mut _a1: IntType) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_80(mut _a0: (), mut _a1: IntType) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_81(
    mut _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_false__false__true__true_,
) -> () {
    return false;
}
pub unsafe fn check_operator_constraints_82(
    mut _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_true__false__false__true_,
) -> () {
    return false;
}
pub unsafe fn iterate_83(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_84(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_85(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_85(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_86(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_87(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_87(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_88(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_89(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_89(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_90(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_90(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_91(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_92(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_92(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_93(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_93(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_94(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_94(mut mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_95(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_95(
    mut mds: (),
    mut args_1: (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_96(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_97(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_98(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_98(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_99(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_100(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_100(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_101(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_102(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_102(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_103(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_103(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_104(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_105(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_105(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_106(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_106(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_107(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_107(mut mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_108(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_108(
    mut mds: (),
    mut args_1: (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_109(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_110(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_111(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_111(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_112(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_113(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_113(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_114(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_115(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_115(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_116(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_116(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_117(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_118(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_118(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_119(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_119(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_120(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_120(mut mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_121(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_121(
    mut mds: (),
    mut args_1: (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_122(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_123(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_123(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_124(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_124(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_125(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_125(mut mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_126(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_126(
    mut mds: (),
    mut args_1: (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_127(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_128(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_128(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_129(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_129(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_130(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_130(mut mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_131(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_131(
    mut mds: (),
    mut args_1: (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_132(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_133(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_133(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_134(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_134(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_135(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_135(mut mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_136(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_136(
    mut mds: (),
    mut args_1: (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn iterate_137(mut mds: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (0));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_138(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_138(mut mds: (), mut args: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (1));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_139(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_139(mut mds: (), mut args_1: (), mut args_2: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (2));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_140(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_140(mut mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let r: () = (((unimplemented!()) - (1_())) - (3));
    {
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            (unsafe { iterate_141(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub unsafe fn iterate_141(
    mut mds: (),
    mut args_1: (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let r: () = (((unimplemented!()) - (1_())) - (4));
    {
        let mut ptr1: *mut () = (&mut (*unimplemented!()) as *mut ());
        let mut ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut args_arr: () = unimplemented!();
        let mut ptr3: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
        let mut ptr4: *mut () = (&mut (*unimplemented!()) as *mut ());
        unimplemented!();
    }
}
pub unsafe fn test_iteration_142(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_83(unimplemented!()) });
}
pub unsafe fn test_iteration_143(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_84(unimplemented!()) });
}
pub unsafe fn test_iteration_144(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_86(unimplemented!()) });
}
pub unsafe fn test_iteration_145(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_88(unimplemented!()) });
}
pub unsafe fn test_iteration_146(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_91(unimplemented!()) });
}
pub unsafe fn test_iteration_147(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_96(unimplemented!()) });
}
pub unsafe fn test_iteration_148(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_97(unimplemented!()) });
}
pub unsafe fn test_iteration_149(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_99(unimplemented!()) });
}
pub unsafe fn test_iteration_150(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_101(unimplemented!()) });
}
pub unsafe fn test_iteration_151(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_104(unimplemented!()) });
}
pub unsafe fn test_iteration_152(mut m: layout_wrapping_integral_4__mapping_std_extents_int__) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_109(unimplemented!()) });
}
pub unsafe fn test_iteration_153(
    mut m: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__,
) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_110(unimplemented!()) });
}
pub unsafe fn test_iteration_154(
    mut m: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_112(unimplemented!()) });
}
pub unsafe fn test_iteration_155(
    mut m: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__,
) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_114(unimplemented!()) });
}
pub unsafe fn test_iteration_156(
    mut m : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__,
) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_117(unimplemented!()) });
}
pub unsafe fn test_iteration_157(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_122(unimplemented!()) });
}
pub unsafe fn test_iteration_158(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_127(unimplemented!()) });
}
pub unsafe fn test_iteration_159(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_132(unimplemented!()) });
}
pub unsafe fn test_iteration_160(mut m: ()) {
    let mut data: () = unimplemented!();
    let mut mds: () = unimplemented!();
    (unsafe { iterate_137(unimplemented!()) });
}
pub unsafe fn test_layout_161() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_162));
    (unsafe {
        test_iteration_142((unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_143((unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_143((unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_144((unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_145((unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_146((unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) }))
    });
    let mut data: [(); 1] = [unimplemented!(); 1];
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D, D>(1, 1))), int(0), size_t(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))), unsigned(0), IntType(0))"
    );;
    let t: () = true;
    let o: () = false;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, o, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, t, o>(0))"
    );;
    {
        const _: () = assert!(
            true,
            "!check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<o, o, t, t>(0)}})"
        );;
        const _: () = assert!(
            true,
            "!check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<o, t, t, t>(0)}})"
        );;
        const _: () = assert!(
            true,
            "!check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<t, o, o, t>(0)}})"
        );;
        const _: () = assert!(
            true,
            "!check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<t, t, o, t>(0)}})"
        );;
        const _: () = assert!(
            true,
            "check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<t, o, t, o>(0)}})"
        );;
        const _: () = assert!(
            true,
            "check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<t, t, t, t>(0)}})"
        );;
        {
            let mut idx: () = unimplemented!();
            let mut s: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut idx: () = unimplemented!();
            let mut s: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut idx: () = unimplemented!();
            let mut s: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut idx: () = unimplemented!();
            let mut s: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut idx: () = unimplemented!();
            let mut s: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut idx: () = unimplemented!();
            let mut s: () = unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_layout_163() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_162));
    (unsafe {
        test_iteration_147((unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_148((unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_148((unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_149((unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_150((unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_151((unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) }))
    });
    let mut data: [(); 1] = [unimplemented!(); 1];
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D, D>(1, 1))), int(0), size_t(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))), unsigned(0), IntType(0))"
    );;
    let t: () = true;
    let o: () = false;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, o, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, t, o>(0))"
    );;
    {}
}
pub unsafe fn test_layout_164() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_162));
    (unsafe {
        test_iteration_152(
            (unsafe {
                construct_mapping_31(<layout_wrapping_integral_4_>::default(), unimplemented!())
            }),
        )
    });
    (unsafe {
        test_iteration_153(
            (unsafe {
                construct_mapping_32(<layout_wrapping_integral_4_>::default(), unimplemented!())
            }),
        )
    });
    (unsafe {
        test_iteration_153(
            (unsafe {
                construct_mapping_32(<layout_wrapping_integral_4_>::default(), unimplemented!())
            }),
        )
    });
    (unsafe {
        test_iteration_154(
            (unsafe {
                construct_mapping_33(<layout_wrapping_integral_4_>::default(), unimplemented!())
            }),
        )
    });
    (unsafe {
        test_iteration_155(
            (unsafe {
                construct_mapping_34(<layout_wrapping_integral_4_>::default(), unimplemented!())
            }),
        )
    });
    (unsafe {
        test_iteration_156(
            (unsafe {
                construct_mapping_35(<layout_wrapping_integral_4_>::default(), unimplemented!())
            }),
        )
    });
    let mut data: [(); 1] = [unimplemented!(); 1];
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D, D>(1, 1))), int(0), size_t(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))), unsigned(0), IntType(0))"
    );;
    let t: () = true;
    let o: () = false;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, o, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, t, o>(0))"
    );;
    {}
}
pub unsafe fn test_layout_large_165() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_162));
    (unsafe {
        test_iteration_157((unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_158((unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) }))
    });
}
pub unsafe fn test_layout_large_166() {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_162));
    (unsafe {
        test_iteration_159((unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) }))
    });
    (unsafe {
        test_iteration_160((unsafe { construct_mapping_30(unimplemented!(), unimplemented!()) }))
    });
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct NoCopyIndex {
    pub val: (),
}
impl NoCopyIndex {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return self.val;
    }
}
impl Default for NoCopyIndex {
    fn default() -> Self {
        unsafe { NoCopyIndex { val: 0_() } }
    }
}
pub unsafe fn test_index_cast_167() {
    let mut data: () = unimplemented!();
    let m: () = unimplemented!();
    let mut indices: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RValueInt {}
impl RValueInt {
    pub unsafe fn to_(&mut self) -> () {
        return 0_();
    }
}
pub unsafe fn test_168() -> () {
    (unsafe { test_layout_161() });
    (unsafe { test_layout_163() });
    (unsafe { test_layout_164() });
    let mut data: [(); 1] = [unimplemented!()];
    let mut m: () = unimplemented!();
    &(unimplemented!());
    (unsafe { test_index_cast_167() });
    return true;
}
pub unsafe fn test_large_169() -> () {
    (unsafe { test_layout_large_165() });
    (unsafe { test_layout_large_166() });
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
    (unsafe { test_168() });
    const _: () = assert!(true, "test()");;
    (unsafe { test_large_169() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Wrap_0);
    std::cell::LazyCell::force(&*&raw const Wrap_1);
    std::cell::LazyCell::force(&*&raw const Wrap_2);
    std::cell::LazyCell::force(&*&raw const Wrap_3);
    std::cell::LazyCell::force(&*&raw const Wrap_4);
    std::cell::LazyCell::force(&*&raw const Wrap_5);
    std::cell::LazyCell::force(&*&raw const Wrap_6);
    std::cell::LazyCell::force(&*&raw const Wrap_7);
    std::cell::LazyCell::force(&*&raw const Wrap_8);
}
