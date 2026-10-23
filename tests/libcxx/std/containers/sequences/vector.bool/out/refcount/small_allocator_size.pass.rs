extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub fn new_1(a: Ptr<sized_allocator_unsigned_char__unsigned_char__signed_char_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub fn new_2(mut d: ()) -> Self {
        Self { data_: d }
    }
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_char__signed_char_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {}
impl sized_allocator_bool__unsigned_short__short_ {
    pub fn new_1(a: Ptr<sized_allocator_unsigned_short__unsigned_short__short_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub fn new_2(mut d: ()) -> Self {
        Self { data_: d }
    }
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_short__short_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_short__unsigned_short__short_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {}
impl sized_allocator_bool__unsigned_int__int_ {
    pub fn new_1(a: Ptr<sized_allocator_unsigned_int__unsigned_int__int_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub fn new_2(mut d: ()) -> Self {
        Self { data_: d }
    }
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_int__int_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_int__unsigned_int__int_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {}
impl sized_allocator_bool__unsigned_long__long_ {
    pub fn new_1(a: Ptr<sized_allocator_unsigned_long__unsigned_long__long_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub fn new_2(mut d: ()) -> Self {
        Self { data_: d }
    }
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_long__long_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    #[offset(0)]
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {}
pub fn tests_0() -> () {
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
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
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub trait sized_allocator_bool__unsigned_char__signed_char_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_bool__unsigned_char__signed_char_>
{
}
pub trait sized_allocator_bool__unsigned_int__int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_int__int_Impl
    for Ptr<sized_allocator_bool__unsigned_int__int_>
{
}
pub trait sized_allocator_bool__unsigned_long__long_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_long__long_Impl
    for Ptr<sized_allocator_bool__unsigned_long__long_>
{
}
pub trait sized_allocator_bool__unsigned_short__short_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_short__short_Impl
    for Ptr<sized_allocator_bool__unsigned_short__short_>
{
}
pub trait sized_allocator_unsigned_char__unsigned_char__signed_char_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_unsigned_char__unsigned_char__signed_char_>
{
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        if (n
            > ({ sized_allocator_unsigned_char__unsigned_char__signed_char_Impl::max_size(self) }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return ({ std_numeric_limits_unsigned_char_::max() })
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait sized_allocator_unsigned_int__unsigned_int__int_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl sized_allocator_unsigned_int__unsigned_int__int_Impl
    for Ptr<sized_allocator_unsigned_int__unsigned_int__int_>
{
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        if (n > ({ sized_allocator_unsigned_int__unsigned_int__int_Impl::max_size(self) })) {
            unimplemented!();
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return ({ std_numeric_limits_unsigned_int_::max() })
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait sized_allocator_unsigned_long__unsigned_long__long_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl sized_allocator_unsigned_long__unsigned_long__long_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_long__long_>
{
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        if (n > ({ sized_allocator_unsigned_long__unsigned_long__long_Impl::max_size(self) })) {
            unimplemented!();
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return (({ std_numeric_limits_unsigned_long_::max() }) as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub trait sized_allocator_unsigned_short__unsigned_short__short_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, n: ());
    fn max_size(&self) -> ();
}
impl sized_allocator_unsigned_short__unsigned_short__short_Impl
    for Ptr<sized_allocator_unsigned_short__unsigned_short__short_>
{
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        if (n > ({ sized_allocator_unsigned_short__unsigned_short__short_Impl::max_size(self) })) {
            unimplemented!();
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!();
    }
    fn max_size(&self) -> () {
        return ({ std_numeric_limits_unsigned_short_::max() })
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub fn __cpp2rust_init_globals() {}
