extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(4)]
pub struct Tracked {
    #[offset(0)]
    pub v: (),
}
impl Tracked {
    pub fn new_1() -> Self {
        Self { v: 0_() }
    }
    pub fn new_2(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Tracked>) -> Self {
        let __this: Tracked = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Tracked>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
        }
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        let __src: Value<Tracked> = Rc::new(RefCell::new(Tracked { v: self.v.clone() }));
        Tracked::copy_from(__src.as_pointer())
    }
}
impl Default for Tracked {
    fn default() -> Self {
        { Tracked::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    #[byte_size(4)]
    pub x: Tracked,
    #[offset(4)]
    #[byte_size(4)]
    pub y: Tracked,
}
impl Pair {
    pub fn move_from(_a0: Ptr<Pair>) -> Self {
        Self {
            x: Tracked::move_from({ field_ptr!(_a0, x) }),
            y: Tracked::move_from({ field_ptr!(_a0, y) }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(16)]
pub struct Arrays {
    #[offset(0)]
    #[byte_size(8)]
    pub a: Box<[Tracked]>,
    #[offset(8)]
    #[byte_size(8)]
    pub b: Box<[Tracked]>,
}
impl Arrays {
    pub fn move_from(_a0: Ptr<Arrays>) -> Self {
        Self {
            a: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                Tracked::move_from({ (array_field_ptr!(_a0, a) as Ptr<Tracked>).offset(__i) })
            })),
            b: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                Tracked::move_from({ (array_field_ptr!(_a0, b) as Ptr<Tracked>).offset(__i) })
            })),
        }
    }
}
impl Default for Arrays {
    fn default() -> Self {
        {
            Arrays {
                a: (0..2)
                    .map(|_| <Tracked>::default())
                    .collect::<Box<[Tracked]>>(),
                b: (0..2)
                    .map(|_| <Tracked>::default())
                    .collect::<Box<[Tracked]>>(),
            }
        }
    }
}
thread_local!(
    pub static copy_only_copies_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static copy_only_dtors_2: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(4)]
