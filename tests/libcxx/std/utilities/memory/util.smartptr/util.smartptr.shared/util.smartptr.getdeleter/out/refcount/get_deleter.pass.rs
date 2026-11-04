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
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_deleter_base {}
impl CppMove for test_deleter_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static count_4: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dealloc_count_5: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct test_deleter_A_ {
    #[offset(0)]
    state_: i32,
}
impl test_deleter_A_ {
    pub fn new_1(mut s: i32) -> Self {
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
impl Callable1<Ptr<A>, ()> for test_deleter_A_ {
    fn call(&self, a1: Ptr<A>) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            test_deleter_A_Impl::operator_call(__it, a1)
        })
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
impl CppMove for test_deleter_A_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new() }
    }
}
thread_local!(
    pub static count_6: Value<i32> = Rc::new(RefCell::new(0));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        {
            let mut ptr: Ptr<A> = Ptr::alloc(A::new());
            let p: Value<std_shared_ptr_A_> =
                Rc::new(RefCell::new(std_shared_ptr_A_::new_1({ (ptr).clone() }, {
                    test_deleter_A_::new_1({ 3 })
                })));
            let mut d: Ptr<test_deleter_A_> = ({ get_deleter_7(p.as_pointer()) });
            assert!((count_4.with(|rc| *rc.borrow()) == 1));
            assert!((dealloc_count_5.with(|rc| *rc.borrow()) == 0));
            assert!(!(d).is_null());
            assert!((({ test_deleter_A_Impl::state(&d,) }) == 3));
        }
        assert!((count_6.with(|rc| *rc.borrow()) == 0));
        assert!((count_4.with(|rc| *rc.borrow()) == 0));
        assert!((dealloc_count_5.with(|rc| *rc.borrow()) == 1));
    }
    dealloc_count_5.with(|rc| *rc.borrow_mut() = 0);
    {
        {
            let p: Value<std_shared_ptr_A_> = Rc::new(RefCell::new(std_shared_ptr_A_::new_2(
                { Default::default() },
                { test_deleter_A_::new_1({ 3 }) },
            )));
            let mut d: Ptr<test_deleter_A_> = ({ get_deleter_7(p.as_pointer()) });
            assert!((count_4.with(|rc| *rc.borrow()) == 1));
            assert!((dealloc_count_5.with(|rc| *rc.borrow()) == 0));
            assert!(!(d).is_null());
            assert!((({ test_deleter_A_Impl::state(&d,) }) == 3));
        }
        assert!((count_6.with(|rc| *rc.borrow()) == 0));
        assert!((count_4.with(|rc| *rc.borrow()) == 0));
        assert!((dealloc_count_5.with(|rc| *rc.borrow()) == 1));
    }
    dealloc_count_5.with(|rc| *rc.borrow_mut() = 0);
    {
        let p: Value<std_shared_ptr_A_> = Rc::new(RefCell::new(std_shared_ptr_A_::new_2(
            { Default::default() },
            { test_deleter_A_::new_1({ 3 }) },
        )));
        let mut d: Ptr<std_default_delete_A_> = ({ get_deleter_8(p.as_pointer()) });
        assert!((d).is_null());
    }
    return 0;
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
    fn state(&self) -> i32;
    fn set_state(&self, mut i: i32) {
        unimplemented!()
    }
    fn operator_call(&self, p: Ptr<A>);
}
impl test_deleter_A_Impl for Ptr<test_deleter_A_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.state_) >= 0));
        (*count_4.with(Value::clone).borrow_mut()).prefix_dec();
        field!((*self), state_).write(-1_i32);
    }
    fn state(&self) -> i32 {
        return (*self).with(|__s| __s.state_);
    }
    fn operator_call(&self, mut p: Ptr<A>) {
        assert!(((*self).with(|__s| __s.state_) >= 0));
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
