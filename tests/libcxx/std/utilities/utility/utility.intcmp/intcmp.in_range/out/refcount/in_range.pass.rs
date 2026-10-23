extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(3)]
pub struct Tuple_unsigned_char_ {
    #[offset(0)]
    pub min: (),
    #[offset(1)]
    pub max: (),
    #[offset(2)]
    pub mid: (),
}
impl Tuple_unsigned_char_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_char_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_char_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_char_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_char_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_char_ {
    fn default() -> Self {
        { Tuple_unsigned_char_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(3)]
pub struct Tuple_signed_char_ {
    #[offset(0)]
    pub min: (),
    #[offset(1)]
    pub max: (),
    #[offset(2)]
    pub mid: (),
}
impl Tuple_signed_char_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_signed_char_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_signed_char_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_signed_char_::min() }));
        field!(this, max).write(({ std_numeric_limits_signed_char_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_signed_char_ {
    fn default() -> Self {
        { Tuple_signed_char_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(6)]
pub struct Tuple_short_ {
    #[offset(0)]
    pub min: (),
    #[offset(2)]
    pub max: (),
    #[offset(4)]
    pub mid: (),
}
impl Tuple_short_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_short_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_short_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_short_::min() }));
        field!(this, max).write(({ std_numeric_limits_short_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_short_ {
    fn default() -> Self {
        { Tuple_short_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(6)]
pub struct Tuple_unsigned_short_ {
    #[offset(0)]
    pub min: (),
    #[offset(2)]
    pub max: (),
    #[offset(4)]
    pub mid: (),
}
impl Tuple_unsigned_short_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_short_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_short_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_short_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_short_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_short_ {
    fn default() -> Self {
        { Tuple_unsigned_short_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Tuple_int_ {
    #[offset(0)]
    pub min: (),
    #[offset(4)]
    pub max: (),
    #[offset(8)]
    pub mid: (),
}
impl Tuple_int_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_int_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_int_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_int_::min() }));
        field!(this, max).write(({ std_numeric_limits_int_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_int_ {
    fn default() -> Self {
        { Tuple_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Tuple_unsigned_int_ {
    #[offset(0)]
    pub min: (),
    #[offset(4)]
    pub max: (),
    #[offset(8)]
    pub mid: (),
}
impl Tuple_unsigned_int_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_int_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_int_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_int_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_int_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_int_ {
    fn default() -> Self {
        { Tuple_unsigned_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Tuple_long_ {
    #[offset(0)]
    pub min: (),
    #[offset(8)]
    pub max: (),
    #[offset(16)]
    pub mid: (),
}
impl Tuple_long_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_long_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_long_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_long_::min() }));
        field!(this, max).write(({ std_numeric_limits_long_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_long_ {
    fn default() -> Self {
        { Tuple_long_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Tuple_unsigned_long_ {
    #[offset(0)]
    pub min: (),
    #[offset(8)]
    pub max: (),
    #[offset(16)]
    pub mid: (),
}
impl Tuple_unsigned_long_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_long_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_long_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_long_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_long_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_long_ {
    fn default() -> Self {
        { Tuple_unsigned_long_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Tuple_long_long_ {
    #[offset(0)]
    pub min: (),
    #[offset(8)]
    pub max: (),
    #[offset(16)]
    pub mid: (),
}
impl Tuple_long_long_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_long_long_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_long_long_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_long_long_::min() }));
        field!(this, max).write(({ std_numeric_limits_long_long_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_long_long_ {
    fn default() -> Self {
        { Tuple_long_long_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Tuple_unsigned_long_long_ {
    #[offset(0)]
    pub min: (),
    #[offset(8)]
    pub max: (),
    #[offset(16)]
    pub mid: (),
}
impl Tuple_unsigned_long_long_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_long_long_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_long_long_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_long_long_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_long_long_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_long_long_ {
    fn default() -> Self {
        { Tuple_unsigned_long_long_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Tuple_unsigned___int128_ {
    #[offset(0)]
    pub min: (),
    #[offset(16)]
    pub max: (),
    #[offset(32)]
    pub mid: (),
}
impl Tuple_unsigned___int128_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned___int128_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned___int128_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned___int128_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned___int128_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned___int128_ {
    fn default() -> Self {
        { Tuple_unsigned___int128_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Tuple___int128_ {
    #[offset(0)]
    pub min: (),
    #[offset(16)]
    pub max: (),
    #[offset(32)]
    pub mid: (),
}
impl Tuple___int128_ {
    pub fn new() -> Self {
        let __this: Value<Tuple___int128_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple___int128_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits___int128_::min() }));
        field!(this, max).write(({ std_numeric_limits___int128_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple___int128_ {
    fn default() -> Self {
        { Tuple___int128_::new() }
    }
}
pub fn test_in_range1_0() {
    let mut tup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_1() {
    let mut tup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_2() {
    let mut tup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_3() {
    let mut tup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_4() {
    let mut tup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_5() {
    let mut tup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_6() {
    let mut tup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_7() {
    let mut tup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_8() {
    let mut tup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_9() {
    let mut tup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_10() {
    let mut tup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range1_11() {
    let mut tup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_in_range_12() {
    let mut utup8: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let mut stup8: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test1_13(_a0: Ptr<()>) {
    {
        ({ test_in_range1_11() });
        {
            ({ test_in_range1_10() });
            {
                ({ test_in_range1_9() });
                {
                    ({ test_in_range1_8() });
                    {
                        ({ test_in_range1_7() });
                        {
                            ({ test_in_range1_6() });
                            {
                                ({ test_in_range1_5() });
                                {
                                    ({ test_in_range1_4() });
                                    {
                                        ({ test_in_range1_3() });
                                        {
                                            ({ test_in_range1_2() });
                                            {
                                                ({ test_in_range1_1() });
                                                ({ test_in_range1_0() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub fn test_14() -> () {
    let types: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_13(types.as_pointer()) });
    ({ test_in_range_12() });
    return true;
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
    const _: () = assert!(true, "");;
    ({ test_14() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
