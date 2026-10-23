extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_scoped_allocator_adaptor_A1_int____std_scoped_allocator_adaptor_A1_int___
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_scoped_allocator_adaptor_A1_int___A2_int____std_scoped_allocator_adaptor_A1_int___A2_int___
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_scoped_allocator_adaptor_A1_int___A2_int___A2_float____std_scoped_allocator_adaptor_A1_int___A2_int___A2_float___
{}
thread_local!(
    pub static copy_called_3: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static move_called_4: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct A1_int_ {
    #[offset(0)]
    id_: (),
}
impl A1_int_ {
    pub fn new(mut id: ()) -> Self {
        Self { id_: id }
    }
    pub fn copy_from(a: Ptr<A1_int_>) -> Self {
        let __this: A1_int_ = Self {
            id_: ({ A1_int_Impl::id(&a) }),
        };
        copy_called_3.with(|rc| *rc.borrow_mut() = true);
        __this
    }
    pub fn move_from(a: Ptr<A1_int_>) -> Self {
        let __this: A1_int_ = Self {
            id_: ({ A1_int_Impl::id(&a) }),
        };
        move_called_4.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl Clone for A1_int_ {
    fn clone(&self) -> Self {
        let __src: Value<A1_int_> = Rc::new(RefCell::new(A1_int_ {
            id_: self.id_.clone(),
        }));
        A1_int_::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static copy_called_7: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A2_int_ {
    #[offset(0)]
    id_: (),
}
impl A2_int_ {
    pub fn new(mut id: ()) -> Self {
        Self { id_: id }
    }
    pub fn copy_from(a: Ptr<A2_int_>) -> Self {
        let __this: A2_int_ = Self {
            id_: ({ A2_int_Impl::id(&a) }),
        };
        copy_called_7.with(|rc| *rc.borrow_mut() = true);
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
thread_local!(
    pub static copy_called_10: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A2_float_ {
    #[offset(0)]
    id_: (),
}
impl A2_float_ {
    pub fn new(mut id: ()) -> Self {
        Self { id_: id }
    }
    pub fn copy_from(a: Ptr<A2_float_>) -> Self {
        let __this: A2_float_ = Self {
            id_: ({ A2_float_Impl::id(&a) }),
        };
        copy_called_10.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl Clone for A2_float_ {
    fn clone(&self) -> Self {
        let __src: Value<A2_float_> = Rc::new(RefCell::new(A2_float_ {
            id_: self.id_.clone(),
        }));
        A2_float_::copy_from(__src.as_pointer())
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
    {
        let outer: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new({ 3_() })));
        let a: Value<()> = Rc::new(RefCell::new(std_scoped_allocator_adaptor_A1_int__::new_1(
            { outer.as_pointer() },
        )));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(A1_int_::new({ 3_() })));
        const _: () = assert!(true, "");;
    }
    {
        let outer: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new({ 3_() })));
        let inner: Value<A2_int_> = Rc::new(RefCell::new(A2_int_::new({ 4_() })));
        let a: Value<()> = Rc::new(RefCell::new(
            std_scoped_allocator_adaptor_A1_int___A2_int__::new_1({ outer.as_pointer() }, {
                inner.as_pointer()
            }),
        ));
        const _: () = assert!(true, "");;
    }
    {
        let outer: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new({ 3_() })));
        let inner: Value<A2_int_> = Rc::new(RefCell::new(A2_int_::new({ 4_() })));
        let a: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<A2_float_> = Rc::new(RefCell::new(A2_float_::new({ 5_() })));
            std_scoped_allocator_adaptor_A1_int___A2_int___A2_float__::new_1(
                { outer.as_pointer() },
                { inner.as_pointer() },
                { __tmp_0.as_pointer() },
            )
        }));
        const _: () = assert!(true, "");;
    }
    return 0_();
}
pub trait A1_int_Impl {
    fn id(&self) -> ();
    fn copy_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_int_>) -> Ptr<A1_int_> {
        unimplemented!()
    }
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
impl A1_int_Impl for Ptr<A1_int_> {
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait A2_float_Impl {
    fn id(&self) -> ();
    fn copy_assign(&self, a: Ptr<A2_float_>) -> Ptr<A2_float_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A2_float_>) -> Ptr<A2_float_> {
        unimplemented!()
    }
    fn allocate(&self, mut _a0: (), mut hint: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl A2_float_Impl for Ptr<A2_float_> {
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait A2_int_Impl {
    fn id(&self) -> ();
    fn copy_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A2_int_>) -> Ptr<A2_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut _a0: (), mut hint: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl A2_int_Impl for Ptr<A2_int_> {
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copy_called_3.with(|_| ());
    let _ = move_called_4.with(|_| ());
    let _ = copy_called_7.with(|_| ());
    let _ = copy_called_10.with(|_| ());
}
