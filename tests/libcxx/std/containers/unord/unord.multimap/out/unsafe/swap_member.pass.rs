extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(a: *const test_equal_to_int_, b: *const test_equal_to_int_) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_equal_to_int_ {
    pub data_: (),
}
impl test_equal_to_int_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const (), b: *const ()) -> () {
        return ((*a) == (*b));
    }
}
impl std::cmp::PartialEq for test_equal_to_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const test_equal_to_int_,
                other as *const test_equal_to_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_equal_to_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_hash_int_ {
    data_: (),
}
impl test_hash_int_ {
    pub unsafe fn new(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, x: *const ()) -> () {
        return (unsafe { (unimplemented!() as std___hash_impl_int__void_)((*x)) });
    }
    pub unsafe fn operator_eq(&self, c: *const test_hash_int_) -> () {
        return ((self.data_) == ((*c).data_));
    }
}
impl std::cmp::PartialEq for test_hash_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { test_hash_int_::operator_eq(self, other as *const test_hash_int_) }
    }
}
impl std::cmp::Eq for test_hash_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: (),
    pub throw_after: (),
    pub count: (),
    pub alloc_count: (),
    pub allocated_size: (),
    pub construct_count: (),
    pub destroy_count: (),
    pub copied: (),
    pub moved: (),
    pub converted: (),
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        unimplemented!();
        self.count = 0_();
        self.time_to_throw = 0_();
        self.alloc_count = 0_();
        self.allocated_size = 0_();
        self.construct_count = 0_();
        self.destroy_count = 0_();
        self.throw_after = 2147483647_();
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0_();
        self.moved = 0_();
        self.converted = 0_();
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
pub static mut destructed_value_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_3(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_4(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_3(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0_(),
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: (),
        mut id: (),
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const test_allocator_void_,
    ) -> *mut test_allocator_void_ {
        self.data_ = (*_a0).data_;
        self.id_ = (*_a0).id_;
        self.stats_ = (*_a0).stats_;
        return &mut (*(self as *mut test_allocator_void_));
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(
                self as *const test_allocator_void_,
                other as *const test_allocator_void_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_void_::copy_from(self as *const test_allocator_void_) }
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        unsafe {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__std_string___rebind_std_pair_const_int__std_string__
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_int__std_string__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__std_string__ {
    pub unsafe fn new_1(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_std_pair_const_int__std_string__) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
}
impl Clone for test_allocator_std_pair_const_int__std_string__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_int__std_string__::copy_from(
                self as *const test_allocator_std_pair_const_int__std_string__,
            )
        }
    }
}
impl Default for test_allocator_std_pair_const_int__std_string__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__std_string__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_5(
    x: *const other_allocator_std_pair_const_int__std_string__,
    y: *const other_allocator_std_pair_const_int__std_string__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std_pair_const_int__std_string__ {
    data_: (),
}
impl other_allocator_std_pair_const_int__std_string__ {
    pub unsafe fn new_1(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for other_allocator_std_pair_const_int__std_string__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const other_allocator_std_pair_const_int__std_string__,
                other as *const other_allocator_std_pair_const_int__std_string__,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_std_pair_const_int__std_string__ {}
impl Default for other_allocator_std_pair_const_int__std_string__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std_pair_const_int__std_string__ {
                data_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_9)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_6)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8)) = 0_();
    }
}
pub static mut outstanding_bytes_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_9: std::cell::LazyCell<()> =
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
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_index ( &self , mut n : () , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_  . offset ( ( n ) as isize ) )  ;
 } }
