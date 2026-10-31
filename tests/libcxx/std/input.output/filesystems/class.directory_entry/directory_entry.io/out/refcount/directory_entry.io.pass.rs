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
thread_local!(
    pub static InStr_0: Value<MultiStringType> = Rc::new(RefCell::new(MultiStringType::new(
        {
            Ptr::<i32>::from_string_literal(&[
                97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32, 103 as i32,
                47 as i32, 34 as i32, 104 as i32, 105 as i32, 106 as i32, 107 as i32, 108 as i32,
                109 as i32, 110 as i32, 111 as i32, 112 as i32, 34 as i32, 47 as i32, 113 as i32,
                114 as i32, 115 as i32, 116 as i32, 117 as i32, 118 as i32, 119 as i32, 120 as i32,
                121 as i32, 122 as i32, 47 as i32, 49 as i32, 50 as i32, 51 as i32, 52 as i32,
                53 as i32, 54 as i32, 55 as i32, 56 as i32, 57 as i32, 0 as i32,
            ])
        },
        {
            ((::std::mem::size_of::<[i32; 41]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)) as usize)
                .wrapping_sub(1_usize)
        },
        {
            Ptr::<u8>::from_string_literal(&[
                97 as u8, 98 as u8, 99 as u8, 100 as u8, 101 as u8, 102 as u8, 103 as u8, 47 as u8,
                34 as u8, 104 as u8, 105 as u8, 106 as u8, 107 as u8, 108 as u8, 109 as u8,
                110 as u8, 111 as u8, 112 as u8, 34 as u8, 47 as u8, 113 as u8, 114 as u8,
                115 as u8, 116 as u8, 117 as u8, 118 as u8, 119 as u8, 120 as u8, 121 as u8,
                122 as u8, 47 as u8, 49 as u8, 50 as u8, 51 as u8, 52 as u8, 53 as u8, 54 as u8,
                55 as u8, 56 as u8, 57 as u8, 0 as u8,
            ])
        },
        {
            ((::std::mem::size_of::<[u8; 41]>() as usize)
                .wrapping_div((::std::mem::size_of::<u8>() as usize)) as usize)
                .wrapping_sub(1_usize)
        },
        {
            Ptr::<u16>::from_string_literal(&[
                97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16, 103 as u16,
                47 as u16, 34 as u16, 104 as u16, 105 as u16, 106 as u16, 107 as u16, 108 as u16,
                109 as u16, 110 as u16, 111 as u16, 112 as u16, 34 as u16, 47 as u16, 113 as u16,
                114 as u16, 115 as u16, 116 as u16, 117 as u16, 118 as u16, 119 as u16, 120 as u16,
                121 as u16, 122 as u16, 47 as u16, 49 as u16, 50 as u16, 51 as u16, 52 as u16,
                53 as u16, 54 as u16, 55 as u16, 56 as u16, 57 as u16, 0 as u16,
            ])
        },
        {
            ((::std::mem::size_of::<[u16; 41]>() as usize)
                .wrapping_div((::std::mem::size_of::<u16>() as usize)) as usize)
                .wrapping_sub(1_usize)
        },
        {
            Ptr::<u32>::from_string_literal(&[
                97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32, 103 as u32,
                47 as u32, 34 as u32, 104 as u32, 105 as u32, 106 as u32, 107 as u32, 108 as u32,
                109 as u32, 110 as u32, 111 as u32, 112 as u32, 34 as u32, 47 as u32, 113 as u32,
                114 as u32, 115 as u32, 116 as u32, 117 as u32, 118 as u32, 119 as u32, 120 as u32,
                121 as u32, 122 as u32, 47 as u32, 49 as u32, 50 as u32, 51 as u32, 52 as u32,
                53 as u32, 54 as u32, 55 as u32, 56 as u32, 57 as u32, 0 as u32,
            ])
        },
        {
            ((::std::mem::size_of::<[u32; 41]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)) as usize)
                .wrapping_sub(1_usize)
        },
        { Ptr::<i8>::from_string_literal(b"abcdefg/\"hijklmnop\"/qrstuvwxyz/123456789") },
        { (::std::mem::size_of::<[i8; 41]>() as usize).wrapping_sub(1_usize) },
    )));
);
thread_local!(
    pub static OutStr_1: Value<MultiStringType> = Rc::new(RefCell::new(MultiStringType::new(
        {
            Ptr::<i32>::from_string_literal(&[
                34 as i32, 97 as i32, 98 as i32, 99 as i32, 100 as i32, 101 as i32, 102 as i32,
                103 as i32, 47 as i32, 92 as i32, 34 as i32, 104 as i32, 105 as i32, 106 as i32,
                107 as i32, 108 as i32, 109 as i32, 110 as i32, 111 as i32, 112 as i32, 92 as i32,
                34 as i32, 47 as i32, 113 as i32, 114 as i32, 115 as i32, 116 as i32, 117 as i32,
                118 as i32, 119 as i32, 120 as i32, 121 as i32, 122 as i32, 47 as i32, 49 as i32,
                50 as i32, 51 as i32, 52 as i32, 53 as i32, 54 as i32, 55 as i32, 56 as i32,
                57 as i32, 34 as i32, 0 as i32,
            ])
        },
        {
            ((::std::mem::size_of::<[i32; 45]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize)) as usize)
                .wrapping_sub(1_usize)
        },
        {
            Ptr::<u8>::from_string_literal(&[
                34 as u8, 97 as u8, 98 as u8, 99 as u8, 100 as u8, 101 as u8, 102 as u8, 103 as u8,
                47 as u8, 92 as u8, 34 as u8, 104 as u8, 105 as u8, 106 as u8, 107 as u8,
                108 as u8, 109 as u8, 110 as u8, 111 as u8, 112 as u8, 92 as u8, 34 as u8,
                47 as u8, 113 as u8, 114 as u8, 115 as u8, 116 as u8, 117 as u8, 118 as u8,
                119 as u8, 120 as u8, 121 as u8, 122 as u8, 47 as u8, 49 as u8, 50 as u8, 51 as u8,
                52 as u8, 53 as u8, 54 as u8, 55 as u8, 56 as u8, 57 as u8, 34 as u8, 0 as u8,
            ])
        },
        {
            ((::std::mem::size_of::<[u8; 45]>() as usize)
                .wrapping_div((::std::mem::size_of::<u8>() as usize)) as usize)
                .wrapping_sub(1_usize)
        },
        {
            Ptr::<u16>::from_string_literal(&[
                34 as u16, 97 as u16, 98 as u16, 99 as u16, 100 as u16, 101 as u16, 102 as u16,
                103 as u16, 47 as u16, 92 as u16, 34 as u16, 104 as u16, 105 as u16, 106 as u16,
                107 as u16, 108 as u16, 109 as u16, 110 as u16, 111 as u16, 112 as u16, 92 as u16,
                34 as u16, 47 as u16, 113 as u16, 114 as u16, 115 as u16, 116 as u16, 117 as u16,
                118 as u16, 119 as u16, 120 as u16, 121 as u16, 122 as u16, 47 as u16, 49 as u16,
                50 as u16, 51 as u16, 52 as u16, 53 as u16, 54 as u16, 55 as u16, 56 as u16,
                57 as u16, 34 as u16, 0 as u16,
            ])
        },
        {
            ((::std::mem::size_of::<[u16; 45]>() as usize)
                .wrapping_div((::std::mem::size_of::<u16>() as usize)) as usize)
                .wrapping_sub(1_usize)
        },
        {
            Ptr::<u32>::from_string_literal(&[
                34 as u32, 97 as u32, 98 as u32, 99 as u32, 100 as u32, 101 as u32, 102 as u32,
                103 as u32, 47 as u32, 92 as u32, 34 as u32, 104 as u32, 105 as u32, 106 as u32,
                107 as u32, 108 as u32, 109 as u32, 110 as u32, 111 as u32, 112 as u32, 92 as u32,
                34 as u32, 47 as u32, 113 as u32, 114 as u32, 115 as u32, 116 as u32, 117 as u32,
                118 as u32, 119 as u32, 120 as u32, 121 as u32, 122 as u32, 47 as u32, 49 as u32,
                50 as u32, 51 as u32, 52 as u32, 53 as u32, 54 as u32, 55 as u32, 56 as u32,
                57 as u32, 34 as u32, 0 as u32,
            ])
        },
        {
            ((::std::mem::size_of::<[u32; 45]>() as usize)
                .wrapping_div((::std::mem::size_of::<u32>() as usize)) as usize)
                .wrapping_sub(1_usize)
        },
        { Ptr::<i8>::from_string_literal(b"\"abcdefg/\\\"hijklmnop\\\"/qrstuvwxyz/123456789\"") },
        { (::std::mem::size_of::<[i8; 45]>() as usize).wrapping_sub(1_usize) },
    )));
);
pub fn TestOutput_2() {
    let mut input: Ptr<i8> =
        ({ MultiStringTypeImpl::to_pconsti8(&InStr_0.with(|v| v.as_pointer())) })
            .reinterpret_cast::<i8>();
    let mut expected_output: Ptr<i8> =
        ({ MultiStringTypeImpl::to_pconsti8(&OutStr_1.with(|v| v.as_pointer())) })
            .reinterpret_cast::<i8>();
    let dir: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let stream: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Ptr<::std::fs::File> = unimplemented!();
    assert!(
        expected_output
            .with_c_str(|__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s)
    );
    assert!(({ (result).clone() } == { (stream.as_pointer()) }));
}
pub fn TestOutput_3() {
    let mut input: Ptr<i8> =
        ({ MultiStringTypeImpl::to_pconsti8(&InStr_0.with(|v| v.as_pointer())) })
            .reinterpret_cast::<i8>();
    let mut expected_output: Ptr<i32> =
        ({ MultiStringTypeImpl::to_pconsti32(&OutStr_1.with(|v| v.as_pointer())) })
            .reinterpret_cast::<i32>();
    let dir: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let stream: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let result: Ptr<::std::fs::File> = unimplemented!();
    assert!(
        expected_output
            .with_c_str(|__s| unimplemented!()[..unimplemented!().len().saturating_sub(1)] == *__s)
    );
    assert!(({ (result).clone() } == { (stream.as_pointer()) }));
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
    ({ TestOutput_2() });
    ({ TestOutput_3() });
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
pub fn __cpp2rust_init_globals() {
    let _ = InStr_0.with(|_| ());
    let _ = OutStr_1.with(|_| ());
}
