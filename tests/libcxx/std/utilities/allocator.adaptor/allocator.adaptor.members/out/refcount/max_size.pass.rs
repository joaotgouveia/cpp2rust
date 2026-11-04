extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static move_called_1: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct A1_int_ {
    #[offset(0)]
    id_: i32,
}
impl A1_int_ {
    pub fn new(mut id: i32) -> Self {
        Self { id_: id }
    }
    pub fn move_from(a: Ptr<A1_int_>) -> Self {
        let __this: A1_int_ = Self {
            id_: ({ A1_int_Impl::id(&a) }),
        };
        move_called_1.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl CppMove for A1_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
thread_local!(
    pub static copy_called_4: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A2_int_ {
    #[offset(0)]
    id_: i32,
}
impl A2_int_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
    pub fn copy_from(a: Ptr<A2_int_>) -> Self {
        let __this: A2_int_ = Self {
            id_: ({ A2_int_Impl::id(&a) }),
        };
        copy_called_4.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl Clone for A2_int_ {
    fn clone(&self) -> Self {
        let __src: Value<A2_int_> = Rc::new(RefCell::new(A2_int_ {
            id_: self.id_.clone(),
        }));
        A2_int_::copy_from(__src.as_pointer())
    }
}
impl CppMove for A2_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for A2_int_ {
    fn default() -> Self {
        { A2_int_::new(None) }
    }
}
thread_local!(
    pub static copy_called_7: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A3_int_ {
    #[offset(0)]
    id_: i32,
}
impl A3_int_ {
    pub fn new(id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| 0);
        Self { id_: id }
    }
    pub fn copy_from(a: Ptr<A3_int_>) -> Self {
        let __this: A3_int_ = Self {
            id_: ({ A3_int_Impl::id(&a) }),
        };
        copy_called_7.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl Clone for A3_int_ {
    fn clone(&self) -> Self {
        let __src: Value<A3_int_> = Rc::new(RefCell::new(A3_int_ {
            id_: self.id_.clone(),
        }));
        A3_int_::copy_from(__src.as_pointer())
    }
}
impl CppMove for A3_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for A3_int_ {
    fn default() -> Self {
        { A3_int_::new(None) }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let a: Value<std_scoped_allocator_adaptor_A1_int__> =
            Rc::new(RefCell::new(A1_int_::new({ 100 })));
        assert!(({ (({ (*a.borrow()).max_size() }) == 100_u64) }));
    }
    {
        let a: Value<std_scoped_allocator_adaptor_A1_int___A2_int__> = Rc::new(RefCell::new({
            let __tmp_0: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new({ 20 })));
            let __tmp_1: Value<A2_int_> = Rc::new(RefCell::new(A2_int_::new(None)));
            std_scoped_allocator_adaptor_A1_int___A2_int__::new_1({ __tmp_0.as_pointer() }, {
                __tmp_1.as_pointer()
            })
        }));
        assert!(({ (({ (*a.borrow()).max_size() }) == 20_u64) }));
    }
    {
        let a: Value<std_scoped_allocator_adaptor_A1_int___A2_int___A3_int__> =
            Rc::new(RefCell::new({
                let __tmp_2: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new({ 200 })));
                let __tmp_3: Value<A2_int_> = Rc::new(RefCell::new(A2_int_::new(None)));
                let __tmp_4: Value<A3_int_> = Rc::new(RefCell::new(A3_int_::new(None)));
                std_scoped_allocator_adaptor_A1_int___A2_int___A3_int__::new_1(
                    { __tmp_2.as_pointer() },
                    { __tmp_3.as_pointer() },
                    { __tmp_4.as_pointer() },
                )
            }));
        assert!(({ (({ (*a.borrow()).max_size() }) == 200_u64) }));
    }
    return 0;
}
pub trait A1_int_Impl {
    fn id(&self) -> i32;
    fn copy_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut n: usize) {
        unimplemented!()
    }
    fn max_size(&self) -> usize;
}
impl A1_int_Impl for Ptr<A1_int_> {
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
    fn max_size(&self) -> usize {
        return ((*self).with(|__s| __s.id_) as usize);
    }
}
pub trait A2_int_Impl {
    fn id(&self) -> i32;
    fn copy_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut _a0: usize, mut hint: AnyPtr) -> Ptr<i32> {
        unimplemented!()
    }
}
impl A2_int_Impl for Ptr<A2_int_> {
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait A3_int_Impl {
    fn id(&self) -> i32;
    fn copy_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A3_int_>) -> Ptr<A3_int_> {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> A3_int_ {
        unimplemented!()
    }
}
impl A3_int_Impl for Ptr<A3_int_> {
    fn id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = move_called_1.with(|_| ());
    let _ = copy_called_4.with(|_| ());
    let _ = copy_called_7.with(|_| ());
}
