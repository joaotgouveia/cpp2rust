extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct rand1_unsigned_int__0__10_ {
    #[offset(0)]
    x_: u32,
}
impl rand1_unsigned_int__0__10_ {
    pub fn min() -> u32 {
        return 0_u32;
    }
    pub fn max() -> u32 {
        return 10_u32;
    }
    pub fn new(sd: Option<u32>) -> Self {
        let mut sd: u32 = sd.unwrap_or_else(|| 0_u32);
        let __this: Value<rand1_unsigned_int__0__10_> = Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_int__0__10_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 10_u32) {
            field!(this, x_).write(10_u32);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable0<u32> for rand1_unsigned_int__0__10_ {
    fn call(&self) -> u32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            rand1_unsigned_int__0__10_Impl::operator_call(__it)
        })
    }
}
impl CppMove for rand1_unsigned_int__0__10_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for rand1_unsigned_int__0__10_ {
    fn default() -> Self {
        { rand1_unsigned_int__0__10_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct rand1_unsigned_int__0__100_ {
    #[offset(0)]
    x_: u32,
}
impl rand1_unsigned_int__0__100_ {
    pub fn min() -> u32 {
        return 0_u32;
    }
    pub fn max() -> u32 {
        return 100_u32;
    }
    pub fn new(sd: Option<u32>) -> Self {
        let mut sd: u32 = sd.unwrap_or_else(|| 0_u32);
        let __this: Value<rand1_unsigned_int__0__100_> = Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_int__0__100_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 100_u32) {
            field!(this, x_).write(100_u32);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable0<u32> for rand1_unsigned_int__0__100_ {
    fn call(&self) -> u32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            rand1_unsigned_int__0__100_Impl::operator_call(__it)
        })
    }
}
impl CppMove for rand1_unsigned_int__0__100_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for rand1_unsigned_int__0__100_ {
    fn default() -> Self {
        { rand1_unsigned_int__0__100_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct rand1_unsigned_int__0__4294967295_ {
    #[offset(0)]
    x_: u32,
}
impl rand1_unsigned_int__0__4294967295_ {
    pub fn min() -> u32 {
        return 0_u32;
    }
    pub fn max() -> u32 {
        return 4294967295_u32;
    }
    pub fn new(mut sd: u32) -> Self {
        let __this: Value<rand1_unsigned_int__0__4294967295_> =
            Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_int__0__4294967295_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 4294967295_u32) {
            field!(this, x_).write(4294967295_u32);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable0<u32> for rand1_unsigned_int__0__4294967295_ {
    fn call(&self) -> u32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            rand1_unsigned_int__0__4294967295_Impl::operator_call(__it)
        })
    }
}
impl CppMove for rand1_unsigned_int__0__4294967295_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct rand1_unsigned_int__2__3_ {
    #[offset(0)]
    x_: u32,
}
impl rand1_unsigned_int__2__3_ {
    pub fn min() -> u32 {
        return 2_u32;
    }
    pub fn max() -> u32 {
        return 3_u32;
    }
    pub fn new(mut sd: u32) -> Self {
        let __this: Value<rand1_unsigned_int__2__3_> = Rc::new(RefCell::new(Self { x_: sd }));
        let this: Ptr<rand1_unsigned_int__2__3_> = __this.as_pointer();
        if (this.with(|__s| __s.x_) > 3_u32) {
            field!(this, x_).write(3_u32);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable0<u32> for rand1_unsigned_int__2__3_ {
    fn call(&self) -> u32 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            rand1_unsigned_int__2__3_Impl::operator_call(__it)
        })
    }
}
impl CppMove for rand1_unsigned_int__2__3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test1_0() {
    let e: Value<std_independent_bits_engine_rand1_unsigned_int__0__10___16__unsigned_int_> =
        Rc::new(RefCell::new(
            <std_independent_bits_engine_rand1_unsigned_int__0__10___16__unsigned_int_>::default(),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 6958_u32) }));
}
pub fn test2_1() {
    let e: Value<std_independent_bits_engine_rand1_unsigned_int__0__100___16__unsigned_int_> =
        Rc::new(RefCell::new(
            <std_independent_bits_engine_rand1_unsigned_int__0__100___16__unsigned_int_>::default(),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 66_u32) }));
}
pub fn test3_2() {
    let e: Value<
        std_independent_bits_engine_rand1_unsigned_int__0__4294967295___32__unsigned_int_,
    > = Rc::new(RefCell::new(
        std_independent_bits_engine_rand1_unsigned_int__0__4294967295___32__unsigned_int_::new_1({
            5_u32
        }),
    ));
    assert!(({ (({ (*e.borrow_mut())() }) == 5_u32) }));
}
pub fn test4_3() {
    let e: Value<std_independent_bits_engine_rand1_unsigned_int__0__4294967295___7__unsigned_int_> =
        Rc::new(RefCell::new(
            std_independent_bits_engine_rand1_unsigned_int__0__4294967295___7__unsigned_int_::new_1(
                { 129_u32 },
            ),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 1_u32) }));
}
pub fn test5_4() {
    let e: Value<std_independent_bits_engine_rand1_unsigned_int__2__3___1__unsigned_int_> =
        Rc::new(RefCell::new(
            std_independent_bits_engine_rand1_unsigned_int__2__3___1__unsigned_int_::new_1({
                6_u32
            }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 1_u32) }));
}
pub fn test6_5() {
    let e: Value<std_independent_bits_engine_rand1_unsigned_int__2__3___11__unsigned_int_> =
        Rc::new(RefCell::new(
            std_independent_bits_engine_rand1_unsigned_int__2__3___11__unsigned_int_::new_1({
                6_u32
            }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 1365_u32) }));
}
pub fn test7_6() {
    let e: Value<std_independent_bits_engine_rand1_unsigned_int__2__3___32__unsigned_int_> =
        Rc::new(RefCell::new(
            std_independent_bits_engine_rand1_unsigned_int__2__3___32__unsigned_int_::new_1({
                6_u32
            }),
        ));
    assert!(({ (({ (*e.borrow_mut())() }) == 2863311530_u32) }));
}
pub fn test8_7() {
    let e : Value<std_independent_bits_engine_std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253___64__unsigned_long_long_ > = Rc::new(RefCell::new(std_independent_bits_engine_std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253___64__unsigned_long_long_ :: new_1 ( { 6_u64  } , )  )) ;
    assert!(({ (({ (*e.borrow_mut())() }) == 16470362623952407241_u64) }));
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
    ({ test1_0() });
    ({ test2_1() });
    ({ test3_2() });
    ({ test4_3() });
    ({ test5_4() });
    ({ test6_5() });
    ({ test7_6() });
    ({ test8_7() });
    return 0;
}
pub trait rand1_unsigned_int__0__100_Impl {
    fn operator_call(&self) -> u32;
}
impl rand1_unsigned_int__0__100_Impl for Ptr<rand1_unsigned_int__0__100_> {
    fn operator_call(&self) -> u32 {
        let r: Value<u32> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 100_u32) {
            field!((*self), x_).with_mut(|__v| {
                __v.prefix_inc();
            });
        } else {
            field!((*self), x_).write(0_u32);
        }
        return (*r.borrow());
    }
}
pub trait rand1_unsigned_int__0__10_Impl {
    fn operator_call(&self) -> u32;
}
impl rand1_unsigned_int__0__10_Impl for Ptr<rand1_unsigned_int__0__10_> {
    fn operator_call(&self) -> u32 {
        let r: Value<u32> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 10_u32) {
            field!((*self), x_).with_mut(|__v| {
                __v.prefix_inc();
            });
        } else {
            field!((*self), x_).write(0_u32);
        }
        return (*r.borrow());
    }
}
pub trait rand1_unsigned_int__0__4294967295_Impl {
    fn operator_call(&self) -> u32;
}
impl rand1_unsigned_int__0__4294967295_Impl for Ptr<rand1_unsigned_int__0__4294967295_> {
    fn operator_call(&self) -> u32 {
        let r: Value<u32> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 4294967295_u32) {
            field!((*self), x_).with_mut(|__v| {
                __v.prefix_inc();
            });
        } else {
            field!((*self), x_).write(0_u32);
        }
        return (*r.borrow());
    }
}
pub trait rand1_unsigned_int__2__3_Impl {
    fn operator_call(&self) -> u32;
}
impl rand1_unsigned_int__2__3_Impl for Ptr<rand1_unsigned_int__2__3_> {
    fn operator_call(&self) -> u32 {
        let r: Value<u32> = Rc::new(RefCell::new((*self).with(|__s| __s.x_)));
        if ((*self).with(|__s| __s.x_) < 3_u32) {
            field!((*self), x_).with_mut(|__v| {
                __v.prefix_inc();
            });
        } else {
            field!((*self), x_).write(2_u32);
        }
        return (*r.borrow());
    }
}
pub fn __cpp2rust_init_globals() {}
