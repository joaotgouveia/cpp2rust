extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(4)]
pub struct NoValueCtor {
    #[offset(0)]
    pub id: i32,
}
impl NoValueCtor {
    pub fn new() -> Self {
        Self {
            id: (*(*count_0.with(Value::clone).borrow_mut()).prefix_inc()),
        }
    }
    pub fn copy_from(other: Ptr<NoValueCtor>) -> Self {
        let __this: NoValueCtor = Self {
            id: other.with(|__s| __s.id),
        };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for NoValueCtor {
    fn clone(&self) -> Self {
        let __src: Value<NoValueCtor> = Rc::new(RefCell::new(NoValueCtor {
            id: self.id.clone(),
        }));
        NoValueCtor::copy_from(__src.as_pointer())
    }
}
impl CppMove for NoValueCtor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for NoValueCtor {
    fn default() -> Self {
        { NoValueCtor::new() }
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(1)]
pub struct NoValueCtorEmpty {}
impl NoValueCtorEmpty {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<NoValueCtorEmpty>) -> Self {
        Self {}
    }
}
impl Clone for NoValueCtorEmpty {
    fn clone(&self) -> Self {
        let __src: Value<NoValueCtorEmpty> = Rc::new(RefCell::new(NoValueCtorEmpty {}));
        NoValueCtorEmpty::copy_from(__src.as_pointer())
    }
}
impl CppMove for NoValueCtorEmpty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for NoValueCtorEmpty {
    fn default() -> Self {
        { NoValueCtorEmpty::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(1)]
pub struct ImplicitCopy {}
impl ImplicitCopy {
    pub fn new(mut _a0: i32) -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<ImplicitCopy>) -> Self {
        Self {}
    }
}
impl Clone for ImplicitCopy {
    fn clone(&self) -> Self {
        let __src: Value<ImplicitCopy> = Rc::new(RefCell::new(ImplicitCopy {}));
        ImplicitCopy::copy_from(__src.as_pointer())
    }
}
impl CppMove for ImplicitCopy {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn testImplicitCopy1_1() -> std_tuple_ImplicitCopy_ {
    let i: Value<ImplicitCopy> = Rc::new(RefCell::new(ImplicitCopy::new({ 42 })));
    return std_tuple_ImplicitCopy_::new_1({ i.as_pointer() });
}
pub fn testImplicitCopy2_2() -> std_tuple_ImplicitCopy_ {
    let i: Value<ImplicitCopy> = Rc::new(RefCell::new(ImplicitCopy::new({ 42 })));
    return std_tuple_ImplicitCopy_::new_2({ i.as_pointer() });
}
pub fn testImplicitCopy3_3() -> std_tuple_ImplicitCopy_ {
    let i: Value<ImplicitCopy> = Rc::new(RefCell::new(ImplicitCopy::new({ 42 })));
    return (*i.borrow_mut()).clone();
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
        let t: Value<std_tuple_int_ptr_> = Rc::new(RefCell::new({
            let __tmp_0: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
            std_tuple_int_ptr_::new_1({ __tmp_0.as_pointer() })
        }));
        assert!((({ get_4(t.as_pointer(),) }).read()).is_null());
    }
    {
        let t: Value<std_tuple_int_> = Rc::new(RefCell::new(2));
        assert!(((({ get_5(t.as_pointer(),) }).read()) == 2));
    }
    {
        let t: Value<std_tuple_int_> = Rc::new(RefCell::new(2));
        const _: () = assert!(true, "std::get<0>(t) == 2");;
    }
    {
        let t: Value<std_tuple_int_> = Rc::new(RefCell::new(<std_tuple_int_>::default()));
        const _: () = assert!(true, "std::get<0>(t) == 0");;
    }
    {
        let t: Value<std_tuple_int__char_ptr_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
            let __tmp_2: Value<Ptr<i8>> = Rc::new(RefCell::new(Ptr::<i8>::null()));
            std_tuple_int__char_ptr_::new_1({ __tmp_1.as_pointer() }, { __tmp_2.as_pointer() })
        }));
        assert!(((({ get_6(t.as_pointer(),) }).read()) == 2));
        assert!((({ get_7(t.as_pointer(),) }).read()).is_null());
    }
    {
        let t: Value<std_tuple_int__char_ptr_> = Rc::new(RefCell::new({
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(2));
            let __tmp_4: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
            std_tuple_int__char_ptr_::new_2({ __tmp_3.as_pointer() }, { __tmp_4.as_pointer() })
        }));
        const _: () = assert!(true, "std::get<0>(t) == 2");;
        const _: () = assert!(true, "std::get<1>(t) == nullptr");;
    }
    {
        let t: Value<std_tuple_int__char_ptr_> = Rc::new(RefCell::new({
            let __tmp_5: Value<i32> = Rc::new(RefCell::new(2));
            let __tmp_6: Value<AnyPtr> = Rc::new(RefCell::new(Default::default()));
            std_tuple_int__char_ptr_::new_2({ __tmp_5.as_pointer() }, { __tmp_6.as_pointer() })
        }));
        assert!(((({ get_6(t.as_pointer(),) }).read()) == 2));
        assert!((({ get_7(t.as_pointer(),) }).read()).is_null());
    }
    {
        let t : Value<std_tuple_int__char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ > = Rc::new(RefCell::new({ let __tmp_7 : Value<i32 > = Rc::new(RefCell::new(2 ));let __tmp_8 : Value<AnyPtr > = Rc::new(RefCell::new(Default::default() )); std_tuple_int__char_ptr__std_basic_string_char__std_char_traits_char___std_allocator_char___ :: new_1 ( { __tmp_7.as_pointer() } , { __tmp_8.as_pointer() } , { Ptr::<i8 >::from_string_literal(b"text" ) } , )  } )) ;
        assert!(((({ get_8(t.as_pointer(),) }).read()) == 2));
        assert!((({ get_9(t.as_pointer(),) }).read()).is_null());
        assert!(
            ({
                match (({ get_10(t.as_pointer()) }),) {
                    (__tmp_9,) => Ptr::<i8>::from_string_literal(b"text").with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    }),
                }
            })
        );
    }
    {
        let t: Value<std_tuple_int__NoValueCtor__int__int_> = Rc::new(RefCell::new({
            let __tmp_10: Value<i32> = Rc::new(RefCell::new(1));
            let __tmp_11: Value<NoValueCtor> = Rc::new(RefCell::new(NoValueCtor::new()));
            let __tmp_12: Value<i32> = Rc::new(RefCell::new(2));
            let __tmp_13: Value<i32> = Rc::new(RefCell::new(3));
            std_tuple_int__NoValueCtor__int__int_::new_1(
                { __tmp_10.as_pointer() },
                { __tmp_11.as_pointer() },
                { __tmp_12.as_pointer() },
                { __tmp_13.as_pointer() },
            )
        }));
        assert!(((({ get_11(t.as_pointer(),) }).read()) == 1));
        assert!((({ get_12(t.as_pointer(),) }).with(|__s| __s.id) == 1));
        assert!(((({ get_13(t.as_pointer(),) }).read()) == 2));
        assert!(((({ get_14(t.as_pointer(),) }).read()) == 3));
    }
    {
        let t: Value<std_tuple_int__NoValueCtorEmpty__int__int_> = Rc::new(RefCell::new({
            let __tmp_14: Value<i32> = Rc::new(RefCell::new(1));
            let __tmp_15: Value<NoValueCtorEmpty> = Rc::new(RefCell::new(NoValueCtorEmpty::new()));
            let __tmp_16: Value<i32> = Rc::new(RefCell::new(2));
            let __tmp_17: Value<i32> = Rc::new(RefCell::new(3));
            std_tuple_int__NoValueCtorEmpty__int__int_::new_1(
                { __tmp_14.as_pointer() },
                { __tmp_15.as_pointer() },
                { __tmp_16.as_pointer() },
                { __tmp_17.as_pointer() },
            )
        }));
        assert!(((({ get_15(t.as_pointer(),) }).read()) == 1));
        assert!(((({ get_16(t.as_pointer(),) }).read()) == 2));
        assert!(((({ get_17(t.as_pointer(),) }).read()) == 3));
    }
    return 0;
}
pub trait ImplicitCopyImpl {
    fn copy_assign(&self, _a0: Ptr<ImplicitCopy>) -> Ptr<ImplicitCopy>;
}
impl ImplicitCopyImpl for Ptr<ImplicitCopy> {
    fn copy_assign(&self, _a0: Ptr<ImplicitCopy>) -> Ptr<ImplicitCopy> {
        return (*self).clone();
    }
}
pub trait NoValueCtorImpl {
    fn copy_assign(&self, _a0: Ptr<NoValueCtor>) -> Ptr<NoValueCtor>;
}
impl NoValueCtorImpl for Ptr<NoValueCtor> {
    fn copy_assign(&self, _a0: Ptr<NoValueCtor>) -> Ptr<NoValueCtor> {
        field!((*self), id).write({ _a0.with(|__s| __s.id) });
        return (*self).clone();
    }
}
pub trait NoValueCtorEmptyImpl {
    fn copy_assign(&self, _a0: Ptr<NoValueCtorEmpty>) -> Ptr<NoValueCtorEmpty>;
}
impl NoValueCtorEmptyImpl for Ptr<NoValueCtorEmpty> {
    fn copy_assign(&self, _a0: Ptr<NoValueCtorEmpty>) -> Ptr<NoValueCtorEmpty> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
