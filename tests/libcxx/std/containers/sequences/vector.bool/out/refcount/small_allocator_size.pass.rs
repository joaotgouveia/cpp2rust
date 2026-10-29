extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub fn new(mut d: i32) -> Self {
        Self { data_: d }
    }
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_char__signed_char_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub fn new(mut d: i32) -> Self {
        Self { data_: d }
    }
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_short__short_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_short__short_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub fn new(mut d: i32) -> Self {
        Self { data_: d }
    }
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {
    pub fn new_1(a: Ptr<sized_allocator_bool__unsigned_int__int_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_unsigned_long__unsigned_int__int_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    #[offset(0)]
    data_: i32,
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub fn new(mut d: i32) -> Self {
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
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {}
pub fn tests_0() -> bool {
    {
        let mut c: Vec<bool> = {
            let __tmp_0: Value<sized_allocator_bool__unsigned_char__signed_char_> =
                Rc::new(RefCell::new(
                    sized_allocator_bool__unsigned_char__signed_char_::new({ 1 }),
                ));
            std_vector_bool__sized_allocator_bool__unsigned_char__signed_char__::new_1({
                __tmp_0.as_pointer()
            })
        };
        {
            let __a0 = 10_usize as usize;
            c.resize_with(__a0, || <bool>::default())
        };
        assert!((c.len() == 10_usize));
        assert!((c.capacity() >= 10_usize));
    }
    {
        let c: Value<Vec<bool>> = Rc::new(RefCell::new({
            let __tmp_1: Value<sized_allocator_bool__unsigned_char__signed_char_> =
                Rc::new(RefCell::new(
                    sized_allocator_bool__unsigned_char__signed_char_::new({ 1 }),
                ));
            std_vector_bool__sized_allocator_bool__unsigned_char__signed_char__::new_1({
                __tmp_1.as_pointer()
            })
        }));
        ({
            let ___x: Value<bool> = Rc::new(RefCell::new(true));
            (*c.borrow_mut()).assign_19(10_usize, ___x.as_pointer())
        });
        assert!(((*c.borrow()).len() == 10_usize));
        assert!(((*c.borrow()).capacity() >= 10_usize));
    }
    {
        let c: Value<Vec<bool>> = Rc::new(RefCell::new({
            let __tmp_2: Value<sized_allocator_bool__unsigned_char__signed_char_> =
                Rc::new(RefCell::new(
                    sized_allocator_bool__unsigned_char__signed_char_::new({ 1 }),
                ));
            std_vector_bool__sized_allocator_bool__unsigned_char__signed_char__::new_1({
                __tmp_2.as_pointer()
            })
        }));
        {
            let __it = <RandomAccessIterator<bool>>::from(c.decay().end());
            let __off = __it.index();
            (*c.borrow_mut()).insert(__off, true);
            __it
        };
        assert!(((*c.borrow()).len() == 1_usize));
        assert!(((*c.borrow()).capacity() >= 1_usize));
    }
    {
        let c: Value<Vec<bool>> = Rc::new(RefCell::new({
            let __tmp_3: Value<sized_allocator_bool__unsigned_short__short_> = Rc::new(
                RefCell::new(sized_allocator_bool__unsigned_short__short_::new({ 1 })),
            );
            std_vector_bool__sized_allocator_bool__unsigned_short__short__::new_1({
                __tmp_3.as_pointer()
            })
        }));
        {
            let __it = <RandomAccessIterator<bool>>::from(c.decay().end());
            let __off = __it.index();
            (*c.borrow_mut()).splice(__off..__off, std::iter::repeat_n(true, 10_usize));
            __it
        };
        assert!(((*c.borrow()).len() == 10_usize));
        assert!(((*c.borrow()).capacity() >= 10_usize));
    }
    {
        let mut c: Vec<bool> = {
            let __tmp_4: Value<sized_allocator_bool__unsigned_short__short_> = Rc::new(
                RefCell::new(sized_allocator_bool__unsigned_short__short_::new({ 1 })),
            );
            std_vector_bool__sized_allocator_bool__unsigned_short__short__::new_1({
                __tmp_4.as_pointer()
            })
        };
        {
            let a0_clone = true.clone();
            c.push(a0_clone)
        };
        assert!((c.len() == 1_usize));
        assert!((c.capacity() >= 1_usize));
    }
    {
        let mut c: Vec<bool> = {
            let __tmp_5: Value<sized_allocator_bool__unsigned_short__short_> = Rc::new(
                RefCell::new(sized_allocator_bool__unsigned_short__short_::new({ 1 })),
            );
            std_vector_bool__sized_allocator_bool__unsigned_short__short__::new_1({
                __tmp_5.as_pointer()
            })
        };
        {
            let __a0 = 10_usize as usize;
            c.resize(__a0, true)
        };
        assert!((c.len() == 10_usize));
        assert!((c.capacity() >= 10_usize));
    }
    {
        let mut c: Vec<bool> = {
            let __tmp_6: Value<sized_allocator_bool__unsigned_int__int_> = Rc::new(RefCell::new(
                sized_allocator_bool__unsigned_int__int_::new({ 1 }),
            ));
            std_vector_bool__sized_allocator_bool__unsigned_int__int__::new_1({
                __tmp_6.as_pointer()
            })
        };
        {
            let __a0 = 10_usize as usize;
            c.resize_with(__a0, || <bool>::default())
        };
        assert!((c.len() == 10_usize));
        assert!((c.capacity() >= 10_usize));
    }
    {
        let mut c: Vec<bool> = {
            let __tmp_7: Value<sized_allocator_bool__unsigned_long__long_> = Rc::new(RefCell::new(
                sized_allocator_bool__unsigned_long__long_::new({ 1 }),
            ));
            std_vector_bool__sized_allocator_bool__unsigned_long__long__::new_1({
                __tmp_7.as_pointer()
            })
        };
        {
            let __a0 = 10_usize as usize;
            c.resize_with(__a0, || <bool>::default())
        };
        assert!((c.len() == 10_usize));
        assert!((c.capacity() >= 10_usize));
    }
    {
        let mut c: Vec<bool> = {
            let __tmp_8: Value<sized_allocator_bool__unsigned_long__long_> = Rc::new(RefCell::new(
                sized_allocator_bool__unsigned_long__long_::new({ 1 }),
            ));
            std_vector_bool__sized_allocator_bool__unsigned_long__long__::new_1({
                __tmp_8.as_pointer()
            })
        };
        {
            let __a0 = 10_usize as usize;
            c.resize_with(__a0, || <bool>::default())
        };
        assert!((c.len() == 10_usize));
        assert!((c.capacity() >= 10_usize));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub trait sized_allocator_bool__unsigned_char__signed_char_Impl {
    fn allocate(&self, mut n: u8) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: u8) {
        unimplemented!()
    }
    fn max_size(&self) -> u8 {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_bool__unsigned_char__signed_char_>
{
}
pub trait sized_allocator_bool__unsigned_int__int_Impl {
    fn allocate(&self, mut n: u32) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_int__int_Impl
    for Ptr<sized_allocator_bool__unsigned_int__int_>
{
}
pub trait sized_allocator_bool__unsigned_long__long_Impl {
    fn allocate(&self, mut n: u64) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_long__long_Impl
    for Ptr<sized_allocator_bool__unsigned_long__long_>
{
}
pub trait sized_allocator_bool__unsigned_short__short_Impl {
    fn allocate(&self, mut n: u16) -> Ptr<bool> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<bool>, mut n: u16) {
        unimplemented!()
    }
    fn max_size(&self) -> u16 {
        unimplemented!()
    }
}
impl sized_allocator_bool__unsigned_short__short_Impl
    for Ptr<sized_allocator_bool__unsigned_short__short_>
{
}
pub trait sized_allocator_unsigned_long__unsigned_char__signed_char_Impl {
    fn allocate(&self, n: u8) -> Ptr<u64>;
    fn deallocate(&self, p: Ptr<u64>, n: u8);
    fn max_size(&self) -> u8;
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_char__signed_char_>
{
    fn allocate(&self, mut n: u8) -> Ptr<u64> {
        if ((n as i32)
            > (({ sized_allocator_unsigned_long__unsigned_char__signed_char_Impl::max_size(self) })
                as i32))
        {
            std::process::abort();
        }
        return ({ <std_allocator_unsigned_long_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: u8) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u8 {
        return (((<u8>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u8);
    }
}
pub trait sized_allocator_unsigned_long__unsigned_int__int_Impl {
    fn allocate(&self, n: u32) -> Ptr<u64>;
    fn deallocate(&self, p: Ptr<u64>, n: u32);
    fn max_size(&self) -> u32;
}
impl sized_allocator_unsigned_long__unsigned_int__int_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_int__int_>
{
    fn allocate(&self, mut n: u32) -> Ptr<u64> {
        if (n > ({ sized_allocator_unsigned_long__unsigned_int__int_Impl::max_size(self) })) {
            std::process::abort();
        }
        return ({ <std_allocator_unsigned_long_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: u32) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u32 {
        return (((<u32>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u32);
    }
}
pub trait sized_allocator_unsigned_long__unsigned_long__long_Impl {
    fn allocate(&self, n: u64) -> Ptr<u64>;
    fn deallocate(&self, p: Ptr<u64>, n: u64);
    fn max_size(&self) -> u64;
}
impl sized_allocator_unsigned_long__unsigned_long__long_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_long__long_>
{
    fn allocate(&self, mut n: u64) -> Ptr<u64> {
        if (n > ({ sized_allocator_unsigned_long__unsigned_long__long_Impl::max_size(self) })) {
            std::process::abort();
        }
        return ({ <std_allocator_unsigned_long_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: u64) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u64 {
        return (<u64>::MAX as u64).wrapping_div((::std::mem::size_of::<u64>() as u64));
    }
}
pub trait sized_allocator_unsigned_long__unsigned_short__short_Impl {
    fn allocate(&self, n: u16) -> Ptr<u64>;
    fn deallocate(&self, p: Ptr<u64>, n: u16);
    fn max_size(&self) -> u16;
}
impl sized_allocator_unsigned_long__unsigned_short__short_Impl
    for Ptr<sized_allocator_unsigned_long__unsigned_short__short_>
{
    fn allocate(&self, mut n: u16) -> Ptr<u64> {
        if ((n as i32)
            > (({ sized_allocator_unsigned_long__unsigned_short__short_Impl::max_size(self) })
                as i32))
        {
            std::process::abort();
        }
        return ({ <std_allocator_unsigned_long_>::default().allocate((n as usize)) });
    }
    fn deallocate(&self, mut p: Ptr<u64>, mut n: u16) {
        ({
            let ___p: Ptr<u64> = (p).clone();
            let ___n: usize = (n as usize);
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
    fn max_size(&self) -> u16 {
        return (((<u16>::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u16);
    }
}
pub fn __cpp2rust_init_globals() {}
