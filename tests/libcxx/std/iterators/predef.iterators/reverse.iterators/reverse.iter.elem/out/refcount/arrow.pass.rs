extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct cpp20_bidirectional_iterator_with_arrow_B_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<B>,
}
impl cpp20_bidirectional_iterator_with_arrow_B_ptr_ {
    pub fn new_1(mut it: Ptr<B>) -> Self {
        Self { it_: (it).clone() }
    }
}
pub fn operator_eq_0(x: Ptr<A>, y: Ptr<A>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    data_: (),
}
impl A {
    pub fn new() -> Self {
        Self { data_: 1_() }
    }
}
impl CppEq for A {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
pub fn test_1(mut i: Ptr<A>, x: A) {
    let x: Value<A> = Rc::new(RefCell::new(x));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn operator_eq_2(x: Ptr<B>, y: Ptr<B>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    data_: (),
}
impl B {
    pub fn new(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 1_());
        Self { data_: d }
    }
}
impl CppEq for B {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl Default for B {
    fn default() -> Self {
        { B::new(None) }
    }
}
pub fn operator_eq_3(x: Ptr<C>, y: Ptr<C>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct C {
    #[offset(0)]
    data_: (),
}
impl C {
    pub fn new() -> Self {
        Self { data_: 1_() }
    }
}
impl CppEq for C {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl Default for C {
    fn default() -> Self {
        { C::new() }
    }
}
thread_local!(
    pub static gC_4: Value<C> = Rc::new(RefCell::new(C::new()));
);

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
    let a: Value<A> = Rc::new(RefCell::new(A::new()));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    ({ test_1((a.as_pointer()).offset((1_()) as isize), A::new()) });
    {
        let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let ri: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let it1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "it1->get() == gC.get()");;
    }
    {
        let data: Value<Box<[B]>> = Rc::new(RefCell::new(Box::new([
            B::new({ Some(1_()) }),
            B::new({ Some(2_()) }),
            B::new({ Some(3_()) }),
        ])));
        let mut iter: cpp20_bidirectional_iterator_with_arrow_B_ptr_ =
            cpp20_bidirectional_iterator_with_arrow_B_ptr_::new_1({
                (data.as_pointer() as Ptr<B>).offset((3_()) as isize)
            });
        let ri: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        (&(*gC_4.with(Value::clone).borrow_mut()));
    }
    return 0_();
}
pub trait AImpl {
    fn destructor(&self);
    fn get(&self) -> ();
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        field!((*self), data_).write(-1_());
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
}
pub trait BImpl {
    fn destructor(&self);
    fn get(&self) -> ();
    fn operator_addr_6(&self) -> Ptr<B>;
    fn operator_addr_7(&self) -> Ptr<B>;
}
impl BImpl for Ptr<B> {
    fn destructor(&self) {
        field!((*self), data_).write(-1_());
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_addr_6(&self) -> Ptr<B> {
        return Ptr::<B>::null();
    }
    fn operator_addr_7(&self) -> Ptr<B> {
        return Ptr::<B>::null();
    }
}
pub trait CImpl {
    fn get(&self) -> ();
}
impl CImpl for Ptr<C> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
}
pub trait cpp20_bidirectional_iterator_with_arrow_B_ptr_Impl {
    fn operator_deref(&self) -> Ptr<B>;
    fn operator_arrow(&self) -> Ptr<B>;
    fn operator_inc_5(&self) -> Ptr<cpp20_bidirectional_iterator_with_arrow_B_ptr_> {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<cpp20_bidirectional_iterator_with_arrow_B_ptr_>;
    fn operator_post_inc_7(&self, mut _a0: ()) -> cpp20_bidirectional_iterator_with_arrow_B_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_8(&self, mut _a0: ()) -> cpp20_bidirectional_iterator_with_arrow_B_ptr_ {
        unimplemented!()
    }
}
impl cpp20_bidirectional_iterator_with_arrow_B_ptr_Impl
    for Ptr<cpp20_bidirectional_iterator_with_arrow_B_ptr_>
{
    fn operator_deref(&self) -> Ptr<B> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_arrow(&self) -> Ptr<B> {
        {
            return (*self).with(|__s| __s.it_.clone());
        }
        panic!("ub: non-void function does not return a value")
    }
    fn operator_dec_6(&self) -> Ptr<cpp20_bidirectional_iterator_with_arrow_B_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = gC_4.with(|_| ());
}
