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
#[byte_size(8)]
pub struct min_pointer_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i64>,
}
impl min_pointer_long__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_long_ {}
thread_local!(
    pub static t1_4: Value<Box<[isize]>> = Rc::new(RefCell::new(Box::new([
        0_isize, 1_isize, 2_isize, 3_isize, 4_isize, 5_isize, 6_isize, 7_isize,
    ])));
);
thread_local!(
    pub static t2_5: Value<Box<[isize]>> = Rc::new(RefCell::new(Box::new([
        10_isize, 11_isize, 12_isize, 13_isize, 14_isize, 15_isize,
    ])));
);
thread_local!(
    pub static size_t1_6: Value<isize> = Rc::new(RefCell::new(
        (((t1_4.with(|v| v.as_pointer()) as Ptr<isize>).to_end() - unimplemented!()) as i64
            as isize),
    ));
);
thread_local!(
    pub static size_t2_7: Value<isize> = Rc::new(RefCell::new(
        (((t2_5.with(|v| v.as_pointer()) as Ptr<isize>).to_end() - unimplemented!()) as i64
            as isize),
    ));
);
pub fn testd_8(c: Ptr<()>, mut p: isize, mut f: isize, mut l: isize) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n1: isize = 0_isize;
    'loop_: while (n1 < p) {
        assert!(
            ((((*i.borrow()).iter_deref().read()) as isize)
                == ({
                    let __idx0 = (n1) as usize;
                    t1_4.with(|rc| rc.borrow()[__idx0])
                }))
        );
        {
            n1.prefix_inc();
            (*i.borrow_mut()).iter_inc()
        };
    }
    let mut n2: isize = f;
    'loop_: while (n2 < (l - 1_isize)) {
        assert!(
            ((((*i.borrow()).iter_deref().read()) as isize)
                == ({
                    let __idx0 = (n2) as usize;
                    t2_5.with(|rc| rc.borrow()[__idx0])
                }))
        );
        {
            n2.prefix_inc();
            (*i.borrow_mut()).iter_inc()
        };
    }
    'loop_: while (n1 < size_t1_6.with(|rc| *rc.borrow())) {
        assert!(
            ((((*i.borrow()).iter_deref().read()) as isize)
                == ({
                    let __idx0 = (n1) as usize;
                    t1_4.with(|rc| rc.borrow()[__idx0])
                }))
        );
        {
            n1.prefix_inc();
            (*i.borrow_mut()).iter_inc()
        };
    }
    assert!(
        ({ unimplemented!() } == {
            (((size_t1_6.with(|rc| *rc.borrow()) as i64)
                + ((if (l > (f + 1_isize)) {
                    ((l - 1_isize) - f)
                } else {
                    0_isize
                }) as i64)) as isize)
        })
    );
}
pub fn testd_9(c: Ptr<()>, mut p: isize, mut f: isize, mut l: isize) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n1: isize = 0_isize;
    'loop_: while (n1 < p) {
        assert!(
            ((((*i.borrow()).iter_deref().read()) as isize)
                == ({
                    let __idx0 = (n1) as usize;
                    t1_4.with(|rc| rc.borrow()[__idx0])
                }))
        );
        {
            n1.prefix_inc();
            (*i.borrow_mut()).iter_inc()
        };
    }
    let mut n2: isize = f;
    'loop_: while (n2 < (l - 1_isize)) {
        assert!(
            ((((*i.borrow()).iter_deref().read()) as isize)
                == ({
                    let __idx0 = (n2) as usize;
                    t2_5.with(|rc| rc.borrow()[__idx0])
                }))
        );
        {
            n2.prefix_inc();
            (*i.borrow_mut()).iter_inc()
        };
    }
    'loop_: while (n1 < size_t1_6.with(|rc| *rc.borrow())) {
        assert!(
            ((((*i.borrow()).iter_deref().read()) as isize)
                == ({
                    let __idx0 = (n1) as usize;
                    t1_4.with(|rc| rc.borrow()[__idx0])
                }))
        );
        {
            n1.prefix_inc();
            (*i.borrow_mut()).iter_inc()
        };
    }
    assert!(
        ({ unimplemented!() } == {
            (((size_t1_6.with(|rc| *rc.borrow()) as i64)
                + ((if (l > (f + 1_isize)) {
                    ((l - 1_isize) - f)
                } else {
                    0_isize
                }) as i64)) as isize)
        })
    );
}
pub fn tests_10(c: Ptr<()>, mut p: isize, mut f: isize, mut l: isize) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n: isize = 0_isize;
    let mut d: isize = if (l > (f + 1_isize)) {
        ((l - 1_isize) - f)
    } else {
        0_isize
    };
    if (d == 0_isize) || (p == f) {
        n = 0_isize;
        'loop_: while (n < size_t1_6.with(|rc| *rc.borrow())) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
    } else if (p < f) {
        n = 0_isize;
        'loop_: while (n < p) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = f;
        'loop_: while (n < (l - 1_isize)) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = p;
        'loop_: while (n < f) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = (l - 1_isize);
        'loop_: while (n < size_t1_6.with(|rc| *rc.borrow())) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
    } else {
        n = 0_isize;
        'loop_: while (n < f) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = (l - 1_isize);
        'loop_: while (n < p) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = f;
        'loop_: while (n < (l - 1_isize)) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = p;
        'loop_: while (n < size_t1_6.with(|rc| *rc.borrow())) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
    }
    assert!(({ unimplemented!() } == { size_t1_6.with(|rc| *rc.borrow()) }));
}
pub fn tests_11(c: Ptr<()>, mut p: isize, mut f: isize, mut l: isize) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n: isize = 0_isize;
    let mut d: isize = if (l > (f + 1_isize)) {
        ((l - 1_isize) - f)
    } else {
        0_isize
    };
    if (d == 0_isize) || (p == f) {
        n = 0_isize;
        'loop_: while (n < size_t1_6.with(|rc| *rc.borrow())) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
    } else if (p < f) {
        n = 0_isize;
        'loop_: while (n < p) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = f;
        'loop_: while (n < (l - 1_isize)) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = p;
        'loop_: while (n < f) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = (l - 1_isize);
        'loop_: while (n < size_t1_6.with(|rc| *rc.borrow())) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
    } else {
        n = 0_isize;
        'loop_: while (n < f) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = (l - 1_isize);
        'loop_: while (n < p) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = f;
        'loop_: while (n < (l - 1_isize)) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
        n = p;
        'loop_: while (n < size_t1_6.with(|rc| *rc.borrow())) {
            assert!(
                ((((*i.borrow()).iter_deref().read()) as isize)
                    == ({
                        let __idx0 = (n) as usize;
                        t1_4.with(|rc| rc.borrow()[__idx0])
                    }))
            );
            {
                n.prefix_inc();
                (*i.borrow_mut()).iter_inc()
            };
        }
    }
    assert!(({ unimplemented!() } == { size_t1_6.with(|rc| *rc.borrow()) }));
}
pub fn test_12() -> bool {
    {
        let mut f: isize = 0_isize;
        'loop_: while (f <= (size_t2_7.with(|rc| *rc.borrow()) + 1_isize)) {
            let mut l: isize = f;
            'loop_: while (l <= (size_t2_7.with(|rc| *rc.borrow()) + 1_isize)) {
                let mut p: isize = 0_isize;
                'loop_: while (p <= size_t1_6.with(|rc| *rc.borrow())) {
                    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    unimplemented!();
                    ({ testd_8(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: isize = 0_isize;
        'loop_: while (f <= (size_t1_6.with(|rc| *rc.borrow()) + 1_isize)) {
            let mut l: isize = f;
            'loop_: while (l <= size_t1_6.with(|rc| *rc.borrow())) {
                let mut p: isize = 0_isize;
                'loop_: while (p <= f) {
                    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    unimplemented!();
                    ({ tests_10(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                let mut p: isize = l;
                'loop_: while (p <= size_t1_6.with(|rc| *rc.borrow())) {
                    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    unimplemented!();
                    ({ tests_10(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
    }
    {
        let mut f: isize = 0_isize;
        'loop_: while (f <= (size_t2_7.with(|rc| *rc.borrow()) + 1_isize)) {
            let mut l: isize = f;
            'loop_: while (l <= (size_t2_7.with(|rc| *rc.borrow()) + 1_isize)) {
                let mut p: isize = 0_isize;
                'loop_: while (p <= size_t1_6.with(|rc| *rc.borrow())) {
                    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    unimplemented!();
                    ({ testd_9(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: isize = 0_isize;
        'loop_: while (f <= (size_t1_6.with(|rc| *rc.borrow()) + 1_isize)) {
            let mut l: isize = f;
            'loop_: while (l <= size_t1_6.with(|rc| *rc.borrow())) {
                let mut p: isize = 0_isize;
                'loop_: while (p <= f) {
                    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    unimplemented!();
                    ({ tests_11(c1.as_pointer(), p, f, l) });
                    p.prefix_inc();
                }
                let mut p: isize = l;
                'loop_: while (p <= size_t1_6.with(|rc| *rc.borrow())) {
                    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                    unimplemented!();
                    ({ tests_11(c1.as_pointer(), p, f, l) });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    assert!(({ test_12() }));
    return 0;
}
pub trait min_allocator_long_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_long_Impl for Ptr<min_allocator_long_> {}
pub trait min_pointer_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i64> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i64> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i64> {
        unimplemented!()
    }
}
impl min_pointer_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_long__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
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
    let _ = t1_4.with(|_| ());
    let _ = t2_5.with(|_| ());
    let _ = size_t1_6.with(|_| ());
    let _ = size_t2_7.with(|_| ());
}
