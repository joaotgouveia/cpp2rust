extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_1(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
pub unsafe fn makeTypeIDImp_2() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_3));
}
pub unsafe fn makeTypeIDImp_4() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_5));
}
pub unsafe fn makeTypeIDImp_6() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_7));
}
pub unsafe fn makeTypeIDImp_8() -> *const TypeID {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_9));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TypeID {
    m_id: *const (),
}
impl TypeID {
    pub unsafe fn name(&self) -> () {
        return unimplemented!();
    }
    unsafe fn new(mut xid: *const ()) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const TypeID, other as *const TypeID) }
    }
}
impl std::cmp::Eq for TypeID {}
static mut id_3: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<int &, const int &, unsigned int &>]"
            .as_ptr()
    })
});
static mut id_5: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<const int &, const int &, unsigned int &>]" .as_ptr()
    })
});
static mut id_7: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<int &&, const int &, unsigned int &&>]"
            .as_ptr()
    })
});
static mut id_9: std::cell::LazyCell<TypeID> = std::cell::LazyCell::new(|| unsafe {
    TypeID::new({
        c"const TypeID &makeTypeIDImp() [T = ArgumentListID<const int &&, const int &, unsigned int &&>]" .as_ptr()
    })
});
pub unsafe fn makeArgumentID_10() -> *const TypeID {
    return (unsafe { makeTypeIDImp_2() });
}
pub unsafe fn makeArgumentID_11() -> *const TypeID {
    return (unsafe { makeTypeIDImp_4() });
}
pub unsafe fn makeArgumentID_12() -> *const TypeID {
    return (unsafe { makeTypeIDImp_6() });
}
pub unsafe fn makeArgumentID_13() -> *const TypeID {
    return (unsafe { makeTypeIDImp_8() });
}
pub unsafe fn constexpr_sum_fn_14() -> () {
    return 0_();
}
pub unsafe fn constexpr_sum_fn_15(mut x1: ()) -> () {
    return ((x1) + (unsafe { constexpr_sum_fn_14() }));
}
pub unsafe fn constexpr_sum_fn_16(mut x1: (), mut rest: ()) -> () {
    return ((x1) + (unsafe { constexpr_sum_fn_15(rest) }));
}
pub unsafe fn constexpr_sum_fn_17(mut x1: (), mut rest: ()) -> () {
    return ((x1) + (unsafe { constexpr_sum_fn_15(rest) }));
}
pub unsafe fn constexpr_sum_fn_18(mut x1: (), mut rest_1: (), mut rest_2: ()) -> () {
    return ((x1) + (unsafe { constexpr_sum_fn_16(rest_1, rest_2) }));
}
pub unsafe fn constexpr_sum_fn_19(mut x1: (), mut rest_1: (), mut rest_2: ()) -> () {
    return ((x1) + (unsafe { constexpr_sum_fn_16(rest_1, rest_2) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConstexprSumT {}
impl ConstexprSumT {
    pub unsafe fn operator_call_2(&self) -> () {
        return (unsafe { constexpr_sum_fn_14() });
    }
    pub unsafe fn operator_call_2_int(&self, mut values: ()) -> () {
        return (unsafe { constexpr_sum_fn_15(values) });
    }
    pub unsafe fn operator_call_2_int_long(&self, mut values_0: (), mut values_1: ()) -> () {
        return (unsafe { constexpr_sum_fn_17(values_0, values_1) });
    }
    pub unsafe fn operator_call_2_int_long_int(
        &self,
        mut values_0: (),
        mut values_1: (),
        mut values_2: (),
    ) -> () {
        return (unsafe { constexpr_sum_fn_19(values_0, values_1, values_2) });
    }
    pub unsafe fn operator_call_2_int_int_int(
        &self,
        mut values_0: (),
        mut values_1: (),
        mut values_2: (),
    ) -> () {
        return (unsafe { constexpr_sum_fn_18(values_0, values_1, values_2) });
    }
}
pub unsafe fn test_constexpr_evaluation_20() {
    let sum_obj: ConstexprSumT = <ConstexprSumT>::default();
    {
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 0"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 0");;
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 42"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 42");;
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 143"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 143");;
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 143"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 143");;
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 142"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 142");;
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::apply(static_cast<Fn>(constexpr_sum_fn), t) == 142"
        );;
        const _: () = assert!(true, "std::apply(sum_obj, t) == 142");;
    }
}
pub type CallQuals = ();
pub const CallQuals_CQ_None: CallQuals = 0;
pub const CallQuals_CQ_LValue: CallQuals = 1;
pub const CallQuals_CQ_ConstLValue: CallQuals = 2;
pub const CallQuals_CQ_RValue: CallQuals = 3;
pub const CallQuals_CQ_ConstRValue: CallQuals = 4;
impl CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
    pub unsafe fn new(
        mut q: CallQuals,
        xargs_1: *mut (),
        xargs_2: *const (),
        xargs_3: *mut (),
    ) -> Self {
        let mut this = Self {
            quals: q,
            arg_types: (unsafe { makeArgumentID_10() }),
            args: unimplemented!(),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
    pub quals: CallQuals,
    pub arg_types: *const TypeID,
    pub args: (),
}
impl CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
    pub unsafe fn new(
        mut q: CallQuals,
        xargs_1: *const (),
        xargs_2: *const (),
        xargs_3: *mut (),
    ) -> Self {
        let mut this = Self {
            quals: q,
            arg_types: (unsafe { makeArgumentID_11() }),
            args: unimplemented!(),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
    pub quals: CallQuals,
    pub arg_types: *const TypeID,
    pub args: (),
}
impl CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
    pub unsafe fn new(
        mut q: CallQuals,
        xargs_1: *mut (),
        xargs_2: *const (),
        xargs_3: *mut (),
    ) -> Self {
        let mut this = Self {
            quals: q,
            arg_types: (unsafe { makeArgumentID_12() }),
            args: unimplemented!(),
        };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
    pub quals: CallQuals,
    pub arg_types: *const TypeID,
    pub args: (),
}
impl CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
    pub unsafe fn move_from(
        _a0: *mut CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__,
    ) -> Self {
        let mut this = Self {
            quals: (*_a0).quals,
            arg_types: (*_a0).arg_types,
            args: unimplemented!(),
        };
        this
    }
}
impl CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
    pub unsafe fn new(
        mut q: CallQuals,
        xargs_1: *const (),
        xargs_2: *const (),
        xargs_3: *mut (),
    ) -> Self {
        let mut this = Self {
            quals: q,
            arg_types: (unsafe { makeArgumentID_13() }),
            args: unimplemented!(),
        };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
    pub quals: CallQuals,
    pub arg_types: *const TypeID,
    pub args: (),
}
impl CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
    pub unsafe fn move_from(
        _a0: *mut CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__,
    ) -> Self {
        let mut this = Self {
            quals: (*_a0).quals,
            arg_types: (*_a0).arg_types,
            args: unimplemented!(),
        };
        this
    }
}
pub unsafe fn makeCallInfo_21(
    mut quals: CallQuals,
    args_1: *mut (),
    args_2: *const (),
    args_3: *mut (),
) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
    return CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__::new(
        { quals },
        { args_1 },
        { args_2 },
        { args_3 },
    );
}
pub unsafe fn makeCallInfo_22(
    mut quals: CallQuals,
    args_1: *const (),
    args_2: *const (),
    args_3: *mut (),
) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
    return CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__::new(
        { quals },
        { args_1 },
        { args_2 },
        { args_3 },
    );
}
pub unsafe fn makeCallInfo_23(
    mut quals: CallQuals,
    args_1: *mut (),
    args_2: *const (),
    args_3: *mut (),
) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
    return CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__::new(
        { quals },
        { args_1 },
        { args_2 },
        { args_3 },
    );
}
pub unsafe fn makeCallInfo_24(
    mut quals: CallQuals,
    args_1: *const (),
    args_2: *const (),
    args_3: *mut (),
) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
    return CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__::new(
        { quals },
        { args_1 },
        { args_2 },
        { args_3 },
    );
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TrackedCallable {}
impl TrackedCallable {
    pub unsafe fn operator_call_2_int_ref_const_int_ref_unsigned_int_ref(
        &mut self,
        xargs_0: *mut (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return (unsafe {
            let _args_1: *mut () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_21(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_2_const_int_ref_const_int_ref_unsigned_int_ref(
        &mut self,
        xargs_0: *const (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return (unsafe {
            let _args_1: *const () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_22(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_2_int_const_int_ref_unsigned_int(
        &mut self,
        xargs_0: *mut (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return (unsafe {
            let _args_1: *mut () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_23(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_2_const_int_const_int_ref_unsigned_int(
        &mut self,
        xargs_0: *const (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return (unsafe {
            let _args_1: *const () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_24(CallQuals_CQ_LValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_3_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: *mut (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return (unsafe {
            let _args_1: *mut () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_21(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_3_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: *const (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return (unsafe {
            let _args_1: *const () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_22(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_3_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: *mut (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return (unsafe {
            let _args_1: *mut () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_23(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_3_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: *const (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return (unsafe {
            let _args_1: *const () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_24(CallQuals_CQ_ConstLValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_4_int_ref_const_int_ref_unsigned_int_ref(
        &mut self,
        xargs_0: *mut (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return (unsafe {
            let _args_1: *mut () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_21(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_4_const_int_ref_const_int_ref_unsigned_int_ref(
        &mut self,
        xargs_0: *const (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return (unsafe {
            let _args_1: *const () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_22(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_4_int_const_int_ref_unsigned_int(
        &mut self,
        xargs_0: *mut (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return (unsafe {
            let _args_1: *mut () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_23(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_4_const_int_const_int_ref_unsigned_int(
        &mut self,
        xargs_0: *const (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return (unsafe {
            let _args_1: *const () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_24(CallQuals_CQ_RValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_5_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: *mut (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ {
        return (unsafe {
            let _args_1: *mut () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_21(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_5_const_int_ref_const_int_ref_unsigned_int_ref(
        &self,
        xargs_0: *const (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ {
        return (unsafe {
            let _args_1: *const () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_22(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_5_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: *mut (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ {
        return (unsafe {
            let _args_1: *mut () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_23(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
    pub unsafe fn operator_call_5_const_int_const_int_ref_unsigned_int(
        &self,
        xargs_0: *const (),
        xargs_1: *const (),
        xargs_2: *mut (),
    ) -> CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ {
        return (unsafe {
            let _args_1: *const () = xargs_0;
            let _args_2: *const () = xargs_1;
            let _args_3: *mut () = xargs_2;
            makeCallInfo_24(CallQuals_CQ_ConstRValue, _args_1, _args_2, _args_3)
        });
    }
}
pub unsafe fn check_apply_quals_and_types_25(t: *mut ()) {
    let expect_args: *const TypeID = (unsafe { makeArgumentID_10() });
    let mut obj: TrackedCallable = <TrackedCallable>::default();
    let cobj: *const TrackedCallable = &obj;
    {
        let mut ret: CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_apply_quals_and_types_26(t: *const ()) {
    let expect_args: *const TypeID = (unsafe { makeArgumentID_11() });
    let mut obj: TrackedCallable = <TrackedCallable>::default();
    let cobj: *const TrackedCallable = &obj;
    {
        let mut ret: CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_ref__const_int_ref__unsigned_int_ref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_apply_quals_and_types_27(t: *mut ()) {
    let expect_args: *const TypeID = (unsafe { makeArgumentID_12() });
    let mut obj: TrackedCallable = <TrackedCallable>::default();
    let cobj: *const TrackedCallable = &obj;
    {
        let mut ret: CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn check_apply_quals_and_types_28(t: *const ()) {
    let expect_args: *const TypeID = (unsafe { makeArgumentID_13() });
    let mut obj: TrackedCallable = <TrackedCallable>::default();
    let cobj: *const TrackedCallable = &obj;
    {
        let mut ret: CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: CallInfo_std_tuple_const_int_refref__const_int_ref__unsigned_int_refref__ =
            unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_call_quals_and_arg_types_29() {
    let x: () = 42_();
    let mut y: () = 101_();
    let mut t: () = unimplemented!();
    let ct: *const () = &t;
    (unsafe { check_apply_quals_and_types_25(&mut t) });
    (unsafe { check_apply_quals_and_types_26(ct) });
    (unsafe { check_apply_quals_and_types_27(&mut *(&raw mut (*unimplemented!())).cast::<()>()) });
    (unsafe { check_apply_quals_and_types_28(unimplemented!()) });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NothrowMoveable {}
impl NothrowMoveable {
    pub unsafe fn copy_from(_a0: *const NothrowMoveable) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut NothrowMoveable) -> Self {
        let mut this = Self {};
        this
    }
}
impl Clone for NothrowMoveable {
    fn clone(&self) -> Self {
        unsafe { NothrowMoveable::copy_from(self as *const NothrowMoveable) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestNoexceptCallable_true_ {}
impl TestNoexceptCallable_true_ {
    pub unsafe fn operator_call_1_int_const_char_ptr_long(
        &self,
        mut _a0: (),
        mut _a1: *const (),
        mut _a2: (),
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
    pub unsafe fn operator_call_1_struct_NothrowMoveable_int(
        &self,
        mut _a0: NothrowMoveable,
        mut _a1: (),
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestNoexceptCallable_false_ {}
impl TestNoexceptCallable_false_ {
    pub unsafe fn operator_call_1_int_const_char_ptr_long(
        &self,
        mut _a0: (),
        mut _a1: *const (),
        mut _a2: (),
    ) -> NothrowMoveable {
        return <NothrowMoveable>::default();
    }
}
pub unsafe fn test_noexcept_30() {
    let mut nec: TestNoexceptCallable_true_ = <TestNoexceptCallable_true_>::default();
    let mut tc: TestNoexceptCallable_false_ = <TestNoexceptCallable_false_>::default();
    {
        let mut t: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut t: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub static mut my_int_31: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 42_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_0_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_1_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_2_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_3_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_4_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_5_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_6_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_7_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_8_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_9_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_10_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_11_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_12_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnTypeTest_index_13_ {}
pub unsafe fn f_32(mut _a0: ReturnTypeTest_index_0_) {}
pub unsafe fn f_33(mut _a0: ReturnTypeTest_index_1_) -> () {
    return 0_();
}
pub unsafe fn f_34(mut _a0: ReturnTypeTest_index_2_) -> *mut () {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31));
}
pub unsafe fn f_35(mut _a0: ReturnTypeTest_index_3_) -> *const () {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31));
}
pub unsafe fn f_36(mut _a0: ReturnTypeTest_index_4_) -> *mut () {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31));
}
pub unsafe fn f_37(mut _a0: ReturnTypeTest_index_5_) -> *const () {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31));
}
pub unsafe fn f_38(mut _a0: ReturnTypeTest_index_6_) -> *mut () {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31));
}
pub unsafe fn f_39(mut _a0: ReturnTypeTest_index_7_) -> *const () {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31));
}
pub unsafe fn f_40(mut _a0: ReturnTypeTest_index_8_) -> *mut () {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31));
}
pub unsafe fn f_41(mut _a0: ReturnTypeTest_index_9_) -> *const () {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31));
}
pub unsafe fn f_42(mut _a0: ReturnTypeTest_index_10_) -> *mut () {
    return (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31)) as *mut ());
}
pub unsafe fn f_43(mut _a0: ReturnTypeTest_index_11_) -> *const () {
    return (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31)) as *mut ())
        .cast_const();
}
pub unsafe fn f_44(mut _a0: ReturnTypeTest_index_12_) -> *mut () {
    return (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31)) as *mut ());
}
pub unsafe fn f_45(mut _a0: ReturnTypeTest_index_13_) -> *const () {
    return (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut my_int_31)) as *mut ())
        .cast_const();
}
pub unsafe fn test_46() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_0_)> = Some(f_32);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_47() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_1_) -> ()> = Some(f_33);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_48() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_2_) -> *mut ()> = Some(f_34);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_49() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_3_) -> *const ()> = Some(f_35);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_50() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_4_) -> *mut ()> = Some(f_36);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_51() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_5_) -> *const ()> = Some(f_37);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_52() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_6_) -> *mut ()> = Some(f_38);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_53() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_7_) -> *const ()> = Some(f_39);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_54() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_8_) -> *mut ()> = Some(f_40);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_55() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_9_) -> *const ()> = Some(f_41);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_56() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_10_) -> *mut ()> = Some(f_42);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_57() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_11_) -> *const ()> = Some(f_43);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_58() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_12_) -> *mut ()> = Some(f_44);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_59() {
    const _: () = assert!(true, "std::is_same<RawInvokeResult, Expect>::value");;
    let mut fn_: Option<unsafe fn(ReturnTypeTest_index_13_) -> *const ()> = Some(f_45);
    let mut t: () = unimplemented!();
    (&(t));
    const _: () = assert!(true, "std::is_same<InvokeResult, Expect>::value");;
}
pub unsafe fn test_return_type_60() {
    (unsafe { test_46() });
    (unsafe { test_47() });
    (unsafe { test_48() });
    (unsafe { test_49() });
    (unsafe { test_50() });
    (unsafe { test_51() });
    (unsafe { test_52() });
    (unsafe { test_53() });
    (unsafe { test_54() });
    (unsafe { test_55() });
    (unsafe { test_56() });
    (unsafe { test_57() });
    (unsafe { test_58() });
    (unsafe { test_59() });
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_constexpr_evaluation_20() });
    (unsafe { test_call_quals_and_arg_types_29() });
    (unsafe { test_return_type_60() });
    (unsafe { test_noexcept_30() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const my_int_31);
}
