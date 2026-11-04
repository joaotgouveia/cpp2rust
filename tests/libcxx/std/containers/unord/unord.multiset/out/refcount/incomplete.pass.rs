extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct MyHash_A_ {}
impl MyHash_A_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl CppMove for MyHash_A_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for MyHash_A_ {
    fn default() -> Self {
        { MyHash_A_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(120)]
pub struct A { #[offset(0)] #[byte_size(56)] pub m : std_unordered_multiset_A__MyHash_A___std_equal_to_A___std_allocator_A__  , #[offset(56)] #[byte_size(8)] pub it : std___detail__Node_iterator_A__true__true_  , #[offset(64)] #[byte_size(8)] pub cit : std___detail__Node_const_iterator_A__true__true_  , #[offset(72)] #[byte_size(24)] pub lit : std___detail__Local_iterator_A__A__std___detail__Identity__MyHash_A___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__true_  , #[offset(96)] #[byte_size(24)] pub clit : std___detail__Local_const_iterator_A__A__std___detail__Identity__MyHash_A___std___detail__Mod_range_hashing__std___detail__Default_ranged_hash__true__true_  , }
impl A {
    pub fn move_from(_a0: Ptr<A>) -> Self {
        Self {
            m: field!(_a0, m).with_mut(std::mem::take),
            it: field!(_a0, it).with_mut(std::mem::take),
            cit: field!(_a0, cit).with_mut(std::mem::take),
            lit: field!(_a0, lit).with_mut(std::mem::take),
            clit: field!(_a0, clit).with_mut(std::mem::take),
        }
    }
}
impl CppEq for A {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_0(L: Ptr<A>, R: Ptr<A>) -> bool {
    return ({ (L).clone() } == { (R).clone() });
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
    let mut a: A = <A>::default();
    return 0;
}
pub trait AImpl {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        field!((*self), m).write({ { (*_a0.upgrade().deref()).m.clone() } });
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).it.clone() } });
            field_ptr!((*self), it).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), it)
        };
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).cit.clone() } });
            field_ptr!((*self), cit).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), cit)
        };
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).lit.clone() } });
            field_ptr!((*self), lit).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), lit)
        };
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).clit.clone() } });
            field_ptr!((*self), clit).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), clit)
        };
        return (*self).clone();
    }
}
pub trait MyHash_A_Impl {
    fn operator_call(&self, _a0: Ptr<A>) -> usize {
        unimplemented!()
    }
}
impl MyHash_A_Impl for Ptr<MyHash_A_> {}
pub fn __cpp2rust_init_globals() {}
