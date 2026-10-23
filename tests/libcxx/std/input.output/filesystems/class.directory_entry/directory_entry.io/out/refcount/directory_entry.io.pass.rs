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
thread_local!(
    pub static InStr_0: Value<MultiStringType> = Rc::new(RefCell::new(MultiStringType::new(
        {
            Ptr::<()>::from_string_literal(&[
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (), 47 as (),
                34 as (), 104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (),
                110 as (), 111 as (), 112 as (), 34 as (), 47 as (), 113 as (), 114 as (),
                115 as (), 116 as (), 117 as (), 118 as (), 119 as (), 120 as (), 121 as (),
                122 as (), 47 as (), 49 as (), 50 as (), 51 as (), 52 as (), 53 as (), 54 as (),
                55 as (), 56 as (), 57 as (), 0 as (),
            ])
        },
        {
            ((::std::mem::size_of::<[(); 41]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())
        },
        {
            Ptr::<()>::from_string_literal(&[
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (), 47 as (),
                34 as (), 104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (),
                110 as (), 111 as (), 112 as (), 34 as (), 47 as (), 113 as (), 114 as (),
                115 as (), 116 as (), 117 as (), 118 as (), 119 as (), 120 as (), 121 as (),
                122 as (), 47 as (), 49 as (), 50 as (), 51 as (), 52 as (), 53 as (), 54 as (),
                55 as (), 56 as (), 57 as (), 0 as (),
            ])
        },
        {
            ((::std::mem::size_of::<[(); 41]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())
        },
        {
            Ptr::<()>::from_string_literal(&[
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (), 47 as (),
                34 as (), 104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (),
                110 as (), 111 as (), 112 as (), 34 as (), 47 as (), 113 as (), 114 as (),
                115 as (), 116 as (), 117 as (), 118 as (), 119 as (), 120 as (), 121 as (),
                122 as (), 47 as (), 49 as (), 50 as (), 51 as (), 52 as (), 53 as (), 54 as (),
                55 as (), 56 as (), 57 as (), 0 as (),
            ])
        },
        {
            ((::std::mem::size_of::<[(); 41]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())
        },
        {
            Ptr::<()>::from_string_literal(&[
                97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (), 47 as (),
                34 as (), 104 as (), 105 as (), 106 as (), 107 as (), 108 as (), 109 as (),
                110 as (), 111 as (), 112 as (), 34 as (), 47 as (), 113 as (), 114 as (),
                115 as (), 116 as (), 117 as (), 118 as (), 119 as (), 120 as (), 121 as (),
                122 as (), 47 as (), 49 as (), 50 as (), 51 as (), 52 as (), 53 as (), 54 as (),
                55 as (), 56 as (), 57 as (), 0 as (),
            ])
        },
        {
            ((::std::mem::size_of::<[(); 41]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())
        },
        { Ptr::<()>::from_string_literal(b"abcdefg/\"hijklmnop\"/qrstuvwxyz/123456789") },
        { (::std::mem::size_of::<[(); 41]>() as ()).wrapping_sub(1_()) },
    )));
);
thread_local!(
    pub static OutStr_1: Value<MultiStringType> = Rc::new(RefCell::new(MultiStringType::new(
        {
            Ptr::<()>::from_string_literal(&[
                34 as (), 97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                47 as (), 92 as (), 34 as (), 104 as (), 105 as (), 106 as (), 107 as (),
                108 as (), 109 as (), 110 as (), 111 as (), 112 as (), 92 as (), 34 as (),
                47 as (), 113 as (), 114 as (), 115 as (), 116 as (), 117 as (), 118 as (),
                119 as (), 120 as (), 121 as (), 122 as (), 47 as (), 49 as (), 50 as (), 51 as (),
                52 as (), 53 as (), 54 as (), 55 as (), 56 as (), 57 as (), 34 as (), 0 as (),
            ])
        },
        {
            ((::std::mem::size_of::<[(); 45]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())
        },
        {
            Ptr::<()>::from_string_literal(&[
                34 as (), 97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                47 as (), 92 as (), 34 as (), 104 as (), 105 as (), 106 as (), 107 as (),
                108 as (), 109 as (), 110 as (), 111 as (), 112 as (), 92 as (), 34 as (),
                47 as (), 113 as (), 114 as (), 115 as (), 116 as (), 117 as (), 118 as (),
                119 as (), 120 as (), 121 as (), 122 as (), 47 as (), 49 as (), 50 as (), 51 as (),
                52 as (), 53 as (), 54 as (), 55 as (), 56 as (), 57 as (), 34 as (), 0 as (),
            ])
        },
        {
            ((::std::mem::size_of::<[(); 45]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())
        },
        {
            Ptr::<()>::from_string_literal(&[
                34 as (), 97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                47 as (), 92 as (), 34 as (), 104 as (), 105 as (), 106 as (), 107 as (),
                108 as (), 109 as (), 110 as (), 111 as (), 112 as (), 92 as (), 34 as (),
                47 as (), 113 as (), 114 as (), 115 as (), 116 as (), 117 as (), 118 as (),
                119 as (), 120 as (), 121 as (), 122 as (), 47 as (), 49 as (), 50 as (), 51 as (),
                52 as (), 53 as (), 54 as (), 55 as (), 56 as (), 57 as (), 34 as (), 0 as (),
            ])
        },
        {
            ((::std::mem::size_of::<[(); 45]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())
        },
        {
            Ptr::<()>::from_string_literal(&[
                34 as (), 97 as (), 98 as (), 99 as (), 100 as (), 101 as (), 102 as (), 103 as (),
                47 as (), 92 as (), 34 as (), 104 as (), 105 as (), 106 as (), 107 as (),
                108 as (), 109 as (), 110 as (), 111 as (), 112 as (), 92 as (), 34 as (),
                47 as (), 113 as (), 114 as (), 115 as (), 116 as (), 117 as (), 118 as (),
                119 as (), 120 as (), 121 as (), 122 as (), 47 as (), 49 as (), 50 as (), 51 as (),
                52 as (), 53 as (), 54 as (), 55 as (), 56 as (), 57 as (), 34 as (), 0 as (),
            ])
        },
        {
            ((::std::mem::size_of::<[(); 45]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
                .wrapping_sub(1_())
        },
        { Ptr::<()>::from_string_literal(b"\"abcdefg/\\\"hijklmnop\\\"/qrstuvwxyz/123456789\"") },
        { (::std::mem::size_of::<[(); 45]>() as ()).wrapping_sub(1_()) },
    )));
);
pub fn TestOutput_2() {
    let input: Value<Ptr<()>> = Rc::new(RefCell::new(
        ({ MultiStringTypeImpl::to_pconst_12(&InStr_0.with(|v| v.as_pointer())) })
            .reinterpret_cast::<()>(),
    ));
    let mut expected_output: Ptr<()> =
        ({ MultiStringTypeImpl::to_pconst_12(&OutStr_1.with(|v| v.as_pointer())) })
            .reinterpret_cast::<()>();
    let dir: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let stream: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Ptr<()> = write!(stream.as_pointer(), "{:}", dir.as_pointer(),);
    unimplemented!();
    unimplemented!();
}
pub fn TestOutput_3() {
    let input: Value<Ptr<()>> = Rc::new(RefCell::new(
        ({ MultiStringTypeImpl::to_pconst_12(&InStr_0.with(|v| v.as_pointer())) })
            .reinterpret_cast::<()>(),
    ));
    let mut expected_output: Ptr<()> =
        ({ MultiStringTypeImpl::to_pconst_13(&OutStr_1.with(|v| v.as_pointer())) })
            .reinterpret_cast::<()>();
    let dir: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let stream: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Ptr<()> = write!(stream.as_pointer(), "{:}", dir.as_pointer(),);
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
    ({ TestOutput_2() });
    ({ TestOutput_3() });
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
pub fn __cpp2rust_init_globals() {
    let _ = InStr_0.with(|_| ());
    let _ = OutStr_1.with(|_| ());
}
