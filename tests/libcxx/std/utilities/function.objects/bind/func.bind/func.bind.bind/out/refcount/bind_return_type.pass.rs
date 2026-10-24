extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static dummy_0: Value<()> = Rc::new(RefCell::new(42_()));
);
pub fn return_value_1(mut _a0: ()) -> () {
    return dummy_0.with(|rc| *rc.borrow());
}
pub fn return_lvalue_2(mut _a0: ()) -> Ptr<()> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_const_lvalue_3(mut _a0: ()) -> Ptr<()> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_rvalue_4(mut _a0: ()) -> Ptr<()> {
    return (unimplemented!()).reinterpret_cast::<()>();
}
pub fn return_const_rvalue_5(mut _a0: ()) -> Ptr<()> {
    return (unimplemented!()).reinterpret_cast::<()>();
}
pub fn CheckCall_6() -> () {
    return unimplemented!();
}
pub fn CheckCall_7() -> () {
    return unimplemented!();
}
pub fn CheckCall_8() -> () {
    return unimplemented!();
}
pub fn CheckCall_9() -> () {
    return unimplemented!();
}
pub fn CheckCall_10() -> () {
    return unimplemented!();
}
pub fn CheckCall_11() -> () {
    return unimplemented!();
}
pub fn CheckCall_12() -> () {
    return unimplemented!();
}
pub fn CheckCall_13() -> () {
    return unimplemented!();
}
pub fn CheckCall_14() -> () {
    return unimplemented!();
}
pub fn CheckCall_15() -> () {
    return unimplemented!();
}
pub fn CheckCall_16() -> () {
    return unimplemented!();
}
pub fn CheckCall_17() -> () {
    return unimplemented!();
}
pub fn CheckCall_18() -> () {
    return unimplemented!();
}
pub fn CheckCall_19() -> () {
    return unimplemented!();
}
pub fn CheckCall_20() -> () {
    return unimplemented!();
}
pub fn CheckCall_21() -> () {
    return unimplemented!();
}
pub fn CheckCall_22() -> () {
    return unimplemented!();
}
pub fn CheckCall_23() -> () {
    return unimplemented!();
}
pub fn CheckCall_24() -> () {
    return unimplemented!();
}
pub fn CheckCall_25() -> () {
    return unimplemented!();
}
pub fn CheckCall_26() -> () {
    return unimplemented!();
}
pub fn CheckCall_27() -> () {
    return unimplemented!();
}
pub fn CheckCall_28() -> () {
    return unimplemented!();
}
pub fn CheckCall_29() -> () {
    return unimplemented!();
}
pub fn CheckCall_30() -> () {
    return unimplemented!();
}
pub fn CheckCall_31() -> () {
    return unimplemented!();
}
pub fn CheckCall_32() -> () {
    return unimplemented!();
}
pub fn CheckCall_33() -> () {
    return unimplemented!();
}
pub fn CheckCall_34() -> () {
    return unimplemented!();
}
pub fn CheckCall_35() -> () {
    return unimplemented!();
}
pub fn CheckCall_36() -> () {
    return unimplemented!();
}
pub fn CheckCall_37() -> () {
    return unimplemented!();
}
pub fn CheckCall_38() -> () {
    return unimplemented!();
}
pub fn CheckCall_39() -> () {
    return unimplemented!();
}
pub fn CheckCall_40() -> () {
    return unimplemented!();
}
pub fn CheckCall_41() -> () {
    return unimplemented!();
}
pub fn CheckCall_42() -> () {
    return unimplemented!();
}
pub fn CheckCall_43() -> () {
    return unimplemented!();
}
pub fn CheckCall_44() -> () {
    return unimplemented!();
}
pub fn CheckCall_45() -> () {
    return unimplemented!();
}
pub fn CheckCall_46() -> () {
    return unimplemented!();
}
pub fn CheckCall_47() -> () {
    return unimplemented!();
}
pub fn CheckCall_48() -> () {
    return unimplemented!();
}
pub fn CheckCall_49() -> () {
    return unimplemented!();
}
pub fn CheckCall_50() -> () {
    return unimplemented!();
}
pub fn do_test_51(func: FnPtr<fn(()) -> ()>) {
    let func: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(func));
    let ret: Value<std___bind_int__ptrref__int___const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<std___bind_r_int__int__ptrref__int___const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(
            ({ bind_52(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
        ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Value<()> = Rc::new(RefCell::new(
        (({
            let ___args: Value<()> = Rc::new(RefCell::new(100_()));
            (*ret.borrow_mut())(___args.as_pointer())
        }) as ()),
    ));
    let exp2: Value<()> = Rc::new(RefCell::new(
        (({
            let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
            let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
            (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
        }) as ()),
    ));
    let exp_r: Value<()> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(100_()));
            (*ret_r.borrow_mut())(___args.as_pointer())
        })
        .clone(),
    ));
    let exp_r2: Value<()> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
            let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
            (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
        })
        .clone(),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_54(func: FnPtr<fn(()) -> Ptr<()>>) {
    let func: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(func));
    let ret: Value<std___bind_int_ref_ptrref__int___const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<
        std___bind_r_int_ref__int_ref_ptrref__int___const_std_placeholders___ph_1__ref_,
    > = Rc::new(RefCell::new(
        ({ bind_55(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
    ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_56(func: FnPtr<fn(()) -> Ptr<()>>) {
    let func: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(func));
    let ret: Value<std___bind_const_int_ref_ptrref__int___const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<
        std___bind_r_const_int_ref__const_int_ref_ptrref__int___const_std_placeholders___ph_1__ref_,
    > = Rc::new(RefCell::new(
        ({ bind_57(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
    ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_58(func: FnPtr<fn(()) -> Ptr<()>>) {
    let func: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(func));
    let ret: Value<std___bind_int_refref_ptrref__int___const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<
        std___bind_r_int_refref__int_refref_ptrref__int___const_std_placeholders___ph_1__ref_,
    > = Rc::new(RefCell::new(
        ({ bind_59(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
    ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_60(func: FnPtr<fn(()) -> Ptr<()>>) {
    let func: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(func));
    let ret: Value<std___bind_const_int_refref_ptrref__int___const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(unimplemented!()));
    let ret_r : Value<std___bind_r_const_int_refref__const_int_refref_ptrref__int___const_std_placeholders___ph_1__ref_ > = Rc::new(RefCell::new(( { bind_61 ( func .as_pointer()  , _1_53.with(|v| v.as_pointer())  , ) } )  )) ;
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_r_62(func: FnPtr<fn(()) -> ()>) {
    let func: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(func));
    let ret: Value<std___bind_r_long__int__ptrref__int___const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(
            ({ bind_63(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
        ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_64(func: FnPtr<fn(()) -> Ptr<()>>) {
    let func: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(func));
    let ret: Value<std___bind_r_long__int_ref_ptrref__int___const_std_placeholders___ph_1__ref_> =
        Rc::new(RefCell::new(
            ({ bind_65(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
        ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_66(func: FnPtr<fn(()) -> Ptr<()>>) {
    let func: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(func));
    let ret: Value<
        std___bind_r_long__const_int_ref_ptrref__int___const_std_placeholders___ph_1__ref_,
    > = Rc::new(RefCell::new(
        ({ bind_67(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
    ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_68(func: FnPtr<fn(()) -> Ptr<()>>) {
    let func: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(func));
    let ret: Value<
        std___bind_r_long__int_refref_ptrref__int___const_std_placeholders___ph_1__ref_,
    > = Rc::new(RefCell::new(
        ({ bind_69(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
    ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_70(func: FnPtr<fn(()) -> Ptr<()>>) {
    let func: Value<FnPtr<fn(()) -> Ptr<()>>> = Rc::new(RefCell::new(func));
    let ret: Value<
        std___bind_r_long__const_int_refref_ptrref__int___const_std_placeholders___ph_1__ref_,
    > = Rc::new(RefCell::new(
        ({ bind_71(func.as_pointer(), _1_53.with(|v| v.as_pointer())) }),
    ));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ do_test_51(FnPtr::<fn(()) -> ()>::new(return_value_1)) });
    ({ do_test_54(FnPtr::<fn(()) -> Ptr<()>>::new(return_lvalue_2)) });
    ({ do_test_56(FnPtr::<fn(()) -> Ptr<()>>::new(return_const_lvalue_3)) });
    ({ do_test_58(FnPtr::<fn(()) -> Ptr<()>>::new(return_rvalue_4)) });
    ({ do_test_60(FnPtr::<fn(()) -> Ptr<()>>::new(return_const_rvalue_5)) });
    ({ do_test_r_62(FnPtr::<fn(()) -> ()>::new(return_value_1)) });
    ({ do_test_r_64(FnPtr::<fn(()) -> Ptr<()>>::new(return_lvalue_2)) });
    ({ do_test_r_66(FnPtr::<fn(()) -> Ptr<()>>::new(return_const_lvalue_3)) });
    ({ do_test_r_68(FnPtr::<fn(()) -> Ptr<()>>::new(return_rvalue_4)) });
    ({ do_test_r_70(FnPtr::<fn(()) -> Ptr<()>>::new(return_const_rvalue_5)) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = dummy_0.with(|_| ());
}
