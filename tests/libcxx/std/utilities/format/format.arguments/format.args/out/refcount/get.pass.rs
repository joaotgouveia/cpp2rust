extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub fn test_0(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_1(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_2(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_3(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_4(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_5(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_6(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_7(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_8(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_9(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_10(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_11(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_12(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_13(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_14(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_15(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_16(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_17(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_18(mut value: Ptr<()>) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<Ptr<()>> = Rc::new(RefCell::new((value).clone()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
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
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_20(mut value: ()) {
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
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
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
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_22(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_23(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_24(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_25(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_26(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_27(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_28(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_29(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_30(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_31(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_32(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_33(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_34(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_35(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_36(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_37(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_38(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_39(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(value));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_40(mut value: Ptr<()>) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<Ptr<()>> = Rc::new(RefCell::new((value).clone()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
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
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_42(mut value: ()) {
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
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
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
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_handle_44(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        }
    )));
    unimplemented!();
}
pub fn test_handle_45(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        }
    )));
    unimplemented!();
}
pub fn test_handle_46(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        }
    )));
    unimplemented!();
}
pub fn test_handle_47(mut value: ()) {
    let store: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: Ptr<()>| {
            let a: Value<Ptr<()>> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        }
    )));
    unimplemented!();
}
pub fn test_48() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    ({ test_0(true) });
    ({ test_0(false) });
    ({ test_1(('a' as ())) });
    ({ test_1(('z' as ())) });
    ({ test_1(('0' as ())) });
    ({ test_1(('9' as ())) });
    {
        ({ test_1(('a' as ())) });
        ({ test_1(('z' as ())) });
        ({ test_1(('0' as ())) });
        ({ test_1(('9' as ())) });
    }
    ({ test_5(unimplemented!()) });
    ({ test_5(0_()) });
    ({ test_5(unimplemented!()) });
    ({ test_6(unimplemented!()) });
    ({ test_6(unimplemented!()) });
    ({ test_6(0_()) });
    ({ test_6(unimplemented!()) });
    ({ test_6(unimplemented!()) });
    ({ test_7(unimplemented!()) });
    ({ test_7(unimplemented!()) });
    ({ test_7(unimplemented!()) });
    ({ test_7(0_()) });
    ({ test_7(unimplemented!()) });
    ({ test_7(unimplemented!()) });
    ({ test_7(unimplemented!()) });
    ({ test_8(unimplemented!()) });
    ({ test_8(unimplemented!()) });
    ({ test_8(unimplemented!()) });
    ({ test_8(unimplemented!()) });
    ({ test_8(0_()) });
    ({ test_8(unimplemented!()) });
    ({ test_8(unimplemented!()) });
    ({ test_8(unimplemented!()) });
    ({ test_8(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(0_()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_handle_44(0_()) });
    ({ test_10(0_()) });
    ({ test_10(unimplemented!()) });
    ({ test_11(0_()) });
    ({ test_11(unimplemented!()) });
    ({ test_11(unimplemented!()) });
    ({ test_12(0_()) });
    ({ test_12(unimplemented!()) });
    ({ test_12(unimplemented!()) });
    ({ test_12(unimplemented!()) });
    ({ test_13(0_()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_14(0_()) });
    ({ test_14(unimplemented!()) });
    ({ test_14(unimplemented!()) });
    ({ test_14(unimplemented!()) });
    ({ test_14(unimplemented!()) });
    ({ test_14(unimplemented!()) });
    ({ test_handle_45(0_()) });
    ({ test_15(-unimplemented!()) });
    ({ test_15(-unimplemented!()) });
    ({ test_15(-0_f64) });
    ({ test_15(0_f64) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_16(-unimplemented!()) });
    ({ test_16(-unimplemented!()) });
    ({ test_16(-0_f64) });
    ({ test_16(0_f64) });
    ({ test_16(unimplemented!()) });
    ({ test_16(unimplemented!()) });
    ({ test_17(-unimplemented!()) });
    ({ test_17(-unimplemented!()) });
    ({ test_17(-0_f64) });
    ({ test_17(0_f64) });
    ({ test_17(unimplemented!()) });
    ({ test_17(unimplemented!()) });
    ({ test_18(unimplemented!()) });
    ({ test_18(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_20(unimplemented!()) });
    ({ test_20(unimplemented!()) });
    ({ test_20(unimplemented!()) });
    ({ test_21(Default::default()) });
}
pub fn test_49() {
    let mut empty: () = unimplemented!();
    let mut str: () = unimplemented!();
    ({ test_22(true) });
    ({ test_22(false) });
    ({ test_23(('a' as ())) });
    ({ test_23(('z' as ())) });
    ({ test_23(('0' as ())) });
    ({ test_23(('9' as ())) });
    {
        {
            ({ test_24(('a' as ())) });
            ({ test_24(('z' as ())) });
            ({ test_24(('0' as ())) });
            ({ test_24(('9' as ())) });
        }
    }
    ({ test_27(unimplemented!()) });
    ({ test_27(0_()) });
    ({ test_27(unimplemented!()) });
    ({ test_28(unimplemented!()) });
    ({ test_28(unimplemented!()) });
    ({ test_28(0_()) });
    ({ test_28(unimplemented!()) });
    ({ test_28(unimplemented!()) });
    ({ test_29(unimplemented!()) });
    ({ test_29(unimplemented!()) });
    ({ test_29(unimplemented!()) });
    ({ test_29(0_()) });
    ({ test_29(unimplemented!()) });
    ({ test_29(unimplemented!()) });
    ({ test_29(unimplemented!()) });
    ({ test_30(unimplemented!()) });
    ({ test_30(unimplemented!()) });
    ({ test_30(unimplemented!()) });
    ({ test_30(unimplemented!()) });
    ({ test_30(0_()) });
    ({ test_30(unimplemented!()) });
    ({ test_30(unimplemented!()) });
    ({ test_30(unimplemented!()) });
    ({ test_30(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(0_()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_handle_46(0_()) });
    ({ test_32(0_()) });
    ({ test_32(unimplemented!()) });
    ({ test_33(0_()) });
    ({ test_33(unimplemented!()) });
    ({ test_33(unimplemented!()) });
    ({ test_34(0_()) });
    ({ test_34(unimplemented!()) });
    ({ test_34(unimplemented!()) });
    ({ test_34(unimplemented!()) });
    ({ test_35(0_()) });
    ({ test_35(unimplemented!()) });
    ({ test_35(unimplemented!()) });
    ({ test_35(unimplemented!()) });
    ({ test_35(unimplemented!()) });
    ({ test_36(0_()) });
    ({ test_36(unimplemented!()) });
    ({ test_36(unimplemented!()) });
    ({ test_36(unimplemented!()) });
    ({ test_36(unimplemented!()) });
    ({ test_36(unimplemented!()) });
    ({ test_handle_47(0_()) });
    ({ test_37(-unimplemented!()) });
    ({ test_37(-unimplemented!()) });
    ({ test_37(-0_f64) });
    ({ test_37(0_f64) });
    ({ test_37(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_38(-unimplemented!()) });
    ({ test_38(-unimplemented!()) });
    ({ test_38(-0_f64) });
    ({ test_38(0_f64) });
    ({ test_38(unimplemented!()) });
    ({ test_38(unimplemented!()) });
    ({ test_39(-unimplemented!()) });
    ({ test_39(-unimplemented!()) });
    ({ test_39(-0_f64) });
    ({ test_39(0_f64) });
    ({ test_39(unimplemented!()) });
    ({ test_39(unimplemented!()) });
    ({ test_40(unimplemented!()) });
    ({ test_40(unimplemented!()) });
    ({ test_41(unimplemented!()) });
    ({ test_41(unimplemented!()) });
    ({ test_41(unimplemented!()) });
    ({ test_42(unimplemented!()) });
    ({ test_42(unimplemented!()) });
    ({ test_42(unimplemented!()) });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_50() });
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
