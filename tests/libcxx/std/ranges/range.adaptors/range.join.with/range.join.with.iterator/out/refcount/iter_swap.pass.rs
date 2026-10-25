extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type SwapKind = ();
pub const SwapKind_no_swap: SwapKind = 0;
pub const SwapKind_with_same_type: SwapKind = 1;
pub const SwapKind_with_different_type: SwapKind = 2;
pub type IterKind = ();
pub const IterKind_inner_view: IterKind = 0;
pub const IterKind_pattern: IterKind = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {
    #[offset(0)]
    #[byte_size(8)]
    iter_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    flag_: Ptr<SwapKind>,
}
impl IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {}
impl Default for IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {
    fn default() -> Self {
        {
            IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {
                iter_: Ptr::<()>::null(),
                flag_: Ptr::<SwapKind>::null(),
            }
        }
    }
}
pub fn iter_swap_0(
    lhs: Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_>,
    rhs: Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_>,
) {
    unimplemented!();
    let __rhs = {
        rhs.with(|__s| __s.flag_.clone())
            .write(SwapKind_with_same_type);
        (rhs.with(|__s| __s.flag_.clone()).read())
    };
    lhs.with(|__s| __s.flag_.clone()).write(__rhs);
    ({
        let ___x: Ptr<()> = field_ptr!(lhs, iter_);
        let ___y: Ptr<()> = field_ptr!(rhs, iter_);
        (*iter_swap_1.with(Value::clone).borrow_mut())(___x, ___y)
    });
    return;
}
pub fn iter_swap_2(
    lhs: Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_>,
    rhs: Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_>,
) {
    unimplemented!();
    let __rhs = {
        ({ IterSwapTrackingIterator_int_ptr__IterKind_pattern_Impl::get_flag(&rhs) })
            .write(SwapKind_with_different_type);
        (({ IterSwapTrackingIterator_int_ptr__IterKind_pattern_Impl::get_flag(&rhs) }).read())
    };
    lhs.with(|__s| __s.flag_.clone()).write(__rhs);
    ({
        let ___y: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ IterSwapTrackingIterator_int_ptr__IterKind_pattern_Impl::get_iter(&rhs) }),
        ));
        (*iter_swap_1.with(Value::clone).borrow_mut())(field_ptr!(lhs, iter_), ___y.as_pointer())
    });
    return;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
    #[offset(0)]
    #[byte_size(8)]
    iter_: (),
    #[offset(8)]
    #[byte_size(8)]
    flag_: Ptr<SwapKind>,
}
impl IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
    pub fn new_1(iter: (), flag: Option<Ptr<SwapKind>>) -> Self {
        let iter: Value<()> = Rc::new(RefCell::new(iter));
        let mut flag: Ptr<SwapKind> = flag.unwrap_or_else(|| Ptr::<SwapKind>::null());
        Self {
            iter_: (*iter.borrow_mut()).clone(),
            flag_: (flag).clone(),
        }
    }
}
impl std::cmp::PartialEq
    for IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_
{
    fn eq(&self, other: &Self) -> bool {
        {
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_Impl::operator_eq(
                &Rc::new(RefCell::new(
                    IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
                        iter_: self.iter_.clone(),
                        flag_: self.flag_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
                        iter_: other.iter_.clone(),
                        flag_: other.flag_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {}
impl Default for IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
    fn default() -> Self {
        {
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
                iter_: unimplemented!(),
                flag_: Ptr::<SwapKind>::null(),
            }
        }
    }
}
pub fn iter_swap_3(
    lhs: Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_>,
    rhs: Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_>,
) {
    unimplemented!();
    let __rhs = {
        rhs.with(|__s| __s.flag_.clone())
            .write(SwapKind_with_same_type);
        (rhs.with(|__s| __s.flag_.clone()).read())
    };
    lhs.with(|__s| __s.flag_.clone()).write(__rhs);
    ({
        let ___x: Ptr<Ptr<()>> = field_ptr!(lhs, iter_);
        let ___y: Ptr<Ptr<()>> = field_ptr!(rhs, iter_);
        (*iter_swap_1.with(Value::clone).borrow_mut())(___x, ___y)
    });
    return;
}
pub fn iter_swap_4(
    lhs: Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_>,
    rhs: Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_>,
) {
    unimplemented!();
    let __rhs = {
        ({
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_Impl::get_flag(
                &rhs,
            )
        })
        .write(SwapKind_with_different_type);
        (({
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_Impl::get_flag(
                &rhs,
            )
        })
        .read())
    };
    lhs.with(|__s| __s.flag_.clone()).write(__rhs);
    ({
        let ___y: Value<()> = Rc::new(RefCell::new(
            ({
                IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_Impl :: get_iter ( &rhs  , )
            }),
        ));
        (*iter_swap_1.with(Value::clone).borrow_mut())(field_ptr!(lhs, iter_), ___y.as_pointer())
    });
    return;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
    #[offset(0)]
    #[byte_size(8)]
    iter_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    flag_: Ptr<SwapKind>,
}
impl IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
    pub fn new_1(mut iter: Ptr<()>, flag: Option<Ptr<SwapKind>>) -> Self {
        let mut flag: Ptr<SwapKind> = flag.unwrap_or_else(|| Ptr::<SwapKind>::null());
        Self {
            iter_: (iter).clone(),
            flag_: (flag).clone(),
        }
    }
}
impl std::cmp::PartialEq for IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
    fn eq(&self, other: &Self) -> bool {
        {
            IterSwapTrackingIterator_int_ptr__IterKind_pattern_Impl::operator_eq(
                &Rc::new(RefCell::new(
                    IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
                        iter_: self.iter_.clone(),
                        flag_: self.flag_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
                        iter_: other.iter_.clone(),
                        flag_: other.flag_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {}
impl Default for IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
    fn default() -> Self {
        {
            IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
                iter_: Ptr::<()>::null(),
                flag_: Ptr::<SwapKind>::null(),
            }
        }
    }
}
const _: () = assert!(
    true,
    "std::forward_iterator<IterSwapTrackingIterator<int*, IterKind::inner_view>> &&\n              !std::bidirectional_iterator<IterSwapTrackingIterator<int*, IterKind::inner_view>>"
);
pub fn test_5() -> () {
    {
        let jwv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        ({ (*advance_6.with(Value::clone).borrow_mut())(it.as_pointer(), 3_()) });
        unimplemented!();
        unimplemented!();
        ({ (*advance_6.with(Value::clone).borrow_mut())(it.as_pointer(), 3_()) });
        let it2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        ({ (*advance_6.with(Value::clone).borrow_mut())(it.as_pointer(), 6_()) });
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "std::is_void_v<decltype(iter_swap(it, it))>");;
        const _: () = assert!(true, "std::is_void_v<decltype(iter_swap(it2, it2))>");;
        const _: () = assert!(!(false), "!CanIterSwap<std::ranges::iterator_t<const JWV>>");;
        const _: () = assert!(
            !(false),
            "!CanIterSwap<const std::ranges::iterator_t<const JWV>>"
        );;
    }
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pat: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v_swap_kind: Value<SwapKind> = Rc::new(RefCell::new(SwapKind_no_swap));
        let tracking_v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pat_swap_kind: Value<SwapKind> = Rc::new(RefCell::new(SwapKind_no_swap));
        let tracking_pat: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let jwv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let it2: Value<()> = Rc::new(RefCell::new(
            ({ (*next_7.with(Value::clone).borrow_mut())((*it1.borrow()).clone()) }),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ (*advance_6.with(Value::clone).borrow_mut())(it2.as_pointer(), 2_()) });
        (*v_swap_kind.borrow_mut()) = SwapKind_no_swap;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ (*advance_6.with(Value::clone).borrow_mut())(it1.as_pointer(), 4_()) });
        (*v_swap_kind.borrow_mut()) = {
            (*pat_swap_kind.borrow_mut()) = SwapKind_no_swap;
            (*pat_swap_kind.borrow())
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ (*advance_6.with(Value::clone).borrow_mut())(it2.as_pointer(), 3_()) });
        (*v_swap_kind.borrow_mut()) = {
            (*pat_swap_kind.borrow_mut()) = SwapKind_no_swap;
            (*pat_swap_kind.borrow())
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(!(false), "!CanIterSwap<std::ranges::iterator_t<JWV>>");;
        const _: () = assert!(!(false), "!CanIterSwap<const std::ranges::iterator_t<JWV>>");;
        const _: () = assert!(!(false), "!CanIterSwap<std::ranges::iterator_t<const JWV>>");;
        const _: () = assert!(
            !(false),
            "!CanIterSwap<const std::ranges::iterator_t<const JWV>>"
        );;
    }
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
    ({ test_5() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait IterSwapTrackingIterator_int_ptr__IterKind_inner_view_Impl {
    fn get_iter(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn get_flag(&self) -> Ptr<SwapKind> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<IterSwapTrackingIterator_int_ptr__IterKind_inner_view_> {
        unimplemented!()
    }
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_eq(
        &self,
        other: Ptr<IterSwapTrackingIterator_int_ptr__IterKind_inner_view_>,
    ) -> () {
        unimplemented!()
    }
}
impl IterSwapTrackingIterator_int_ptr__IterKind_inner_view_Impl
    for Ptr<IterSwapTrackingIterator_int_ptr__IterKind_inner_view_>
{
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.iter_.clone());
    }
}
pub trait IterSwapTrackingIterator_int_ptr__IterKind_pattern_Impl {
    fn get_iter(&self) -> Ptr<()>;
    fn get_flag(&self) -> Ptr<SwapKind>;
    fn operator_inc_5(&self) -> Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_eq(&self, other: Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_>) -> ();
}
impl IterSwapTrackingIterator_int_ptr__IterKind_pattern_Impl
    for Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_>
{
    fn get_iter(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.iter_.clone());
    }
    fn get_flag(&self) -> Ptr<SwapKind> {
        return (*self).with(|__s| __s.flag_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_> {
        field!((*self), iter_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.iter_.clone());
    }
    fn operator_eq(&self, other: Ptr<IterSwapTrackingIterator_int_ptr__IterKind_pattern_>) -> () {
        return ({ (*self).with(|__s| __s.iter_.clone()) } == {
            other.with(|__s| __s.iter_.clone())
        });
    }
}
pub trait IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_Impl {
    fn get_iter(&self) -> ();
    fn get_flag(&self) -> Ptr<SwapKind>;
    fn operator_inc_5(
        &self,
    ) -> Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_>;
    fn operator_post_inc_6(
        &self,
        mut _a0: (),
    ) -> IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_eq(
        &self,
        other: Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_>,
    ) -> ();
}
impl IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_Impl
    for Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_>
{
    fn get_iter(&self) -> () {
        return unimplemented!();
    }
    fn get_flag(&self) -> Ptr<SwapKind> {
        return (*self).with(|__s| __s.flag_.clone());
    }
    fn operator_inc_5(
        &self,
    ) -> Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_> {
        unimplemented!();
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<()> {
        return unimplemented!();
    }
    fn operator_eq(
        &self,
        other: Ptr<IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_>,
    ) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