pub struct CopyOnly {
    #[offset(0)]
    pub v: (),
}
impl CopyOnly {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<CopyOnly>) -> Self {
        let __this: CopyOnly = Self {
            v: o.with(|__s| __s.v),
        };
        (*copy_only_copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for CopyOnly {
    fn clone(&self) -> Self {
        let __src: Value<CopyOnly> = Rc::new(RefCell::new(CopyOnly { v: self.v.clone() }));
        CopyOnly::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(8)]
pub struct CopyOnlyPair {
    #[offset(0)]
    #[byte_size(4)]
    pub x: CopyOnly,
    #[offset(4)]
    #[byte_size(4)]
    pub y: CopyOnly,
}
thread_local!(
    pub static moves_3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static move_assigns_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moved_dtors_5: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(
    Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor, Default,
)]
#[byte_size(4)]
pub struct Moved {
    #[offset(0)]
    pub v: (),
}
impl Moved {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Moved>) -> Self {
        let __this: Moved = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Moved>) -> Self {
        let __this: Moved = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(-1_());
        (*moves_3.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Moved {
    fn clone(&self) -> Self {
        let __src: Value<Moved> = Rc::new(RefCell::new(Moved { v: self.v.clone() }));
        Moved::copy_from(__src.as_pointer())
    }
}
#[derive(
    Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Destructor, Default,
)]
#[byte_size(8)]
pub struct MovedPair {
    #[offset(0)]
    #[byte_size(4)]
    pub x: Moved,
    #[offset(4)]
    #[byte_size(4)]
    pub y: Moved,
}
impl MovedPair {
    pub fn move_from(_a0: Ptr<MovedPair>) -> Self {
        Self {
            x: Moved::move_from({ field_ptr!(_a0, x) }),
            y: Moved::move_from({ field_ptr!(_a0, y) }),
        }
    }
}
pub fn swapped_once_6() -> () {
    let mut ok: () = ((moves_3.with(|rc| *rc.borrow()) == 1_())
        && (move_assigns_4.with(|rc| *rc.borrow()) == 2_()))
        && (moved_dtors_5.with(|rc| *rc.borrow()) == 1_());
    moves_3.with(|rc| {
        *rc.borrow_mut() = {
            move_assigns_4.with(|rc| {
                *rc.borrow_mut() = {
                    moved_dtors_5.with(|rc| *rc.borrow_mut() = 0_());
                    moved_dtors_5.with(|rc| *rc.borrow())
                }
            });
            move_assigns_4.with(|rc| *rc.borrow())
        }
    });
    return ok;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(8)]
    pub p: std_unique_ptr_int__std_default_delete_int__,
}
impl Owner {
    pub fn move_from(_a0: Ptr<Owner>) -> Self {
        Self {
            p: field!(_a0, p).with_mut(std::mem::take),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_2({ 1_() })));
    let b: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_2({ 2_() })));
    ({ swap_7(a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let m1 : Value<std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___ > = Rc::new(RefCell::new(<std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___  >::default() )) ;
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_2({ 10_() })));
        TrackedImpl::move_assign(&m1.as_pointer().offset(1_()), _o.as_pointer())
    });
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_2({ 20_() })));
        TrackedImpl::move_assign(&m1.as_pointer().offset(2_()), _o.as_pointer())
    });
    let m2 : Value<std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___ > = Rc::new(RefCell::new(<std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___  >::default() )) ;
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_2({ 30_() })));
        TrackedImpl::move_assign(&m2.as_pointer().offset(3_()), _o.as_pointer())
    });
    unimplemented!();
    ({
        let ___a: Ptr<Tracked> = m1.as_pointer().offset(1_());
        let ___b: Ptr<Tracked> = m1.as_pointer().offset(2_());
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    ({ swap_8(m1.as_pointer(), m2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let arr: Value<Box<[Tracked]>> = Rc::new(RefCell::new(Box::new([
        Tracked::new_2({ 1_() }),
        Tracked::new_2({ 2_() }),
        Tracked::new_2({ 3_() }),
    ])));
    ({
        let ___a: Ptr<Tracked> = (arr.as_pointer() as Ptr<Tracked>).offset(0_());
        let ___b: Ptr<Tracked> = (arr.as_pointer() as Ptr<Tracked>).offset(2_());
        swap_7(___a, ___b)
    });
    unimplemented!();
    ({
        let ___a: Ptr<Tracked> = (arr.as_pointer() as Ptr<Tracked>).offset(1_());
        let ___b: Ptr<Tracked> = (arr.as_pointer() as Ptr<Tracked>).offset(1_());
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let vec_: Value<std_vector_Tracked__std_allocator_Tracked__> = Rc::new(RefCell::new(
        <std_vector_Tracked__std_allocator_Tracked__>::default(),
    ));
    ({
        let ___args: Value<()> = Rc::new(RefCell::new(4_()));
        (*vec_.borrow()).emplace_back(___args.as_pointer())
    });
    ({
        let ___args: Value<()> = Rc::new(RefCell::new(5_()));
        (*vec_.borrow()).emplace_back(___args.as_pointer())
    });
    ({
        let ___a: Ptr<Tracked> = vec_.as_pointer().offset(0_());
        let ___b: Ptr<Tracked> = vec_.as_pointer().offset(1_());
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let pairs: Value<Box<[Pair]>> = Rc::new(RefCell::new(Box::new([
        Pair {
            x: Tracked::new_2({ 6_() }),
            y: Tracked::new_2({ 7_() }),
        },
        Pair {
            x: Tracked::new_2({ 8_() }),
            y: Tracked::new_2({ 9_() }),
        },
    ])));
    let p: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: Tracked::new_2({ 10_() }),
        y: Tracked::new_2({ 11_() }),
    }));
    ({
        swap_7(
            field_ptr!((pairs.as_pointer() as Ptr<Pair>).offset(0_()), x),
            field_ptr!(p.as_pointer(), y),
        )
    });
    unimplemented!();
    ({
        let ___a: Ptr<Pair> = (pairs.as_pointer() as Ptr<Pair>).offset(0_());
        let ___b: Ptr<Pair> = (pairs.as_pointer() as Ptr<Pair>).offset(1_());
        swap_9(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    ({
        let ___a: Ptr<Tracked> = field_ptr!(p.as_pointer(), x);
        let ___b: Ptr<Tracked> = field_ptr!(p.as_pointer(), y);
        swap_7(___a, ___b)
    });
    unimplemented!();
    ({
        let ___a: Ptr<Tracked> = field_ptr!((pairs.as_pointer() as Ptr<Pair>).offset(0_()), x);
        let ___b: Ptr<Tracked> = field_ptr!((pairs.as_pointer() as Ptr<Pair>).offset(1_()), y);
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let arrays: Value<Arrays> = Rc::new(RefCell::new(<Arrays>::default()));
    field!(array_elem!(arrays.as_pointer(), a, 0_()), v).write(1_());
    field!(array_elem!(arrays.as_pointer(), b, 1_()), v).write(2_());
    ({
        let ___a: Ptr<Tracked> =
            (array_field_ptr!(arrays.as_pointer(), a) as Ptr<Tracked>).offset((0_()) as isize);
        let ___b: Ptr<Tracked> =
            (array_field_ptr!(arrays.as_pointer(), b) as Ptr<Tracked>).offset((1_()) as isize);
        swap_7(___a, ___b)
    });
    unimplemented!();
    ({
        let ___a: Ptr<Tracked> =
            (array_field_ptr!(arrays.as_pointer(), a) as Ptr<Tracked>).offset((0_()) as isize);
        let ___b: Ptr<Tracked> =
            (array_field_ptr!(arrays.as_pointer(), a) as Ptr<Tracked>).offset((1_()) as isize);
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let grid : Value<std_vector_std_vector_Tracked__std_allocator_Tracked____std_allocator_std_vector_Tracked__std_allocator_Tracked____ > = Rc::new(RefCell::new(std_vector_std_vector_Tracked__std_allocator_Tracked____std_allocator_std_vector_Tracked__std_allocator_Tracked____ :: new_1 ( { 2_()  } , None , )  )) ;
    ({
        let ___args: Value<()> = Rc::new(RefCell::new(1_()));
        (*grid.borrow())[0_()].emplace_back(___args.as_pointer())
    });
    ({
        let ___args: Value<()> = Rc::new(RefCell::new(2_()));
        (*grid.borrow())[0_()].emplace_back(___args.as_pointer())
    });
    ({
        let ___args: Value<()> = Rc::new(RefCell::new(3_()));
        (*grid.borrow())[1_()].emplace_back(___args.as_pointer())
    });
    ({
        let ___a: Ptr<Tracked> = grid.as_pointer().offset(0_()).offset(1_());
        let ___b: Ptr<Tracked> = grid.as_pointer().offset(1_()).offset(0_());
        swap_7(___a, ___b)
    });
    unimplemented!();
    ({
        let ___x: Ptr<std_vector_Tracked__std_allocator_Tracked__> = grid.as_pointer().offset(0_());
        let ___y: Ptr<std_vector_Tracked__std_allocator_Tracked__> = grid.as_pointer().offset(1_());
        swap_10(___x, ___y)
    });
    unimplemented!();
    unimplemented!();
    let maps : Value<Box<[ std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___  ]> > = Rc::new(RefCell::new((0..2).map(|_| <std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___  >::default()).collect::<Box<[std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___  ]>>() )) ;
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_2({ 1_() })));
        TrackedImpl :: move_assign ( &(maps .as_pointer()  as Ptr<std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___   >).offset(0_() ) .offset(1_() )  , _o.as_pointer() , )
    });
    ({
        let _o: Value<Tracked> = Rc::new(RefCell::new(Tracked::new_2({ 2_() })));
        TrackedImpl :: move_assign ( &(maps .as_pointer()  as Ptr<std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___   >).offset(1_() ) .offset(2_() )  , _o.as_pointer() , )
    });
    ({
        let ___x: Ptr<
            std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___,
        > = (maps.as_pointer()
            as Ptr<
                std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___,
            >)
            .offset(0_());
        let ___y: Ptr<
            std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___,
        > = (maps.as_pointer()
            as Ptr<
                std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___,
            >)
            .offset(1_());
        swap_8(___x, ___y)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c1: Value<CopyOnly> = Rc::new(RefCell::new(CopyOnly::new({ 1_() })));
    let _dtor_c1 = ScopedDestructor::new(&c1, |__p| __p.destructor());
    let c2: Value<CopyOnly> = Rc::new(RefCell::new(CopyOnly::new({ 2_() })));
    let _dtor_c2 = ScopedDestructor::new(&c2, |__p| __p.destructor());
    ({ swap_11(c1.as_pointer(), c2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let cp: Value<CopyOnlyPair> = Rc::new(RefCell::new(CopyOnlyPair {
        x: CopyOnly::new({ 3_() }),
        y: CopyOnly::new({ 4_() }),
    }));
    let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
    ({
        let ___a: Ptr<CopyOnly> = field_ptr!(cp.as_pointer(), x);
        let ___b: Ptr<CopyOnly> = field_ptr!(cp.as_pointer(), y);
        swap_11(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let carr: Value<Box<[CopyOnly]>> = Rc::new(RefCell::new(Box::new([
        CopyOnly::new({ 5_() }),
        CopyOnly::new({ 6_() }),
    ])));
    ({
        let ___a: Ptr<CopyOnly> = (carr.as_pointer() as Ptr<CopyOnly>).offset(0_());
        let ___b: Ptr<CopyOnly> = (carr.as_pointer() as Ptr<CopyOnly>).offset(1_());
        swap_11(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let mv1: Value<Moved> = Rc::new(RefCell::new(Moved::new({ 1_() })));
    let _dtor_mv1 = ScopedDestructor::new(&mv1, |__p| __p.destructor());
    let mv2: Value<Moved> = Rc::new(RefCell::new(Moved::new({ 2_() })));
    let _dtor_mv2 = ScopedDestructor::new(&mv2, |__p| __p.destructor());
    ({ swap_12(mv1.as_pointer(), mv2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let mp: Value<MovedPair> = Rc::new(RefCell::new(MovedPair {
        x: Moved::new({ 3_() }),
        y: Moved::new({ 4_() }),
    }));
    let _dtor_mp = ScopedDestructor::new(&mp, |__p| __p.destructor());
    ({
        let ___a: Ptr<Moved> = field_ptr!(mp.as_pointer(), x);
        let ___b: Ptr<Moved> = field_ptr!(mp.as_pointer(), y);
        swap_12(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let marr: Value<Box<[Moved]>> = Rc::new(RefCell::new(Box::new([
        Moved::new({ 5_() }),
        Moved::new({ 6_() }),
    ])));
    ({
        let ___a: Ptr<Moved> = (marr.as_pointer() as Ptr<Moved>).offset(0_());
        let ___b: Ptr<Moved> = (marr.as_pointer() as Ptr<Moved>).offset(1_());
        swap_12(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    ({
        let ___a: Ptr<Moved> = mv1.as_pointer();
        let ___b: Ptr<Moved> = mv1.as_pointer();
        swap_12(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let o1: Value<Owner> = Rc::new(RefCell::new(Owner {
        p: ({
            let ___args: Value<()> = Rc::new(RefCell::new(1_()));
            make_unique_13(___args.as_pointer())
        }),
    }));
    let o2: Value<Owner> = Rc::new(RefCell::new(Owner {
        p: ({
            let ___args: Value<()> = Rc::new(RefCell::new(2_()));
            make_unique_13(___args.as_pointer())
        }),
    }));
    ({ swap_14(o1.as_pointer(), o2.as_pointer()) });
    unimplemented!();
    return copies_0.with(|rc| *rc.borrow());
}
pub trait ArraysImpl {
    fn copy_assign(&self, _a0: Ptr<Arrays>) -> Ptr<Arrays>;
    fn move_assign(&self, _a0: Ptr<Arrays>) -> Ptr<Arrays>;
}
impl ArraysImpl for Ptr<Arrays> {
    fn copy_assign(&self, _a0: Ptr<Arrays>) -> Ptr<Arrays> {
        let mut __i0: () = 0_();
        'loop_: while (__i0 != 2_()) {
            ({
                let _o: Ptr<Tracked> =
                    (array_field_ptr!(_a0, a) as Ptr<Tracked>).offset((__i0) as isize);
                TrackedImpl::copy_assign(
                    &(array_field_ptr!((*self), a) as Ptr<Tracked>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        let mut __i0: () = 0_();
        'loop_: while (__i0 != 2_()) {
            ({
                let _o: Ptr<Tracked> =
                    (array_field_ptr!(_a0, b) as Ptr<Tracked>).offset((__i0) as isize);
                TrackedImpl::copy_assign(
                    &(array_field_ptr!((*self), b) as Ptr<Tracked>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Arrays>) -> Ptr<Arrays> {
        let mut __i0: () = 0_();
        'loop_: while (__i0 != 2_()) {
            ({
                let _o: Ptr<Tracked> =
                    (array_field_ptr!(_a0, a) as Ptr<Tracked>).offset((__i0) as isize);
                TrackedImpl::move_assign(
                    &(array_field_ptr!((*self), a) as Ptr<Tracked>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        let mut __i0: () = 0_();
        'loop_: while (__i0 != 2_()) {
            ({
                let _o: Ptr<Tracked> =
                    (array_field_ptr!(_a0, b) as Ptr<Tracked>).offset((__i0) as isize);
                TrackedImpl::move_assign(
                    &(array_field_ptr!((*self), b) as Ptr<Tracked>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        return (*self).clone();
    }
}
pub trait CopyOnlyImpl {
    fn destructor(&self);
    fn copy_assign(&self, o: Ptr<CopyOnly>) -> Ptr<CopyOnly>;
}
impl CopyOnlyImpl for Ptr<CopyOnly> {
    fn destructor(&self) {
        (*copy_only_dtors_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn copy_assign(&self, o: Ptr<CopyOnly>) -> Ptr<CopyOnly> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*copy_only_copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub trait CopyOnlyPairImpl {
    fn copy_assign(&self, _a0: Ptr<CopyOnlyPair>) -> Ptr<CopyOnlyPair>;
    fn destructor(&self);
}
impl CopyOnlyPairImpl for Ptr<CopyOnlyPair> {
    fn copy_assign(&self, _a0: Ptr<CopyOnlyPair>) -> Ptr<CopyOnlyPair> {
        ({
            let _o: Ptr<CopyOnly> = field_ptr!(_a0, x);
            CopyOnlyImpl::copy_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<CopyOnly> = field_ptr!(_a0, y);
            CopyOnlyImpl::copy_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, y).destructor();
        field_ptr!(self, x).destructor();
    }
}
pub trait MovedImpl {
    fn copy_assign(&self, o: Ptr<Moved>) -> Ptr<Moved>;
    fn move_assign(&self, o: Ptr<Moved>) -> Ptr<Moved>;
    fn destructor(&self);
}
impl MovedImpl for Ptr<Moved> {
    fn copy_assign(&self, o: Ptr<Moved>) -> Ptr<Moved> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<Moved>) -> Ptr<Moved> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(-1_());
        (*move_assigns_4.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn destructor(&self) {
        (*moved_dtors_5.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub trait MovedPairImpl {
    fn copy_assign(&self, _a0: Ptr<MovedPair>) -> Ptr<MovedPair>;
    fn move_assign(&self, _a0: Ptr<MovedPair>) -> Ptr<MovedPair>;
    fn destructor(&self);
}
impl MovedPairImpl for Ptr<MovedPair> {
    fn copy_assign(&self, _a0: Ptr<MovedPair>) -> Ptr<MovedPair> {
        ({
            let _o: Ptr<Moved> = field_ptr!(_a0, x);
            MovedImpl::copy_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<Moved> = field_ptr!(_a0, y);
            MovedImpl::copy_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<MovedPair>) -> Ptr<MovedPair> {
        ({
            let _o: Ptr<Moved> = field_ptr!(_a0, x);
            MovedImpl::move_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<Moved> = field_ptr!(_a0, y);
            MovedImpl::move_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, y).destructor();
        field_ptr!(self, x).destructor();
    }
}
pub trait OwnerImpl {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner>;
}
impl OwnerImpl for Ptr<Owner> {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner> {
        field!((*self), p).write({ { (*_a0.upgrade().deref()).p.clone() } });
        return (*self).clone();
    }
}
pub trait PairImpl {
    fn copy_assign(&self, _a0: Ptr<Pair>) -> Ptr<Pair>;
    fn move_assign(&self, _a0: Ptr<Pair>) -> Ptr<Pair>;
}
impl PairImpl for Ptr<Pair> {
    fn copy_assign(&self, _a0: Ptr<Pair>) -> Ptr<Pair> {
        ({
            let _o: Ptr<Tracked> = field_ptr!(_a0, x);
            TrackedImpl::copy_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<Tracked> = field_ptr!(_a0, y);
            TrackedImpl::copy_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Pair>) -> Ptr<Pair> {
        ({
            let _o: Ptr<Tracked> = field_ptr!(_a0, x);
            TrackedImpl::move_assign(&field_ptr!((*self), x), _o)
        });
        ({
            let _o: Ptr<Tracked> = field_ptr!(_a0, y);
            TrackedImpl::move_assign(&field_ptr!((*self), y), _o)
        });
        return (*self).clone();
    }
}
pub trait TrackedImpl {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
    fn move_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
}
impl TrackedImpl for Ptr<Tracked> {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = copy_only_copies_1.with(|_| ());
    let _ = copy_only_dtors_2.with(|_| ());
    let _ = moves_3.with(|_| ());
    let _ = move_assigns_4.with(|_| ());
    let _ = moved_dtors_5.with(|_| ());
}
