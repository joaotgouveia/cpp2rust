extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct ExplicitT {
    #[offset(0)]
    pub value: i32,
}
impl ExplicitT {
    pub fn new(mut x: i32) -> Self {
        Self { value: x }
    }
    pub fn copy_from(o: Ptr<ExplicitT>) -> Self {
        Self {
            value: o.with(|__s| __s.value),
        }
    }
}
impl Clone for ExplicitT {
    fn clone(&self) -> Self {
        let __src: Value<ExplicitT> = Rc::new(RefCell::new(ExplicitT {
            value: self.value.clone(),
        }));
        ExplicitT::copy_from(__src.as_pointer())
    }
}
impl CppMove for ExplicitT {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct ImplicitT {
    #[offset(0)]
    pub value: i32,
}
impl ImplicitT {
    pub fn new(mut x: i32) -> Self {
        Self { value: x }
    }
    pub fn copy_from(o: Ptr<ImplicitT>) -> Self {
        Self {
            value: o.with(|__s| __s.value),
        }
    }
}
impl Clone for ImplicitT {
    fn clone(&self) -> Self {
        let __src: Value<ImplicitT> = Rc::new(RefCell::new(ImplicitT {
            value: self.value.clone(),
        }));
        ImplicitT::copy_from(__src.as_pointer())
    }
}
impl CppMove for ImplicitT {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn can_construct_with_brace_init_0(__args: Box<[VaArg]>) -> bool {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_construct_with_ctad_brace_init_1(__args: Box<[VaArg]>) -> bool {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
pub fn can_construct_with_ctad_brace_init_2(__args: Box<[VaArg]>) -> bool {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct BraceInit {}
impl CppMove for BraceInit {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoBraceInit {}
impl NoBraceInit {}
impl CppMove for NoBraceInit {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ExplicitBraceInit {}
impl CppMove for ExplicitBraceInit {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn explicit_vs_implicit_brace_init_3(
    _a0: std_pair_ExplicitBraceInit__ExplicitBraceInit_,
) -> i32 {
    let _a0: Value<std_pair_ExplicitBraceInit__ExplicitBraceInit_> = Rc::new(RefCell::new(_a0));
    return 1;
}
pub fn explicit_vs_implicit_brace_init_4(_a0: std_pair_BraceInit__BraceInit_) -> i32 {
    let _a0: Value<std_pair_BraceInit__BraceInit_> = Rc::new(RefCell::new(_a0));
    return 2;
}
pub fn test_5() -> bool {
    {
        let p1: Value<std_pair_ExplicitT__BraceInit_> = Rc::new(RefCell::new({
            let __tmp_0: Value<ExplicitT> = Rc::new(RefCell::new(ExplicitT::new({ 42 })));
            let __tmp_1: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            std_pair_ExplicitT__BraceInit_::new_1({ __tmp_0.as_pointer() }, {
                __tmp_1.as_pointer()
            })
        }));
        ({ cpp2rust_assert_fail_6(({ (*p1.borrow()).first.value } == 42)) });
        let p2: Value<std_pair_ExplicitT__BraceInit_> = Rc::new(RefCell::new({
            let __tmp_2: Value<ExplicitT> = Rc::new(RefCell::new(ExplicitT::new({ 42 })));
            let __tmp_3: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            std_pair_ExplicitT__BraceInit_::new_1({ __tmp_2.as_pointer() }, {
                __tmp_3.as_pointer()
            })
        }));
        ({ cpp2rust_assert_fail_6(({ (*p2.borrow()).first.value } == 42)) });
    }
    {
        let p1: Value<std_pair_BraceInit__ExplicitT_> = Rc::new(RefCell::new({
            let __tmp_4: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            let __tmp_5: Value<ExplicitT> = Rc::new(RefCell::new(ExplicitT::new({ 42 })));
            std_pair_BraceInit__ExplicitT_::new_1({ __tmp_4.as_pointer() }, {
                __tmp_5.as_pointer()
            })
        }));
        ({ cpp2rust_assert_fail_6(({ (*p1.borrow()).second.value } == 42)) });
        let p2: Value<std_pair_BraceInit__ExplicitT_> = Rc::new(RefCell::new({
            let __tmp_6: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            let __tmp_7: Value<ExplicitT> = Rc::new(RefCell::new(ExplicitT::new({ 42 })));
            std_pair_BraceInit__ExplicitT_::new_1({ __tmp_6.as_pointer() }, {
                __tmp_7.as_pointer()
            })
        }));
        ({ cpp2rust_assert_fail_6(({ (*p2.borrow()).second.value } == 42)) });
    }
    {
        let p: Value<std_pair_BraceInit__BraceInit_> = Rc::new(RefCell::new({
            let __tmp_8: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            let __tmp_9: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            std_pair_BraceInit__BraceInit_::new_1({ __tmp_8.as_pointer() }, {
                __tmp_9.as_pointer()
            })
        }));
        &(*p.borrow_mut());
    }
    {
        let p: Value<std_pair_ImplicitT__BraceInit_> = Rc::new(RefCell::new({
            let __tmp_10: Value<i32> = Rc::new(RefCell::new(42));
            let __tmp_11: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            std_pair_ImplicitT__BraceInit_::new_1({ __tmp_10.as_pointer() }, {
                __tmp_11.as_pointer()
            })
        }));
        ({ cpp2rust_assert_fail_6(({ (*p.borrow()).first.value } == 42)) });
    }
    {
        let p: Value<std_pair_BraceInit__ImplicitT_> = Rc::new(RefCell::new({
            let __tmp_12: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            let __tmp_13: Value<i32> = Rc::new(RefCell::new(42));
            std_pair_BraceInit__ImplicitT_::new_1({ __tmp_12.as_pointer() }, {
                __tmp_13.as_pointer()
            })
        }));
        ({ cpp2rust_assert_fail_6(({ (*p.borrow()).second.value } == 42)) });
    }
    {
        let p: Value<std_pair_BraceInit__BraceInit_> = Rc::new(RefCell::new({
            let __tmp_14: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            let __tmp_15: Value<BraceInit> = Rc::new(RefCell::new(<BraceInit>::default()));
            std_pair_BraceInit__BraceInit_::new_1({ __tmp_14.as_pointer() }, {
                __tmp_15.as_pointer()
            })
        }));
        &(*p.borrow_mut());
    }
    {
        const _: () = assert!(true, "can_construct_with_brace_init<BraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_brace_init<NoBraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_ctad_brace_init<BraceInit>(0)");;
        const _: () = assert!(true, "!can_construct_with_ctad_brace_init<int>(0)");;
    }
    {
        ({
            cpp2rust_assert_fail_6(
                (({
                    explicit_vs_implicit_brace_init_4({
                        let __tmp_16: Value<BraceInit> =
                            Rc::new(RefCell::new(<BraceInit>::default()));
                        let __tmp_17: Value<BraceInit> =
                            Rc::new(RefCell::new(<BraceInit>::default()));
                        std_pair_BraceInit__BraceInit_::new_1({ __tmp_16.as_pointer() }, {
                            __tmp_17.as_pointer()
                        })
                    })
                }) == 2),
            )
        });
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
    ({ test_5() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait ExplicitTImpl {
    fn copy_assign(&self, _a0: Ptr<ExplicitT>) -> Ptr<ExplicitT>;
}
impl ExplicitTImpl for Ptr<ExplicitT> {
    fn copy_assign(&self, _a0: Ptr<ExplicitT>) -> Ptr<ExplicitT> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        return (*self).clone();
    }
}
pub trait ImplicitTImpl {
    fn copy_assign(&self, _a0: Ptr<ImplicitT>) -> Ptr<ImplicitT>;
}
impl ImplicitTImpl for Ptr<ImplicitT> {
    fn copy_assign(&self, _a0: Ptr<ImplicitT>) -> Ptr<ImplicitT> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
