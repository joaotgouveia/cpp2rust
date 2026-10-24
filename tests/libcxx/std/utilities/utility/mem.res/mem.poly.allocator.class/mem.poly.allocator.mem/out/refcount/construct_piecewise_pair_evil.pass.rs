extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct EvilAlloc_char_ {
    #[offset(0)]
    #[byte_size(8)]
    pub inner_: (),
}
impl EvilAlloc_char_ {
    pub fn new_1() -> Self {
        Self {
            inner_: unimplemented!(),
        }
    }
    pub fn new_2(a: Ptr<()>) -> Self {
        Self {
            inner_: unimplemented!(),
        }
    }
}
impl Default for EvilAlloc_char_ {
    fn default() -> Self {
        { EvilAlloc_char_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct WidgetV0 {
    #[offset(0)]
    value_: (),
}
impl WidgetV0 {
    pub fn new(mut v: ()) -> Self {
        Self { value_: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct WidgetV1 {
    #[offset(0)]
    value_: (),
    #[offset(8)]
    #[byte_size(8)]
    alloc_: EvilAlloc_char_,
}
impl WidgetV1 {
    pub fn new_1(mut v: ()) -> Self {
        Self {
            value_: v,
            alloc_: EvilAlloc_char_::new_1(),
        }
    }
    pub fn new_2(_a0: (), mut a: EvilAlloc_char_, mut v: ()) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            value_: v,
            alloc_: (a).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct WidgetV2 {
    #[offset(0)]
    value_: (),
    #[offset(8)]
    #[byte_size(8)]
    alloc_: EvilAlloc_char_,
}
impl WidgetV2 {
    pub fn new_1(mut v: ()) -> Self {
        Self {
            value_: v,
            alloc_: EvilAlloc_char_::new_1(),
        }
    }
    pub fn new_2(mut v: (), mut a: EvilAlloc_char_) -> Self {
        Self {
            value_: v,
            alloc_: (a).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct WidgetV3 {
    #[offset(0)]
    value_: (),
    #[offset(8)]
    #[byte_size(8)]
    alloc_: EvilAlloc_char_,
}
impl WidgetV3 {
    pub fn new_1(mut v: ()) -> Self {
        Self {
            value_: v,
            alloc_: EvilAlloc_char_::new_1(),
        }
    }
    pub fn new_2(_a0: (), mut a: EvilAlloc_char_, mut v: ()) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            value_: v,
            alloc_: (a).clone(),
        }
    }
    pub fn new_3(mut v: (), mut a: EvilAlloc_char_) -> Self {
        Self {
            value_: v,
            alloc_: (a).clone(),
        }
    }
}
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV1, EvilAlloc<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV2, EvilAlloc<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV3, EvilAlloc<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV1, std::pmr::polymorphic_allocator<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV2, std::pmr::polymorphic_allocator<char>>::value"
);
const _: () = assert!(
    true,
    "std::uses_allocator<WidgetV3, std::pmr::polymorphic_allocator<char>>::value"
);
pub fn test_evil_0() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_1() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..24).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_2() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..24).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_3() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..24).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_4() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..24).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_5() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_6() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_7() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_8() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..24).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_9() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_10() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_11() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_12() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..24).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_13() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_14() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_evil_15() {
    let pma: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..32).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        let mut p: Ptr<()> = (buffer.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
    ({ test_evil_0() });
    ({ test_evil_1() });
    ({ test_evil_2() });
    ({ test_evil_3() });
    ({ test_evil_4() });
    ({ test_evil_5() });
    ({ test_evil_6() });
    ({ test_evil_7() });
    ({ test_evil_8() });
    ({ test_evil_9() });
    ({ test_evil_10() });
    ({ test_evil_11() });
    ({ test_evil_12() });
    ({ test_evil_13() });
    ({ test_evil_14() });
    ({ test_evil_15() });
    return 0_();
}
pub trait WidgetV0Impl {
    fn holds(&self, v: (), _a1: Ptr<()>) -> ();
}
impl WidgetV0Impl for Ptr<WidgetV0> {
    fn holds(&self, mut v: (), _a1: Ptr<()>) -> () {
        return ((*self).with(|__s| __s.value_) == v);
    }
}
pub trait WidgetV1Impl {
    fn holds(&self, v: (), a: Ptr<()>) -> ();
}
impl WidgetV1Impl for Ptr<WidgetV1> {
    fn holds(&self, mut v: (), a: Ptr<()>) -> () {
        return ((*self).with(|__s| __s.value_) == v) && (unimplemented!());
    }
}
pub trait WidgetV2Impl {
    fn holds(&self, v: (), a: ()) -> ();
}
impl WidgetV2Impl for Ptr<WidgetV2> {
    fn holds(&self, mut v: (), a: ()) -> () {
        let a: Value<()> = Rc::new(RefCell::new(a));
        return ((*self).with(|__s| __s.value_) == v) && (unimplemented!());
    }
}
pub trait WidgetV3Impl {
    fn holds(&self, v: (), a: ()) -> ();
}
impl WidgetV3Impl for Ptr<WidgetV3> {
    fn holds(&self, mut v: (), a: ()) -> () {
        let a: Value<()> = Rc::new(RefCell::new(a));
        return ((*self).with(|__s| __s.value_) == v) && (unimplemented!());
    }
}
pub fn __cpp2rust_init_globals() {}
