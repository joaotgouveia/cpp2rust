extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntType {
    #[offset(0)]
    pub val: (),
}
impl IntType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        {
            IntTypeImpl::operator_eq(
                &Rc::new(RefCell::new(IntType {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(IntType {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for IntType {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntTypeNC {
    #[offset(0)]
    pub val: (),
}
impl IntTypeNC {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
pub fn check_operator_constraints_0(m: (), mut idxs: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_1(m: (), mut idxs: IntType) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_2(_a0: (), mut _a1: (), mut _a2: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_3(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_4(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn iterate_stride_5(m: (), strides: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        const _: () = assert!(true, "");;
        let mut expected_val: () = ({
            lambda!(
                Generic,
                {
                    let strides: Ptr<()> = (strides).clone();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    return 0_();
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(<()>::default())
        });
        unimplemented!();
    }
}
pub fn iterate_stride_6(m: (), strides: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args: () = i;
                iterate_stride_7(_m, _strides, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_7(m: (), strides: Ptr<()>, args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let args: Value<()> = Rc::new(RefCell::new(args));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        const _: () = assert!(true, "");;
        let mut expected_val: () = ({
            lambda!(
                Generic,
                {
                    let args: Ptr<()> = args.as_pointer();
                    let strides: Ptr<()> = (strides).clone();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    return ((args.read()).wrapping_mul((unimplemented!().read())))
                        .wrapping_add(0_());
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(<()>::default())
        });
        unimplemented!();
    }
}
pub fn iterate_stride_8(m: (), strides: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args: () = i;
                iterate_stride_9(_m, _strides, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_9(m: (), strides: Ptr<()>, args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let args: Value<()> = Rc::new(RefCell::new(args));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        const _: () = assert!(true, "");;
        let mut expected_val: () = ({
            lambda!(
                Generic,
                {
                    let args: Ptr<()> = args.as_pointer();
                    let strides: Ptr<()> = (strides).clone();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    return ((args.read()).wrapping_mul((unimplemented!().read())))
                        .wrapping_add(0_());
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(<()>::default())
        });
        unimplemented!();
    }
}
pub fn iterate_stride_10(m: (), strides: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args: () = i;
                iterate_stride_11(_m, _strides, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_11(m: (), strides: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_stride_12(_m, _strides, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_12(m: (), strides: Ptr<()>, args_2: (), args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let args_2: Value<()> = Rc::new(RefCell::new(args_2));
    let args_3: Value<()> = Rc::new(RefCell::new(args_3));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        const _: () = assert!(true, "");;
        let mut expected_val: () = ({
            lambda!(
                Generic,
                {
                    let args_2: Ptr<()> = args_2.as_pointer();
                    let args_3: Ptr<()> = args_3.as_pointer();
                    let strides: Ptr<()> = (strides).clone();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    return ((args_2.read()).wrapping_mul((unimplemented!().read()))).wrapping_add(
                        ((args_3.read()).wrapping_mul((unimplemented!().read())))
                            .wrapping_add(0_()),
                    );
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(<()>::default())
        });
        unimplemented!();
    }
}
pub fn iterate_stride_13(m: (), strides: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args: () = i;
                iterate_stride_14(_m, _strides, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_14(m: (), strides: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_stride_15(_m, _strides, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_15(m: (), strides: Ptr<()>, mut args_2: (), mut args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_stride_16(_m, _strides, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_16(m: (), strides: Ptr<()>, mut args_2: (), mut args_3: (), mut args_4: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_stride_17(_m, _strides, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_17(m: (), strides: Ptr<()>, args_2: (), args_3: (), args_4: (), args_5: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let args_2: Value<()> = Rc::new(RefCell::new(args_2));
    let args_3: Value<()> = Rc::new(RefCell::new(args_3));
    let args_4: Value<()> = Rc::new(RefCell::new(args_4));
    let args_5: Value<()> = Rc::new(RefCell::new(args_5));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        const _: () = assert!(true, "");;
        let mut expected_val: () = ({
            lambda!(
                Generic,
                {
                    let args_2: Ptr<()> = args_2.as_pointer();
                    let args_3: Ptr<()> = args_3.as_pointer();
                    let args_4: Ptr<()> = args_4.as_pointer();
                    let args_5: Ptr<()> = args_5.as_pointer();
                    let strides: Ptr<()> = (strides).clone();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    return ({ ({ (args_2.read()) } * { (unimplemented!().read()) }) } + {
                        ({ ({ (args_3.read()) } * { (unimplemented!().read()) }) } + {
                            ({ ({ (args_4.read()) } * { (unimplemented!().read()) }) } + {
                                (({ (args_5.read()) } * { (unimplemented!().read()) }) + 0_())
                            })
                        })
                    });
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(<()>::default())
        });
        unimplemented!();
    }
}
pub fn iterate_stride_18(m: (), strides: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args: () = i;
                iterate_stride_19(_m, _strides, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_19(m: (), strides: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_stride_20(_m, _strides, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_20(m: (), strides: Ptr<()>, mut args_2: (), mut args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_stride_21(_m, _strides, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_21(m: (), strides: Ptr<()>, mut args_2: (), mut args_3: (), mut args_4: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_stride_22(_m, _strides, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_22(m: (), strides: Ptr<()>, args_2: (), args_3: (), args_4: (), args_5: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let args_2: Value<()> = Rc::new(RefCell::new(args_2));
    let args_3: Value<()> = Rc::new(RefCell::new(args_3));
    let args_4: Value<()> = Rc::new(RefCell::new(args_4));
    let args_5: Value<()> = Rc::new(RefCell::new(args_5));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        const _: () = assert!(true, "");;
        let mut expected_val: () = ({
            lambda!(
                Generic,
                {
                    let args_2: Ptr<()> = args_2.as_pointer();
                    let args_3: Ptr<()> = args_3.as_pointer();
                    let args_4: Ptr<()> = args_4.as_pointer();
                    let args_5: Ptr<()> = args_5.as_pointer();
                    let strides: Ptr<()> = (strides).clone();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    return ({ ({ (args_2.read()) } * { (unimplemented!().read()) }) } + {
                        ({ ({ (args_3.read()) } * { (unimplemented!().read()) }) } + {
                            ({ ({ (args_4.read()) } * { (unimplemented!().read()) }) } + {
                                (({ (args_5.read()) } * { (unimplemented!().read()) }) + 0_())
                            })
                        })
                    });
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(<()>::default())
        });
        unimplemented!();
    }
}
pub fn iterate_stride_23(m: (), strides: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args: () = i;
                iterate_stride_24(_m, _strides, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_24(m: (), strides: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_stride_25(_m, _strides, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_25(m: (), strides: Ptr<()>, mut args_2: (), mut args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_stride_26(_m, _strides, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_26(m: (), strides: Ptr<()>, mut args_2: (), mut args_3: (), mut args_4: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _strides: Ptr<()> = (strides).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_stride_27(_m, _strides, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_stride_27(m: (), strides: Ptr<()>, args_2: (), args_3: (), args_4: (), args_5: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let args_2: Value<()> = Rc::new(RefCell::new(args_2));
    let args_3: Value<()> = Rc::new(RefCell::new(args_3));
    let args_4: Value<()> = Rc::new(RefCell::new(args_4));
    let args_5: Value<()> = Rc::new(RefCell::new(args_5));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        const _: () = assert!(true, "");;
        let mut expected_val: () = ({
            lambda!(
                Generic,
                {
                    let args_2: Ptr<()> = args_2.as_pointer();
                    let args_3: Ptr<()> = args_3.as_pointer();
                    let args_4: Ptr<()> = args_4.as_pointer();
                    let args_5: Ptr<()> = args_5.as_pointer();
                    let strides: Ptr<()> = (strides).clone();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    return ({ ({ (args_2.read()) } * { (unimplemented!().read()) }) } + {
                        ({ ({ (args_3.read()) } * { (unimplemented!().read()) }) } + {
                            ({ ({ (args_4.read()) } * { (unimplemented!().read()) }) } + {
                                (({ (args_5.read()) } * { (unimplemented!().read()) }) + 0_())
                            })
                        })
                    });
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(<()>::default())
        });
        unimplemented!();
    }
}
pub fn test_iteration_28(strides: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_stride_5(unimplemented!(), strides.as_pointer()) });
}
pub fn test_iteration_29(strides: (), mut args: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_stride_6(unimplemented!(), strides.as_pointer()) });
}
pub fn test_iteration_30(strides: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_stride_8(unimplemented!(), strides.as_pointer()) });
}
pub fn test_iteration_31(strides: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_stride_10(unimplemented!(), strides.as_pointer()) });
}
pub fn test_iteration_32(
    strides: (),
    mut args_1: (),
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_stride_13(unimplemented!(), strides.as_pointer()) });
}
pub fn test_iteration_33(strides: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_stride_18(unimplemented!(), strides.as_pointer()) });
}
pub fn test_iteration_34(strides: (), mut args_1: (), mut args_2: ()) {
    let strides: Value<()> = Rc::new(RefCell::new(strides));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_stride_23(unimplemented!(), strides.as_pointer()) });
}
pub fn test_35() -> () {
    let mut D: () = dynamic_extent_36.with(|rc| *rc.borrow());
    ({ test_iteration_28(unimplemented!()) });
    ({ test_iteration_29(unimplemented!(), 1_()) });
    ({ test_iteration_29(unimplemented!(), 7_()) });
    ({ test_iteration_30(unimplemented!()) });
    ({ test_iteration_31(unimplemented!()) });
    ({ test_iteration_32(unimplemented!(), 1_(), 1_(), 1_(), 1_()) });
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::layout_stride::mapping<std::extents<int, D>>(std::extents<int, D>(1), std::array{{1}}), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::layout_stride::mapping<std::extents<int, D>>(std::extents<int, D>(1), std::array{{1}}), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::layout_stride::mapping<std::extents<int, D>>(std::extents<int, D>(1), std::array{{1}}), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::layout_stride::mapping<std::extents<unsigned, D>>(std::extents<unsigned, D>(1), std::array{{1}}), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::layout_stride::mapping<std::extents<unsigned char, D>>(std::extents<unsigned char, D>(1), std::array{{1}}),\n      IntType(0))"
    );;
    return true;
}
pub fn test_large_37() -> () {
    let mut D: () = dynamic_extent_36.with(|rc| *rc.borrow());
    ({ test_iteration_33(unimplemented!(), 7_(), 9_(), 10_()) });
    ({ test_iteration_34(unimplemented!(), 7_(), 10_()) });
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
    ({ test_35() });
    const _: () = assert!(true, "test()");;
    ({ test_large_37() });
    return 0_();
}
pub trait IntTypeImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeImpl for Ptr<IntType> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntTypeNCImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeNCImpl for Ptr<IntTypeNC> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub fn __cpp2rust_init_globals() {}
