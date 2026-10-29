extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(144)]
pub struct Pointers {
    #[offset(0)]
    #[byte_size(8)]
    pub x1: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    pub x2: Ptr<()>,
    #[offset(16)]
    #[byte_size(40)]
    pub x3: Box<[Ptr<()>]>,
    #[offset(56)]
    #[byte_size(80)]
    pub x4: Box<[Ptr<()>]>,
    #[offset(136)]
    pub x5: (),
}
impl Default for Pointers {
    fn default() -> Self {
        {
            Pointers {
                x1: Ptr::<()>::null(),
                x2: Ptr::<()>::null(),
                x3: (0..5)
                    .map(|_| Ptr::<()>::null())
                    .collect::<Box<[Ptr<()>]>>(),
                x4: (0..10)
                    .map(|_| Ptr::<()>::null())
                    .collect::<Box<[Ptr<()>]>>(),
                x5: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(440)]
pub struct SmallArrays {
    #[offset(0)]
    #[byte_size(128)]
    pub a: Box<[()]>,
    #[offset(128)]
    #[byte_size(8)]
    pub f: FnPtr<fn(()) -> ()>,
    #[offset(136)]
    #[byte_size(16)]
    pub fs: Box<[FnPtr<fn(()) -> ()>]>,
    #[offset(152)]
    #[byte_size(288)]
    pub p: Box<[Pointers]>,
}
impl Default for SmallArrays {
    fn default() -> Self {
        {
            SmallArrays {
                a: (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                f: FnPtr::<fn(()) -> ()>::null(),
                fs: (0..2)
                    .map(|_| FnPtr::<fn(()) -> ()>::null())
                    .collect::<Box<[FnPtr<fn(()) -> ()>]>>(),
                p: (0..2)
                    .map(|_| <Pointers>::default())
                    .collect::<Box<[Pointers]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(132)]
pub struct BigArray {
    #[offset(0)]
    #[byte_size(132)]
    pub a: Box<[()]>,
}
impl Default for BigArray {
    fn default() -> Self {
        {
            BigArray {
                a: (0..33).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut default_pointers: Ptr<Pointers> = Ptr::alloc_array(
        (0..10_())
            .map(|_| <Pointers>::default())
            .collect::<Box<[Pointers]>>(),
    );
    default_pointers.delete();
    let mut small: Ptr<SmallArrays> = Ptr::alloc_array(
        (0..2_())
            .map(|_| <SmallArrays>::default())
            .collect::<Box<[SmallArrays]>>(),
    );
    small.delete();
    let mut big: Ptr<BigArray> = Ptr::alloc_array(
        (0..2_())
            .map(|_| <BigArray>::default())
            .collect::<Box<[BigArray]>>(),
    );
    big.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
