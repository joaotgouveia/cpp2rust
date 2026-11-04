extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static dummy_0: Value<i32> = Rc::new(RefCell::new(42));
);
pub fn return_value_1(mut _a0: i32) -> i32 {
    return dummy_0.with(|rc| *rc.borrow());
}
pub fn return_lvalue_2(mut _a0: i32) -> Ptr<i32> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_const_lvalue_3(mut _a0: i32) -> Ptr<i32> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_rvalue_4(mut _a0: i32) -> Ptr<i32> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_const_rvalue_5(mut _a0: i32) -> Ptr<i32> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn CheckCall_6() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_8() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_9() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_10() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_11() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_12() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_13() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_14() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_15() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_16() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_17() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_18() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_19() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_20() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_21() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_22() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_23() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_24() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_25() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_26() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_27() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_28() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_29() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_30() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_31() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_32() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_33() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_34() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_35() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn CheckCall_36() -> bool {
    return value_7.with(|rc| *rc.borrow());
}
pub fn do_test_37(func: FnPtr<fn(i32) -> i32>) {
    let func: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_int__ptr_std__Placeholder_1____int__> = Rc::new(RefCell::new(
        ({ bind_38(func.as_pointer(), _1_39.with(|v| v.as_pointer())) }),
    ));
    let ret_r: Value<std__Bind_result_int__int__ptr_std__Placeholder_1____int__> = Rc::new(
        RefCell::new(({ bind_40(func.as_pointer(), _1_39.with(|v| v.as_pointer())) })),
    );
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Value<i32> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(100));
            (*ret.borrow_mut())(___args.as_pointer())
        })
        .clone(),
    ));
    let exp2: Value<i32> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
            (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
        })
        .clone(),
    ));
    let exp_r: Value<i32> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(100));
            (*ret_r.borrow_mut())(___args.as_pointer())
        })
        .clone(),
    ));
    let exp_r2: Value<i32> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
            (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
        })
        .clone(),
    ));
    assert!(({ ((*exp.borrow()) == 42) }));
    assert!(({ ((*exp2.borrow()) == 42) }));
    assert!(({ ((*exp_r.borrow()) == 42) }));
    assert!(({ ((*exp_r2.borrow()) == 42) }));
    {}
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_41(func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let func: Value<FnPtr<fn(i32) -> Ptr<i32>>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_int_ref_ptr_std__Placeholder_1____int__> = Rc::new(RefCell::new(
        ({ bind_42(func.as_pointer(), _1_39.with(|v| v.as_pointer())) }),
    ));
    let ret_r: Value<std__Bind_result_int_ref__int_ref_ptr_std__Placeholder_1____int__> = Rc::new(
        RefCell::new(({ bind_43(func.as_pointer(), _1_39.with(|v| v.as_pointer())) })),
    );
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!(((exp.read()) == 42));
    assert!(((exp2.read()) == 42));
    assert!(((exp_r.read()) == 42));
    assert!(((exp_r2.read()) == 42));
    {
        assert!(({ (exp).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_44(func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let func: Value<FnPtr<fn(i32) -> Ptr<i32>>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_const_int_ref_ptr_std__Placeholder_1____int__> = Rc::new(
        RefCell::new(({ bind_45(func.as_pointer(), _1_39.with(|v| v.as_pointer())) })),
    );
    let ret_r: Value<
        std__Bind_result_const_int_ref__const_int_ref_ptr_std__Placeholder_1____int__,
    > = Rc::new(RefCell::new(
        ({ bind_46(func.as_pointer(), _1_39.with(|v| v.as_pointer())) }),
    ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!(((exp.read()) == 42));
    assert!(((exp2.read()) == 42));
    assert!(((exp_r.read()) == 42));
    assert!(((exp_r2.read()) == 42));
    {
        assert!(({ (exp).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_47(func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let func: Value<FnPtr<fn(i32) -> Ptr<i32>>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_int_refref_ptr_std__Placeholder_1____int__> = Rc::new(RefCell::new(
        ({ bind_48(func.as_pointer(), _1_39.with(|v| v.as_pointer())) }),
    ));
    let ret_r: Value<std__Bind_result_int_refref__int_refref_ptr_std__Placeholder_1____int__> =
        Rc::new(RefCell::new(
            ({ bind_49(func.as_pointer(), _1_39.with(|v| v.as_pointer())) }),
        ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!(((exp.read()) == 42));
    assert!(((exp2.read()) == 42));
    assert!(((exp_r.read()) == 42));
    assert!(((exp_r2.read()) == 42));
    {
        assert!(({ (exp).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_50(func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let func: Value<FnPtr<fn(i32) -> Ptr<i32>>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_const_int_refref_ptr_std__Placeholder_1____int__> = Rc::new(
        RefCell::new(({ bind_51(func.as_pointer(), _1_39.with(|v| v.as_pointer())) })),
    );
    let ret_r: Value<
        std__Bind_result_const_int_refref__const_int_refref_ptr_std__Placeholder_1____int__,
    > = Rc::new(RefCell::new(
        ({ bind_52(func.as_pointer(), _1_39.with(|v| v.as_pointer())) }),
    ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!(((exp.read()) == 42));
    assert!(((exp2.read()) == 42));
    assert!(((exp_r.read()) == 42));
    assert!(((exp_r2.read()) == 42));
    {
        assert!(({ (exp).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_r_53(func: FnPtr<fn(i32) -> i32>) {
    let func: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_result_long__int__ptr_std__Placeholder_1____int__> = Rc::new(
        RefCell::new(({ bind_54(func.as_pointer(), _1_39.with(|v| v.as_pointer())) })),
    );
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_55(func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let func: Value<FnPtr<fn(i32) -> Ptr<i32>>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_result_long__int_ref_ptr_std__Placeholder_1____int__> = Rc::new(
        RefCell::new(({ bind_56(func.as_pointer(), _1_39.with(|v| v.as_pointer())) })),
    );
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_57(func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let func: Value<FnPtr<fn(i32) -> Ptr<i32>>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_result_long__const_int_ref_ptr_std__Placeholder_1____int__> = Rc::new(
        RefCell::new(({ bind_58(func.as_pointer(), _1_39.with(|v| v.as_pointer())) })),
    );
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_59(func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let func: Value<FnPtr<fn(i32) -> Ptr<i32>>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_result_long__int_refref_ptr_std__Placeholder_1____int__> = Rc::new(
        RefCell::new(({ bind_60(func.as_pointer(), _1_39.with(|v| v.as_pointer())) })),
    );
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_61(func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let func: Value<FnPtr<fn(i32) -> Ptr<i32>>> = Rc::new(RefCell::new(func));
    let ret: Value<std__Bind_result_long__const_int_refref_ptr_std__Placeholder_1____int__> =
        Rc::new(RefCell::new(
            ({ bind_62(func.as_pointer(), _1_39.with(|v| v.as_pointer())) }),
        ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
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
    ({ do_test_37(FnPtr::<fn(i32) -> i32>::new(return_value_1)) });
    ({ do_test_41(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_lvalue_2)) });
    ({ do_test_44(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_const_lvalue_3)) });
    ({ do_test_47(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_rvalue_4)) });
    ({ do_test_50(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_const_rvalue_5)) });
    ({ do_test_r_53(FnPtr::<fn(i32) -> i32>::new(return_value_1)) });
    ({ do_test_r_55(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_lvalue_2)) });
    ({ do_test_r_57(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_const_lvalue_3)) });
    ({ do_test_r_59(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_rvalue_4)) });
    ({ do_test_r_61(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_const_rvalue_5)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = dummy_0.with(|_| ());
}
