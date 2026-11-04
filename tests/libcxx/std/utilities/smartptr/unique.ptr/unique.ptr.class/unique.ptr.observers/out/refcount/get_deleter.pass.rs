extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_Deleter_ref__const_Deleter_ref_ {}
impl CppMove for test_macros_detail_is_same_const_Deleter_ref__const_Deleter_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_Deleter_ref__Deleter_ref_ {}
impl CppMove for test_macros_detail_is_same_Deleter_ref__Deleter_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Deleter {}
impl Deleter {
    pub fn new() -> Self {
        Self {}
    }
}
impl Callable1<AnyPtr, ()> for Deleter {
    fn call(&self, a1: AnyPtr) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            DeleterImpl::operator_call(__it, a1)
        })
    }
}
impl CppMove for Deleter {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Deleter {
    fn default() -> Self {
        { Deleter::new() }
    }
}
pub fn test_basic_2() {
    {
        let p: Value<Option<Value<i32>>> = Rc::new(RefCell::new(None));
        assert!(({ (({ DeleterImpl::test_3(&({ (*p.borrow()).get_deleter_17() }),) }) == 5) }));
    }
    {
        let p: Value<Option<Value<i32>>> = Rc::new(RefCell::new(None));
        assert!(({ (({ DeleterImpl::test_4(&({ (*p.borrow()).get_deleter_18() }),) }) == 6) }));
    }
    {
        let d: Value<Deleter> = Rc::new(RefCell::new(Deleter::new()));
        let p: Value<Option<Value<i32>>> = Rc::new(RefCell::new(
            std_unique_ptr_int__const_Deleter_ref_::new_1({ Ptr::null() }, { d.as_pointer() }),
        ));
        let cp: Ptr<Option<Value<i32>>> = p.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(({ (({ DeleterImpl::test_4(&({ (*p.borrow()).get_deleter_17() }),) }) == 6) }));
        assert!(
            ({
                (({ DeleterImpl::test_4(&({ (*cp.upgrade().deref()).get_deleter_18() })) }) == 6)
            })
        );
    }
    {
        let d: Value<Deleter> = Rc::new(RefCell::new(Deleter::new()));
        let p: Value<Option<Value<i32>>> = Rc::new(RefCell::new(
            std_unique_ptr_int__Deleter_ref_::new_1({ Ptr::null() }, { d.as_pointer() }),
        ));
        let cp: Ptr<Option<Value<i32>>> = p.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(({ (({ DeleterImpl::test_3(&({ (*p.borrow()).get_deleter_17() }),) }) == 5) }));
        assert!(
            ({
                (({ DeleterImpl::test_3(&({ (*cp.upgrade().deref()).get_deleter_18() })) }) == 5)
            })
        );
    }
}
pub fn test_basic_3() {
    {
        let p: Value<Option<Value<i32>>> = Rc::new(RefCell::new(None));
        assert!(({ (({ DeleterImpl::test_3(&({ (*p.borrow()).get_deleter_17() }),) }) == 5) }));
    }
    {
        let p: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(None));
        assert!(({ (({ DeleterImpl::test_4(&({ (*p.borrow()).get_deleter_16() }),) }) == 6) }));
    }
    {
        let d: Value<Deleter> = Rc::new(RefCell::new(Deleter::new()));
        let p: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
            std_unique_ptr_intarrarr__const_Deleter_ref_::new_1({ Default::default() }, {
                d.as_pointer()
            }),
        ));
        let cp: Ptr<Option<Value<Box<[i32]>>>> = p.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(({ (({ DeleterImpl::test_4(&({ (*p.borrow()).get_deleter_15() }),) }) == 6) }));
        assert!(
            ({
                (({ DeleterImpl::test_4(&({ (*cp.upgrade().deref()).get_deleter_16() })) }) == 6)
            })
        );
    }
    {
        let d: Value<Deleter> = Rc::new(RefCell::new(Deleter::new()));
        let p: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
            std_unique_ptr_intarrarr__Deleter_ref_::new_1({ Default::default() }, {
                d.as_pointer()
            }),
        ));
        let cp: Ptr<Option<Value<Box<[i32]>>>> = p.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(({ (({ DeleterImpl::test_3(&({ (*p.borrow()).get_deleter_15() }),) }) == 5) }));
        assert!(
            ({
                (({ DeleterImpl::test_3(&({ (*cp.upgrade().deref()).get_deleter_16() })) }) == 5)
            })
        );
    }
}
pub fn test_4() -> bool {
    ({ test_basic_2() });
    ({ test_basic_3() });
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
    ({ test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait DeleterImpl {
    fn operator_call(&self, _a0: AnyPtr);
    fn test_3(&self) -> i32;
    fn test_4(&self) -> i32;
}
impl DeleterImpl for Ptr<Deleter> {
    fn operator_call(&self, mut _a0: AnyPtr) {}
    fn test_3(&self) -> i32 {
        return 5;
    }
    fn test_4(&self) -> i32 {
        return 6;
    }
}
pub fn __cpp2rust_init_globals() {}
