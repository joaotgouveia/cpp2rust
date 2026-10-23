extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
impl A1_double_ {
    pub fn new_1(a: Ptr<A1_int_>) -> Self {
        let __this: A1_double_ = Self {
            id_: ({ A1_int_Impl::id(&a) }),
        };
        move_called_0.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
thread_local!(
    pub static move_called_0: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A1_double_ {
    #[offset(0)]
    id_: (),
}
impl A1_double_ {}
impl A1_int_ {
    pub fn new_1(a: Ptr<A1_double_>) -> Self {
        let __this: A1_int_ = Self {
            id_: ({ A1_double_Impl::id(&a) }),
        };
        move_called_4.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
thread_local!(
    pub static copy_called_5: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static move_called_4: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct A1_int_ {
    #[offset(0)]
    id_: (),
}
impl A1_int_ {
    pub fn new_2(mut id: ()) -> Self {
        Self { id_: id }
    }
}
pub fn operator_eq_8(x: Ptr<A1_int_>, y: Ptr<A1_double_>) -> () {
    return ({ ({ A1_int_Impl::id(&x) }) } == { ({ A1_double_Impl::id(&y) }) });
}
thread_local!(
    pub static copy_called_9: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static move_called_10: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
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
        copy_called_9.with(|rc| *rc.borrow_mut() = true);
        __this
    }
    pub fn move_from(a: Ptr<A2_int_>) -> Self {
        let __this: A2_int_ = Self {
            id_: ({ A2_int_Impl::id(&a) }),
        };
        move_called_10.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl std::cmp::PartialEq for A2_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_12(
                Rc::new(RefCell::new(A2_int_ {
                    id_: self.id_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(A2_int_ {
                    id_: other.id_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for A2_int_ {}
impl Clone for A2_int_ {
    fn clone(&self) -> Self {
        let __src: Value<A2_int_> = Rc::new(RefCell::new(A2_int_ {
            id_: self.id_.clone(),
        }));
        A2_int_::copy_from(__src.as_pointer())
    }
}
pub fn operator_eq_12(x: Ptr<A2_int_>, y: Ptr<A2_int_>) -> () {
    return ({ ({ A2_int_Impl::id(&x) }) } == { ({ A2_int_Impl::id(&y) }) });
}
thread_local!(
    pub static copy_called_13: Value<()> = Rc::new(RefCell::new(false));
);
thread_local!(
    pub static move_called_14: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(4)]
pub struct A3_int_ {
    #[offset(0)]
    id_: (),
}
impl A3_int_ {
    pub fn new(id: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| 0_());
        Self { id_: id }
    }
    pub fn copy_from(a: Ptr<A3_int_>) -> Self {
        let __this: A3_int_ = Self {
            id_: ({ A3_int_Impl::id(&a) }),
        };
        copy_called_13.with(|rc| *rc.borrow_mut() = true);
        __this
    }
    pub fn move_from(a: Ptr<A3_int_>) -> Self {
        let __this: A3_int_ = Self {
            id_: ({ A3_int_Impl::id(&a) }),
        };
        move_called_14.with(|rc| *rc.borrow_mut() = true);
        __this
    }
}
impl std::cmp::PartialEq for A3_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_17(
                Rc::new(RefCell::new(A3_int_ {
                    id_: self.id_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(A3_int_ {
                    id_: other.id_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for A3_int_ {}
impl Clone for A3_int_ {
    fn clone(&self) -> Self {
        let __src: Value<A3_int_> = Rc::new(RefCell::new(A3_int_ {
            id_: self.id_.clone(),
        }));
        A3_int_::copy_from(__src.as_pointer())
    }
}
impl Default for A3_int_ {
    fn default() -> Self {
        { A3_int_::new(None) }
    }
}
pub fn operator_eq_17(x: Ptr<A3_int_>, y: Ptr<A3_int_>) -> () {
    return ({ ({ A3_int_Impl::id(&x) }) } == { ({ A3_int_Impl::id(&y) }) });
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
        let a1: Value<()> = Rc::new(RefCell::new(A1_int_::new_2({ 3_() })));
        copy_called_5.with(|rc| *rc.borrow_mut() = false);
        move_called_4.with(|rc| *rc.borrow_mut() = false);
        let a2: Value<()> = Rc::new(RefCell::new((*a1.borrow_mut()).clone()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a1: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new_2({ 4_() })));
            let __tmp_1: Value<A2_int_> = Rc::new(RefCell::new(A2_int_::new({ 5_() })));
            std_scoped_allocator_adaptor_A1_double___A2_int__::new_1({ __tmp_0.as_pointer() }, {
                __tmp_1.as_pointer()
            })
        }));
        copy_called_5.with(|rc| *rc.borrow_mut() = false);
        move_called_4.with(|rc| *rc.borrow_mut() = false);
        copy_called_9.with(|rc| *rc.borrow_mut() = false);
        move_called_10.with(|rc| *rc.borrow_mut() = false);
        let a2: Value<()> = Rc::new(RefCell::new((*a1.borrow_mut()).clone()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let a1: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<A1_int_> = Rc::new(RefCell::new(A1_int_::new_2({ 4_() })));
            let __tmp_3: Value<A2_int_> = Rc::new(RefCell::new(A2_int_::new({ 5_() })));
            let __tmp_4: Value<A3_int_> = Rc::new(RefCell::new(A3_int_::new({ Some(6_()) })));
            std_scoped_allocator_adaptor_A1_double___A2_int___A3_int__::new_1(
                { __tmp_2.as_pointer() },
                { __tmp_3.as_pointer() },
                { __tmp_4.as_pointer() },
            )
        }));
        copy_called_5.with(|rc| *rc.borrow_mut() = false);
        move_called_4.with(|rc| *rc.borrow_mut() = false);
        copy_called_9.with(|rc| *rc.borrow_mut() = false);
        move_called_10.with(|rc| *rc.borrow_mut() = false);
        copy_called_13.with(|rc| *rc.borrow_mut() = false);
        move_called_14.with(|rc| *rc.borrow_mut() = false);
        let a2: Value<()> = Rc::new(RefCell::new((*a1.borrow_mut()).clone()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait A1_double_Impl {
    fn id(&self) -> ();
    fn copy_assign(&self, a: Ptr<A1_double_>) -> Ptr<A1_double_> {
        unimplemented!()
    }
    fn move_assign(&self, a: Ptr<A1_double_>) -> Ptr<A1_double_> {
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
impl A1_double_Impl for Ptr<A1_double_> {
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
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
pub trait A3_int_Impl {
    fn id(&self) -> ();
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
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = move_called_0.with(|_| ());
    let _ = copy_called_5.with(|_| ());
    let _ = move_called_4.with(|_| ());
    let _ = copy_called_9.with(|_| ());
    let _ = move_called_10.with(|_| ());
    let _ = copy_called_13.with(|_| ());
    let _ = move_called_14.with(|_| ());
}
