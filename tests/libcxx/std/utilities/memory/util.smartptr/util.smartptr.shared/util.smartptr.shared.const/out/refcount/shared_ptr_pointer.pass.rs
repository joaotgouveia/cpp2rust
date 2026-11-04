extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct B {}
impl B {
    pub fn new() -> Self {
        let __this: B = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<B>) -> Self {
        let __this: B = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for B {
    fn clone(&self) -> Self {
        let __src: Value<B> = Rc::new(RefCell::new(B {}));
        B::copy_from(__src.as_pointer())
    }
}
impl CppMove for B {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for B {
    fn default() -> Self {
        { B::new() }
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct A {}
impl A {
    pub fn new() -> Self {
        let __this: A = Self {};
        (*count_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<A>) -> Self {
        let __this: A = Self {};
        (*count_1.with(Value::clone).borrow_mut()).prefix_inc();
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
    pub static count_1: Value<i32> = Rc::new(RefCell::new(0));
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
        let pA: Value<std_shared_ptr_A_> = Rc::new(RefCell::new(std_shared_ptr_A_::new_1({
            Ptr::alloc(A::new())
        })));
        ({
            cpp2rust_assert_fail_2(
                (({ ((*pA.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).use_count() })
                    == 1_i64),
            )
        });
        {
            let b: Value<B> = Rc::new(RefCell::new(B::new()));
            let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
            let pB: Value<std_shared_ptr_B_> = Rc::new(RefCell::new(std_shared_ptr_B_::new_1(
                { pA.as_pointer() },
                { (b.as_pointer()) },
            )));
            ({ cpp2rust_assert_fail_2((count_1.with(|rc| *rc.borrow()) == 1)) });
            ({ cpp2rust_assert_fail_2((count_0.with(|rc| *rc.borrow()) == 1)) });
            ({
                cpp2rust_assert_fail_2(
                    (({
                        ((*pA.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).use_count()
                    }) == 2_i64),
                )
            });
            ({
                cpp2rust_assert_fail_2(
                    (({
                        ((*pB.borrow()) as std___shared_ptr_B____gnu_cxx__S_atomic_).use_count()
                    }) == 2_i64),
                )
            });
            ({
                cpp2rust_assert_fail_2(
                    (({ ((*pB.borrow()) as std___shared_ptr_B____gnu_cxx__S_atomic_).get() })
                        == (b.as_pointer())),
                )
            });
        }
        ({
            cpp2rust_assert_fail_2(
                (({ ((*pA.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).use_count() })
                    == 1_i64),
            )
        });
        ({ cpp2rust_assert_fail_2((count_1.with(|rc| *rc.borrow()) == 1)) });
        ({ cpp2rust_assert_fail_2((count_0.with(|rc| *rc.borrow()) == 0)) });
    }
    ({ cpp2rust_assert_fail_2((count_1.with(|rc| *rc.borrow()) == 0)) });
    ({ cpp2rust_assert_fail_2((count_0.with(|rc| *rc.borrow()) == 0)) });
    {
        let pA: Value<std_shared_ptr_const_A_> =
            Rc::new(RefCell::new(std_shared_ptr_const_A_::new_1({
                Ptr::alloc(A::new())
            })));
        ({
            cpp2rust_assert_fail_2(
                (({
                    ((*pA.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).use_count()
                }) == 1_i64),
            )
        });
        {
            let b: Value<B> = Rc::new(RefCell::new(B::new()));
            let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
            let pB: Value<std_shared_ptr_const_B_> = Rc::new(RefCell::new(
                std_shared_ptr_const_B_::new_1({ pA.as_pointer() }, { (b.as_pointer()) }),
            ));
            ({ cpp2rust_assert_fail_2((count_1.with(|rc| *rc.borrow()) == 1)) });
            ({ cpp2rust_assert_fail_2((count_0.with(|rc| *rc.borrow()) == 1)) });
            ({
                cpp2rust_assert_fail_2(
                    (({
                        ((*pA.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_)
                            .use_count()
                    }) == 2_i64),
                )
            });
            ({
                cpp2rust_assert_fail_2(
                    (({
                        ((*pB.borrow()) as std___shared_ptr_const_B____gnu_cxx__S_atomic_)
                            .use_count()
                    }) == 2_i64),
                )
            });
            ({
                cpp2rust_assert_fail_2(
                    (({
                        ((*pB.borrow()) as std___shared_ptr_const_B____gnu_cxx__S_atomic_).get()
                    }) == (b.as_pointer())),
                )
            });
        }
        ({
            cpp2rust_assert_fail_2(
                (({
                    ((*pA.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).use_count()
                }) == 1_i64),
            )
        });
        ({ cpp2rust_assert_fail_2((count_1.with(|rc| *rc.borrow()) == 1)) });
        ({ cpp2rust_assert_fail_2((count_0.with(|rc| *rc.borrow()) == 0)) });
    }
    ({ cpp2rust_assert_fail_2((count_1.with(|rc| *rc.borrow()) == 0)) });
    ({ cpp2rust_assert_fail_2((count_0.with(|rc| *rc.borrow()) == 0)) });
    let mut pi: Ptr<i32> = Ptr::alloc(Default::default());
    {
        let p1: Value<std_shared_ptr_int_> = Rc::new(RefCell::new(std_shared_ptr_int_::new_1({
            Default::default()
        })));
        let p2: Value<std_shared_ptr_int_> = Rc::new(RefCell::new(std_shared_ptr_int_::new_2(
            { p1.as_pointer() },
            { (pi).clone() },
        )));
        ({
            cpp2rust_assert_fail_2(
                ({ ({ ((*p2.borrow()) as std___shared_ptr_int____gnu_cxx__S_atomic_).get() }) }
                    == { (pi).clone() }),
            )
        });
    }
    pi.delete();
    {
        let p1: Value<std_shared_ptr_int_> = Rc::new(RefCell::new(std_shared_ptr_int_::new_3({
            Ptr::alloc(Default::default())
        })));
        let p2: Value<std_shared_ptr_int_> = Rc::new(RefCell::new(std_shared_ptr_int_::new_2(
            { p1.as_pointer() },
            { Ptr::<i32>::null() },
        )));
        ({
            cpp2rust_assert_fail_2(
                ({ ((*p2.borrow()) as std___shared_ptr_int____gnu_cxx__S_atomic_).get() })
                    .is_null(),
            )
        });
    }
    return 0;
}
pub trait AImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        (*count_1.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        return (*self).clone();
    }
}
pub trait BImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<B>) -> Ptr<B>;
}
impl BImpl for Ptr<B> {
    fn destructor(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<B>) -> Ptr<B> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = count_1.with(|_| ());
}
