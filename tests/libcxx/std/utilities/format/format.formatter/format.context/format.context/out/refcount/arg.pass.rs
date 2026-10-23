extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_basic_format_arg_0(arg: (), expected: ()) -> () {
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let visitor : Value<FnPtr<Generic> > = Rc::new(RefCell::new(lambda! ( Generic , { let expected : Value<() > = Rc::new(RefCell::new((*expected.borrow()) )) ;
 } , | a : std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_string___char___handle  , | -> ()  { let a : Value<std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_string___char___handle >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return ( a == (*expected.borrow()) )   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : Ptr< ()  >  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } ) )) ;
    return unimplemented!();
}
pub fn test_basic_format_arg_1(arg: (), expected: ()) -> () {
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let visitor : Value<FnPtr<Generic> > = Rc::new(RefCell::new(lambda! ( Generic , { let expected : Value<() > = Rc::new(RefCell::new((*expected.borrow()) )) ;
 } , | a : std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_string___char___handle  , | -> ()  { let a : Value<std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_string___char___handle >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return ( a == (*expected.borrow()) )   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : Ptr< ()  >  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } ) )) ;
    return unimplemented!();
}
pub fn test_basic_format_arg_2(arg: (), expected: ()) -> () {
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let visitor : Value<FnPtr<Generic> > = Rc::new(RefCell::new(lambda! ( Generic , { let expected : Value<() > = Rc::new(RefCell::new((*expected.borrow()) )) ;
 } , | a : std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_string___char___handle  , | -> ()  { let a : Value<std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_string___char___handle >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return ( a == (*expected.borrow()) )   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : Ptr< ()  >  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } ) )) ;
    return unimplemented!();
}
pub fn test_basic_format_arg_3(arg: (), expected: ()) -> () {
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let visitor : Value<FnPtr<Generic> > = Rc::new(RefCell::new(lambda! ( Generic , { let expected : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
 } , | a : std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_string___char___handle  , | -> ()  { let a : Value<std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_string___char___handle >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : Ptr< ()  >  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return unimplemented!()    ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } ) )) ;
    return unimplemented!();
}
pub fn test_basic_format_arg_4(arg: (), expected: ()) -> () {
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let visitor : Value<FnPtr<Generic> > = Rc::new(RefCell::new(lambda! ( Generic , { let expected : Value<() > = Rc::new(RefCell::new((*expected.borrow()) )) ;
 } , | a : std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___handle  , | -> ()  { let a : Value<std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___handle >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return ( a == (*expected.borrow()) )   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : Ptr< ()  >  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } ) )) ;
    return unimplemented!();
}
pub fn test_basic_format_arg_5(arg: (), expected: ()) -> () {
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let visitor : Value<FnPtr<Generic> > = Rc::new(RefCell::new(lambda! ( Generic , { let expected : Value<() > = Rc::new(RefCell::new((*expected.borrow()) )) ;
 } , | a : std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___handle  , | -> ()  { let a : Value<std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___handle >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return ( a == (*expected.borrow()) )   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : Ptr< ()  >  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } ) )) ;
    return unimplemented!();
}
pub fn test_basic_format_arg_6(arg: (), expected: ()) -> () {
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let visitor : Value<FnPtr<Generic> > = Rc::new(RefCell::new(lambda! ( Generic , { let expected : Value<() > = Rc::new(RefCell::new((*expected.borrow()) )) ;
 } , | a : std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___handle  , | -> ()  { let a : Value<std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___handle >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return ( a == (*expected.borrow()) )   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : Ptr< ()  >  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } ) )) ;
    return unimplemented!();
}
pub fn test_basic_format_arg_7(arg: (), expected: ()) -> () {
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let visitor : Value<FnPtr<Generic> > = Rc::new(RefCell::new(lambda! ( Generic , { let expected : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
 } , | a : std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___handle  , | -> ()  { let a : Value<std_basic_format_arg_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___handle >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | mut a : Ptr< ()  >  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } , | a : ()  , | -> ()  { let a : Value<() >  = Rc::new(RefCell::new(a)) ;
 { return unimplemented!()    ;
 } panic!("ub: non-void function does not return a value") } , | mut a : ()  , | -> ()  { { return false   ;
 } panic!("ub: non-void function does not return a value") } ) )) ;
    return unimplemented!();
}
pub fn test_format_context_create_8(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_9(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
pub fn test_format_context_create_10(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_11(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
pub fn test_12() {
    let b: Value<()> = Rc::new(RefCell::new(true));
    let c: Value<()> = Rc::new(RefCell::new(('a' as ())));
    let a: Value<()> = Rc::new(RefCell::new(42_()));
    let string: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let store : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___bool__char__int__std_string_ > = Rc::new(RefCell::new(( { make_format_args_13 ( b .as_pointer()  , c .as_pointer()  , a .as_pointer()  , string .as_pointer()  , ) } )  )) ;
    let args : Value<() > = Rc::new(RefCell::new(std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new ( { store .as_pointer()  } , )  )) ;
    let mut output: () = unimplemented!();
    let context: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_8(unimplemented!(), (*args.borrow()).clone()) }),
    ));
    unimplemented!();
    const _: () = assert!(true, "");;
    let mut i: () = 0_();
    let mut e: () = ({ (*args.borrow()).__size() });
    'loop_: while (i != e) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_14() {
    let b: Value<()> = Rc::new(RefCell::new(true));
    let c: Value<()> = Rc::new(RefCell::new(('a' as ())));
    let a: Value<()> = Rc::new(RefCell::new(42_()));
    let string: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let store : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___bool__wchar_t__int__std_wstring_ > = Rc::new(RefCell::new(( { make_format_args_15 ( b .as_pointer()  , c .as_pointer()  , a .as_pointer()  , string .as_pointer()  , ) } )  )) ;
    let args : Value<() > = Rc::new(RefCell::new(std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new ( { store .as_pointer()  } , )  )) ;
    let mut output: () = unimplemented!();
    let context: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_10(unimplemented!(), (*args.borrow()).clone()) }),
    ));
    unimplemented!();
    const _: () = assert!(true, "");;
    let mut i: () = 0_();
    let mut e: () = ({ (*args.borrow()).__size() });
    'loop_: while (i != e) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({ test_12() });
    ({ test_14() });
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub fn __cpp2rust_init_globals() {}
