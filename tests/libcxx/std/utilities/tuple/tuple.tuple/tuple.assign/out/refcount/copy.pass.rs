extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonAssignable {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyAssignable {}
const _: () = assert!(true, "std::is_copy_assignable<CopyAssignable>::value");
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct MoveAssignable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct NothrowCopyAssignable {}
impl CppMove for NothrowCopyAssignable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct PotentiallyThrowingCopyAssignable {}
impl CppMove for PotentiallyThrowingCopyAssignable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CopyAssignableInt {}
impl CppMove for CopyAssignableInt {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() -> bool {
    {
        let t0: Value<std_tuple__> = Rc::new(RefCell::new(<std_tuple__>::default()));
        let t: Value<std_tuple__> = Rc::new(RefCell::new(<std_tuple__>::default()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
    }
    {
        let t0: Value<std_tuple_int_> = Rc::new(RefCell::new(2));
        let t: Value<std_tuple_int_> = Rc::new(RefCell::new(<std_tuple_int_>::default()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        assert!(((({ libcc2rs::get_refcount(t.as_pointer(),) }).read()) == 2));
    }
    {
        let t0: Value<std_tuple_int__char_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(2));
            let __tmp_1: Value<i8> = Rc::new(RefCell::new(('a' as i8)));
            std_tuple_int__char_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        let t: Value<std_tuple_int__char_> =
            Rc::new(RefCell::new(<std_tuple_int__char_>::default()));
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        assert!(((({ libcc2rs::get_refcount(t.as_pointer(),) }).read()) == 2));
        assert!(
            (((({ libcc2rs::get_refcount(t.as_pointer(),) }).read()) as i32)
                == (('a' as i8) as i32))
        );
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(42));
        let y: Value<i32> = Rc::new(RefCell::new(100));
        let x2: Value<i32> = Rc::new(RefCell::new(-1_i32));
        let y2: Value<i32> = Rc::new(RefCell::new(500));
        let t: Value<std_tuple_int_ref__int_refref_> = Rc::new(RefCell::new(
            std_tuple_int_ref__int_refref_::new_1({ x.as_pointer() }, { y.as_pointer() }),
        ));
        let t2: Value<std_tuple_int_ref__int_refref_> = Rc::new(RefCell::new(
            std_tuple_int_ref__int_refref_::new_1({ x2.as_pointer() }, { y2.as_pointer() }),
        ));
        (*t.borrow_mut()) = (*t2.borrow()).clone();
        assert!(({ ((({ libcc2rs::get_refcount(t.as_pointer(),) }).read()) == (*x2.borrow())) }));
        assert!((({ libcc2rs::get_refcount(t.as_pointer(),) }) == (x.as_pointer())));
        assert!(({ ((({ libcc2rs::get_refcount(t.as_pointer(),) }).read()) == (*y2.borrow())) }));
        assert!((({ libcc2rs::get_refcount(t.as_pointer(),) }) == (y.as_pointer())));
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t0 : Value<std_tuple_int__char__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new({ let __tmp_2 : Value<i32 > = Rc::new(RefCell::new(2 ));let __tmp_3 : Value<i8 > = Rc::new(RefCell::new(( 'a' as i8  ) )); std_tuple_int__char__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_2.as_pointer() } , { __tmp_3.as_pointer() } , { Ptr::<i8 >::from_string_literal(b"some text" ) } , )  } )) ;
        let t : Value<std_tuple_int__char__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new(<std_tuple_int__char__std_basic_string_char__std_char_traits_char___std_allocator_char___  >::default() )) ;
        (*t.borrow_mut()) = (*t0.borrow()).clone();
        assert!(((({ libcc2rs::get_refcount(t.as_pointer(),) }).read()) == 2));
        assert!(
            (((({ libcc2rs::get_refcount(t.as_pointer(),) }).read()) as i32)
                == (('a' as i8) as i32))
        );
        assert!(
            ({
                match (({ libcc2rs::get_refcount(t.as_pointer()) }),) {
                    (__tmp_4,) => Ptr::<i8>::from_string_literal(b"some text").with_c_str(|__s| {
                        (*__tmp_4.upgrade().deref())
                            [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P const>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P&>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<T>::value");;
        const _: () = assert!(true, "!std::is_nothrow_copy_assignable<T>::value");;
    }
    return 0;
}
pub trait CopyAssignableIntImpl {
    fn operator_assign_1(&self, _a0: Ptr<i32>) -> Ptr<CopyAssignableInt>;
}
impl CopyAssignableIntImpl for Ptr<CopyAssignableInt> {
    fn operator_assign_1(&self, _a0: Ptr<i32>) -> Ptr<CopyAssignableInt> {
        return (*self).clone();
    }
}
pub trait MoveAssignableImpl {
    fn move_assign(&self, _a0: Ptr<MoveAssignable>) -> Ptr<MoveAssignable>;
}
impl MoveAssignableImpl for Ptr<MoveAssignable> {
    fn move_assign(&self, _a0: Ptr<MoveAssignable>) -> Ptr<MoveAssignable> {
        return (*self).clone();
    }
}
pub trait NothrowCopyAssignableImpl {
    fn copy_assign(&self, _a0: Ptr<NothrowCopyAssignable>) -> Ptr<NothrowCopyAssignable>;
}
impl NothrowCopyAssignableImpl for Ptr<NothrowCopyAssignable> {
    fn copy_assign(&self, _a0: Ptr<NothrowCopyAssignable>) -> Ptr<NothrowCopyAssignable> {
        return (*self).clone();
    }
}
pub trait PotentiallyThrowingCopyAssignableImpl {
    fn copy_assign(
        &self,
        _a0: Ptr<PotentiallyThrowingCopyAssignable>,
    ) -> Ptr<PotentiallyThrowingCopyAssignable>;
}
impl PotentiallyThrowingCopyAssignableImpl for Ptr<PotentiallyThrowingCopyAssignable> {
    fn copy_assign(
        &self,
        _a0: Ptr<PotentiallyThrowingCopyAssignable>,
    ) -> Ptr<PotentiallyThrowingCopyAssignable> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
