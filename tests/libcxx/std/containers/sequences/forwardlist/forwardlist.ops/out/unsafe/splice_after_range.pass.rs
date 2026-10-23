extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_long__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_long__std_integral_constant_unsigned_long__0__ {}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p : min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_ as ()),
        };
        this
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_2(
        mut p : min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_ as ()),
        };
        this
    }
}
pub unsafe fn operator_eq_4(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_5(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_void__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_4(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl std::cmp::PartialEq for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_6(
    mut x : min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_7(
    mut x : min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_6(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_6(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_8(
    mut x : min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_8(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___forward_begin_node_min_pointer_std___forward_list_node_long__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____
{}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub static mut t1_9: std::cell::LazyCell<[(); 8]> =
    std::cell::LazyCell::new(|| unsafe { [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_()] });
pub static mut t2_10: std::cell::LazyCell<[(); 6]> =
    std::cell::LazyCell::new(|| unsafe { [10_(), 11_(), 12_(), 13_(), 14_(), 15_()] });
pub static mut size_t1_11: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (((unimplemented!() as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ()
        as ())
});
pub static mut size_t2_12: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (((unimplemented!() as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ()
        as ())
});
pub unsafe fn testd_13(c: *const (), mut p: (), mut f: (), mut l: ()) {
    let mut i: () = unimplemented!();
    let mut n1: () = 0_();
    'loop_: while ((n1) < (p)) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    let mut n2: () = f;
    'loop_: while ((n2) < ((l) - (1_()))) {
        unimplemented!();
        {
            n2.prefix_inc();
            unimplemented!()
        };
    }
    'loop_: while ((n1) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    unimplemented!();
}
pub unsafe fn testd_14(c: *const (), mut p: (), mut f: (), mut l: ()) {
    let mut i: () = unimplemented!();
    let mut n1: () = 0_();
    'loop_: while ((n1) < (p)) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    let mut n2: () = f;
    'loop_: while ((n2) < ((l) - (1_()))) {
        unimplemented!();
        {
            n2.prefix_inc();
            unimplemented!()
        };
    }
    'loop_: while ((n1) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
        unimplemented!();
        {
            n1.prefix_inc();
            unimplemented!()
        };
    }
    unimplemented!();
}
pub unsafe fn tests_15(c: *const (), mut p: (), mut f: (), mut l: ()) {
    let mut i: () = unimplemented!();
    let mut n: () = 0_();
    let mut d: () = if ((l) > ((f) + (1_()))) {
        (((l) - (1_())) - (f))
    } else {
        0_()
    };
    if ((d) == (0_())) || ((p) == (f)) {
        n = 0_();
        'loop_: while ((n) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    } else if ((p) < (f)) {
        n = 0_();
        'loop_: while ((n) < (p)) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = f;
        'loop_: while ((n) < ((l) - (1_()))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = p;
        'loop_: while ((n) < (f)) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = ((l) - (1_()));
        'loop_: while ((n) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    } else {
        n = 0_();
        'loop_: while ((n) < (f)) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = ((l) - (1_()));
        'loop_: while ((n) < (p)) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = f;
        'loop_: while ((n) < ((l) - (1_()))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = p;
        'loop_: while ((n) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    }
    unimplemented!();
}
pub unsafe fn tests_16(c: *const (), mut p: (), mut f: (), mut l: ()) {
    let mut i: () = unimplemented!();
    let mut n: () = 0_();
    let mut d: () = if ((l) > ((f) + (1_()))) {
        (((l) - (1_())) - (f))
    } else {
        0_()
    };
    if ((d) == (0_())) || ((p) == (f)) {
        n = 0_();
        'loop_: while ((n) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    } else if ((p) < (f)) {
        n = 0_();
        'loop_: while ((n) < (p)) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = f;
        'loop_: while ((n) < ((l) - (1_()))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = p;
        'loop_: while ((n) < (f)) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = ((l) - (1_()));
        'loop_: while ((n) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    } else {
        n = 0_();
        'loop_: while ((n) < (f)) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = ((l) - (1_()));
        'loop_: while ((n) < (p)) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = f;
        'loop_: while ((n) < ((l) - (1_()))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
        n = p;
        'loop_: while ((n) < (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
            unimplemented!();
            {
                n.prefix_inc();
                unimplemented!()
            };
        }
    }
    unimplemented!();
}
pub unsafe fn test_17() -> () {
    {
        let mut f: () = 0_();
        'loop_: while ((f)
            <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t2_12)) + (1_())))
        {
            let mut l: () = f;
            'loop_: while ((l)
                <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t2_12)) + (1_())))
            {
                let mut p: () = 0_();
                'loop_: while ((p) <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11)))
                {
                    let mut c1: () =
                        std_forward_list_long__std_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        });
                    let mut c2: () =
                        std_forward_list_long__std_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        });
                    unimplemented!();
                    (unsafe { testd_13(&c1, p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: () = 0_();
        'loop_: while ((f)
            <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11)) + (1_())))
        {
            let mut l: () = f;
            'loop_: while ((l) <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
                let mut p: () = 0_();
                'loop_: while ((p) <= (f)) {
                    let mut c1: () =
                        std_forward_list_long__std_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        });
                    unimplemented!();
                    (unsafe { tests_15(&c1, p, f, l) });
                    p.prefix_inc();
                }
                let mut p: () = l;
                'loop_: while ((p) <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11)))
                {
                    let mut c1: () =
                        std_forward_list_long__std_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        });
                    unimplemented!();
                    (unsafe { tests_15(&c1, p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
    }
    {
        let mut f: () = 0_();
        'loop_: while ((f)
            <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t2_12)) + (1_())))
        {
            let mut l: () = f;
            'loop_: while ((l)
                <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t2_12)) + (1_())))
            {
                let mut p: () = 0_();
                'loop_: while ((p) <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11)))
                {
                    let mut c1: () =
                        std_forward_list_long__min_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        });
                    let mut c2: () =
                        std_forward_list_long__min_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        });
                    unimplemented!();
                    (unsafe { testd_14(&c1, p, f, l) });
                    p.prefix_inc();
                }
                l.prefix_inc();
            }
            f.prefix_inc();
        }
        let mut f: () = 0_();
        'loop_: while ((f)
            <= ((*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11)) + (1_())))
        {
            let mut l: () = f;
            'loop_: while ((l) <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11))) {
                let mut p: () = 0_();
                'loop_: while ((p) <= (f)) {
                    let mut c1: () =
                        std_forward_list_long__min_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        });
                    unimplemented!();
                    (unsafe { tests_16(&c1, p, f, l) });
                    p.prefix_inc();
                }
                let mut p: () = l;
                'loop_: while ((p) <= (*std::cell::LazyCell::force_mut(&mut *&raw mut size_t1_11)))
                {
                    let mut c1: () =
                        std_forward_list_long__min_allocator_long__::new_1({ unimplemented!() }, {
                            unimplemented!()
                        });
                    unimplemented!();
                    (unsafe { tests_16(&c1, p, f, l) });
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
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const t1_9);
    std::cell::LazyCell::force(&*&raw const t2_10);
    std::cell::LazyCell::force(&*&raw const size_t1_11);
    std::cell::LazyCell::force(&*&raw const size_t2_12);
}
