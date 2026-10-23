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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_deleter_base {}
thread_local!(
    pub static count_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct test_deleter_A_ {
    #[offset(0)]
    state_: (),
}
impl test_deleter_A_ {
    pub fn new_1(mut s: ()) -> Self {
        let __this: test_deleter_A_ = Self { state_: s };
        (*count_4.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(d: Ptr<test_deleter_A_>) -> Self {
        let __this: test_deleter_A_ = Self {
            state_: d.with(|__s| __s.state_),
        };
        (*count_4.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for test_deleter_A_ {
    fn clone(&self) -> Self {
        let __src: Value<test_deleter_A_> = Rc::new(RefCell::new(test_deleter_A_ {
            state_: self.state_.clone(),
        }));
        test_deleter_A_::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct A {}
impl A {
    pub fn new() -> Self {
        let __this: A = Self {};
        (*count_6.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<A>) -> Self {
        let __this: A = Self {};
        (*count_6.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        let __src: Value<A> = Rc::new(RefCell::new(A {}));
        A::copy_from(__src.as_pointer())
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
thread_local!(
    pub static count_6: Value<()> = Rc::new(RefCell::new(0_()));
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
    {
        {
            let mut ptr: Ptr<A> = Ptr::alloc(A::new());
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut d: Ptr<test_deleter_A_> = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
    {
        {
            let p: Value<()> = Rc::new(RefCell::new(std_shared_ptr_A_::new_1(
                { Default::default() },
                { test_deleter_A_::new_1({ 3_() }) },
                None,
            )));
            let mut d: Ptr<test_deleter_A_> = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
    {
        let p: Value<()> = Rc::new(RefCell::new(std_shared_ptr_A_::new_1(
            { Default::default() },
            { test_deleter_A_::new_1({ 3_() }) },
            None,
        )));
        let mut d: Ptr<()> = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait AImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        (*count_6.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        return (*self).clone();
    }
}
pub trait test_deleter_A_Impl {
    fn destructor(&self);
    fn state(&self) -> ();
    fn set_state(&self, mut i: ()) {
        unimplemented!()
    }
    fn operator_call(&self, p: Ptr<A>);
}
impl test_deleter_A_Impl for Ptr<test_deleter_A_> {
    fn destructor(&self) {
        unimplemented!();
        (*count_4.with(Value::clone).borrow_mut()).prefix_dec();
        field!((*self), state_).write(-1_());
    }
    fn state(&self) -> () {
        return (*self).with(|__s| __s.state_);
    }
    fn operator_call(&self, mut p: Ptr<A>) {
        unimplemented!();
        (*dealloc_count_5.with(Value::clone).borrow_mut()).prefix_inc();
        {
            let __p = p;
            __p.destructor();
            __p.delete();
        };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = count_6.with(|_| ());
}
