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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor, Default)]
#[byte_size(4)]
pub struct Deleter_A_ {
    #[offset(0)]
    state_: (),
}
impl Deleter_A_ {
    pub fn move_from(r: Ptr<Deleter_A_>) -> Self {
        let __this: Deleter_A_ = Self {
            state_: r.with(|__s| __s.state_),
        };
        field!(r, state_).write(0_());
        __this
    }
    pub fn new_1(mut s: ()) -> Self {
        Self { state_: s }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor, Default)]
#[byte_size(4)]
pub struct Deleter_Aarrarr_ {
    #[offset(0)]
    state_: (),
}
impl Deleter_Aarrarr_ {
    pub fn move_from(r: Ptr<Deleter_Aarrarr_>) -> Self {
        let __this: Deleter_Aarrarr_ = Self {
            state_: r.with(|__s| __s.state_),
        };
        field!(r, state_).write(0_());
        __this
    }
    pub fn new_1(mut s: ()) -> Self {
        Self { state_: s }
    }
}
pub fn swap_4(x: Ptr<Deleter_A_>, y: Ptr<Deleter_A_>) {
    let t: Value<Deleter_A_> = Rc::new(RefCell::new(Deleter_A_::move_from({ (x).clone() })));
    let _dtor_t = ScopedDestructor::new(&t, |__p| __p.destructor());
    ({
        let _r: Ptr<Deleter_A_> = (y).clone();
        Deleter_A_Impl::move_assign(&x, _r)
    });
    ({
        let _r: Ptr<Deleter_A_> = t.as_pointer();
        Deleter_A_Impl::move_assign(&y, _r)
    });
}
pub fn swap_5(x: Ptr<Deleter_Aarrarr_>, y: Ptr<Deleter_Aarrarr_>) {
    let t: Value<Deleter_Aarrarr_> =
        Rc::new(RefCell::new(Deleter_Aarrarr_::move_from({ (x).clone() })));
    let _dtor_t = ScopedDestructor::new(&t, |__p| __p.destructor());
    ({
        let _r: Ptr<Deleter_Aarrarr_> = (y).clone();
        Deleter_Aarrarr_Impl::move_assign(&x, _r)
    });
    ({
        let _r: Ptr<Deleter_Aarrarr_> = t.as_pointer();
        Deleter_Aarrarr_Impl::move_assign(&y, _r)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_deleter_base {}
thread_local!(
    pub static count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_7: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn operator_eq_9(x: Ptr<A>, y: Ptr<A>) -> () {
    return ({ x.with(|__s| __s.state_) } == { y.with(|__s| __s.state_) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub state_: (),
}
impl A {
    pub fn new_1() -> Self {
        let __this: A = Self { state_: 0_() };
        {
            (*count_8.with(Value::clone).borrow_mut()).prefix_inc();
        }
        __this
    }
    pub fn new_2(mut i: ()) -> Self {
        let __this: A = Self { state_: i };
        {
            (*count_8.with(Value::clone).borrow_mut()).prefix_inc();
        }
        __this
    }
    pub fn copy_from(a: Ptr<A>) -> Self {
        let __this: A = Self {
            state_: a.with(|__s| __s.state_),
        };
        {
            (*count_8.with(Value::clone).borrow_mut()).prefix_inc();
        }
        __this
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_9(
                Rc::new(RefCell::new(A {
                    state_: self.state_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(A {
                    state_: other.state_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for A {}
impl Clone for A {
    fn clone(&self) -> Self {
        let __src: Value<A> = Rc::new(RefCell::new(A {
            state_: self.state_.clone(),
        }));
        A::copy_from(__src.as_pointer())
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new_1() }
    }
}
thread_local!(
    pub static count_8: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct NonSwappableDeleter_int_ {}
impl NonSwappableDeleter_int_ {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
pub fn test_10() -> () {
    {
        let mut p1: Ptr<A> = Ptr::alloc(A::new_2({ 1_() }));
        let s1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut p2: Ptr<A> = Ptr::alloc(A::new_2({ 2_() }));
        let s2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        }
    }
    {
        unimplemented!();
    }
    {
        let mut p1: Ptr<A> =
            Ptr::alloc_array((0..3_()).map(|_| <A>::default()).collect::<Box<[A]>>());
        let mut s1: () = unimplemented!();
        let mut p2: Ptr<A> =
            Ptr::alloc_array((0..3_()).map(|_| <A>::default()).collect::<Box<[A]>>());
        let mut s2: () = unimplemented!();
        unimplemented!();
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
        }
    }
    {
        unimplemented!();
    }
    {
        let mut d: NonSwappableDeleter_int_ = NonSwappableDeleter_int_::new({ 42_() });
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let y: Value<()> = Rc::new(RefCell::new(43_()));
        let mut p: () = unimplemented!();
        let mut p2: () = unimplemented!();
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
    ({ test_10() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait AImpl {
    fn copy_assign(&self, a: Ptr<A>) -> Ptr<A>;
    fn destructor(&self);
}
impl AImpl for Ptr<A> {
    fn copy_assign(&self, a: Ptr<A>) -> Ptr<A> {
        field!((*self), state_).write({ a.with(|__s| __s.state_) });
        return (*self).clone();
    }
    fn destructor(&self) {
        {
            (*count_8.with(Value::clone).borrow_mut()).prefix_dec();
        }
    }
}
pub trait Deleter_A_Impl {
    fn copy_assign(&self, _a0: Ptr<Deleter_A_>) -> Ptr<Deleter_A_> {
        unimplemented!()
    }
    fn move_assign(&self, r: Ptr<Deleter_A_>) -> Ptr<Deleter_A_>;
    fn destructor(&self);
    fn state(&self) -> ();
    fn set_state(&self, mut i: ()) {
        unimplemented!()
    }
    fn operator_call(&self, p: Ptr<A>);
}
impl Deleter_A_Impl for Ptr<Deleter_A_> {
    fn move_assign(&self, r: Ptr<Deleter_A_>) -> Ptr<Deleter_A_> {
        field!((*self), state_).write({ r.with(|__s| __s.state_) });
        field!(r, state_).write(0_());
        return (*self).clone();
    }
    fn destructor(&self) {
        unimplemented!();
        field!((*self), state_).write(-1_());
    }
    fn state(&self) -> () {
        return (*self).with(|__s| __s.state_);
    }
    fn operator_call(&self, mut p: Ptr<A>) {
        {
            let __p = p;
            __p.destructor();
            __p.delete();
        };
    }
}
pub trait Deleter_Aarrarr_Impl {
    fn copy_assign(&self, _a0: Ptr<Deleter_Aarrarr_>) -> Ptr<Deleter_Aarrarr_> {
        unimplemented!()
    }
    fn move_assign(&self, r: Ptr<Deleter_Aarrarr_>) -> Ptr<Deleter_Aarrarr_>;
    fn destructor(&self);
    fn state(&self) -> ();
    fn set_state(&self, mut i: ()) {
        unimplemented!()
    }
    fn operator_call(&self, p: Ptr<A>);
}
impl Deleter_Aarrarr_Impl for Ptr<Deleter_Aarrarr_> {
    fn move_assign(&self, r: Ptr<Deleter_Aarrarr_>) -> Ptr<Deleter_Aarrarr_> {
        field!((*self), state_).write({ r.with(|__s| __s.state_) });
        field!(r, state_).write(0_());
        return (*self).clone();
    }
    fn destructor(&self) {
        unimplemented!();
        field!((*self), state_).write(-1_());
    }
    fn state(&self) -> () {
        return (*self).with(|__s| __s.state_);
    }
    fn operator_call(&self, mut p: Ptr<A>) {
        {
            let __p = p;
            for __i in 0..__p.len() {
                __p.offset(__i as isize).destructor();
            }
            __p.delete();
        };
    }
}
pub trait NonSwappableDeleter_int_Impl {
    fn copy_assign(&self, _a0: Ptr<NonSwappableDeleter_int_>) -> Ptr<NonSwappableDeleter_int_>;
    fn operator_call(&self, _a0: Ptr<()>);
}
impl NonSwappableDeleter_int_Impl for Ptr<NonSwappableDeleter_int_> {
    fn copy_assign(&self, _a0: Ptr<NonSwappableDeleter_int_>) -> Ptr<NonSwappableDeleter_int_> {
        return (*self).clone();
    }
    fn operator_call(&self, mut _a0: Ptr<()>) {}
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = count_8.with(|_| ());
}
