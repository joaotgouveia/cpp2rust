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
pub fn check_vector_reverse_iterators_4() {
    {
        let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        assert!(({ (*vec_.borrow_mut()).rbegin_32() }) == ({ (*vec_.borrow_mut()).rend_34() }));
        assert!(({ (*vec_.borrow()).crbegin() }) == ({ (*vec_.borrow()).crend() }));
    }
    {
        let mut n: i32 = 10;
        let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        let cvec: Ptr<Vec<i32>> = vec_.as_pointer();
        if (n as usize) as usize > (*vec_.borrow()).capacity() as usize {
            let len_0 = (*vec_.borrow()).len();
            (*vec_.borrow_mut()).reserve_exact((n as usize) as usize - len_0 as usize);
        };
        let mut i: i32 = 0;
        'loop_: while (i < n) {
            {
                let a0_clone = i.clone();
                (*vec_.borrow_mut()).push(a0_clone)
            };
            i.prefix_inc();
        }
        {
            let mut iterations: i32 = 0;
            let it : Value<std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(( { (*vec_.borrow()) . crbegin ( ) } )  )) ;
            'loop_: while (*it.borrow()) != ({ (*vec_.borrow()).crend() }) {
                assert!(({ (((*it.borrow()).iter_deref().read()) == ((n - iterations) - 1)) }));
                iterations.prefix_inc();
                {
                    it.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    it.as_pointer()
                };
            }
            assert!((iterations == n));
        }
        {
            assert!(
                ({ (*cvec.upgrade().deref()).rbegin_33() }) == ({ (*vec_.borrow()).crbegin() })
            );
            assert!(({ (*cvec.upgrade().deref()).rend_35() }) == ({ (*vec_.borrow()).crend() }));
        }
        {
            let mut iterations: i32 = 0;
            let it : Value<std_reverse_iterator___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int____ > = Rc::new(RefCell::new(( { (*vec_.borrow_mut()) . rbegin_32 ( ) } )  )) ;
            'loop_: while (*it.borrow()) != ({ (*vec_.borrow_mut()).rend_34() }) {
                assert!(({ (((*it.borrow()).iter_deref().read()) == ((n - iterations) - 1)) }));
                (*it.borrow()).iter_deref().write(40);
                assert!(({ (((*it.borrow()).iter_deref().read()) == 40) }));
                iterations.prefix_inc();
                {
                    it.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    it.as_pointer()
                };
            }
            assert!((iterations == n));
        }
        assert!(
            ({
                (({
                    let ___first: std_reverse_iterator___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int____   = ( { (*vec_.borrow_mut()) . rbegin_32 ( ) } )   ;
                    let ___last: std_reverse_iterator___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int____   = ( { (*vec_.borrow_mut()) . rend_34 ( ) } )   ;
                    distance_5(___first, ___last)
                }) == (n as i64))
            })
        );
        assert!(
            ({
                ({
                    ({
                        let ___first: std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____   = ( { (*cvec.upgrade().deref()) . rbegin_33 ( ) } )   ;
                        let ___last: std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____   = ( { (*cvec.upgrade().deref()) . rend_35 ( ) } )   ;
                        distance_6(___first, ___last)
                    })
                } == { (n as i64) })
            })
        );
        assert!(
            ({
                (({
                    let ___first: std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____   = ( { (*vec_.borrow()) . crbegin ( ) } )   ;
                    let ___last: std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____   = ( { (*vec_.borrow()) . crend ( ) } )   ;
                    distance_6(___first, ___last)
                }) == (n as i64))
            })
        );
        assert!(
            ({
                ({
                    ({
                        let ___first: std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____   = ( { (*cvec.upgrade().deref()) . crbegin ( ) } )   ;
                        let ___last: std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____   = ( { (*cvec.upgrade().deref()) . crend ( ) } )   ;
                        distance_6(___first, ___last)
                    })
                } == { (n as i64) })
            })
        );
    }
}
pub fn check_vector_reverse_iterators_7() {
    {
        let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        assert!(({ (*vec_.borrow_mut()).rbegin_32() }) == ({ (*vec_.borrow_mut()).rend_34() }));
        assert!(({ (*vec_.borrow()).crbegin() }) == ({ (*vec_.borrow()).crend() }));
    }
    {
        let mut n: i32 = 10;
        let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        let cvec: Ptr<Vec<i32>> = vec_.as_pointer();
        if (n as usize) as usize > (*vec_.borrow()).capacity() as usize {
            let len_0 = (*vec_.borrow()).len();
            (*vec_.borrow_mut()).reserve_exact((n as usize) as usize - len_0 as usize);
        };
        let mut i: i32 = 0;
        'loop_: while (i < n) {
            {
                let a0_clone = i.clone();
                (*vec_.borrow_mut()).push(a0_clone)
            };
            i.prefix_inc();
        }
        {
            let mut iterations: i32 = 0;
            let it : Value<std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____ > = Rc::new(RefCell::new(( { (*vec_.borrow()) . crbegin ( ) } )  )) ;
            'loop_: while (*it.borrow()) != ({ (*vec_.borrow()).crend() }) {
                assert!(({ (((*it.borrow()).iter_deref().read()) == ((n - iterations) - 1)) }));
                iterations.prefix_inc();
                {
                    it.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    it.as_pointer()
                };
            }
            assert!((iterations == n));
        }
        {
            assert!(
                ({ (*cvec.upgrade().deref()).rbegin_33() }) == ({ (*vec_.borrow()).crbegin() })
            );
            assert!(({ (*cvec.upgrade().deref()).rend_35() }) == ({ (*vec_.borrow()).crend() }));
        }
        {
            let mut iterations: i32 = 0;
            let it : Value<std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____ > = Rc::new(RefCell::new(( { (*vec_.borrow_mut()) . rbegin_32 ( ) } )  )) ;
            'loop_: while (*it.borrow()) != ({ (*vec_.borrow_mut()).rend_34() }) {
                assert!(({ (((*it.borrow()).iter_deref().read()) == ((n - iterations) - 1)) }));
                (*it.borrow()).iter_deref().write(40);
                assert!(({ (((*it.borrow()).iter_deref().read()) == 40) }));
                iterations.prefix_inc();
                {
                    it.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    it.as_pointer()
                };
            }
            assert!((iterations == n));
        }
        assert!(
            ({
                (({
                    let ___first: std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____   = ( { (*vec_.borrow_mut()) . rbegin_32 ( ) } )   ;
                    let ___last: std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____   = ( { (*vec_.borrow_mut()) . rend_34 ( ) } )   ;
                    distance_8(___first, ___last)
                }) == (n as i64))
            })
        );
        assert!(
            ({
                ({
                    ({
                        let ___first: std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____   = ( { (*cvec.upgrade().deref()) . rbegin_33 ( ) } )   ;
                        let ___last: std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____   = ( { (*cvec.upgrade().deref()) . rend_35 ( ) } )   ;
                        distance_9(___first, ___last)
                    })
                } == { (n as i64) })
            })
        );
        assert!(
            ({
                (({
                    let ___first: std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____   = ( { (*vec_.borrow()) . crbegin ( ) } )   ;
                    let ___last: std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____   = ( { (*vec_.borrow()) . crend ( ) } )   ;
                    distance_9(___first, ___last)
                }) == (n as i64))
            })
        );
        assert!(
            ({
                ({
                    ({
                        let ___first: std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____   = ( { (*cvec.upgrade().deref()) . crbegin ( ) } )   ;
                        let ___last: std_reverse_iterator___gnu_cxx___normal_iterator_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_vector_int__min_allocator_int____   = ( { (*cvec.upgrade().deref()) . crend ( ) } )   ;
                        distance_9(___first, ___last)
                    })
                } == { (n as i64) })
            })
        );
    }
}
pub fn test_10() -> bool {
    ({ check_vector_reverse_iterators_4() });
    ({ check_vector_reverse_iterators_7() });
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
    ({ test_10() });
    const _: () = assert!(true, "test()");;
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            ({ <std_allocator_int_>::default().allocate(n) })
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        ({ <std_allocator_int_>::default().deallocate((p.ptr_).clone(), n) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