pub unsafe fn operator_eq_10(
    mut _a0: min_allocator_std_pair_const_int__std_string__,
    mut _a1: min_allocator_std_pair_const_int__std_string__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__std_string__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_int__std_string__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_int__std_string__ {}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn new_1 ( mut _a0 : min_allocator_std_pair_const_int__std_string__ , ) -> Self { let mut this = Self { } ;
 this } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { return min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { }
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
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        let mut a2: [(); 8] = [
            {
                let mut __tmp_0: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_0 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_1: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_1 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_2: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_2 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_3: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_3 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_4: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_4 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_5: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_5 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_6: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_6 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_7: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_7 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        let mut a1: [(); 6] = [
            {
                let mut __tmp_8: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_8 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_9: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_9 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_10: () = 3_();
                std_pair_int__std_string_::new_3({ &mut __tmp_10 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_11: () = 4_();
                std_pair_int__std_string_::new_5({ &mut __tmp_11 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_12: () = 1_();
                std_pair_int__std_string_::new_5({ &mut __tmp_12 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_13: () = 2_();
                std_pair_int__std_string_::new_5({ &mut __tmp_13 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
        let mut a1: [(); 6] = [
            {
                let mut __tmp_14: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_14 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_15: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_15 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_16: () = 3_();
                std_pair_int__std_string_::new_3({ &mut __tmp_16 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_17: () = 4_();
                std_pair_int__std_string_::new_5({ &mut __tmp_17 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_18: () = 1_();
                std_pair_int__std_string_::new_5({ &mut __tmp_18 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_19: () = 2_();
                std_pair_int__std_string_::new_5({ &mut __tmp_19 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut a2: [(); 8] = [
            {
                let mut __tmp_20: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_20 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_21: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_21 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_22: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_22 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_23: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_23 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_24: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_24 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_25: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_25 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_26: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_26 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_27: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_27 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        let mut a2: [(); 8] = [
            {
                let mut __tmp_28: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_28 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_29: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_29 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_30: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_30 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_31: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_31 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_32: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_32 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_33: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_33 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_34: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_34 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_35: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_35 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        let mut a1: [(); 6] = [
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
                std_pair_int__std_string_::new_3({ &mut __tmp_38 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_39: () = 4_();
                std_pair_int__std_string_::new_5({ &mut __tmp_39 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_40: () = 1_();
                std_pair_int__std_string_::new_5({ &mut __tmp_40 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_41: () = 2_();
                std_pair_int__std_string_::new_5({ &mut __tmp_41 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
        let mut a1: [(); 6] = [
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
                std_pair_int__std_string_::new_3({ &mut __tmp_44 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_45: () = 4_();
                std_pair_int__std_string_::new_5({ &mut __tmp_45 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_46: () = 1_();
                std_pair_int__std_string_::new_5({ &mut __tmp_46 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_47: () = 2_();
                std_pair_int__std_string_::new_5({ &mut __tmp_47 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut a2: [(); 8] = [
            {
                let mut __tmp_48: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_48 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_49: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_49 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_50: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_50 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_51: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_51 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_52: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_52 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_53: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_53 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_54: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_54 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_55: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_55 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        let mut a2: [(); 8] = [
            {
                let mut __tmp_56: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_56 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_57: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_57 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_58: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_58 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_59: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_59 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_60: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_60 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_61: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_61 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_62: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_62 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_63: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_63 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        let mut a1: [(); 6] = [
            {
                let mut __tmp_64: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_64 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_65: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_65 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_66: () = 3_();
                std_pair_int__std_string_::new_3({ &mut __tmp_66 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_67: () = 4_();
                std_pair_int__std_string_::new_5({ &mut __tmp_67 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_68: () = 1_();
                std_pair_int__std_string_::new_5({ &mut __tmp_68 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_69: () = 2_();
                std_pair_int__std_string_::new_5({ &mut __tmp_69 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
        let mut a1: [(); 6] = [
            {
                let mut __tmp_70: () = 1_();
                std_pair_int__std_string_::new_1({ &mut __tmp_70 }, {
                    &std::mem::transmute(*b"one\0")
                })
            },
            {
                let mut __tmp_71: () = 2_();
                std_pair_int__std_string_::new_1({ &mut __tmp_71 }, {
                    &std::mem::transmute(*b"two\0")
                })
            },
            {
                let mut __tmp_72: () = 3_();
                std_pair_int__std_string_::new_3({ &mut __tmp_72 }, {
                    &std::mem::transmute(*b"three\0")
                })
            },
            {
                let mut __tmp_73: () = 4_();
                std_pair_int__std_string_::new_5({ &mut __tmp_73 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_74: () = 1_();
                std_pair_int__std_string_::new_5({ &mut __tmp_74 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
            {
                let mut __tmp_75: () = 2_();
                std_pair_int__std_string_::new_5({ &mut __tmp_75 }, {
                    &std::mem::transmute(*b"four\0")
                })
            },
        ];
        let mut a2: [(); 8] = [
            {
                let mut __tmp_76: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_76 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_77: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_77 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_78: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_78 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_79: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_79 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_80: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_80 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_81: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_81 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_82: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_82 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_83: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_83 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_6);
    std::cell::LazyCell::force(&*&raw const alloc_count_7);
    std::cell::LazyCell::force(&*&raw const dealloc_count_8);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_9);
}
