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
pub struct NonT {
    #[offset(0)]
    pub value: (),
}
impl NonT {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn copy_from(o: Ptr<NonT>) -> Self {
        Self {
            value: o.with(|__s| __s.value),
        }
    }
}
impl Clone for NonT {
    fn clone(&self) -> Self {
        let __src: Value<NonT> = Rc::new(RefCell::new(NonT {
            value: self.value.clone(),
        }));
        NonT::copy_from(__src.as_pointer())
    }
}
const _: () = assert!(true, "!std::is_trivially_copy_constructible<NonT>::value");
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoCopy {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct MoveOnly {}
impl MoveOnly {
    pub fn move_from(_a0: Ptr<MoveOnly>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct MoveOnlyNT {}
impl MoveOnlyNT {
    pub fn move_from(_a0: Ptr<MoveOnlyNT>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NTCopy {
    #[offset(0)]
    pub value: (),
}
impl NTCopy {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn copy_from(that: Ptr<NTCopy>) -> Self {
        Self {
            value: that.with(|__s| __s.value),
        }
    }
}
impl Clone for NTCopy {
    fn clone(&self) -> Self {
        let __src: Value<NTCopy> = Rc::new(RefCell::new(NTCopy {
            value: self.value.clone(),
        }));
        NTCopy::copy_from(__src.as_pointer())
    }
}
const _: () = assert!(true, "!std::is_trivially_copy_constructible<NTCopy>::value");
const _: () = assert!(true, "std::is_copy_constructible<NTCopy>::value");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TCopy {
    #[offset(0)]
    pub value: (),
}
impl TCopy {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
const _: () = assert!(true, "std::is_trivially_copy_constructible<TCopy>::value");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct TCopyNTMove {
    #[offset(0)]
    pub value: (),
}
impl TCopyNTMove {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
    pub fn move_from(that: Ptr<TCopyNTMove>) -> Self {
        let __this: TCopyNTMove = Self {
            value: that.with(|__s| __s.value),
        };
        field!(that, value).write(-1_());
        __this
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_copy_constructible<TCopyNTMove>::value"
);
pub fn test_copy_ctor_sfinae_0() {
    {
        const _: () = assert!(true, "std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_copy_constructible<V>::value");;
        const _: () = assert!(true, "std::is_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_constructible<V>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_copy_constructible<V>::value");;
    }
}
pub fn test_copy_ctor_basic_1() {
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 0");;
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v2.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v2) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 1");;
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v2.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v2) == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 0");;
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v2.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v2).value == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 1");;
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v2.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v2).value == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 0");;
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v2.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v2).value == 42");;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v.index() == 1");;
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "v2.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v2).value == 42");;
    }
}
pub fn test_copy_ctor_valueless_by_exception_2() {}
pub fn test_constexpr_copy_ctor_imp_3(v: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_copy_ctor_imp_4(v: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_copy_ctor_imp_5(v: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_copy_ctor_imp_6(v: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_copy_ctor_imp_7(v: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_copy_ctor_imp_8(v: Ptr<()>) {
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_constexpr_copy_ctor_trivial_9() {
    const _: () = assert!(true, "std::is_trivially_copyable<V>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<V>::value");;
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_constexpr_copy_ctor_imp_3(_v.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_constexpr_copy_ctor_imp_4(_v.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_constexpr_copy_ctor_imp_5(_v.as_pointer())
    });
}
pub fn operator_eq_10(x: Ptr<NonTrivialCopyCtor>, y: Ptr<NonTrivialCopyCtor>) -> () {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(4)]
pub struct NonTrivialCopyCtor {
    #[offset(0)]
    pub i: (),
}
impl NonTrivialCopyCtor {
    pub fn new(mut ii: ()) -> Self {
        Self { i: ii }
    }
    pub fn copy_from(other: Ptr<NonTrivialCopyCtor>) -> Self {
        Self {
            i: other.with(|__s| __s.i),
        }
    }
    pub fn move_from(other: Ptr<NonTrivialCopyCtor>) -> Self {
        Self {
            i: { (*other.upgrade().deref()).i },
        }
    }
}
impl std::cmp::PartialEq for NonTrivialCopyCtor {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_10(
                Rc::new(RefCell::new(NonTrivialCopyCtor { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(NonTrivialCopyCtor { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialCopyCtor {}
impl Clone for NonTrivialCopyCtor {
    fn clone(&self) -> Self {
        let __src: Value<NonTrivialCopyCtor> =
            Rc::new(RefCell::new(NonTrivialCopyCtor { i: self.i.clone() }));
        NonTrivialCopyCtor::copy_from(__src.as_pointer())
    }
}
impl Default for NonTrivialCopyCtor {
    fn default() -> Self {
        { NonTrivialCopyCtor { i: 0_() } }
    }
}
pub fn test_constexpr_copy_ctor_non_trivial_11() {
    const _: () = assert!(true, "!std::is_trivially_copy_constructible<V>::value");;
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_constexpr_copy_ctor_imp_6(_v.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_constexpr_copy_ctor_imp_7(_v.as_pointer())
    });
    ({
        let _v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_constexpr_copy_ctor_imp_8(_v.as_pointer())
    });
}
pub fn non_constexpr_test_12() {
    ({ test_copy_ctor_valueless_by_exception_2() });
}
pub fn cxx17_constexpr_test_13() -> () {
    ({ test_copy_ctor_sfinae_0() });
    ({ test_constexpr_copy_ctor_trivial_9() });
    return true;
}
pub fn cxx20_constexpr_test_14() -> () {
    ({ test_copy_ctor_basic_1() });
    ({ test_constexpr_copy_ctor_non_trivial_11() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ non_constexpr_test_12() });
    ({ cxx17_constexpr_test_13() });
    ({ cxx20_constexpr_test_14() });
    const _: () = assert!(true, "cxx17_constexpr_test()");;
    const _: () = assert!(true, "cxx20_constexpr_test()");;
    return 0_();
}
pub trait NonTImpl {
    fn copy_assign(&self, _a0: Ptr<NonT>) -> Ptr<NonT>;
}
impl NonTImpl for Ptr<NonT> {
    fn copy_assign(&self, _a0: Ptr<NonT>) -> Ptr<NonT> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
