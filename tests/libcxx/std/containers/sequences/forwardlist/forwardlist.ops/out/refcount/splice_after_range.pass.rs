extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_long__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_4(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_5(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_void__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_4(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl std::cmp::PartialEq for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_long_ {}
thread_local!(
    pub static t1_6: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
    ])));
);
thread_local!(
    pub static t2_7: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
    ])));
);
thread_local!(
    pub static size_t1_8: Value<()> = Rc::new(RefCell::new(
        ((unimplemented!() - unimplemented!()) as () as ()),
    ));
);
thread_local!(
    pub static size_t2_9: Value<()> = Rc::new(RefCell::new(
        ((unimplemented!() - unimplemented!()) as () as ()),
    ));
);
pub fn testd_10(c: Ptr<()>, mut p: (), mut f: (), mut l: ()) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n1: () = 0_();
    'loop_: while (n1 < p) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    let mut n2: () = f;
    'loop_: while (n2 < (l - 1_())) {
        unimplemented!();
        {
            n2.prefix_inc();
            unimplemented!()
        };
    }
    'loop_: while (n1 < size_t1_8.with(|rc| *rc.borrow())) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    unimplemented!();
}
pub fn testd_11(c: Ptr<()>, mut p: (), mut f: (), mut l: ()) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n1: () = 0_();
    'loop_: while (n1 < p) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    let mut n2: () = f;
    'loop_: while (n2 < (l - 1_())) {
        unimplemented!();
        {
            n2.prefix_inc();
            unimplemented!()
        };
    }
    'loop_: while (n1 < size_t1_8.with(|rc| *rc.borrow())) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    unimplemented!();
}
pub fn tests_12(c: Ptr<()>, mut p: (), mut f: (), mut l: ()) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n: () = 0_();
    let mut d: () = if (l > (f + 1_())) {
        ((l - 1_()) - f)
    } else {
        0_()
    };
    if (d == 0_()) || (p == f) {
        n = 0_();
        'loop_: while (n < size_t1_8.with(|rc| *rc.borrow())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    } else if (p < f) {
        n = 0_();
        'loop_: while (n < p) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = f;
        'loop_: while (n < (l - 1_())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = p;
        'loop_: while (n < f) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = (l - 1_());
        'loop_: while (n < size_t1_8.with(|rc| *rc.borrow())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    } else {
        n = 0_();
        'loop_: while (n < f) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = (l - 1_());
        'loop_: while (n < p) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = f;
        'loop_: while (n < (l - 1_())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = p;
        'loop_: while (n < size_t1_8.with(|rc| *rc.borrow())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    }
    unimplemented!();
}
pub fn tests_13(c: Ptr<()>, mut p: (), mut f: (), mut l: ()) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n: () = 0_();
    let mut d: () = if (l > (f + 1_())) {
        ((l - 1_()) - f)
    } else {
        0_()
    };
    if (d == 0_()) || (p == f) {
        n = 0_();
        'loop_: while (n < size_t1_8.with(|rc| *rc.borrow())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    } else if (p < f) {
        n = 0_();
        'loop_: while (n < p) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = f;
        'loop_: while (n < (l - 1_())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = p;
        'loop_: while (n < f) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = (l - 1_());
        'loop_: while (n < size_t1_8.with(|rc| *rc.borrow())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    } else {
        n = 0_();
        'loop_: while (n < f) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = (l - 1_());
        'loop_: while (n < p) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = f;
        'loop_: while (n < (l - 1_())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = p;
        'loop_: while (n < size_t1_8.with(|rc| *rc.borrow())) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    }
    unimplemented!();
}
pub fn test_14() -> () {
    {
        let mut f: () = 0_();
        'loop_: while (f <= (size_t2_9.with(|rc| *rc.borrow()) + 1_())) {
            let mut l: () = f;
            'loop_: while (l <= (size_t2_9.with(|rc| *rc.borrow()) + 1_())) {
                let mut p: () = 0_();
                'loop_: while (p <= size_t1_8.with(|rc| *rc.borrow())) {
                    let c1: Value<()> = Rc::new(RefCell::new(
                        std_forward_list_long__std_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        }),
                    ));
                    let c2: Value<()> = Rc::new(RefCell::new(
                        std_forward_list_long__std_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        }),
                    ));
                    unimplemented!();
                    ({ testd_10(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: () = 0_();
        'loop_: while (f <= (size_t1_8.with(|rc| *rc.borrow()) + 1_())) {
            let mut l: () = f;
            'loop_: while (l <= size_t1_8.with(|rc| *rc.borrow())) {
                let mut p: () = 0_();
                'loop_: while (p <= f) {
                    let c1: Value<()> = Rc::new(RefCell::new(
                        std_forward_list_long__std_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        }),
                    ));
                    unimplemented!();
                    ({ tests_12(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                let mut p: () = l;
                'loop_: while (p <= size_t1_8.with(|rc| *rc.borrow())) {
                    let c1: Value<()> = Rc::new(RefCell::new(
                        std_forward_list_long__std_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        }),
                    ));
                    unimplemented!();
                    ({ tests_12(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
    }
    {
        let mut f: () = 0_();
        'loop_: while (f <= (size_t2_9.with(|rc| *rc.borrow()) + 1_())) {
            let mut l: () = f;
            'loop_: while (l <= (size_t2_9.with(|rc| *rc.borrow()) + 1_())) {
                let mut p: () = 0_();
                'loop_: while (p <= size_t1_8.with(|rc| *rc.borrow())) {
                    let c1: Value<()> = Rc::new(RefCell::new(
                        std_forward_list_long__min_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        }),
                    ));
                    let c2: Value<()> = Rc::new(RefCell::new(
                        std_forward_list_long__min_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        }),
                    ));
                    unimplemented!();
                    ({ testd_11(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: () = 0_();
        'loop_: while (f <= (size_t1_8.with(|rc| *rc.borrow()) + 1_())) {
            let mut l: () = f;
            'loop_: while (l <= size_t1_8.with(|rc| *rc.borrow())) {
                let mut p: () = 0_();
                'loop_: while (p <= f) {
                    let c1: Value<()> = Rc::new(RefCell::new(
                        std_forward_list_long__min_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        }),
                    ));
                    unimplemented!();
                    ({ tests_13(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                let mut p: () = l;
                'loop_: while (p <= size_t1_8.with(|rc| *rc.borrow())) {
                    let c1: Value<()> = Rc::new(RefCell::new(
                        std_forward_list_long__min_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        }),
                    ));
                    unimplemented!();
                    ({ tests_13(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
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
    unimplemented!();
    return 0_();
}
pub trait min_allocator_long_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_long__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_long_Impl for Ptr<min_allocator_long_> {}
pub trait min_pointer_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = t1_6.with(|_| ());
    let _ = t2_7.with(|_| ());
    let _ = size_t1_8.with(|_| ());
    let _ = size_t2_9.with(|_| ());
}
