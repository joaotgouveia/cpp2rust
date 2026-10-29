extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(5)]
pub struct TracedBase_false__state {
    #[offset(0)]
    pub copyCtorCalled: (),
    #[offset(1)]
    pub copyAssignCalled: (),
    #[offset(2)]
    pub moveCtorCalled: (),
    #[offset(3)]
    pub moveAssignCalled: (),
    #[offset(4)]
    pub dtorCalled: (),
}
impl Default for TracedBase_false__state {
    fn default() -> Self {
        {
            TracedBase_false__state {
                copyCtorCalled: false,
                copyAssignCalled: false,
                moveCtorCalled: false,
                moveAssignCalled: false,
                dtorCalled: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor)]
#[byte_size(16)]
pub struct TracedBase_false__true_ {
    #[offset(0)]
    #[byte_size(8)]
    pub state_: Ptr<TracedBase_false__state>,
    #[offset(8)]
    pub copiedFromInt: (),
    #[offset(9)]
    pub movedFromInt: (),
    #[offset(10)]
    pub copiedFromTmp: (),
    #[offset(11)]
    pub movedFromTmp: (),
    #[offset(12)]
    pub data_: (),
}
impl TracedBase_false__true_ {
    pub fn new_1(s: Ptr<TracedBase_false__state>, mut ii: ()) -> Self {
        Self {
            state_: (s).clone(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: ii,
        }
    }
    pub fn move_from(other: Ptr<TracedBase_false__true_>) -> Self {
        let __this: Value<TracedBase_false__true_> = Rc::new(RefCell::new(Self {
            state_: other.with(|__s| __s.state_.clone()),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: other.with(|__s| __s.data_),
        }));
        let this: Ptr<TracedBase_false__true_> = __this.as_pointer();
        if !(this.with(|__s| __s.state_.clone())).is_null() {
            field!(this.with(|__s| __s.state_.clone()), moveCtorCalled).write(true);
        } else {
            field!(this, movedFromTmp).write(true);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for TracedBase_false__true_ {
    fn default() -> Self {
        {
            TracedBase_false__true_ {
                state_: Ptr::<TracedBase_false__state>::null(),
                copiedFromInt: false,
                movedFromInt: false,
                copiedFromTmp: false,
                movedFromTmp: false,
                data_: unimplemented!(),
            }
        }
    }
}
pub fn swap_0(x: Ptr<ADLSwap>, y: Ptr<ADLSwap>) {
    unimplemented!();
    field!(x, adlSwapCalled).write(true);
    field!(y, adlSwapCalled).write(true);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct ADLSwap {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub adlSwapCalled: (),
}
impl ADLSwap {
    pub fn new(mut ii: ()) -> Self {
        Self {
            i: ii,
            adlSwapCalled: false,
        }
    }
}
impl Default for ADLSwap {
    fn default() -> Self {
        {
            ADLSwap {
                i: unimplemented!(),
                adlSwapCalled: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MoveOnlyErrorType {}
impl MoveOnlyErrorType {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
    pub fn new_1(_a0: Ptr<MoveOnlyErrorType>) -> Self {
        Self {}
    }
    pub fn new_2(_a0: Ptr<MoveOnlyErrorType>) -> Self {
        Self {}
    }
}
pub fn operator_eq_1(_a0: Ptr<TailClobberer_0_>, _a1: Ptr<TailClobberer_0_>) -> () {
    return ({ _a0.with(|__s| __s.b) } == { _a1.with(|__s| __s.b) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(2)]
pub struct TailClobberer_0_ {
    #[offset(0)]
    b: (),
}
impl TailClobberer_0_ {
    pub fn move_from(_a0: Ptr<TailClobberer_0_>) -> Self {
        Self {
            b: { (*_a0.upgrade().deref()).b },
        }
    }
}
impl CppEq for TailClobberer_0_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_2(_a0: Ptr<TailClobberer_1_>, _a1: Ptr<TailClobberer_1_>) -> () {
    return ({ _a0.with(|__s| __s.b) } == { _a1.with(|__s| __s.b) });
}
pub fn swap_3(_a0: Ptr<TailClobberer_1_>, _a1: Ptr<TailClobberer_1_>) {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(2)]
pub struct TailClobberer_1_ {
    #[offset(0)]
    b: (),
}
impl TailClobberer_1_ {
    pub fn new_1() -> Self {
        let __this: Value<TailClobberer_1_> = Rc::new(RefCell::new(Self {
            b: unimplemented!(),
        }));
        let this: Ptr<TailClobberer_1_> = __this.as_pointer();
        {
            unimplemented!();
        }
        field!(this, b).write(false);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<TailClobberer_1_>) -> Self {
        Self {
            b: { (*_a0.upgrade().deref()).b },
        }
    }
}
impl CppEq for TailClobberer_1_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TailClobberer_1_ {
    fn default() -> Self {
        { TailClobberer_1_::new_1() }
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_constructible_v<TailClobberer<0>>"
);
const _: () = assert!(
    true,
    "std::is_trivially_move_constructible_v<TailClobberer<0>>"
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(2)]
pub struct TailClobbererNonTrivialMove_0__true__false_ {}
impl TailClobbererNonTrivialMove_0__true__false_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(2)]
pub struct TailClobbererNonTrivialMove_0__false__false_ {}
impl TailClobbererNonTrivialMove_0__false__false_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(2)]
pub struct TailClobbererNonTrivialMove_1__true__false_ {}
impl TailClobbererNonTrivialMove_1__true__false_ {
    pub fn move_from(_a0: Ptr<TailClobbererNonTrivialMove_1__true__false_>) -> Self {
        Self {}
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copy_constructible_v<TailClobbererNonTrivialMove<0>>"
);
const _: () = assert!(
    true,
    "std::is_move_constructible_v<TailClobbererNonTrivialMove<0>>"
);
const _: () = assert!(
    true,
    "!std::is_trivially_move_constructible_v<TailClobbererNonTrivialMove<0>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_move_constructible_v<TailClobbererNonTrivialMove<0, true>>"
);
const _: () = assert!(
    true,
    "!std::is_nothrow_move_constructible_v<TailClobbererNonTrivialMove<0, false>>"
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign)]
#[byte_size(8)]
pub struct BoolWithPadding {
    #[offset(0)]
    val_: (),
}
impl BoolWithPadding {
    pub fn new_1() -> Self {
        BoolWithPadding::new_2({ false })
    }
    pub fn new_2(mut val: ()) -> Self {
        let __this: Value<BoolWithPadding> = Rc::new(RefCell::new(Self {
            val_: unimplemented!(),
        }));
        let this: Ptr<BoolWithPadding> = __this.as_pointer();
        {
            unimplemented!();
        }
        field!(this, val_).write(val);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(other: Ptr<BoolWithPadding>) -> Self {
        BoolWithPadding::new_2({ other.with(|__s| __s.val_) })
    }
    pub fn move_from(_a0: Ptr<BoolWithPadding>) -> Self {
        Self {
            val_: { (*_a0.upgrade().deref()).val_ },
        }
    }
}
impl Clone for BoolWithPadding {
    fn clone(&self) -> Self {
        let __src: Value<BoolWithPadding> = Rc::new(RefCell::new(BoolWithPadding {
            val_: self.val_.clone(),
        }));
        BoolWithPadding::copy_from(__src.as_pointer())
    }
}
impl Default for BoolWithPadding {
    fn default() -> Self {
        { BoolWithPadding::new_1() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign)]
#[byte_size(4)]
pub struct IntWithoutPadding {
    #[offset(0)]
    val_: (),
}
impl IntWithoutPadding {
    pub fn new_1() -> Self {
        IntWithoutPadding::new_2({ 0_() })
    }
    pub fn new_2(mut val: ()) -> Self {
        let __this: Value<IntWithoutPadding> = Rc::new(RefCell::new(Self {
            val_: unimplemented!(),
        }));
        let this: Ptr<IntWithoutPadding> = __this.as_pointer();
        {
            unimplemented!();
        }
        field!(this, val_).write(val);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(other: Ptr<IntWithoutPadding>) -> Self {
        IntWithoutPadding::new_2({ other.with(|__s| __s.val_) })
    }
    pub fn move_from(_a0: Ptr<IntWithoutPadding>) -> Self {
        Self {
            val_: { (*_a0.upgrade().deref()).val_ },
        }
    }
}
impl Clone for IntWithoutPadding {
    fn clone(&self) -> Self {
        let __src: Value<IntWithoutPadding> = Rc::new(RefCell::new(IntWithoutPadding {
            val_: self.val_.clone(),
        }));
        IntWithoutPadding::copy_from(__src.as_pointer())
    }
}
impl Default for IntWithoutPadding {
    fn default() -> Self {
        { IntWithoutPadding::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckForInvalidWritesBaseImpl_true__false_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckForInvalidWritesBaseImpl_false__false_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckForInvalidWritesBaseImpl_true__true_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckForInvalidWritesBaseImpl_false__true_ {}
const _: () = assert!(true, "std::is_swappable_v<std::expected<void, int>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotSwappable {}
const _: () = assert!(
    true,
    "!std::is_swappable_v<std::expected<void, NotSwappable>>"
);
pub fn swap_4(_a0: Ptr<NotMoveConstructible>, _a1: Ptr<NotMoveConstructible>) {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotMoveConstructible {}
const _: () = assert!(
    true,
    "!std::is_swappable_v<std::expected<void, NotMoveConstructible>>"
);
pub fn swap_5(_a0: Ptr<MoveMayThrow>, _a1: Ptr<MoveMayThrow>) {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MoveMayThrow {}
impl MoveMayThrow {}
const _: () = assert!(true, "FreeSwapNoexcept<int>");
const _: () = assert!(!(false), "!FreeSwapNoexcept<MoveMayThrow>");
pub fn swap_6(_a0: Ptr<SwapMayThrow>, _a1: Ptr<SwapMayThrow>) {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SwapMayThrow {}
const _: () = assert!(!(false), "!FreeSwapNoexcept<SwapMayThrow>");
pub fn test_7() -> () {
    {
        let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: TracedBase_false__state = <TracedBase_false__state>::default();
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: TracedBase_false__state = <TracedBase_false__state>::default();
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn testException_8() {}
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
    ({ test_7() });
    const _: () = assert!(true, "test()");;
    ({ testException_8() });
    return 0_();
}
pub trait BoolWithPaddingImpl {
    fn copy_assign(&self, other: Ptr<BoolWithPadding>) -> Ptr<BoolWithPadding>;
}
impl BoolWithPaddingImpl for Ptr<BoolWithPadding> {
    fn copy_assign(&self, other: Ptr<BoolWithPadding>) -> Ptr<BoolWithPadding> {
        field!((*self), val_).write({ other.with(|__s| __s.val_) });
        return (*self).clone();
    }
}
pub trait IntWithoutPaddingImpl {
    fn copy_assign(&self, other: Ptr<IntWithoutPadding>) -> Ptr<IntWithoutPadding>;
}
impl IntWithoutPaddingImpl for Ptr<IntWithoutPadding> {
    fn copy_assign(&self, other: Ptr<IntWithoutPadding>) -> Ptr<IntWithoutPadding> {
        field!((*self), val_).write({ other.with(|__s| __s.val_) });
        return (*self).clone();
    }
}
pub trait TracedBase_false__true_Impl {
    fn copy_assign(&self, other: Ptr<TracedBase_false__true_>) -> Ptr<TracedBase_false__true_> {
        unimplemented!()
    }
    fn move_assign(&self, other: Ptr<TracedBase_false__true_>) -> Ptr<TracedBase_false__true_>;
    fn destructor(&self);
}
impl TracedBase_false__true_Impl for Ptr<TracedBase_false__true_> {
    fn move_assign(&self, other: Ptr<TracedBase_false__true_>) -> Ptr<TracedBase_false__true_> {
        field!((*self), data_).write({ other.with(|__s| __s.data_) });
        field!((*self).with(|__s| __s.state_.clone()), moveAssignCalled).write(true);
        return (*self).clone();
    }
    fn destructor(&self) {
        if !((*self).with(|__s| __s.state_.clone())).is_null() {
            field!((*self).with(|__s| __s.state_.clone()), dtorCalled).write(true);
        }
    }
}
pub fn __cpp2rust_init_globals() {}
