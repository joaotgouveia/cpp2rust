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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
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
pub struct min_allocator_int_ {}
thread_local!(
    pub static t1_4: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5, 6, 7])));
);
thread_local!(
    pub static t2_5: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([10, 11, 12, 13, 14, 15])));
);
thread_local!(
    pub static size_t1_6: Value<isize> = Rc::new(RefCell::new(
        (((t1_4.with(|v| v.as_pointer()) as Ptr<i32>).to_end() - unimplemented!()) as i64 as isize),
    ));
);
thread_local!(
    pub static size_t2_7: Value<isize> = Rc::new(RefCell::new(
        (((t2_5.with(|v| v.as_pointer()) as Ptr<i32>).to_end() - unimplemented!()) as i64 as isize),
    ));
);
pub fn testd_8(c: Ptr<()>, mut p: i32, mut l: i32) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n1: i32 = 0;
    'loop_: while (n1 < p) {
        assert!(
            (((*i.borrow()).iter_deref().read())
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
    let mut n2: i32 = 0;
    'loop_: while (n2 < l) {
        assert!(
            (((*i.borrow()).iter_deref().read())
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
    'loop_: while ((n1 as isize) < size_t1_6.with(|rc| *rc.borrow())) {
        assert!(
            (((*i.borrow()).iter_deref().read())
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
    assert!(({ unimplemented!() } == { (size_t1_6.with(|rc| *rc.borrow()) + (l as isize)) }));
}
pub fn testd_9(c: Ptr<()>, mut p: i32, mut l: i32) {
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n1: i32 = 0;
    'loop_: while (n1 < p) {
        assert!(
            (((*i.borrow()).iter_deref().read())
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
    let mut n2: i32 = 0;
    'loop_: while (n2 < l) {
        assert!(
            (((*i.borrow()).iter_deref().read())
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
    'loop_: while ((n1 as isize) < size_t1_6.with(|rc| *rc.borrow())) {
        assert!(
            (((*i.borrow()).iter_deref().read())
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
    assert!(({ unimplemented!() } == { (size_t1_6.with(|rc| *rc.borrow()) + (l as isize)) }));
}
pub fn test_10() -> bool {
    {
        let mut l: i32 = 0;
        'loop_: while ((l as isize) <= size_t2_7.with(|rc| *rc.borrow())) {
            let mut p: i32 = 0;
            'loop_: while ((p as isize) <= size_t1_6.with(|rc| *rc.borrow())) {
                let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                ({ testd_8(c1.as_pointer(), p, l) });
                p.prefix_inc();
            }
            l.prefix_inc();
        }
    }
    {
        let mut l: i32 = 0;
        'loop_: while ((l as isize) <= size_t2_7.with(|rc| *rc.borrow())) {
            let mut p: i32 = 0;
            'loop_: while ((p as isize) <= size_t1_6.with(|rc| *rc.borrow())) {
                let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                ({ testd_9(c1.as_pointer(), p, l) });
                p.prefix_inc();
            }
            l.prefix_inc();
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
    assert!(({ test_10() }));
    return 0;
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
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
