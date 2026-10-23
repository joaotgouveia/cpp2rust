extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_6(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: (),
}
impl TrivialMoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_13(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_7(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
pub fn swap_14(x: Ptr<ADLSwap>, y: Ptr<ADLSwap>) {
    ({
        let ___x: Ptr<()> = field_ptr!(x, i);
        let ___y: Ptr<()> = field_ptr!(y, i);
        swap_15(___x, ___y)
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
pub fn operator_eq_16(_a0: Ptr<TailClobberer_0_>, _a1: Ptr<TailClobberer_0_>) -> () {
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
impl std::cmp::PartialEq for TailClobberer_0_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_16(
                Rc::new(RefCell::new(TailClobberer_0_ { b: self.b.clone() })).as_pointer(),
                Rc::new(RefCell::new(TailClobberer_0_ { b: other.b.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TailClobberer_0_ {}
pub fn operator_eq_17(_a0: Ptr<TailClobberer_1_>, _a1: Ptr<TailClobberer_1_>) -> () {
    return ({ _a0.with(|__s| __s.b) } == { _a1.with(|__s| __s.b) });
}
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
                memset_18(___s, 1_(), ___n)
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
    pub fn new_2(mut _a0: ()) -> Self {
        TailClobberer_1_::new_1()
    }
}
impl std::cmp::PartialEq for TailClobberer_1_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_17(
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
impl TailClobbererNonTrivialMove_0__true__false_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(2)]
pub struct TailClobbererNonTrivialMove_0__false__false_ {}
impl TailClobbererNonTrivialMove_0__false__false_ {}
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
                memset_18(___s, 0_(), ___n)
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
                memset_18(___s, 0_(), ___n)
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
const _: () = assert!(
    true,
    "std::is_constructible_v<std::expected<int, int>, std::unexpected<int>>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::expected<int, MoveOnly>, std::unexpected<MoveOnly>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct foo {}
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::expected<int, int>, std::unexpected<foo>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotConvertible {}
impl NotConvertible {}
const _: () = assert!(
    true,
    "std::is_convertible_v<std::unexpected<int>&&, std::expected<int, int>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<std::unexpected<int>&&, std::expected<int, NotConvertible>>"
);
pub fn operator_eq_19(_a0: Ptr<MyInt>, _a1: Ptr<MyInt>) -> () {
    return ({ _a0.with(|__s| __s.i) } == { _a1.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MyInt {
    #[offset(0)]
    pub i: (),
}
impl MyInt {
    pub fn new(mut ii: ()) -> Self {
        Self { i: ii }
    }
}
impl std::cmp::PartialEq for MyInt {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_19(
                Rc::new(RefCell::new(MyInt { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(MyInt { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for MyInt {}
pub fn testInt_20() {
    let u: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn testInt_21() {
    let u: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn testInt_22() {
    let u: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn testInt_23() {
    let u: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn testMoveOnly_24() {
    let u: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_25() -> () {
    ({ testInt_20() });
    ({ testInt_21() });
    ({ testInt_22() });
    ({ testInt_23() });
    ({ testMoveOnly_24() });
    return true;
}
pub fn testException_26() {}
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
    ({ test_25() });
    const _: () = assert!(true, "test()");;
    ({ testException_26() });
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
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub fn __cpp2rust_init_globals() {}
