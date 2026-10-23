extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub fn assign_1(c1: Ptr<()>, c2: Ptr<()>) {
        c1.write({ (c2.read()) });
    }
    pub fn eq(mut c1: (), mut c2: ()) -> () {
        return (c1 == c2);
    }
    pub fn lt(mut c1: (), mut c2: ()) -> () {
        return (c1 < c2);
    }
    pub fn compare(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> () {
        'loop_: while (n != 0) {
            if ({
                let _c1: () = (s1.read());
                let _c2: () = (s2.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if ({
                let _c1: () = (s2.read());
                let _c2: () = (s1.read());
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
    }
    pub fn length(mut s: Ptr<()>) -> () {
        let len: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while !({
            let _c1: () = (s.read());
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            (*len.borrow_mut()).prefix_inc();
            s.prefix_inc();
        }
        return (*len.borrow());
    }
    pub fn copy(mut s1: Ptr<()>, mut s2: Ptr<()>, mut n: ()) -> Ptr<()> {
        {
            unimplemented!();
        }
        let r: Value<Ptr<()>> = Rc::new(RefCell::new((s1).clone()));
        'loop_: while (n != 0) {
            ({
                let _c1: Ptr<()> = (s1).clone();
                let _c2: Ptr<()> = (s2).clone();
                constexpr_char_traits_char_::assign_1(_c1, _c2)
            });
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return (*r.borrow()).clone();
    }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ConvertibleTo_std_string_view_ {
    #[offset(0)]
    #[byte_size(16)]
    pub t_: (),
}
impl ConvertibleTo_std_string_view_ {
    pub fn new(t: ()) -> Self {
        let t: Value<()> = Rc::new(RefCell::new(t));
        Self {
            t_: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ConvertibleTo_std_wstring_view_ {
    #[offset(0)]
    #[byte_size(16)]
    pub t_: (),
}
impl ConvertibleTo_std_wstring_view_ {
    pub fn new(t: ()) -> Self {
        let t: Value<()> = Rc::new(RefCell::new(t));
        Self {
            t_: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ConvertibleTo_std_u16string_view_ {
    #[offset(0)]
    #[byte_size(16)]
    pub t_: (),
}
impl ConvertibleTo_std_u16string_view_ {
    pub fn new(t: ()) -> Self {
        let t: Value<()> = Rc::new(RefCell::new(t));
        Self {
            t_: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ConvertibleTo_std_u32string_view_ {
    #[offset(0)]
    #[byte_size(16)]
    pub t_: (),
}
impl ConvertibleTo_std_u32string_view_ {
    pub fn new(t: ()) -> Self {
        let t: Value<()> = Rc::new(RefCell::new(t));
        Self {
            t_: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ConvertibleTo_std_basic_string_view_char__constexpr_char_traits_char___ {
    #[offset(0)]
    #[byte_size(16)]
    pub t_: (),
}
impl ConvertibleTo_std_basic_string_view_char__constexpr_char_traits_char___ {
    pub fn new(t: ()) -> Self {
        let t: Value<()> = Rc::new(RefCell::new(t));
        Self {
            t_: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ConvertibleTo_std_u8string_view_ {
    #[offset(0)]
    #[byte_size(16)]
    pub t_: (),
}
impl ConvertibleTo_std_u8string_view_ {
    pub fn new(t: ()) -> Self {
        let t: Value<()> = Rc::new(RefCell::new(t));
        Self {
            t_: unimplemented!(),
        }
    }
}
pub fn test_0() -> () {
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        let mut j: () = 0_();
        'loop_: while (j < 4_()) {
            let mut expected: () = (i < j);
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let abc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abc0def: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abcdef: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn test_1() -> () {
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        let mut j: () = 0_();
        'loop_: while (j < 4_()) {
            let mut expected: () = (i < j);
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let abc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abc0def: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abcdef: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn test_2() -> () {
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        let mut j: () = 0_();
        'loop_: while (j < 4_()) {
            let mut expected: () = (i < j);
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let abc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abc0def: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abcdef: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn test_3() -> () {
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        let mut j: () = 0_();
        'loop_: while (j < 4_()) {
            let mut expected: () = (i < j);
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let abc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abc0def: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abcdef: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn test_4() -> () {
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        let mut j: () = 0_();
        'loop_: while (j < 4_()) {
            let mut expected: () = (i < j);
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let abc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abc0def: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abcdef: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn test_5() -> () {
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut i: () = 0_();
    'loop_: while (i < 4_()) {
        let mut j: () = 0_();
        'loop_: while (j < 4_()) {
            let mut expected: () = (i < j);
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                unimplemented!();
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let abc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abc0def: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let abcdef: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    const _: () = assert!(true, "test<std::string_view>()");;
    const _: () = assert!(true, "test<std::wstring_view>()");;
    const _: () = assert!(true, "test<std::u16string_view>()");;
    const _: () = assert!(true, "test<std::u32string_view>()");;
    ({ test_4() });
    const _: () = assert!(
        true,
        "test<std::basic_string_view<char, constexpr_char_traits<char>>>()"
    );;
    ({ test_5() });
    const _: () = assert!(true, "test<std::u8string_view>()");;
    return 0_();
}
pub trait ConvertibleTo_std_basic_string_view_char__constexpr_char_traits_char___Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_std_basic_string_view_char__constexpr_char_traits_char___Impl
    for Ptr<ConvertibleTo_std_basic_string_view_char__constexpr_char_traits_char___>
{
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait ConvertibleTo_std_string_view_Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_std_string_view_Impl for Ptr<ConvertibleTo_std_string_view_> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait ConvertibleTo_std_u16string_view_Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_std_u16string_view_Impl for Ptr<ConvertibleTo_std_u16string_view_> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait ConvertibleTo_std_u32string_view_Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_std_u32string_view_Impl for Ptr<ConvertibleTo_std_u32string_view_> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait ConvertibleTo_std_u8string_view_Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_std_u8string_view_Impl for Ptr<ConvertibleTo_std_u8string_view_> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait ConvertibleTo_std_wstring_view_Impl {
    fn to_(&self) -> ();
}
impl ConvertibleTo_std_wstring_view_Impl for Ptr<ConvertibleTo_std_wstring_view_> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
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
