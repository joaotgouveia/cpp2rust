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
pub struct A {}
impl A {
    pub fn new() -> Self {
        let __this: A = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<A>) -> Self {
        let __this: A = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
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
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
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
        let mut ptr1: Ptr<A> = Ptr::alloc(A::new());
        let mut ptr2: Ptr<A> = Ptr::alloc(A::new());
        let p1: Value<std_shared_ptr_A_> =
            Rc::new(RefCell::new(std_shared_ptr_A_::new_1({ (ptr1).clone() })));
        let w1: Value<std_weak_ptr_A_> =
            Rc::new(RefCell::new(std_weak_ptr_A_::new_1({ p1.as_pointer() })));
        {
            let p2: Value<std_shared_ptr_A_> =
                Rc::new(RefCell::new(std_shared_ptr_A_::new_1({ (ptr2).clone() })));
            let w2: Value<std_weak_ptr_A_> =
                Rc::new(RefCell::new(std_weak_ptr_A_::new_1({ p2.as_pointer() })));
            ({ ((*w1.borrow()) as std___weak_ptr_A____gnu_cxx__S_atomic_).swap(w2.as_pointer()) });
            ({
                cpp2rust_assert_fail_1(
                    (({ ((*w1.borrow()) as std___weak_ptr_A____gnu_cxx__S_atomic_).use_count() })
                        == 1_i64),
                )
            });
            ({
                cpp2rust_assert_fail_1(
                    ({
                        ({
                            (({ (*w1.borrow()).lock() })
                                as std___shared_ptr_A____gnu_cxx__S_atomic_)
                                .get()
                        })
                    } == { (ptr2).clone() }),
                )
            });
            ({
                cpp2rust_assert_fail_1(
                    (({ ((*w2.borrow()) as std___weak_ptr_A____gnu_cxx__S_atomic_).use_count() })
                        == 1_i64),
                )
            });
            ({
                cpp2rust_assert_fail_1(
                    ({
                        ({
                            (({ (*w2.borrow()).lock() })
                                as std___shared_ptr_A____gnu_cxx__S_atomic_)
                                .get()
                        })
                    } == { (ptr1).clone() }),
                )
            });
            ({ cpp2rust_assert_fail_1((count_0.with(|rc| *rc.borrow()) == 2)) });
        }
    }
    ({ cpp2rust_assert_fail_1((count_0.with(|rc| *rc.borrow()) == 0)) });
    return 0;
}
pub trait AImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
