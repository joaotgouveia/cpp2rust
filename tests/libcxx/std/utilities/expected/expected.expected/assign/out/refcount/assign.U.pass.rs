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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor)]
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
    pub fn new_1(ii: Ptr<()>) -> Self {
        let __this: Value<TracedBase_false__true_> = Rc::new(RefCell::new(Self {
            state_: Ptr::<TracedBase_false__state>::null(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: (ii.read()),
        }));
        let this: Ptr<TracedBase_false__true_> = __this.as_pointer();
        field!(this, copiedFromInt).write(true);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(ii: Ptr<()>) -> Self {
        let __this: Value<TracedBase_false__true_> = Rc::new(RefCell::new(Self {
            state_: Ptr::<TracedBase_false__state>::null(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: (ii.read()),
        }));
        let this: Ptr<TracedBase_false__true_> = __this.as_pointer();
        field!(this, movedFromInt).write(true);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3(s: Ptr<TracedBase_false__state>, mut ii: ()) -> Self {
        Self {
            state_: (s).clone(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: ii,
        }
    }
    pub fn copy_from(other: Ptr<TracedBase_false__true_>) -> Self {
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
            field!(this.with(|__s| __s.state_.clone()), copyCtorCalled).write(true);
        } else {
            field!(this, copiedFromTmp).write(true);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
impl Clone for TracedBase_false__true_ {
    fn clone(&self) -> Self {
        let __src: Value<TracedBase_false__true_> =
            Rc::new(RefCell::new(TracedBase_false__true_ {
                state_: self.state_.clone(),
                copiedFromInt: self.copiedFromInt.clone(),
                movedFromInt: self.movedFromInt.clone(),
                copiedFromTmp: self.copiedFromTmp.clone(),
                movedFromTmp: self.movedFromTmp.clone(),
                data_: self.data_.clone(),
            }));
        TracedBase_false__true_::copy_from(__src.as_pointer())
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(5)]
pub struct TracedBase_true__state {
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
impl Default for TracedBase_true__state {
    fn default() -> Self {
        {
            TracedBase_true__state {
                copyCtorCalled: false,
                copyAssignCalled: false,
                moveCtorCalled: false,
                moveAssignCalled: false,
                dtorCalled: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct TracedBase_true__true_ {
    #[offset(0)]
    #[byte_size(8)]
    pub state_: Ptr<TracedBase_true__state>,
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
impl TracedBase_true__true_ {
    pub fn new_1(s: Ptr<TracedBase_true__state>, mut ii: ()) -> Self {
        Self {
            state_: (s).clone(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: ii,
        }
    }
    pub fn copy_from(other: Ptr<TracedBase_true__true_>) -> Self {
        let __this: Value<TracedBase_true__true_> = Rc::new(RefCell::new(Self {
            state_: other.with(|__s| __s.state_.clone()),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: other.with(|__s| __s.data_),
        }));
        let this: Ptr<TracedBase_true__true_> = __this.as_pointer();
        if !(this.with(|__s| __s.state_.clone())).is_null() {
            field!(this.with(|__s| __s.state_.clone()), copyCtorCalled).write(true);
        } else {
            field!(this, copiedFromTmp).write(true);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<TracedBase_true__true_>) -> Self {
        let __this: Value<TracedBase_true__true_> = Rc::new(RefCell::new(Self {
            state_: other.with(|__s| __s.state_.clone()),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: other.with(|__s| __s.data_),
        }));
        let this: Ptr<TracedBase_true__true_> = __this.as_pointer();
        if !(this.with(|__s| __s.state_.clone())).is_null() {
            field!(this.with(|__s| __s.state_.clone()), moveCtorCalled).write(true);
        } else {
            field!(this, movedFromTmp).write(true);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for TracedBase_true__true_ {
    fn clone(&self) -> Self {
        let __src: Value<TracedBase_true__true_> = Rc::new(RefCell::new(TracedBase_true__true_ {
            state_: self.state_.clone(),
            copiedFromInt: self.copiedFromInt.clone(),
            movedFromInt: self.movedFromInt.clone(),
            copiedFromTmp: self.copiedFromTmp.clone(),
            movedFromTmp: self.movedFromTmp.clone(),
            data_: self.data_.clone(),
        }));
        TracedBase_true__true_::copy_from(__src.as_pointer())
    }
}
impl Default for TracedBase_true__true_ {
    fn default() -> Self {
        {
            TracedBase_true__true_ {
                state_: Ptr::<TracedBase_true__state>::null(),
                copiedFromInt: false,
                movedFromInt: false,
                copiedFromTmp: false,
                movedFromTmp: false,
                data_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(5)]
pub struct TracedBase_true__false__state {
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
impl Default for TracedBase_true__false__state {
    fn default() -> Self {
        {
            TracedBase_true__false__state {
                copyCtorCalled: unimplemented!(),
                copyAssignCalled: unimplemented!(),
                moveCtorCalled: unimplemented!(),
                moveAssignCalled: unimplemented!(),
                dtorCalled: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor)]
#[byte_size(16)]
pub struct TracedBase_true__false_ {
    #[offset(0)]
    #[byte_size(8)]
    pub state_: Ptr<TracedBase_true__false__state>,
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
impl TracedBase_true__false_ {
    pub fn new_1(ii: Ptr<()>) -> Self {
        let __this: Value<TracedBase_true__false_> = Rc::new(RefCell::new(Self {
            state_: Ptr::<TracedBase_true__false__state>::null(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: (ii.read()),
        }));
        let this: Ptr<TracedBase_true__false_> = __this.as_pointer();
        field!(this, copiedFromInt).write(true);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(ii: Ptr<()>) -> Self {
        let __this: Value<TracedBase_true__false_> = Rc::new(RefCell::new(Self {
            state_: Ptr::<TracedBase_true__false__state>::null(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: (ii.read()),
        }));
        let this: Ptr<TracedBase_true__false_> = __this.as_pointer();
        field!(this, movedFromInt).write(true);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<TracedBase_true__false_>) -> Self {
        let __this: Value<TracedBase_true__false_> = Rc::new(RefCell::new(Self {
            state_: other.with(|__s| __s.state_.clone()),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: other.with(|__s| __s.data_),
        }));
        let this: Ptr<TracedBase_true__false_> = __this.as_pointer();
        if !(this.with(|__s| __s.state_.clone())).is_null() {
            field!(this.with(|__s| __s.state_.clone()), moveCtorCalled).write(true);
        } else {
            field!(this, movedFromTmp).write(true);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for TracedBase_true__false_ {
    fn default() -> Self {
        {
            TracedBase_true__false_ {
                state_: Ptr::<TracedBase_true__false__state>::null(),
                copiedFromInt: false,
                movedFromInt: false,
                copiedFromTmp: false,
                movedFromTmp: false,
                data_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(5)]
pub struct TracedBase_false__false__state {
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
impl Default for TracedBase_false__false__state {
    fn default() -> Self {
        {
            TracedBase_false__false__state {
                copyCtorCalled: false,
                copyAssignCalled: false,
                moveCtorCalled: false,
                moveAssignCalled: false,
                dtorCalled: false,
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Destructor)]
#[byte_size(16)]
pub struct TracedBase_false__false_ {
    #[offset(0)]
    #[byte_size(8)]
    pub state_: Ptr<TracedBase_false__false__state>,
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
impl TracedBase_false__false_ {
    pub fn new_1(ii: Ptr<()>) -> Self {
        let __this: Value<TracedBase_false__false_> = Rc::new(RefCell::new(Self {
            state_: Ptr::<TracedBase_false__false__state>::null(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: (ii.read()),
        }));
        let this: Ptr<TracedBase_false__false_> = __this.as_pointer();
        field!(this, copiedFromInt).write(true);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(ii: Ptr<()>) -> Self {
        let __this: Value<TracedBase_false__false_> = Rc::new(RefCell::new(Self {
            state_: Ptr::<TracedBase_false__false__state>::null(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: (ii.read()),
        }));
        let this: Ptr<TracedBase_false__false_> = __this.as_pointer();
        field!(this, movedFromInt).write(true);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3(s: Ptr<TracedBase_false__false__state>, mut ii: ()) -> Self {
        Self {
            state_: (s).clone(),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: ii,
        }
    }
    pub fn copy_from(other: Ptr<TracedBase_false__false_>) -> Self {
        let __this: Value<TracedBase_false__false_> = Rc::new(RefCell::new(Self {
            state_: other.with(|__s| __s.state_.clone()),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: other.with(|__s| __s.data_),
        }));
        let this: Ptr<TracedBase_false__false_> = __this.as_pointer();
        if !(this.with(|__s| __s.state_.clone())).is_null() {
            field!(this.with(|__s| __s.state_.clone()), copyCtorCalled).write(true);
        } else {
            field!(this, copiedFromTmp).write(true);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<TracedBase_false__false_>) -> Self {
        let __this: Value<TracedBase_false__false_> = Rc::new(RefCell::new(Self {
            state_: other.with(|__s| __s.state_.clone()),
            copiedFromInt: false,
            movedFromInt: false,
            copiedFromTmp: false,
            movedFromTmp: false,
            data_: other.with(|__s| __s.data_),
        }));
        let this: Ptr<TracedBase_false__false_> = __this.as_pointer();
        if !(this.with(|__s| __s.state_.clone())).is_null() {
            field!(this.with(|__s| __s.state_.clone()), moveCtorCalled).write(true);
        } else {
            field!(this, movedFromTmp).write(true);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for TracedBase_false__false_ {
    fn clone(&self) -> Self {
        let __src: Value<TracedBase_false__false_> =
            Rc::new(RefCell::new(TracedBase_false__false_ {
                state_: self.state_.clone(),
                copiedFromInt: self.copiedFromInt.clone(),
                movedFromInt: self.movedFromInt.clone(),
                copiedFromTmp: self.copiedFromTmp.clone(),
                movedFromTmp: self.movedFromTmp.clone(),
                data_: self.data_.clone(),
            }));
        TracedBase_false__false_::copy_from(__src.as_pointer())
    }
}
impl Default for TracedBase_false__false_ {
    fn default() -> Self {
        {
            TracedBase_false__false_ {
                state_: Ptr::<TracedBase_false__false__state>::null(),
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
    #[offset(9)]
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
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotCopyConstructible {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotCopyAssignable {}
const _: () = assert!(true, "std::is_assignable_v<std::expected<int, int>&, int>");
const _: () = assert!(
    true,
    "std::is_assignable_v<std::expected<int, int>&, std::expected<int, int>>"
);
const _: () = assert!(
    true,
    "std::is_assignable_v<std::expected<int, int>&, std::unexpected<int>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoCtorFromInt {}
const _: () = assert!(
    true,
    "!std::is_assignable_v<std::expected<NoCtorFromInt, int>&, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoAssignFromInt {}
impl NoAssignFromInt {}
const _: () = assert!(
    true,
    "!std::is_assignable_v<std::expected<NoAssignFromInt, int>&, int>"
);
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct MaybeNoexcept_false__false_ {}
impl MaybeNoexcept_false__false_ {}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct MaybeNoexcept_false__true_ {}
impl MaybeNoexcept_false__true_ {}
#[derive(Record, ByteRepr, FnPtrArg, MoveAssign, Default)]
#[byte_size(1)]
pub struct MaybeNoexcept_true__false_ {}
impl MaybeNoexcept_true__false_ {}
const _: () = assert!(
    true,
    "std::is_assignable_v<std::expected<MaybeNoexcept<false, false>, int>&, int>"
);
const _: () = assert!(
    true,
    "std::is_assignable_v<std::expected<MaybeNoexcept<false, true>, MaybeNoexcept<false, false>>&, int>"
);
const _: () = assert!(
    true,
    "std::is_assignable_v<std::expected<MaybeNoexcept<true, false>, MaybeNoexcept<false, false>>&, int>"
);
const _: () = assert!(
    true,
    "!std::is_assignable_v<std::expected<MaybeNoexcept<false, false>, MaybeNoexcept<false, false>>&, int>"
);
pub fn test_2() -> () {
    {
        let mut oldState: TracedBase_false__state = <TracedBase_false__state>::default();
        let newState: Value<TracedBase_false__state> =
            Rc::new(RefCell::new(<TracedBase_false__state>::default()));
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let u: Value<TracedBase_false__true_> = Rc::new(RefCell::new(
            TracedBase_false__true_::new_3({ newState.as_pointer() }, { 10_() }),
        ));
        let _dtor_u = ScopedDestructor::new(&u, |__p| __p.destructor());
        let x: Ptr<()> = (unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<Traced, int>&>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut oldState: TracedBase_false__state = <TracedBase_false__state>::default();
        let newState: Value<TracedBase_false__state> =
            Rc::new(RefCell::new(<TracedBase_false__state>::default()));
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let u: Value<TracedBase_false__true_> = Rc::new(RefCell::new(
            TracedBase_false__true_::new_3({ newState.as_pointer() }, { 10_() }),
        ));
        let _dtor_u = ScopedDestructor::new(&u, |__p| __p.destructor());
        let x: Ptr<()> = (unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<Traced, int>&>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut oldState: TracedBase_false__false__state =
            <TracedBase_false__false__state>::default();
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 10_();
        let x: Ptr<()> = (unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<MoveThrowConvNoexcept, BothMayThrow>&>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut oldState: TracedBase_false__false__state =
            <TracedBase_false__false__state>::default();
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let x: Ptr<()> = (unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<MoveThrowConvNoexcept, BothMayThrow>&>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut oldState: TracedBase_false__false__state =
            <TracedBase_false__false__state>::default();
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 10_();
        let x: Ptr<()> = (unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<MoveNoexceptConvThrow, BothMayThrow>&>"
        );;
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
        let mut oldState: TracedBase_false__false__state =
            <TracedBase_false__false__state>::default();
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let x: Ptr<()> = (unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<MoveNoexceptConvThrow, BothMayThrow>&>"
        );;
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
        let mut oldState: TracedBase_true__state = <TracedBase_true__state>::default();
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 10_();
        let x: Ptr<()> = (unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<BothMayThrow, TracedNoexcept>&>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut oldState: TracedBase_true__state = <TracedBase_true__state>::default();
        let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let x: Ptr<()> = (unimplemented!());
        const _: () = assert!(
            true,
            "std::same_as<decltype(x), std::expected<BothMayThrow, TracedNoexcept>&>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let e1: Value<CheckForInvalidWrites_true__false_> =
                Rc::new(RefCell::new(CheckForInvalidWrites_true__false_::new_1({
                    unimplemented!()
                })));
            unimplemented!();
            unimplemented!();
        }
        {
            let e1: Value<CheckForInvalidWrites_false__false_> =
                Rc::new(RefCell::new(CheckForInvalidWrites_false__false_::new_1({
                    unimplemented!()
                })));
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut t: MoveOnlyMulti_4 = <MoveOnlyMulti_4>::default();
            let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(std::move(t)), MoveOnlyMulti&&>"
            );;
            unimplemented!();
            unimplemented!();
        }
        {
            let mut t: MoveOnlyMulti_4 = <MoveOnlyMulti_4>::default();
            let e1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(std::move(t)), const MoveOnlyMulti&&>"
            );;
            unimplemented!();
            unimplemented!();
        }
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Bar_3 {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub j: (),
}
impl Bar_3 {
    pub fn new(mut ii: (), mut jj: ()) -> Self {
        Self { i: ii, j: jj }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(2)]
pub struct MoveOnlyMulti_4 {
    #[offset(0)]
    pub used_move1: (),
    #[offset(1)]
    pub used_move2: (),
}
impl MoveOnlyMulti_4 {
    pub fn new_1(_a0: Ptr<MoveOnlyMulti_4>) -> Self {
        Self {
            used_move1: true,
            used_move2: false,
        }
    }
    pub fn new_2(_a0: Ptr<MoveOnlyMulti_4>) -> Self {
        Self {
            used_move1: false,
            used_move2: true,
        }
    }
}
impl Default for MoveOnlyMulti_4 {
    fn default() -> Self {
        {
            MoveOnlyMulti_4 {
                used_move1: false,
                used_move2: false,
            }
        }
    }
}
pub fn testException_5() {}
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    ({ testException_5() });
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
pub trait MaybeNoexcept_false__false_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<MaybeNoexcept_false__false_>,
    ) -> Ptr<MaybeNoexcept_false__false_>;
    fn operator_assign_4(&self, mut _a0: ()) -> Ptr<MaybeNoexcept_false__false_> {
        unimplemented!()
    }
}
impl MaybeNoexcept_false__false_Impl for Ptr<MaybeNoexcept_false__false_> {
    fn move_assign(
        &self,
        _a0: Ptr<MaybeNoexcept_false__false_>,
    ) -> Ptr<MaybeNoexcept_false__false_> {
        return (*self).clone();
    }
}
pub trait MaybeNoexcept_false__true_Impl {
    fn move_assign(&self, _a0: Ptr<MaybeNoexcept_false__true_>) -> Ptr<MaybeNoexcept_false__true_>;
    fn operator_assign_4(&self, mut _a0: ()) -> Ptr<MaybeNoexcept_false__true_> {
        unimplemented!()
    }
}
impl MaybeNoexcept_false__true_Impl for Ptr<MaybeNoexcept_false__true_> {
    fn move_assign(&self, _a0: Ptr<MaybeNoexcept_false__true_>) -> Ptr<MaybeNoexcept_false__true_> {
        return (*self).clone();
    }
}
pub trait MaybeNoexcept_true__false_Impl {
    fn move_assign(&self, _a0: Ptr<MaybeNoexcept_true__false_>) -> Ptr<MaybeNoexcept_true__false_>;
    fn operator_assign_4(&self, mut _a0: ()) -> Ptr<MaybeNoexcept_true__false_> {
        unimplemented!()
    }
}
impl MaybeNoexcept_true__false_Impl for Ptr<MaybeNoexcept_true__false_> {
    fn move_assign(&self, _a0: Ptr<MaybeNoexcept_true__false_>) -> Ptr<MaybeNoexcept_true__false_> {
        return (*self).clone();
    }
}
pub trait MoveOnlyMulti_4Impl {
    fn operator_assign_4(&self, _a0: Ptr<MoveOnlyMulti_4>) -> Ptr<MoveOnlyMulti_4>;
    fn operator_assign_5(&self, _a0: Ptr<MoveOnlyMulti_4>) -> Ptr<MoveOnlyMulti_4>;
}
impl MoveOnlyMulti_4Impl for Ptr<MoveOnlyMulti_4> {
    fn operator_assign_4(&self, _a0: Ptr<MoveOnlyMulti_4>) -> Ptr<MoveOnlyMulti_4> {
        field!((*self), used_move1).write(true);
        return (*self).clone();
    }
    fn operator_assign_5(&self, _a0: Ptr<MoveOnlyMulti_4>) -> Ptr<MoveOnlyMulti_4> {
        field!((*self), used_move2).write(true);
        return (*self).clone();
    }
}
pub trait NoCtorFromIntImpl {
    fn operator_assign_2(&self, mut _a0: ()) -> Ptr<NoCtorFromInt> {
        unimplemented!()
    }
}
impl NoCtorFromIntImpl for Ptr<NoCtorFromInt> {}
pub trait TracedBase_false__false_Impl {
    fn copy_assign(&self, other: Ptr<TracedBase_false__false_>) -> Ptr<TracedBase_false__false_> {
        unimplemented!()
    }
    fn move_assign(&self, other: Ptr<TracedBase_false__false_>) -> Ptr<TracedBase_false__false_>;
    fn destructor(&self);
}
impl TracedBase_false__false_Impl for Ptr<TracedBase_false__false_> {
    fn move_assign(&self, other: Ptr<TracedBase_false__false_>) -> Ptr<TracedBase_false__false_> {
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
pub trait TracedBase_false__true_Impl {
    fn copy_assign(&self, other: Ptr<TracedBase_false__true_>) -> Ptr<TracedBase_false__true_>;
    fn move_assign(&self, other: Ptr<TracedBase_false__true_>) -> Ptr<TracedBase_false__true_>;
    fn destructor(&self);
}
impl TracedBase_false__true_Impl for Ptr<TracedBase_false__true_> {
    fn copy_assign(&self, other: Ptr<TracedBase_false__true_>) -> Ptr<TracedBase_false__true_> {
        field!((*self), data_).write({ other.with(|__s| __s.data_) });
        field!((*self).with(|__s| __s.state_.clone()), copyAssignCalled).write(true);
        return (*self).clone();
    }
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
pub trait TracedBase_true__false_Impl {
    fn copy_assign(&self, other: Ptr<TracedBase_true__false_>) -> Ptr<TracedBase_true__false_> {
        unimplemented!()
    }
    fn move_assign(&self, other: Ptr<TracedBase_true__false_>) -> Ptr<TracedBase_true__false_>;
    fn destructor(&self);
}
impl TracedBase_true__false_Impl for Ptr<TracedBase_true__false_> {
    fn move_assign(&self, other: Ptr<TracedBase_true__false_>) -> Ptr<TracedBase_true__false_> {
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
pub trait TracedBase_true__true_Impl {
    fn copy_assign(&self, other: Ptr<TracedBase_true__true_>) -> Ptr<TracedBase_true__true_> {
        unimplemented!()
    }
    fn move_assign(&self, other: Ptr<TracedBase_true__true_>) -> Ptr<TracedBase_true__true_> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl TracedBase_true__true_Impl for Ptr<TracedBase_true__true_> {
    fn destructor(&self) {
        if !((*self).with(|__s| __s.state_.clone())).is_null() {
            field!((*self).with(|__s| __s.state_.clone()), dtorCalled).write(true);
        }
    }
}
pub fn __cpp2rust_init_globals() {}
