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
pub struct min_allocator_B_ {}
impl CppMove for min_allocator_B_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct A {
    #[offset(0)]
    #[byte_size(8)]
    pub d: std_forward_list_A__std_allocator_A__,
    #[offset(8)]
    #[byte_size(8)]
    pub it: std__Fwd_list_iterator_A_,
    #[offset(16)]
    #[byte_size(8)]
    pub it2: std__Fwd_list_const_iterator_A_,
}
impl A {
    pub fn move_from(_a0: Ptr<A>) -> Self {
        Self {
            d: field!(_a0, d).with_mut(std::mem::take),
            it: field!(_a0, it).with_mut(std::mem::take),
            it2: field!(_a0, it2).with_mut(std::mem::take),
        }
    }
}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct B {
    #[offset(0)]
    #[byte_size(8)]
    pub d: std_forward_list_B__min_allocator_B__,
    #[offset(8)]
    #[byte_size(8)]
    pub it: std___fwdlist__Iterator_false__min_pointer_B__std_integral_constant_unsigned_long__0___,
    #[offset(16)]
    #[byte_size(8)]
    pub it2: std___fwdlist__Iterator_true__min_pointer_B__std_integral_constant_unsigned_long__0___,
}
impl B {
    pub fn move_from(_a0: Ptr<B>) -> Self {
        Self {
            d: field!(_a0, d).with_mut(std::mem::take),
            it: { (*_a0.upgrade().deref()).it.clone() },
            it2: { (*_a0.upgrade().deref()).it2.clone() },
        }
    }
}
impl CppMove for B {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn test_4() -> bool {
    {
        let a: Value<A> = Rc::new(RefCell::new(<A>::default()));
        assert!(({ ({ (*a.borrow()).d.empty() }) }));
        {
            let __a1 = ({ (*a.borrow()).d.begin_25() });
            field_ptr!(a.as_pointer(), it).with_mut(|__it| *__it = __a1);
            field_ptr!(a.as_pointer(), it)
        };
        {
            let __a1 = ({ (*a.borrow()).d.cbefore_begin() });
            field_ptr!(a.as_pointer(), it2).with_mut(|__it| *__it = __a1);
            field_ptr!(a.as_pointer(), it2)
        };
    }
    {
        let b: Value<B> = Rc::new(RefCell::new(<B>::default()));
        assert!(({ ({ (*b.borrow()).d.empty() }) }));
        {
            let __a1 = ({ (*b.borrow()).d.begin_25() });
            field_ptr!(b.as_pointer(), it).with_mut(|__it| *__it = __a1.clone());
            field_ptr!(b.as_pointer(), it)
        };
        {
            let __a1 = ({ (*b.borrow()).d.cbefore_begin() });
            field_ptr!(b.as_pointer(), it2).with_mut(|__it| *__it = __a1.clone());
            field_ptr!(b.as_pointer(), it2)
        };
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    assert!(({ test_4() }));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_B__std_integral_constant_unsigned_long__0__;
pub trait AImpl {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        field!((*self), d).write({ { (*_a0.upgrade().deref()).d.clone() } });
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).it.clone() } });
            field_ptr!((*self), it).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), it)
        };
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).it2.clone() } });
            field_ptr!((*self), it2).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), it2)
        };
        return (*self).clone();
    }
}
pub trait BImpl {
    fn move_assign(&self, _a0: Ptr<B>) -> Ptr<B>;
}
impl BImpl for Ptr<B> {
    fn move_assign(&self, _a0: Ptr<B>) -> Ptr<B> {
        field!((*self), d).write({ { (*_a0.upgrade().deref()).d.clone() } });
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).it.clone() } });
            field_ptr!((*self), it).with_mut(|__it| *__it = __a1.clone());
            field_ptr!((*self), it)
        };
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).it2.clone() } });
            field_ptr!((*self), it2).with_mut(|__it| *__it = __a1.clone());
            field_ptr!((*self), it2)
        };
        return (*self).clone();
    }
}
pub trait min_allocator_B_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_B__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_B__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_B_Impl for Ptr<min_allocator_B_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
