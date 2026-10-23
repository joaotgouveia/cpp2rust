extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn CheckConsecutiveKeys_0(mut pos: (), mut end: (), key: (), values: *mut ()) {
    'loop_: while !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn CheckConsecutiveKeys_1(mut pos: (), mut end: (), key: (), values: *mut ()) {
    'loop_: while !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_3))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_4))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_5)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_2)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_3)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_4)) = 0_();
    }
}
pub static mut outstanding_bytes_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_5: std::cell::LazyCell<()> =
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
        let mut i : () = std___hash_map_const_iterator_std___hash_const_iterator_std___hash_node_std___hash_value_type_int__std_string___void_ptr__ptr__ :: new_1 ( { unimplemented!()  } , )  ;
        let mut j: () = (unsafe { next_6(i.clone(), 2_()) });
        let mut k: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut eq: () = unimplemented!();
        unimplemented!();
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: ()  = std___hash_map_const_iterator_std___hash_const_iterator_std___hash_node_std___hash_value_type_int__std_string___void_ptr__ptr__ :: new_1 ( { unimplemented!()  } , )  ;
            let _end: ()  = std___hash_map_const_iterator_std___hash_const_iterator_std___hash_node_std___hash_value_type_int__std_string___void_ptr__ptr__ :: new_1 ( { unimplemented!()  } , )  ;
            CheckConsecutiveKeys_0(_pos, _end, 1_(), &mut s)
        });
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: ()  = std___hash_map_const_iterator_std___hash_const_iterator_std___hash_node_std___hash_value_type_int__std_string___void_ptr__ptr__ :: new_1 ( { unimplemented!()  } , )  ;
            let _end: ()  = std___hash_map_const_iterator_std___hash_const_iterator_std___hash_node_std___hash_value_type_int__std_string___void_ptr__ptr__ :: new_1 ( { unimplemented!()  } , )  ;
            CheckConsecutiveKeys_0(_pos, _end, 2_(), &mut s)
        });
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: ()  = std___hash_map_const_iterator_std___hash_const_iterator_std___hash_node_std___hash_value_type_int__std_string___void_ptr__ptr__ :: new_1 ( { unimplemented!()  } , )  ;
            let _end: ()  = std___hash_map_const_iterator_std___hash_const_iterator_std___hash_node_std___hash_value_type_int__std_string___void_ptr__ptr__ :: new_1 ( { unimplemented!()  } , )  ;
            CheckConsecutiveKeys_0(_pos, _end, 1_(), &mut s)
        });
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut map: () = unimplemented!();
            (unsafe {
                let mut ___x: () = unimplemented!();
                map.insert_45_struct_std_pair_int__const_char_ptr__0(&mut ___x)
            });
            (unsafe {
                let mut ___x: () = unimplemented!();
                map.insert_45_struct_std_pair_int__const_char_ptr__0(&mut ___x)
            });
            unimplemented!();
        }
        {
            let mut map: () = unimplemented!();
            (unsafe {
                let mut ___x: () = unimplemented!();
                map.insert_45_struct_std_pair_int__const_char_ptr__0(&mut ___x)
            });
            (unsafe {
                let mut ___x: () = unimplemented!();
                map.insert_45_struct_std_pair_int__const_char_ptr__0(&mut ___x)
            });
            unimplemented!();
        }
    }
    {
        let mut map: () = unimplemented!();
        let mut collision_val: () = (2_()).wrapping_add(unimplemented!());
        unimplemented!();
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_6: () = 1_();
                let mut __tmp_7: () = 1_();
                std_pair_unsigned_long__unsigned_long_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
            };
            map.insert_45_struct_std_pair_unsigned_long__unsigned_long__0(&mut ___x)
        });
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_8: () = 1_();
                std_pair_unsigned_long__unsigned_long_::new_2({ &mut collision_val }, {
                    &mut __tmp_8
                })
            };
            map.insert_45_struct_std_pair_unsigned_long__unsigned_long__0(&mut ___x)
        });
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_9: () = 2_();
                let mut __tmp_10: () = 1_();
                std_pair_unsigned_long__unsigned_long_::new_1({ &mut __tmp_9 }, { &mut __tmp_10 })
            };
            map.insert_45_struct_std_pair_unsigned_long__unsigned_long__0(&mut ___x)
        });
        unimplemented!();
        let mut erase: () = unimplemented!();
        unimplemented!();
        'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
            let v: *const std_pair_const_unsigned_long__unsigned_long_ = &(*unimplemented!());
            unimplemented!();
        }
    }
    {
        let mut m: () = unimplemented!();
        (unsafe {
            let mut ___x: () = unimplemented!();
            m.insert_45_struct_std_pair_int__int__0(&mut ___x)
        });
        (unsafe {
            let mut ___x: () = unimplemented!();
            m.insert_45_struct_std_pair_int__int__0(&mut ___x)
        });
        {
            let mut pair: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut pair: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe {
            let mut ___x: () = unimplemented!();
            m.insert_45_struct_std_pair_int__int__0(&mut ___x)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 6] = [
            {
                let mut __tmp_11: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_11 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_12: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_12 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_13: () = 3_();
                std_pair_int__std_string_::new_2({ &mut __tmp_13 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_14: () = 4_();
                std_pair_int__std_string_::new_3({ &mut __tmp_14 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_15: () = 1_();
                std_pair_int__std_string_::new_3({ &mut __tmp_15 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_16: () = 2_();
                std_pair_int__std_string_::new_3({ &mut __tmp_16 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut c: () = unimplemented!();
        let mut i : () = std___hash_map_const_iterator_std___hash_const_iterator_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
        let mut j: () = (unsafe { next_8(i.clone(), 2_()) });
        let mut k: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut eq: () = unimplemented!();
        unimplemented!();
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: ()  = std___hash_map_const_iterator_std___hash_const_iterator_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
            let _end: ()  = std___hash_map_const_iterator_std___hash_const_iterator_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
            CheckConsecutiveKeys_1(_pos, _end, 1_(), &mut s)
        });
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: ()  = std___hash_map_const_iterator_std___hash_const_iterator_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
            let _end: ()  = std___hash_map_const_iterator_std___hash_const_iterator_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
            CheckConsecutiveKeys_1(_pos, _end, 2_(), &mut s)
        });
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: ()  = std___hash_map_const_iterator_std___hash_const_iterator_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
            let _end: ()  = std___hash_map_const_iterator_std___hash_const_iterator_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
            CheckConsecutiveKeys_1(_pos, _end, 1_(), &mut s)
        });
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        eq = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyHash_7 {}
impl MyHash_7 {
    pub unsafe fn operator_call(&self, mut v: ()) -> () {
        return v;
    }
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_2);
    std::cell::LazyCell::force(&*&raw const alloc_count_3);
    std::cell::LazyCell::force(&*&raw const dealloc_count_4);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_5);
}
