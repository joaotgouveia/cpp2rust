extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct soccc_allocator_char_ {
    #[offset(0)]
    #[byte_size(8)]
    soccc_count: Ptr<()>,
    #[offset(8)]
    self_soccc_count: (),
}
impl soccc_allocator_char_ {
    pub fn new(mut soccc_count_: Ptr<()>, self_coccc_count_: Option<()>) -> Self {
        let mut self_coccc_count_: () = self_coccc_count_.unwrap_or_else(|| 0_());
        Self {
            soccc_count: (soccc_count_).clone(),
            self_soccc_count: self_coccc_count_,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct soccc_allocator_wchar_t_ {
    #[offset(0)]
    #[byte_size(8)]
    soccc_count: Ptr<()>,
    #[offset(8)]
    self_soccc_count: (),
}
impl soccc_allocator_wchar_t_ {
    pub fn new(mut soccc_count_: Ptr<()>, self_coccc_count_: Option<()>) -> Self {
        let mut self_coccc_count_: () = self_coccc_count_.unwrap_or_else(|| 0_());
        Self {
            soccc_count: (soccc_count_).clone(),
            self_soccc_count: self_coccc_count_,
        }
    }
}
pub fn test_0() -> () {
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn test_1() -> () {
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let soccc_lhs: Value<()> = Rc::new(RefCell::new(0_()));
        let soccc_rhs: Value<()> = Rc::new(RefCell::new(0_()));
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
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
    ({ test_0() });
    ({ test_1() });
    const _: () = assert!(true, "test<char>()");;
    const _: () = assert!(true, "test<wchar_t>()");;
    return 0_();
}
pub trait soccc_allocator_char_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn select_on_container_copy_construction(&self) -> soccc_allocator_char_;
    fn get_soccc(&self) -> Ptr<()>;
    fn get_self_soccc(&self) -> ();
}
impl soccc_allocator_char_Impl for Ptr<soccc_allocator_char_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
    }
    fn select_on_container_copy_construction(&self) -> soccc_allocator_char_ {
        {
            (*self)
                .with(|__s| __s.soccc_count.clone())
                .with_mut(|__v| *__v = *__v + 1_())
        };
        return soccc_allocator_char_::new({ (*self).with(|__s| __s.soccc_count.clone()) }, {
            Some(((*self).with(|__s| __s.self_soccc_count) + 1_()))
        });
    }
    fn get_soccc(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.soccc_count.clone());
    }
    fn get_self_soccc(&self) -> () {
        return (*self).with(|__s| __s.self_soccc_count);
    }
}
pub trait soccc_allocator_wchar_t_Impl {
    fn allocate(&self, n: ()) -> Ptr<()>;
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn select_on_container_copy_construction(&self) -> soccc_allocator_wchar_t_;
    fn get_soccc(&self) -> Ptr<()>;
    fn get_self_soccc(&self) -> ();
}
impl soccc_allocator_wchar_t_Impl for Ptr<soccc_allocator_wchar_t_> {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!();
    }
    fn select_on_container_copy_construction(&self) -> soccc_allocator_wchar_t_ {
        {
            (*self)
                .with(|__s| __s.soccc_count.clone())
                .with_mut(|__v| *__v = *__v + 1_())
        };
        return soccc_allocator_wchar_t_::new({ (*self).with(|__s| __s.soccc_count.clone()) }, {
            Some(((*self).with(|__s| __s.self_soccc_count) + 1_()))
        });
    }
    fn get_soccc(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.soccc_count.clone());
    }
    fn get_self_soccc(&self) -> () {
        return (*self).with(|__s| __s.self_soccc_count);
    }
}
pub fn __cpp2rust_init_globals() {}
