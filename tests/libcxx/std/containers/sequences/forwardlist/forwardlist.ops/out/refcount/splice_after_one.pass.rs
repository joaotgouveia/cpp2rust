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
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static t1_4: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5, 6, 7])));
);
thread_local!(
    pub static t2_5: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([10, 11, 12])));
);
thread_local!(
    pub static size_t1_6: Value<isize> = Rc::new(RefCell::new(8_isize));
);
thread_local!(
    pub static size_t2_7: Value<isize> = Rc::new(RefCell::new(3_isize));
);
pub fn test_8() -> bool {
    {
        let mut f: i32 = 0;
        'loop_: while ((f as isize) <= (size_t2_7.with(|rc| *rc.borrow()) - 1_isize)) {
            let mut p: i32 = 0;
            'loop_: while ((p as isize) <= size_t1_6.with(|rc| *rc.borrow())) {
                let c1: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
                    std_forward_list_int__std_allocator_int__::new_1(
                        { ({ begin_9(t1_4.with(|v| v.as_pointer())) }) },
                        { (t1_4.with(|v| v.as_pointer()) as Ptr<i32>).to_end() },
                        None,
                    ),
                ));
                let c2: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
                    std_forward_list_int__std_allocator_int__::new_1(
                        { ({ begin_10(t2_5.with(|v| v.as_pointer())) }) },
                        { (t2_5.with(|v| v.as_pointer()) as Ptr<i32>).to_end() },
                        None,
                    ),
                ));
                ({
                    let ___pos: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                        ({ ({ (*c1.borrow()).cbefore_begin() }) }),
                        (p as isize),
                    );
                    let _arg1: Ptr<std_forward_list_int__std_allocator_int__> = c2.as_pointer();
                    let ___i: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                        ({ ({ (*c2.borrow()).cbefore_begin() }) }),
                        (f as isize),
                    );
                    (*c1.borrow()).splice_after_56(___pos, _arg1, ___i)
                });
                ({ testd_11(c1.as_pointer(), p, f) });
                p.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: i32 = 0;
        'loop_: while ((f as isize) <= (size_t1_6.with(|rc| *rc.borrow()) - 1_isize)) {
            let mut p: i32 = 0;
            'loop_: while ((p as isize) <= size_t1_6.with(|rc| *rc.borrow())) {
                let c1: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
                    std_forward_list_int__std_allocator_int__::new_1(
                        { ({ begin_9(t1_4.with(|v| v.as_pointer())) }) },
                        { (t1_4.with(|v| v.as_pointer()) as Ptr<i32>).to_end() },
                        None,
                    ),
                ));
                ({
                    let ___pos: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                        ({ ({ (*c1.borrow()).cbefore_begin() }) }),
                        (p as isize),
                    );
                    let _arg1: Ptr<std_forward_list_int__std_allocator_int__> = c1.as_pointer();
                    let ___i: std__Fwd_list_const_iterator_int_ = libcc2rs::next_refcount(
                        ({ ({ (*c1.borrow()).cbefore_begin() }) }),
                        (f as isize),
                    );
                    (*c1.borrow()).splice_after_56(___pos, _arg1, ___i)
                });
                ({ tests_12(c1.as_pointer(), p, f) });
                p.prefix_inc();
            }
            f.prefix_inc();
        }
    }
    {
        let mut f: i32 = 0;
        'loop_: while ((f as isize) <= (size_t2_7.with(|rc| *rc.borrow()) - 1_isize)) {
            let mut p: i32 = 0;
            'loop_: while ((p as isize) <= size_t1_6.with(|rc| *rc.borrow())) {
                let c1: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
                    std_forward_list_int__min_allocator_int__::new_1(
                        { ({ begin_9(t1_4.with(|v| v.as_pointer())) }) },
                        { (t1_4.with(|v| v.as_pointer()) as Ptr<i32>).to_end() },
                        None,
                    ),
                ));
                let c2: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
                    std_forward_list_int__min_allocator_int__::new_1(
                        { ({ begin_10(t2_5.with(|v| v.as_pointer())) }) },
                        { (t2_5.with(|v| v.as_pointer()) as Ptr<i32>).to_end() },
                        None,
                    ),
                ));
                ({
                    let ___pos: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c1.borrow()) . cbefore_begin ( ) } )   }), ( p as isize ) )   ;
                    let _arg1: Ptr<std_forward_list_int__min_allocator_int__> = c2.as_pointer();
                    let ___i: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c2.borrow()) . cbefore_begin ( ) } )   }), ( f as isize ) )   ;
                    (*c1.borrow()).splice_after_56(___pos, _arg1, ___i)
                });
                ({ testd_13(c1.as_pointer(), p, f) });
                p.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: i32 = 0;
        'loop_: while ((f as isize) <= (size_t1_6.with(|rc| *rc.borrow()) - 1_isize)) {
            let mut p: i32 = 0;
            'loop_: while ((p as isize) <= size_t1_6.with(|rc| *rc.borrow())) {
                let c1: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
                    std_forward_list_int__min_allocator_int__::new_1(
                        { ({ begin_9(t1_4.with(|v| v.as_pointer())) }) },
                        { (t1_4.with(|v| v.as_pointer()) as Ptr<i32>).to_end() },
                        None,
                    ),
                ));
                ({
                    let ___pos: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c1.borrow()) . cbefore_begin ( ) } )   }), ( p as isize ) )   ;
                    let _arg1: Ptr<std_forward_list_int__min_allocator_int__> = c1.as_pointer();
                    let ___i: std___fwdlist__Iterator_true__min_pointer_int__std_integral_constant_unsigned_long__0___   = libcc2rs::next_refcount(({ ( { (*c1.borrow()) . cbefore_begin ( ) } )   }), ( f as isize ) )   ;
                    (*c1.borrow()).splice_after_56(___pos, _arg1, ___i)
                });
                ({ tests_14(c1.as_pointer(), p, f) });
                p.prefix_inc();
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
    assert!(({ test_8() }));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
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
