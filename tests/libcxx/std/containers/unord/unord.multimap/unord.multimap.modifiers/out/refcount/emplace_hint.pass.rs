extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Emplaceable {
    #[offset(0)]
    int_: i32,
    #[offset(8)]
    double_: f64,
}
impl Emplaceable {
    pub fn new_1() -> Self {
        Self {
            int_: 0,
            double_: 0_f64,
        }
    }
    pub fn new_2(mut i: i32, mut d: f64) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Emplaceable>) -> Self {
        let __this: Emplaceable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(0);
        field!(x, double_).write(0_f64);
        __this
    }
}
impl CppLess for Emplaceable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Emplaceable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for Emplaceable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for Emplaceable {
    fn default() -> Self {
        { Emplaceable::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__Emplaceable__ {}
impl CppMove for min_allocator_std_pair_const_int__Emplaceable__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
    {
        let c : Value<std_unordered_multimap_int__Emplaceable__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__Emplaceable___ > = Rc::new(RefCell::new(<std_unordered_multimap_int__Emplaceable__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__Emplaceable___  >::default() )) ;
        let e : Value<std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_ > = Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  ) )) ;
        let r: Value<std___detail__Node_iterator_std_pair_const_int__Emplaceable___false__false_> =
            Rc::new(RefCell::new(
                ({
                    let ___args_2: Value<std_tuple_int_refref_> = Rc::new(RefCell::new(
                        ({
                            let ___args: Value<i32> = Rc::new(RefCell::new(3));
                            forward_as_tuple_4(___args.as_pointer())
                        }),
                    ));
                    let ___args_3: Value<std_tuple__> =
                        Rc::new(RefCell::new(({ forward_as_tuple_5() })));
                    (*c.borrow()).emplace_hint(
                        (*e.borrow()).clone(),
                        piecewise_construct_6.with(|v| v.as_pointer()),
                        ___args_2.as_pointer(),
                        ___args_3.as_pointer(),
                    )
                }),
            ));
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                let _x: Value<Emplaceable> = Rc::new(RefCell::new(Emplaceable::new_1()));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        {
            let __a1 = ({
                let ___pos: std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
                let ___args: Value<(Value<i32>, Value<Emplaceable>)> = Rc::new(RefCell::new((
                    Rc::new(RefCell::new(3)),
                    Rc::new(RefCell::new(Emplaceable::new_2({ 5 }, { 6_f64 }))),
                )));
                (*c.borrow()).emplace_hint(___pos, ___args.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ (({ (*c.borrow()).size() }) == 2_u64) }));
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 5 }, { 6_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        const _: () = assert!(true, "");;
        {
            let __a1 = ({
                let ___args_2: Value<std_tuple_int_refref_> = Rc::new(RefCell::new(
                    ({
                        let ___args: Value<i32> = Rc::new(RefCell::new(3));
                        forward_as_tuple_4(___args.as_pointer())
                    }),
                ));
                let ___args_3: Value<std_tuple_int_refref__int_refref_> = Rc::new(RefCell::new(
                    ({
                        let ___args_0: Value<i32> = Rc::new(RefCell::new(6));
                        let ___args_1: Value<i32> = Rc::new(RefCell::new(7));
                        forward_as_tuple_7(___args_0.as_pointer(), ___args_1.as_pointer())
                    }),
                ));
                (*c.borrow()) . emplace_hint ( <std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_  >::from(({ ((*r.borrow()) ).clone() }))  , piecewise_construct_6.with(|v| v.as_pointer())  , ___args_2.as_pointer() , ___args_3.as_pointer() , )
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 6 }, { 7_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        const _: () = assert!(true, "");;
        {
            let __a1 = ({ (*c.borrow()).begin_27() });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        const _: () = assert!(true, "");;
        {
            let __a1 = libcc2rs::next_refcount(({ (*r.borrow()).clone() }), 2_isize as isize);
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        const _: () = assert!(true, "");;
    }
    {
        let c : Value<std_unordered_multimap_int__Emplaceable__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__Emplaceable___ > = Rc::new(RefCell::new(<std_unordered_multimap_int__Emplaceable__std_hash_int___std_equal_to_int___min_allocator_std_pair_const_int__Emplaceable___  >::default() )) ;
        let e : Value<std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_ > = Rc::new(RefCell::new(<std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  ) )) ;
        let r: Value<std___detail__Node_iterator_std_pair_const_int__Emplaceable___false__false_> =
            Rc::new(RefCell::new(
                ({
                    let ___args_2: Value<std_tuple_int_refref_> = Rc::new(RefCell::new(
                        ({
                            let ___args: Value<i32> = Rc::new(RefCell::new(3));
                            forward_as_tuple_4(___args.as_pointer())
                        }),
                    ));
                    let ___args_3: Value<std_tuple__> =
                        Rc::new(RefCell::new(({ forward_as_tuple_5() })));
                    (*c.borrow()).emplace_hint(
                        (*e.borrow()).clone(),
                        piecewise_construct_6.with(|v| v.as_pointer()),
                        ___args_2.as_pointer(),
                        ___args_3.as_pointer(),
                    )
                }),
            ));
        assert!(({ (({ (*c.borrow()).size() }) == 1_u64) }));
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                let _x: Value<Emplaceable> = Rc::new(RefCell::new(Emplaceable::new_1()));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        {
            let __a1 = ({
                let ___pos: std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_   = <std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_  >::from(( { (*c.borrow()) . end_30 ( ) } )  )  ;
                let ___args: Value<(Value<i32>, Value<Emplaceable>)> = Rc::new(RefCell::new((
                    Rc::new(RefCell::new(3)),
                    Rc::new(RefCell::new(Emplaceable::new_2({ 5 }, { 6_f64 }))),
                )));
                (*c.borrow()).emplace_hint(___pos, ___args.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ (({ (*c.borrow()).size() }) == 2_u64) }));
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 5 }, { 6_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        const _: () = assert!(true, "");;
        {
            let __a1 = ({
                let ___args_2: Value<std_tuple_int_refref_> = Rc::new(RefCell::new(
                    ({
                        let ___args: Value<i32> = Rc::new(RefCell::new(3));
                        forward_as_tuple_4(___args.as_pointer())
                    }),
                ));
                let ___args_3: Value<std_tuple_int_refref__int_refref_> = Rc::new(RefCell::new(
                    ({
                        let ___args_0: Value<i32> = Rc::new(RefCell::new(6));
                        let ___args_1: Value<i32> = Rc::new(RefCell::new(7));
                        forward_as_tuple_7(___args_0.as_pointer(), ___args_1.as_pointer())
                    }),
                ));
                (*c.borrow()) . emplace_hint ( <std___detail__Node_const_iterator_std_pair_const_int__Emplaceable___false__false_  >::from(({ ((*r.borrow()) ).clone() }))  , piecewise_construct_6.with(|v| v.as_pointer())  , ___args_2.as_pointer() , ___args_3.as_pointer() , )
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ (({ (*c.borrow()).size() }) == 3_u64) }));
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 6 }, { 7_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        const _: () = assert!(true, "");;
        {
            let __a1 = ({ (*c.borrow()).begin_27() });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        const _: () = assert!(true, "");;
        {
            let __a1 = libcc2rs::next_refcount(({ (*r.borrow()).clone() }), 2_isize as isize);
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        const _: () = assert!(true, "");;
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__;
pub trait EmplaceableImpl {
    fn copy_assign(&self, _a0: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable>;
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool;
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool;
    fn get(&self) -> i32;
}
impl EmplaceableImpl for Ptr<Emplaceable> {
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(0);
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write(0_f64);
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
}
pub trait min_allocator_std_pair_const_int__Emplaceable__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__Emplaceable__Impl
    for Ptr<min_allocator_std_pair_const_int__Emplaceable__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
