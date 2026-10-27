extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_all_0() -> () {
    let in_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let in2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let output: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let output2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let eq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let less: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sum: Value<FnPtr<fn((), A_1) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn((), A_1) -> ()>::new(|mut lhs: (), mut rhs: A_1| -> () {
            {
                return (lhs + rhs.x);
            }
        }),
    ));
    let proj1: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut x: ()| -> () {
            {
                return (x * -1_());
            }
        },
    )));
    let proj2: Value<FnPtr<fn(A_1) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(A_1) -> ()>::new(
        |mut a: A_1| -> () {
            {
                return (a.x * -1_());
            }
        },
    )));
    ({
        test_2(
            ends_with_3.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_4(
            equal_5.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_6(
            lexicographical_compare_7.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_8(
            is_permutation_9.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_10(
            includes_11.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_12(
            find_first_of_13.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_14(
            mismatch_15.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_16(
            search_17.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_18(
            find_end_19.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_20(
            transform_21.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
            sum.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_22(
            transform_21.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out2.as_pointer(),
            sum.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_23(
            partial_sort_copy_24.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_25(
            merge_26.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_27(
            merge_26.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out2.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_28(
            set_intersection_29.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_30(
            set_intersection_29.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out2.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_31(
            set_difference_32.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_33(
            set_difference_32.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out2.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_34(
            set_symmetric_difference_35.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_36(
            set_symmetric_difference_35.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out2.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_37(
            set_union_38.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_39(
            set_union_38.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            out2.as_pointer(),
            less.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    ({
        test_40(
            starts_with_41.with(|v| v.as_pointer()),
            in_.as_pointer(),
            in2.as_pointer(),
            eq.as_pointer(),
            proj1.as_pointer(),
            proj2.as_pointer(),
        )
    });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A_1 {
    #[offset(0)]
    pub x: (),
}
impl A_1 {
    pub fn new_1(mut value: ()) -> Self {
        Self { x: value }
    }
}
impl std::cmp::Ord for A_1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            A_1Impl::operator_cmp(
                &Rc::new(RefCell::new(A_1 { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(A_1 { x: other.x.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for A_1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for A_1 {
    fn eq(&self, other: &Self) -> bool {
        {
            A_1Impl::operator_cmp(
                &Rc::new(RefCell::new(A_1 { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(A_1 { x: other.x.clone() })).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for A_1 {}
impl Default for A_1 {
    fn default() -> Self {
        { A_1 { x: 0_() } }
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
    ({ test_all_0() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub trait A_1Impl {
    fn to_(&self) -> ();
    fn operator_cmp(&self, _a0: Ptr<A_1>) -> ();
    fn operator_eq(&self, _a0: Ptr<A_1>) -> ();
}
impl A_1Impl for Ptr<A_1> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.x);
    }
    fn operator_cmp(&self, _a0: Ptr<A_1>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.x)),
                &(_a0.with(|__s| __s.x)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<A_1>) -> () {
        return ({ (*self).with(|__s| __s.x) } == { _a0.with(|__s| __s.x) });
    }
}
pub fn __cpp2rust_init_globals() {}
