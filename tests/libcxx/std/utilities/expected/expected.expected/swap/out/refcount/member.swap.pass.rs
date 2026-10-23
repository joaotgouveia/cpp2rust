extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn swap_0(x: Ptr<ADLSwap>, y: Ptr<ADLSwap>) {
    ({
        let ___x: Ptr<()> = field_ptr!(x, i);
        let ___y: Ptr<()> = field_ptr!(y, i);
        swap_1(___x, ___y)
    });
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
pub fn swap_2(x: Ptr<TrackedMove_true_>, y: Ptr<TrackedMove_true_>) {
    ({
        let ___x: Ptr<()> = field_ptr!(x, i);
        let ___y: Ptr<()> = field_ptr!(y, i);
        swap_1(___x, ___y)
    });
    ({
        let ___x: Ptr<()> = field_ptr!(x, numberOfMoves);
        let ___y: Ptr<()> = field_ptr!(y, numberOfMoves);
        swap_1(___x, ___y)
    });
    field!(x, swapCalled).write(true);
    field!(y, swapCalled).write(true);
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(12)]
pub struct TrackedMove_true_ {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub numberOfMoves: (),
    #[offset(8)]
    pub swapCalled: (),
}
impl TrackedMove_true_ {
    pub fn new(mut ii: ()) -> Self {
        Self {
            i: ii,
            numberOfMoves: 0_(),
            swapCalled: false,
        }
    }
    pub fn move_from(other: Ptr<TrackedMove_true_>) -> Self {
        let __this: Value<TrackedMove_true_> = Rc::new(RefCell::new(Self {
            i: other.with(|__s| __s.i),
            numberOfMoves: other.with(|__s| __s.numberOfMoves),
            swapCalled: other.with(|__s| __s.swapCalled),
        }));
        let this: Ptr<TrackedMove_true_> = __this.as_pointer();
        field!(this, numberOfMoves).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for TrackedMove_true_ {
    fn default() -> Self {
        {
            TrackedMove_true_ {
                i: unimplemented!(),
                numberOfMoves: 0_(),
                swapCalled: false,
            }
        }
    }
}
pub fn swap_3(x: Ptr<TrackedMove_false_>, y: Ptr<TrackedMove_false_>) {
    ({
        let ___x: Ptr<()> = field_ptr!(x, i);
        let ___y: Ptr<()> = field_ptr!(y, i);
        swap_1(___x, ___y)
    });
    ({
        let ___x: Ptr<()> = field_ptr!(x, numberOfMoves);
        let ___y: Ptr<()> = field_ptr!(y, numberOfMoves);
        swap_1(___x, ___y)
    });
    field!(x, swapCalled).write(true);
    field!(y, swapCalled).write(true);
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(12)]
pub struct TrackedMove_false_ {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub numberOfMoves: (),
    #[offset(8)]
    pub swapCalled: (),
}
impl TrackedMove_false_ {
    pub fn new(mut ii: ()) -> Self {
        Self {
            i: ii,
            numberOfMoves: 0_(),
            swapCalled: false,
        }
    }
    pub fn move_from(other: Ptr<TrackedMove_false_>) -> Self {
        let __this: Value<TrackedMove_false_> = Rc::new(RefCell::new(Self {
            i: other.with(|__s| __s.i),
            numberOfMoves: other.with(|__s| __s.numberOfMoves),
            swapCalled: other.with(|__s| __s.swapCalled),
        }));
        let this: Ptr<TrackedMove_false_> = __this.as_pointer();
        field!(this, numberOfMoves).with_mut(|__v| __v.prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for TrackedMove_false_ {
    fn default() -> Self {
        {
            TrackedMove_false_ {
                i: unimplemented!(),
                numberOfMoves: 0_(),
                swapCalled: false,
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
pub fn operator_eq_4(_a0: Ptr<TailClobberer_0_>, _a1: Ptr<TailClobberer_0_>) -> () {
    return ({ _a0.with(|__s| __s.b) } == { _a1.with(|__s| __s.b) });
}
pub fn swap_5(_a0: Ptr<TailClobberer_0_>, _a1: Ptr<TailClobberer_0_>) {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(2)]
pub struct TailClobberer_0_ {
    #[offset(0)]
    b: (),
}
impl TailClobberer_0_ {
    pub fn new_1() -> Self {
        let __this: Value<TailClobberer_0_> = Rc::new(RefCell::new(Self {
            b: unimplemented!(),
        }));
        let this: Ptr<TailClobberer_0_> = __this.as_pointer();
        {
            ({
                let ___s: () = (this).to_any();
                let ___n: () = 2usize;
                memset_6(___s, 0_(), ___n)
            });
        }
        field!(this, b).write(false);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<TailClobberer_0_>) -> Self {
        Self {
            b: { (*_a0.upgrade().deref()).b },
        }
    }
}
impl std::cmp::PartialEq for TailClobberer_0_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_4(
                Rc::new(RefCell::new(TailClobberer_0_ { b: self.b.clone() })).as_pointer(),
                Rc::new(RefCell::new(TailClobberer_0_ { b: other.b.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_0_ {}
impl Default for TailClobberer_0_ {
    fn default() -> Self {
        { TailClobberer_0_::new_1() }
    }
}
pub fn operator_eq_7(_a0: Ptr<TailClobberer_1_>, _a1: Ptr<TailClobberer_1_>) -> () {
    return ({ _a0.with(|__s| __s.b) } == { _a1.with(|__s| __s.b) });
}
pub fn swap_8(_a0: Ptr<TailClobberer_1_>, _a1: Ptr<TailClobberer_1_>) {}
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
            ({
                let ___s: () = (this).to_any();
                let ___n: () = 2usize;
                memset_6(___s, 1_(), ___n)
            });
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
impl std::cmp::PartialEq for TailClobberer_1_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_7(
                Rc::new(RefCell::new(TailClobberer_1_ { b: self.b.clone() })).as_pointer(),
                Rc::new(RefCell::new(TailClobberer_1_ { b: other.b.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_1_ {}
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
impl TailClobbererNonTrivialMove_0__true__false_ {
    pub fn move_from(_a0: Ptr<TailClobbererNonTrivialMove_0__true__false_>) -> Self {
        Self {}
    }
}
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
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(2)]
pub struct TailClobbererNonTrivialMove_1__false__false_ {}
impl TailClobbererNonTrivialMove_1__false__false_ {
    pub fn move_from(_a0: Ptr<TailClobbererNonTrivialMove_1__false__false_>) -> Self {
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
            ({
                let ___s: () = (this).to_any();
                let ___n: () = 8usize;
                memset_6(___s, 0_(), ___n)
            });
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
            ({
                let ___s: () = (this).to_any();
                let ___n: () = 4usize;
                memset_6(___s, 0_(), ___n)
            });
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
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(12)]
pub struct CheckForInvalidWrites_true__false_ {
    #[offset(5)]
    #[byte_size(7)]
    pub please_dont_overwrite_me: Box<[()]>,
}
impl CheckForInvalidWrites_true__false_ {
    pub fn new_1(_a0: ()) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            please_dont_overwrite_me: Box::new([true, true, true, true, true, true, true]),
        }
    }
}
impl Default for CheckForInvalidWrites_true__false_ {
    fn default() -> Self {
        {
            CheckForInvalidWrites_true__false_ {
                please_dont_overwrite_me: Box::new([true, true, true, true, true, true, true]),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct CheckForInvalidWrites_false__false_ {
    #[offset(8)]
    #[byte_size(7)]
    pub please_dont_overwrite_me: Box<[()]>,
}
impl CheckForInvalidWrites_false__false_ {
    pub fn new_1(_a0: ()) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            please_dont_overwrite_me: Box::new([true, true, true, true, true, true, true]),
        }
    }
}
impl Default for CheckForInvalidWrites_false__false_ {
    fn default() -> Self {
        {
            CheckForInvalidWrites_false__false_ {
                please_dont_overwrite_me: Box::new([true, true, true, true, true, true, true]),
            }
        }
    }
}
const _: () = assert!(true, "HasMemberSwap<int, int>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotSwappable {}
const _: () = assert!(!(false), "!HasMemberSwap<NotSwappable, int>");
const _: () = assert!(!(false), "!HasMemberSwap<int, NotSwappable>");
pub fn swap_9(_a0: Ptr<NotMoveConstructible>, _a1: Ptr<NotMoveConstructible>) {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotMoveConstructible {}
const _: () = assert!(!(false), "!HasMemberSwap<NotMoveConstructible, int>");
const _: () = assert!(!(false), "!HasMemberSwap<int, NotMoveConstructible>");
pub fn swap_10(_a0: Ptr<MoveMayThrow>, _a1: Ptr<MoveMayThrow>) {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MoveMayThrow {}
impl MoveMayThrow {}
const _: () = assert!(true, "HasMemberSwap<MoveMayThrow, int>");
const _: () = assert!(true, "HasMemberSwap<int, MoveMayThrow>");
const _: () = assert!(!(false), "!HasMemberSwap<MoveMayThrow, MoveMayThrow>");
const _: () = assert!(true, "MemberSwapNoexcept<int, int>");
const _: () = assert!(!(false), "!MemberSwapNoexcept<MoveMayThrow, int>");
const _: () = assert!(!(false), "!MemberSwapNoexcept<int, MoveMayThrow>");
pub fn swap_11(_a0: Ptr<SwapMayThrow>, _a1: Ptr<SwapMayThrow>) {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SwapMayThrow {}
const _: () = assert!(!(false), "!MemberSwapNoexcept<SwapMayThrow, int>");
const _: () = assert!(!(false), "!MemberSwapNoexcept<int, SwapMayThrow>");
pub fn test_12() -> () {
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
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let e2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
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
        }
    }
    {
        {
            let x: Value<CheckForInvalidWrites_true__false_> =
                Rc::new(RefCell::new(CheckForInvalidWrites_true__false_::new_1({
                    unimplemented!()
                })));
            let y: Value<CheckForInvalidWrites_true__false_> =
                Rc::new(RefCell::new(<CheckForInvalidWrites_true__false_>::default()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let x: Value<CheckForInvalidWrites_false__false_> =
                Rc::new(RefCell::new(CheckForInvalidWrites_false__false_::new_1({
                    unimplemented!()
                })));
            let y: Value<CheckForInvalidWrites_false__false_> = Rc::new(RefCell::new(
                <CheckForInvalidWrites_false__false_>::default(),
            ));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    return true;
}
pub fn testException_13() {}
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
    ({ test_12() });
    const _: () = assert!(true, "test()");;
    ({ testException_13() });
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
pub trait CheckForInvalidWrites_false__false_Impl {
    fn copy_assign(
        &self,
        other: Ptr<CheckForInvalidWrites_false__false_>,
    ) -> Ptr<CheckForInvalidWrites_false__false_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        other: Ptr<CheckForInvalidWrites_false__false_>,
    ) -> Ptr<CheckForInvalidWrites_false__false_> {
        unimplemented!()
    }
    fn check(&self) -> ();
}
impl CheckForInvalidWrites_false__false_Impl for Ptr<CheckForInvalidWrites_false__false_> {
    fn check(&self) -> () {
        'loop_: for i in IterRange::new(
            (array_field_ptr!((*self), please_dont_overwrite_me) as Ptr<()>),
            (array_field_ptr!((*self), please_dont_overwrite_me) as Ptr<()>)
                .offset((7_()) as isize),
        ) {
            let mut i: () = (i.read());
            if !(i) {
                return false;
            }
        }
        return true;
    }
}
pub trait CheckForInvalidWrites_true__false_Impl {
    fn copy_assign(
        &self,
        other: Ptr<CheckForInvalidWrites_true__false_>,
    ) -> Ptr<CheckForInvalidWrites_true__false_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        other: Ptr<CheckForInvalidWrites_true__false_>,
    ) -> Ptr<CheckForInvalidWrites_true__false_> {
        unimplemented!()
    }
    fn check(&self) -> ();
}
impl CheckForInvalidWrites_true__false_Impl for Ptr<CheckForInvalidWrites_true__false_> {
    fn check(&self) -> () {
        'loop_: for i in IterRange::new(
            (array_field_ptr!((*self), please_dont_overwrite_me) as Ptr<()>),
            (array_field_ptr!((*self), please_dont_overwrite_me) as Ptr<()>)
                .offset((7_()) as isize),
        ) {
            let mut i: () = (i.read());
            if !(i) {
                return false;
            }
        }
        return true;
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
pub fn __cpp2rust_init_globals() {}
