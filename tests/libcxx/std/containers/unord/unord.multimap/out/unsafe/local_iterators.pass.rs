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
pub struct min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
        return min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
{
    ptr_: *const (),
}
impl min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
    {
        return min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_index ( &self , mut n : () , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_  . offset ( ( n ) as isize ) )  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__std_string__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { return min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
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
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_0: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_0 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_1: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_1 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_2: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_2 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_3: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_3 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_4: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_4 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_5: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_5 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut c: () = unimplemented!();
        unimplemented!();
        let mut b: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_6: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_6 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_7: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_7 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_8: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_8 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_9: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_9 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_10: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_10 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_11: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_11 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let c: () = unimplemented!();
        unimplemented!();
        let mut b: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_12: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_12 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_13: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_13 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_14: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_14 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_15: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_15 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_16: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_16 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_17: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_17 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut c: () = unimplemented!();
        unimplemented!();
        let mut b: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_18: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_18 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_19: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_19 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_20: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_20 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_21: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_21 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_22: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_22 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_23: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_23 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let c: () = unimplemented!();
        unimplemented!();
        let mut b: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_24: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_24 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_25: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_25 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_26: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_26 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_27: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_27 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_28: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_28 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_29: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_29 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut c: () = unimplemented!();
        unimplemented!();
        let mut b: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_30: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_30 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_31: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_31 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_32: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_32 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_33: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_33 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_34: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_34 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_35: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_35 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let c: () = unimplemented!();
        unimplemented!();
        let mut b: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_36: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_36 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_37: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_37 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_38: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_38 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_39: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_39 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_40: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_40 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_41: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_41 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut c: () = unimplemented!();
        unimplemented!();
        let mut b: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_42: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_42 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_43: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_43 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_44: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_44 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_45: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_45 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_46: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_46 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_47: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_47 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let c: () = unimplemented!();
        unimplemented!();
        let mut b: () = unimplemented!();
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut n: () = 0_();
            'loop_: while ((n) < (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                n.prefix_inc();
            }
        }
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        b = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
