extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(Ptr< i8  >  , to_pconsti8)]
#[cpp_into(Ptr< i32  >  , to_pconsti32)]
#[cpp_into(Ptr< u8  >  , to_pconstu8)]
#[cpp_into(Ptr< u16  >  , to_pconstu16)]
#[cpp_into(Ptr< u32  >  , to_pconstu32)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<i32>,
    #[offset(8)]
    pub wn_: usize,
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<u8>,
    #[offset(24)]
    pub u8n_: usize,
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<u16>,
    #[offset(40)]
    pub u16n_: usize,
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<u32>,
    #[offset(56)]
    pub u32n_: usize,
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<i8>,
    #[offset(72)]
    pub sn_: usize,
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<i32>,
        mut wn: usize,
        mut u8: Ptr<u8>,
        mut u8n: usize,
        mut u16: Ptr<u16>,
        mut u16n: usize,
        mut u32: Ptr<u32>,
        mut u32n: usize,
        mut s: Ptr<i8>,
        mut sn: usize,
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
pub fn test_0(mut value: bool) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<bool> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!((((*v.borrow()) as i32) == ((*a.borrow()) as i32)));
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_1(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i8> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!((((*v.borrow()) as i32) == ((*a.borrow()) as i32)));
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_2(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new((value as i32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_3(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new((value as i32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_4(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u32> = Rc::new(RefCell::new((value as u32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_5(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new((value as i32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_6(mut value: i16) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new((value as i32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_7(mut value: i32) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_8(mut value: i64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_9(mut value: i64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_10(mut value: u8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u32> = Rc::new(RefCell::new((value as u32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_11(mut value: u16) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u32> = Rc::new(RefCell::new((value as u32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_12(mut value: u32) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u32> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_13(mut value: u64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_14(mut value: u64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_15(mut value: f32) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<f32> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_16(mut value: f64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<f64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_17(mut value: f64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<f64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_18(mut value: Ptr<i8>) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<Ptr<i8>> = Rc::new(RefCell::new((value).clone()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(({ (*v.borrow()).clone() } == { (*a.borrow()).clone() }));
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_19(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(unimplemented!());
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_20(mut value: Vec<i8>) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(unimplemented!());
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_21(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<AnyPtr> = Rc::new(RefCell::new(AnyPtr::default()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(({ (*v.borrow()).clone() } == { (*a.borrow()).clone() }));
            }
        }
    )));
    unimplemented!();
}
pub fn test_22(mut value: bool) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<bool> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!((((*v.borrow()) as i32) == ((*a.borrow()) as i32)));
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_23(mut value: i32) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_24(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new((value as i32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_25(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new((value as i32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_26(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u32> = Rc::new(RefCell::new((value as u32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_27(mut value: i8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new((value as i32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_28(mut value: i16) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new((value as i32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_29(mut value: i32) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i32> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_30(mut value: i64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_31(mut value: i64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<i64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_32(mut value: u8) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u32> = Rc::new(RefCell::new((value as u32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_33(mut value: u16) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u32> = Rc::new(RefCell::new((value as u32)));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_34(mut value: u32) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u32> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_35(mut value: u64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_36(mut value: u64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<u64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_37(mut value: f32) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<f32> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_38(mut value: f64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<f64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_39(mut value: f64) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<f64> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(((*v.borrow()) == (*a.borrow())));
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_40(mut value: Ptr<i32>) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<Ptr<i32>> = Rc::new(RefCell::new((value).clone()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(({ (*v.borrow()).clone() } == { (*a.borrow()).clone() }));
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_41(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(unimplemented!());
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_42(mut value: Vec<i32>) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(unimplemented!());
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        }
    )));
    unimplemented!();
}
pub fn test_43(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<AnyPtr> = Rc::new(RefCell::new(AnyPtr::default()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                assert!(false);
            }
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            {
                assert!(({ (*v.borrow()).clone() } == { (*a.borrow()).clone() }));
            }
        }
    )));
    unimplemented!();
}
pub fn test_handle_44(mut value: i128) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        }
    )));
    unimplemented!();
}
pub fn test_handle_45(mut value: u128) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i8| {
            let a: Value<i8> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: Ptr<i8>| {
            let a: Value<Ptr<i8>> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        }
    )));
    unimplemented!();
}
pub fn test_handle_46(mut value: i128) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        }
    )));
    unimplemented!();
}
pub fn test_handle_47(mut value: u128) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: bool| {
            let a: Value<bool> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: u32| {
            let a: Value<u32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: i64| {
            let a: Value<i64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: u64| {
            let a: Value<u64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f32| {
            let a: Value<f32> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: f64| {
            let a: Value<f64> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: Ptr<i32>| {
            let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        },
        |a: AnyPtr| {
            let a: Value<AnyPtr> = Rc::new(RefCell::new(a));
            assert!((unimplemented!()));
        }
    )));
    unimplemented!();
}
pub fn test_48() {
    let empty: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    let str: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __v = Vec::with_capacity(
            ({
                MultiStringTypeImpl::length_7(
                    &Rc::new(RefCell::new(MultiStringType::new(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[i32; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u8>::from_string_literal(&[97 as u8, 98 as u8, 99 as u8, 0 as u8])
                        },
                        {
                            ((::std::mem::size_of::<[u8; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u16>::from_string_literal(&[
                                97 as u16, 98 as u16, 99 as u16, 0 as u16,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[u16; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u32>::from_string_literal(&[
                                97 as u32, 98 as u32, 99 as u32, 0 as u32,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[u32; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        { Ptr::<i8>::from_string_literal(b"abc") },
                        { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                    )))
                    .as_pointer(),
                    (Ptr::<()>::null()).reinterpret_cast::<i8>(),
                )
            }) as usize
                + 1,
        );
        ({
            MultiStringTypeImpl::as_ptr_2(
                &Rc::new(RefCell::new(MultiStringType::new(
                    {
                        Ptr::<i32>::from_string_literal(&[
                            97 as i32, 98 as i32, 99 as i32, 0 as i32,
                        ])
                    },
                    {
                        ((::std::mem::size_of::<[i32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<i32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { Ptr::<u8>::from_string_literal(&[97 as u8, 98 as u8, 99 as u8, 0 as u8]) },
                    {
                        ((::std::mem::size_of::<[u8; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u8>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    {
                        Ptr::<u16>::from_string_literal(&[
                            97 as u16, 98 as u16, 99 as u16, 0 as u16,
                        ])
                    },
                    {
                        ((::std::mem::size_of::<[u16; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u16>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    {
                        Ptr::<u32>::from_string_literal(&[
                            97 as u32, 98 as u32, 99 as u32, 0 as u32,
                        ])
                    },
                    {
                        ((::std::mem::size_of::<[u32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { Ptr::<i8>::from_string_literal(b"abc") },
                    { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                )))
                .as_pointer(),
                (Ptr::<()>::null()).reinterpret_cast::<i8>(),
            )
        })
        .with_slice(
            ({
                MultiStringTypeImpl::length_7(
                    &Rc::new(RefCell::new(MultiStringType::new(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[i32; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u8>::from_string_literal(&[97 as u8, 98 as u8, 99 as u8, 0 as u8])
                        },
                        {
                            ((::std::mem::size_of::<[u8; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u16>::from_string_literal(&[
                                97 as u16, 98 as u16, 99 as u16, 0 as u16,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[u16; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u32>::from_string_literal(&[
                                97 as u32, 98 as u32, 99 as u32, 0 as u32,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[u32; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        { Ptr::<i8>::from_string_literal(b"abc") },
                        { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                    )))
                    .as_pointer(),
                    (Ptr::<()>::null()).reinterpret_cast::<i8>(),
                )
            }) as usize,
            |__s| __v.extend_from_slice(__s),
        );
        __v.push(0_i8);
        __v
    }));
    ({ test_0(true) });
    ({ test_0(false) });
    ({ test_1(('a' as i8)) });
    ({ test_1(('z' as i8)) });
    ({ test_1(('0' as i8)) });
    ({ test_1(('9' as i8)) });
    {
        ({ test_1(('a' as i8)) });
        ({ test_1(('z' as i8)) });
        ({ test_1(('0' as i8)) });
        ({ test_1(('9' as i8)) });
    }
    ({ test_5(i8::MIN) });
    ({ test_5(0_i8) });
    ({ test_5(i8::MAX) });
    ({ test_6(i16::MIN) });
    ({ test_6((i8::MIN as i16)) });
    ({ test_6(0_i16) });
    ({ test_6((i8::MAX as i16)) });
    ({ test_6(i16::MAX) });
    ({ test_7(i32::MIN) });
    ({ test_7((i16::MIN as i32)) });
    ({ test_7((i8::MIN as i32)) });
    ({ test_7(0) });
    ({ test_7((i8::MAX as i32)) });
    ({ test_7((i16::MAX as i32)) });
    ({ test_7(i32::MAX) });
    ({ test_8(i64::MIN) });
    ({ test_8((i32::MIN as i64)) });
    ({ test_8((i16::MIN as i64)) });
    ({ test_8((i8::MIN as i64)) });
    ({ test_8(0_i64) });
    ({ test_8((i8::MAX as i64)) });
    ({ test_8((i16::MAX as i64)) });
    ({ test_8((i32::MAX as i64)) });
    ({ test_8(i64::MAX) });
    ({ test_9(i64::MIN) });
    ({ test_9(i64::MIN) });
    ({ test_9((i32::MIN as i64)) });
    ({ test_9((i16::MIN as i64)) });
    ({ test_9((i8::MIN as i64)) });
    ({ test_9(0_i64) });
    ({ test_9((i8::MAX as i64)) });
    ({ test_9((i16::MAX as i64)) });
    ({ test_9((i32::MAX as i64)) });
    ({ test_9(i64::MAX) });
    ({ test_9(i64::MAX) });
    ({ test_handle_44(0_i128) });
    ({ test_10(0_u8) });
    ({ test_10(u8::MAX) });
    ({ test_11(0_u16) });
    ({ test_11((u8::MAX as u16)) });
    ({ test_11(u16::MAX) });
    ({ test_12(0_u32) });
    ({ test_12((u8::MAX as u32)) });
    ({ test_12((u16::MAX as u32)) });
    ({ test_12(u32::MAX) });
    ({ test_13(0_u64) });
    ({ test_13((u8::MAX as u64)) });
    ({ test_13((u16::MAX as u64)) });
    ({ test_13((u32::MAX as u64)) });
    ({ test_13(u64::MAX) });
    ({ test_14(0_u64) });
    ({ test_14((u8::MAX as u64)) });
    ({ test_14((u16::MAX as u64)) });
    ({ test_14((u32::MAX as u64)) });
    ({ test_14(u64::MAX) });
    ({ test_14(unimplemented!()) });
    ({ test_handle_45(0_u128) });
    ({ test_15(-f32::MAX) });
    ({ test_15(-f32::MIN_POSITIVE) });
    ({ test_15((-0_f64 as f32)) });
    ({ test_15((0_f64 as f32)) });
    ({ test_15(f32::MIN_POSITIVE) });
    ({ test_15(f32::MAX) });
    ({ test_16(-f64::MAX) });
    ({ test_16(-f64::MIN_POSITIVE) });
    ({ test_16(-0_f64) });
    ({ test_16(0_f64) });
    ({ test_16(f64::MIN_POSITIVE) });
    ({ test_16(f64::MAX) });
    ({ test_17(-f64::MAX) });
    ({ test_17(-f64::MIN_POSITIVE) });
    ({ test_17(-0_f64) });
    ({ test_17(0_f64) });
    ({ test_17(f64::MIN_POSITIVE) });
    ({ test_17(f64::MAX) });
    ({ test_18(empty.decay()) });
    ({ test_18(str.decay()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_20(vec![i8::default()]) });
    ({ test_20((*empty.borrow()).clone()) });
    ({ test_20((*str.borrow()).clone()) });
    ({ test_21(Default::default()) });
}
pub fn test_49() {
    let empty: Value<Vec<i32>> = Rc::new(RefCell::new(vec![i32::default()]));
    let str: Value<Vec<i32>> = Rc::new(RefCell::new({
        let mut __v = Vec::with_capacity(
            ({
                MultiStringTypeImpl::length_8(
                    &Rc::new(RefCell::new(MultiStringType::new(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[i32; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u8>::from_string_literal(&[97 as u8, 98 as u8, 99 as u8, 0 as u8])
                        },
                        {
                            ((::std::mem::size_of::<[u8; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u16>::from_string_literal(&[
                                97 as u16, 98 as u16, 99 as u16, 0 as u16,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[u16; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u32>::from_string_literal(&[
                                97 as u32, 98 as u32, 99 as u32, 0 as u32,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[u32; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        { Ptr::<i8>::from_string_literal(b"abc") },
                        { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                    )))
                    .as_pointer(),
                    (Ptr::<()>::null()).reinterpret_cast::<i32>(),
                )
            }) as usize
                + 1,
        );
        ({
            MultiStringTypeImpl::as_ptr_3(
                &Rc::new(RefCell::new(MultiStringType::new(
                    {
                        Ptr::<i32>::from_string_literal(&[
                            97 as i32, 98 as i32, 99 as i32, 0 as i32,
                        ])
                    },
                    {
                        ((::std::mem::size_of::<[i32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<i32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { Ptr::<u8>::from_string_literal(&[97 as u8, 98 as u8, 99 as u8, 0 as u8]) },
                    {
                        ((::std::mem::size_of::<[u8; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u8>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    {
                        Ptr::<u16>::from_string_literal(&[
                            97 as u16, 98 as u16, 99 as u16, 0 as u16,
                        ])
                    },
                    {
                        ((::std::mem::size_of::<[u16; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u16>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    {
                        Ptr::<u32>::from_string_literal(&[
                            97 as u32, 98 as u32, 99 as u32, 0 as u32,
                        ])
                    },
                    {
                        ((::std::mem::size_of::<[u32; 4]>() as usize)
                            .wrapping_div((::std::mem::size_of::<u32>() as usize))
                            as usize)
                            .wrapping_sub(1_usize)
                    },
                    { Ptr::<i8>::from_string_literal(b"abc") },
                    { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                )))
                .as_pointer(),
                (Ptr::<()>::null()).reinterpret_cast::<i32>(),
            )
        })
        .with_slice(
            ({
                MultiStringTypeImpl::length_8(
                    &Rc::new(RefCell::new(MultiStringType::new(
                        {
                            Ptr::<i32>::from_string_literal(&[
                                97 as i32, 98 as i32, 99 as i32, 0 as i32,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[i32; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u8>::from_string_literal(&[97 as u8, 98 as u8, 99 as u8, 0 as u8])
                        },
                        {
                            ((::std::mem::size_of::<[u8; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u8>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u16>::from_string_literal(&[
                                97 as u16, 98 as u16, 99 as u16, 0 as u16,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[u16; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u16>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        {
                            Ptr::<u32>::from_string_literal(&[
                                97 as u32, 98 as u32, 99 as u32, 0 as u32,
                            ])
                        },
                        {
                            ((::std::mem::size_of::<[u32; 4]>() as usize)
                                .wrapping_div((::std::mem::size_of::<u32>() as usize))
                                as usize)
                                .wrapping_sub(1_usize)
                        },
                        { Ptr::<i8>::from_string_literal(b"abc") },
                        { (::std::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1_usize) },
                    )))
                    .as_pointer(),
                    (Ptr::<()>::null()).reinterpret_cast::<i32>(),
                )
            }) as usize,
            |__s| __v.extend_from_slice(__s),
        );
        __v.push(0_i32);
        __v
    }));
    ({ test_22(true) });
    ({ test_22(false) });
    ({ test_23((('a' as i8) as i32)) });
    ({ test_23((('z' as i8) as i32)) });
    ({ test_23((('0' as i8) as i32)) });
    ({ test_23((('9' as i8) as i32)) });
    {
        {
            ({ test_24(('a' as i8)) });
            ({ test_24(('z' as i8)) });
            ({ test_24(('0' as i8)) });
            ({ test_24(('9' as i8)) });
        }
    }
    ({ test_27(i8::MIN) });
    ({ test_27(0_i8) });
    ({ test_27(i8::MAX) });
    ({ test_28(i16::MIN) });
    ({ test_28((i8::MIN as i16)) });
    ({ test_28(0_i16) });
    ({ test_28((i8::MAX as i16)) });
    ({ test_28(i16::MAX) });
    ({ test_29(i32::MIN) });
    ({ test_29((i16::MIN as i32)) });
    ({ test_29((i8::MIN as i32)) });
    ({ test_29(0) });
    ({ test_29((i8::MAX as i32)) });
    ({ test_29((i16::MAX as i32)) });
    ({ test_29(i32::MAX) });
    ({ test_30(i64::MIN) });
    ({ test_30((i32::MIN as i64)) });
    ({ test_30((i16::MIN as i64)) });
    ({ test_30((i8::MIN as i64)) });
    ({ test_30(0_i64) });
    ({ test_30((i8::MAX as i64)) });
    ({ test_30((i16::MAX as i64)) });
    ({ test_30((i32::MAX as i64)) });
    ({ test_30(i64::MAX) });
    ({ test_31(i64::MIN) });
    ({ test_31(i64::MIN) });
    ({ test_31((i32::MIN as i64)) });
    ({ test_31((i16::MIN as i64)) });
    ({ test_31((i8::MIN as i64)) });
    ({ test_31(0_i64) });
    ({ test_31((i8::MAX as i64)) });
    ({ test_31((i16::MAX as i64)) });
    ({ test_31((i32::MAX as i64)) });
    ({ test_31(i64::MAX) });
    ({ test_31(i64::MAX) });
    ({ test_handle_46(0_i128) });
    ({ test_32(0_u8) });
    ({ test_32(u8::MAX) });
    ({ test_33(0_u16) });
    ({ test_33((u8::MAX as u16)) });
    ({ test_33(u16::MAX) });
    ({ test_34(0_u32) });
    ({ test_34((u8::MAX as u32)) });
    ({ test_34((u16::MAX as u32)) });
    ({ test_34(u32::MAX) });
    ({ test_35(0_u64) });
    ({ test_35((u8::MAX as u64)) });
    ({ test_35((u16::MAX as u64)) });
    ({ test_35((u32::MAX as u64)) });
    ({ test_35(u64::MAX) });
    ({ test_36(0_u64) });
    ({ test_36((u8::MAX as u64)) });
    ({ test_36((u16::MAX as u64)) });
    ({ test_36((u32::MAX as u64)) });
    ({ test_36(u64::MAX) });
    ({ test_36(unimplemented!()) });
    ({ test_handle_47(0_u128) });
    ({ test_37(-f32::MAX) });
    ({ test_37(-f32::MIN_POSITIVE) });
    ({ test_37((-0_f64 as f32)) });
    ({ test_37((0_f64 as f32)) });
    ({ test_37(f32::MIN_POSITIVE) });
    ({ test_37(f32::MAX) });
    ({ test_38(-f64::MAX) });
    ({ test_38(-f64::MIN_POSITIVE) });
    ({ test_38(-0_f64) });
    ({ test_38(0_f64) });
    ({ test_38(f64::MIN_POSITIVE) });
    ({ test_38(f64::MAX) });
    ({ test_39(-f64::MAX) });
    ({ test_39(-f64::MIN_POSITIVE) });
    ({ test_39(-0_f64) });
    ({ test_39(0_f64) });
    ({ test_39(f64::MIN_POSITIVE) });
    ({ test_39(f64::MAX) });
    ({ test_40(empty.decay()) });
    ({ test_40(str.decay()) });
    ({ test_41(unimplemented!()) });
    ({ test_41(unimplemented!()) });
    ({ test_41(unimplemented!()) });
    ({ test_42(vec![i32::default()]) });
    ({ test_42((*empty.borrow()).clone()) });
    ({ test_42((*str.borrow()).clone()) });
    ({ test_43(Default::default()) });
}
pub fn test_50() {
    ({ test_48() });
    ({ test_49() });
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
    ({ test_50() });
    return 0;
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<i8>) -> Ptr<i8>;
    fn as_ptr_3(&self, _a0: Ptr<i32>) -> Ptr<i32>;
    fn as_ptr_4(&self, _a0: Ptr<u8>) -> Ptr<u8>;
    fn as_ptr_5(&self, _a0: Ptr<u16>) -> Ptr<u16>;
    fn as_ptr_6(&self, _a0: Ptr<u32>) -> Ptr<u32>;
    fn length_7(&self, _a0: Ptr<i8>) -> usize;
    fn length_8(&self, _a0: Ptr<i32>) -> usize;
    fn length_9(&self, _a0: Ptr<u8>) -> usize;
    fn length_10(&self, _a0: Ptr<u16>) -> usize;
    fn length_11(&self, _a0: Ptr<u32>) -> usize;
    fn to_pconsti8(&self) -> Ptr<i8>;
    fn to_pconsti32(&self) -> Ptr<i32>;
    fn to_pconstu8(&self) -> Ptr<u8>;
    fn to_pconstu16(&self) -> Ptr<u16>;
    fn to_pconstu32(&self) -> Ptr<u32>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<i8>) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<i32>) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<u8>) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<u16>) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<u32>) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<i8>) -> usize {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<i32>) -> usize {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<u8>) -> usize {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<u16>) -> usize {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<u32>) -> usize {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconsti8(&self) -> Ptr<i8> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconsti32(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconstu8(&self) -> Ptr<u8> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconstu16(&self) -> Ptr<u16> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconstu32(&self) -> Ptr<u32> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub fn __cpp2rust_init_globals() {}
