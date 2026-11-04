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
pub struct A {}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct D {}
impl Callable1<Ptr<A>, ()> for D {
    fn call(&self, a1: Ptr<A>) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| DImpl::operator_call(__it, a1))
    }
}
impl CppMove for D {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
        let s0: Value<std_shared_ptr_A_> = Rc::new(RefCell::new(std_shared_ptr_A_::new_1({
            Ptr::alloc(<A>::default())
        })));
        let w: Value<std_weak_ptr_A_> =
            Rc::new(RefCell::new(std_weak_ptr_A_::new_1({ s0.as_pointer() })));
        let s: Value<std_shared_ptr_A_> =
            Rc::new(RefCell::new(std_shared_ptr_A_::new_2({ w.as_pointer() })));
        const _: () = assert!(true, "");;
        assert!(
            ({
                (({ ((*s0.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).use_count() })
                    == 2_i64)
            })
        );
        assert!(
            ({
                (({ ((*s.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).use_count() })
                    == 2_i64)
            })
        );
        assert!(
            ({
                (({ ((*s0.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).get() })
                    == ({ ((*s.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).get() }))
            })
        );
    }
    {
        let s0: Value<std_shared_ptr_const_A_> =
            Rc::new(RefCell::new(std_shared_ptr_const_A_::new_1({
                Ptr::alloc(<A>::default())
            })));
        let w: Value<std_weak_ptr_const_A_> =
            Rc::new(RefCell::new(std_weak_ptr_const_A_::new_1({
                s0.as_pointer()
            })));
        let s: Value<std_shared_ptr_const_A_> =
            Rc::new(RefCell::new(std_shared_ptr_const_A_::new_2({
                w.as_pointer()
            })));
        const _: () = assert!(true, "");;
        assert!(
            ({
                (({
                    ((*s0.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).use_count()
                }) == 2_i64)
            })
        );
        assert!(
            ({
                (({
                    ((*s.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).use_count()
                }) == 2_i64)
            })
        );
        assert!(
            ({
                (({ ((*s0.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).get() })
                    == ({
                        ((*s.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).get()
                    }))
            })
        );
    }
    {
        let u: Value<Option<Value<A>>> =
            Rc::new(RefCell::new(Ptr::alloc(<A>::default()).to_owned_opt()));
        let mut uPointee: Ptr<A> = (*u.borrow()).as_pointer();
        let s: Value<std_shared_ptr_A_> = Rc::new(RefCell::new((*u.borrow_mut()).clone()));
        const _: () = assert!(true, "");;
        assert!(({ (*u.borrow()).is_none() }));
        assert!(
            ({
                ({ ({ ((*s.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).get() }) } == {
                    (uPointee).clone()
                })
            })
        );
    }
    {
        let u: Value<Option<Value<A>>> =
            Rc::new(RefCell::new(Ptr::alloc(<A>::default()).to_owned_opt()));
        let mut uPointee: Ptr<A> = (*u.borrow()).as_pointer();
        let s: Value<std_shared_ptr_const_A_> = Rc::new(RefCell::new((*u.borrow_mut()).clone()));
        const _: () = assert!(true, "");;
        assert!(({ (*u.borrow()).is_none() }));
        assert!(
            ({
                ({ ({ ((*s.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).get() }) }
                    == { (uPointee).clone() })
            })
        );
    }
    {
        let u: Value<Option<Value<A>>> = Rc::new(RefCell::new({
            let __tmp_0: Value<D> = Rc::new(RefCell::new(<D>::default()));
            std_unique_ptr_A__D_::new({ Ptr::alloc(<A>::default()) }, { __tmp_0.as_pointer() })
        }));
        let mut uPointee: Ptr<A> = (*u.borrow()).as_pointer();
        let s: Value<std_shared_ptr_A_> = Rc::new(RefCell::new((*u.borrow_mut()).clone()));
        const _: () = assert!(true, "");;
        assert!(({ (*u.borrow()).is_none() }));
        assert!(
            ({
                ({ ({ ((*s.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).get() }) } == {
                    (uPointee).clone()
                })
            })
        );
    }
    {
        let u: Value<Option<Value<A>>> = Rc::new(RefCell::new({
            let __tmp_1: Value<D> = Rc::new(RefCell::new(<D>::default()));
            std_unique_ptr_const_A__D_::new({ Ptr::alloc(<A>::default()) }, {
                __tmp_1.as_pointer()
            })
        }));
        let mut uPointee: Ptr<A> = (*u.borrow()).as_pointer();
        let s: Value<std_shared_ptr_const_A_> = Rc::new(RefCell::new((*u.borrow_mut()).clone()));
        const _: () = assert!(true, "");;
        assert!(({ (*u.borrow()).is_none() }));
        assert!(
            ({
                ({ ({ ((*s.borrow()) as std___shared_ptr_const_A____gnu_cxx__S_atomic_).get() }) }
                    == { (uPointee).clone() })
            })
        );
    }
    return 0;
}
pub trait DImpl {
    fn operator_call(&self, ptr: Ptr<A>);
}
impl DImpl for Ptr<D> {
    fn operator_call(&self, mut ptr: Ptr<A>) {
        ptr.delete();
    }
}
pub fn __cpp2rust_init_globals() {}
