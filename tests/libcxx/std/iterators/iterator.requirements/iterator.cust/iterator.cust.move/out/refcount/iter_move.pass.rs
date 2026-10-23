extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn iter_move_0(
    i: Ptr<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_>,
) -> Ptr<move_tracker> {
    return ({ (*iter_move_1.with(Value::clone).borrow_mut())(field_ptr!(i, base_)) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<move_tracker>,
}
impl check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
    pub fn new_1(i: Ptr<move_tracker>) -> Self {
        let i: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(i));
        Self {
            base_: (*i.borrow()).clone(),
        }
    }
}
impl std::cmp::PartialEq for check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_Impl::operator_eq(
                &Rc::new(RefCell::new(
                    check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
                        base_: self.base_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
                        base_: other.base_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {}
impl Default for check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
    fn default() -> Self {
        {
            check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
                base_: Ptr::<move_tracker>::null(),
            }
        }
    }
}
pub type check_unqualified_lookup_unscoped_enum = ();
pub const check_unqualified_lookup_unscoped_enum_a: check_unqualified_lookup_unscoped_enum = 0;
pub const check_unqualified_lookup_unscoped_enum_b: check_unqualified_lookup_unscoped_enum = 1;
pub const check_unqualified_lookup_unscoped_enum_c: check_unqualified_lookup_unscoped_enum = 2;
pub fn iter_move_2(
    e: Ptr<check_unqualified_lookup_unscoped_enum>,
) -> check_unqualified_lookup_unscoped_enum {
    return (e.read());
}
pub type check_unqualified_lookup_scoped_enum = ();
pub const check_unqualified_lookup_scoped_enum_a: check_unqualified_lookup_scoped_enum = 0;
pub const check_unqualified_lookup_scoped_enum_b: check_unqualified_lookup_scoped_enum = 1;
pub const check_unqualified_lookup_scoped_enum_c: check_unqualified_lookup_scoped_enum = 2;
pub fn iter_move_3(
    _a0: Ptr<check_unqualified_lookup_scoped_enum>,
) -> Ptr<check_unqualified_lookup_scoped_enum> {
    return Ptr::<check_unqualified_lookup_scoped_enum>::null();
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct check_unqualified_lookup_some_union {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl check_unqualified_lookup_some_union {
    pub fn x(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn y(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for check_unqualified_lookup_some_union {
    fn default() -> Self {
        check_unqualified_lookup_some_union {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
pub fn iter_move_4(u: Ptr<check_unqualified_lookup_some_union>) -> () {
    return (check_unqualified_lookup_some_union::x(u.clone()).read());
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct move_tracker {
    #[offset(0)]
    moves_: (),
}
impl move_tracker {
    pub fn move_from(other: Ptr<move_tracker>) -> Self {
        let __this: move_tracker = Self {
            moves_: (other.with(|__s| __s.moves_) + 1_()),
        };
        field!(other, moves_).write(0_());
        __this
    }
}
impl Default for move_tracker {
    fn default() -> Self {
        { move_tracker { moves_: 0_() } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct iterator_wrapper_move_tracker_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    base_: Ptr<move_tracker>,
}
impl iterator_wrapper_move_tracker_ptr_ {
    pub fn new_1(i: Ptr<move_tracker>) -> Self {
        let i: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(i));
        Self {
            base_: (*i.borrow()).clone(),
        }
    }
}
impl std::cmp::PartialEq for iterator_wrapper_move_tracker_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            iterator_wrapper_move_tracker_ptr_Impl::operator_eq(
                &Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_ {
                    base_: self.base_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_ {
                    base_: other.base_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for iterator_wrapper_move_tracker_ptr_ {}
impl Default for iterator_wrapper_move_tracker_ptr_ {
    fn default() -> Self {
        {
            iterator_wrapper_move_tracker_ptr_ {
                base_: Ptr::<move_tracker>::null(),
            }
        }
    }
}
pub fn unqualified_lookup_move_5(
    first_: Ptr<move_tracker>,
    last_: Ptr<move_tracker>,
    result_first_: Ptr<move_tracker>,
    result_last_: Ptr<move_tracker>,
) {
    let first_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(first_));
    let last_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(last_));
    let result_first_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(result_first_));
    let result_last_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(result_last_));
    let first: Value<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(
            check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::new_1({
                (*first_.borrow()).clone()
            }),
        ));
    let last: Value<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(
            check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::new_1({
                (*last_.borrow()).clone()
            }),
        ));
    let result_first: Value<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(
            check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::new_1({
                (*result_first_.borrow()).clone()
            }),
        ));
    let result_last: Value<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(
            check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::new_1({
                (*result_last_.borrow()).clone()
            }),
        ));
    const _: () = assert!(true, "!noexcept(std::ranges::iter_move(first))");;
    'loop_: while (!({
        check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_Impl::operator_eq(
            &first.as_pointer(),
            last.as_pointer(),
        )
    })) && (!({
        check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_Impl::operator_eq(
            &result_first.as_pointer(),
            result_last.as_pointer(),
        )
    })) {
        ({
            move_trackerImpl::move_assign(
                &({
                    check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_Impl :: operator_deref ( &result_first .as_pointer()  , )
                }),
                ({ (*iter_move_1.with(Value::clone).borrow_mut())(first.as_pointer()) }),
            )
        });
        {
            &({
                check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_Impl :: operator_inc_4 ( &first .as_pointer()  , )
            });
            ({
                check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_Impl :: operator_inc_4 ( &result_first .as_pointer()  , )
            })
        };
    }
}
pub fn lvalue_move_6(
    first_: Ptr<move_tracker>,
    last_: Ptr<move_tracker>,
    result_first_: Ptr<move_tracker>,
    result_last_: Ptr<move_tracker>,
) {
    let first_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(first_));
    let last_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(last_));
    let result_first_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(result_first_));
    let result_last_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(result_last_));
    let first: Value<iterator_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_::new_1({
            (*first_.borrow()).clone()
        })));
    let last: Value<iterator_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_::new_1({
            (*last_.borrow()).clone()
        })));
    let result_first: Value<iterator_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_::new_1({
            (*result_first_.borrow()).clone()
        })));
    let result_last: Value<iterator_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_::new_1({
            (*result_last_.borrow()).clone()
        })));
    const _: () = assert!(true, "!noexcept(std::ranges::iter_move(first))");;
    'loop_: while (!({
        iterator_wrapper_move_tracker_ptr_Impl::operator_eq(&first.as_pointer(), last.as_pointer())
    })) && (!({
        iterator_wrapper_move_tracker_ptr_Impl::operator_eq(
            &result_first.as_pointer(),
            result_last.as_pointer(),
        )
    })) {
        ({
            move_trackerImpl::move_assign(
                &({
                    iterator_wrapper_move_tracker_ptr_Impl::operator_deref_3(
                        &result_first.as_pointer(),
                    )
                }),
                ({ (*iter_move_1.with(Value::clone).borrow_mut())(first.as_pointer()) }),
            )
        });
        {
            &({ iterator_wrapper_move_tracker_ptr_Impl::operator_inc_5(&first.as_pointer()) });
            ({ iterator_wrapper_move_tracker_ptr_Impl::operator_inc_5(&result_first.as_pointer()) })
        };
    }
}
pub fn rvalue_move_7(
    first_: Ptr<move_tracker>,
    last_: Ptr<move_tracker>,
    result_first_: Ptr<move_tracker>,
    result_last_: Ptr<move_tracker>,
) {
    let first_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(first_));
    let last_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(last_));
    let result_first_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(result_first_));
    let result_last_: Value<Ptr<move_tracker>> = Rc::new(RefCell::new(result_last_));
    let first: Value<iterator_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_::new_1({
            (*first_.borrow()).clone()
        })));
    let last: Value<iterator_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_::new_1({
            (*last_.borrow()).clone()
        })));
    let result_first: Value<iterator_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_::new_1({
            (*result_first_.borrow()).clone()
        })));
    let result_last: Value<iterator_wrapper_move_tracker_ptr_> =
        Rc::new(RefCell::new(iterator_wrapper_move_tracker_ptr_::new_1({
            (*result_last_.borrow()).clone()
        })));
    const _: () = assert!(true, "noexcept(std::ranges::iter_move(std::move(first)))");;
    'loop_: while (!({
        iterator_wrapper_move_tracker_ptr_Impl::operator_eq(&first.as_pointer(), last.as_pointer())
    })) && (!({
        iterator_wrapper_move_tracker_ptr_Impl::operator_eq(
            &result_first.as_pointer(),
            result_last.as_pointer(),
        )
    })) {
        let i: Value<iterator_wrapper_move_tracker_ptr_> =
            Rc::new(RefCell::new((*first.borrow()).clone()));
        ({
            move_trackerImpl::move_assign(
                &({
                    iterator_wrapper_move_tracker_ptr_Impl::operator_deref_3(
                        &result_first.as_pointer(),
                    )
                }),
                ({ (*iter_move_1.with(Value::clone).borrow_mut())(i.as_pointer()) }),
            )
        });
        {
            &({ iterator_wrapper_move_tracker_ptr_Impl::operator_inc_5(&first.as_pointer()) });
            ({ iterator_wrapper_move_tracker_ptr_Impl::operator_inc_5(&result_first.as_pointer()) })
        };
    }
}
pub fn iter_move_8(_a0: Ptr<WithADL_true_>) -> () {
    return 0_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct WithADL_true_ {}
pub fn iter_move_9(_a0: Ptr<WithADL_false_>) -> () {
    return 0_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct WithADL_false_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct WithoutADL_true_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct WithoutADL_false_ {}
pub fn test_10() -> () {
    let mut full_size: () = 100_();
    let mut half_size: () = (full_size / 2_());
    let mut reset: () = 0_();
    let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let move_counter_is: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut n: ()| -> FnPtr<Generic> {
            return lambda!(
                Generic,
                {
                    let n: Value<()> = Rc::new(RefCell::new(n));
                },
                |x: Ptr<move_tracker>| -> () {
                    return ({ ({ move_trackerImpl::moves(&x) }) } == { n });
                }
            );
        }
    )));
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _first_: Ptr<move_tracker> = unimplemented!();
        let _last_: Ptr<move_tracker> = unimplemented!();
        let _result_first_: Ptr<move_tracker> = unimplemented!();
        let _result_last_: Ptr<move_tracker> = unimplemented!();
        unqualified_lookup_move_5(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    let v3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _first_: Ptr<move_tracker> = unimplemented!().offset((half_size) as isize);
        let _last_: Ptr<move_tracker> = unimplemented!();
        let _result_first_: Ptr<move_tracker> = unimplemented!();
        let _result_last_: Ptr<move_tracker> = unimplemented!();
        unqualified_lookup_move_5(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    let v4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _first_: Ptr<move_tracker> = unimplemented!();
        let _last_: Ptr<move_tracker> = unimplemented!();
        let _result_first_: Ptr<move_tracker> = unimplemented!();
        let _result_last_: Ptr<move_tracker> = unimplemented!();
        unqualified_lookup_move_5(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    ({
        let _first_: Ptr<move_tracker> = unimplemented!();
        let _last_: Ptr<move_tracker> = unimplemented!();
        let _result_first_: Ptr<move_tracker> = unimplemented!().offset((half_size) as isize);
        let _result_last_: Ptr<move_tracker> = unimplemented!();
        lvalue_move_6(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    ({
        let _first_: Ptr<move_tracker> = unimplemented!();
        let _last_: Ptr<move_tracker> = unimplemented!();
        let _result_first_: Ptr<move_tracker> = unimplemented!();
        let _result_last_: Ptr<move_tracker> = unimplemented!();
        lvalue_move_6(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    ({
        let _first_: Ptr<move_tracker> = unimplemented!();
        let _last_: Ptr<move_tracker> = unimplemented!();
        let _result_first_: Ptr<move_tracker> = unimplemented!();
        let _result_last_: Ptr<move_tracker> = unimplemented!();
        rvalue_move_7(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    ({
        let _first_: Ptr<move_tracker> = unimplemented!().offset((half_size) as isize);
        let _last_: Ptr<move_tracker> = unimplemented!();
        let _result_first_: Ptr<move_tracker> = unimplemented!();
        let _result_last_: Ptr<move_tracker> = unimplemented!();
        rvalue_move_7(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    let unscoped: Value<check_unqualified_lookup_unscoped_enum> =
        Rc::new(RefCell::new(check_unqualified_lookup_unscoped_enum_a));
    unimplemented!();
    const _: () = assert!(true, "!noexcept(std::ranges::iter_move(unscoped))");;
    let scoped: Value<check_unqualified_lookup_scoped_enum> =
        Rc::new(RefCell::new(check_unqualified_lookup_scoped_enum_a));
    unimplemented!();
    const _: () = assert!(true, "noexcept(std::ranges::iter_move(scoped))");;
    let some_union: Value<check_unqualified_lookup_some_union> = Rc::new(RefCell::new({
        let __u: Value<check_unqualified_lookup_some_union> = Rc::new(RefCell::new(
            <check_unqualified_lookup_some_union>::default(),
        ));
        check_unqualified_lookup_some_union::x(__u.as_pointer()).write(0_());
        Rc::try_unwrap(__u).ok().unwrap().into_inner()
    }));
    unimplemented!();
    const _: () = assert!(true, "!noexcept(std::ranges::iter_move(some_union))");;
    const _: () = assert!(
        true,
        "noexcept(std::ranges::iter_move(std::declval<WithADL<true>>()))"
    );;
    const _: () = assert!(
        true,
        "!noexcept(std::ranges::iter_move(std::declval<WithADL<false>>()))"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::ranges::iter_move(std::declval<WithoutADL<true>>()))"
    );;
    const _: () = assert!(
        true,
        "!noexcept(std::ranges::iter_move(std::declval<WithoutADL<false>>()))"
    );;
    return true;
}
const _: () = assert!(true, "!std::is_invocable_v<IterMoveT, int*, int*>");
const _: () = assert!(true, "!std::is_invocable_v<IterMoveT, int>");
const _: () = assert!(true, "std::is_invocable_v<IterMoveT, Holder<Incomplete>**>");
const _: () = assert!(
    true,
    "std::is_invocable_v<IterMoveT, Holder<Incomplete>**&>"
);
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
    ({ test_10() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait WithADL_false_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<WithADL_false_> {
        unimplemented!()
    }
    fn operator_post_inc_4(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<WithADL_false_>) -> () {
        unimplemented!()
    }
}
impl WithADL_false_Impl for Ptr<WithADL_false_> {}
pub trait WithADL_true_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<WithADL_true_> {
        unimplemented!()
    }
    fn operator_post_inc_4(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<WithADL_true_>) -> () {
        unimplemented!()
    }
}
impl WithADL_true_Impl for Ptr<WithADL_true_> {}
pub trait WithoutADL_false_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<WithoutADL_false_> {
        unimplemented!()
    }
    fn operator_post_inc_4(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<WithoutADL_false_>) -> () {
        unimplemented!()
    }
}
impl WithoutADL_false_Impl for Ptr<WithoutADL_false_> {}
pub trait WithoutADL_true_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_3(&self) -> Ptr<WithoutADL_true_> {
        unimplemented!()
    }
    fn operator_post_inc_4(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_eq(&self, _a0: Ptr<WithoutADL_true_>) -> () {
        unimplemented!()
    }
}
impl WithoutADL_true_Impl for Ptr<WithoutADL_true_> {}
pub trait check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_Impl {
    fn operator_deref(&self) -> Ptr<move_tracker>;
    fn operator_inc_4(
        &self,
    ) -> Ptr<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_>;
    fn operator_post_inc_5(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_eq(
        &self,
        other: Ptr<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_>,
    ) -> ();
}
impl check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_Impl
    for Ptr<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_>
{
    fn operator_deref(&self) -> Ptr<move_tracker> {
        return (*self).with(|__s| __s.base_.clone());
    }
    fn operator_inc_4(
        &self,
    ) -> Ptr<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_> {
        field!((*self), base_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_eq(
        &self,
        other: Ptr<check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_>,
    ) -> () {
        return ({ (*self).with(|__s| __s.base_.clone()) } == {
            other.with(|__s| __s.base_.clone())
        });
    }
}
pub trait iterator_wrapper_move_tracker_ptr_Impl {
    fn operator_deref_3(&self) -> Ptr<move_tracker>;
    fn operator_deref_4(&self) -> Ptr<move_tracker>;
    fn operator_inc_5(&self) -> Ptr<iterator_wrapper_move_tracker_ptr_>;
    fn operator_post_inc_6(&self, mut _a0: ()) {
        unimplemented!()
    }
    fn operator_eq(&self, other: Ptr<iterator_wrapper_move_tracker_ptr_>) -> ();
}
impl iterator_wrapper_move_tracker_ptr_Impl for Ptr<iterator_wrapper_move_tracker_ptr_> {
    fn operator_deref_3(&self) -> Ptr<move_tracker> {
        return (*self).with(|__s| __s.base_.clone());
    }
    fn operator_deref_4(&self) -> Ptr<move_tracker> {
        return (*self).with(|__s| __s.base_.clone());
    }
    fn operator_inc_5(&self) -> Ptr<iterator_wrapper_move_tracker_ptr_> {
        field!((*self), base_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_eq(&self, other: Ptr<iterator_wrapper_move_tracker_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.base_.clone()) } == {
            other.with(|__s| __s.base_.clone())
        });
    }
}
pub trait move_trackerImpl {
    fn move_assign(&self, other: Ptr<move_tracker>) -> Ptr<move_tracker>;
    fn moves(&self) -> ();
}
impl move_trackerImpl for Ptr<move_tracker> {
    fn move_assign(&self, other: Ptr<move_tracker>) -> Ptr<move_tracker> {
        field!((*self), moves_).write({ (other.with(|__s| __s.moves_) + 1_()) });
        field!(other, moves_).write(0_());
        return (*self).clone();
    }
    fn moves(&self) -> () {
        return (*self).with(|__s| __s.moves_);
    }
}
pub fn __cpp2rust_init_globals() {}
