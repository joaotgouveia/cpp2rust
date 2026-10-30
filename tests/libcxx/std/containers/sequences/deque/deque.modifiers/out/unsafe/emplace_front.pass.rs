extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct Emplaceable {
    int_: i32,
    double_: f64,
}
impl Emplaceable {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            int_: 0,
            double_: 0_f64,
        };
        this
    }
    pub unsafe fn new_2(mut i: i32, mut d: f64) -> Self {
        let mut this = Self {
            int_: i,
            double_: d,
        };
        this
    }
    pub unsafe fn move_from(x: *mut Emplaceable) -> Self {
        let mut this = Self {
            int_: (*x).int_,
            double_: (*x).double_,
        };
        (*x).int_ = 0;
        (*x).double_ = 0_f64;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut Emplaceable) -> *mut Emplaceable {
        self.int_ = (*x).int_;
        (*x).int_ = 0;
        self.double_ = (*x).double_;
        (*x).double_ = 0_f64;
        return &mut (*(self as *mut Emplaceable));
    }
    pub unsafe fn operator_eq(&self, x: *const Emplaceable) -> bool {
        return ((self.int_) == ((*x).int_)) && ((self.double_) == ((*x).double_));
    }
    pub unsafe fn operator_lt(&self, x: *const Emplaceable) -> bool {
        return ((self.int_) < ((*x).int_))
            || (((self.int_) == ((*x).int_)) && ((self.double_) < ((*x).double_)));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.int_;
    }
}
impl std::cmp::Ord for Emplaceable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Emplaceable::operator_lt(self, other as *const Emplaceable) {
                std::cmp::Ordering::Less
            } else if Emplaceable::operator_lt(other, self as *const Emplaceable) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Emplaceable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Emplaceable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Emplaceable::operator_eq(self, other as *const Emplaceable) }
    }
}
impl std::cmp::Eq for Emplaceable {}
impl Default for Emplaceable {
    fn default() -> Self {
        unsafe { Emplaceable::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_Emplaceable_ {}
impl min_allocator_Emplaceable_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        return min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_Emplaceable_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe { <std_allocator_Emplaceable_>::default().deallocate(p.ptr_, n) });
    }
}
impl min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_Emplaceable_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { ( unsafe { <std_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ >::default() . allocate  ( n , ) } )  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            <std_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ >::default() . deallocate  ( p . ptr_  , n , )
        });
    }
}
impl min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: i32,
    pub throw_after: i32,
    pub count: i32,
    pub alloc_count: i32,
    pub allocated_size: i32,
    pub construct_count: i32,
    pub destroy_count: i32,
    pub copied: i32,
    pub moved: i32,
    pub converted: i32,
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        assert!(
            ((self.count) == (0)) && (!(c"clearing leaking allocator data?".as_ptr()).is_null())
        );
        self.count = 0;
        self.time_to_throw = 0;
        self.alloc_count = 0;
        self.allocated_size = 0;
        self.construct_count = 0;
        self.destroy_count = 0;
        self.throw_after = 2147483647;
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0;
        self.moved = 0;
        self.converted = 0;
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
pub static mut destructed_value_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_6(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_7(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_6(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: i32, mut id: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: i32,
        mut id: i32,
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
    }
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> i32 {
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
            operator_eq_6(
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
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TaggingAllocator_Tag_X_ {}
impl TaggingAllocator_Tag_X_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut Tag_X {
        return (unsafe { <std_allocator_Tag_X_>::default().allocate(n) });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut Tag_X, mut n: usize) {
        (unsafe { <std_allocator_Tag_X_>::default().deallocate(p, n) });
    }
    pub unsafe fn construct_3(&mut self, mut p: *mut Tag_X) {
        (unsafe {
            let mut ___args: Ctor_Tag = <Ctor_Tag>::default();
            construct_at_8(p, &mut ___args)
        });
    }
    pub unsafe fn construct_3_int_int_int(
        &mut self,
        mut p: *mut Tag_X,
        args_1: *mut i32,
        args_2: *mut i32,
        args_3: *mut i32,
    ) {
        (unsafe {
            let ___location: *mut Tag_X = p;
            let mut ___args_1: Ctor_Tag = <Ctor_Tag>::default();
            let ___args_2: *mut i32 = args_1;
            let ___args_3: *mut i32 = args_2;
            let ___args_4: *mut i32 = args_3;
            construct_at_9(___location, &mut ___args_1, ___args_2, ___args_3, ___args_4)
        });
    }
    pub unsafe fn destroy_4_struct_Tag_X(&mut self, mut p: *mut Tag_X) {}
}
impl TaggingAllocator_Tag_X_ptr_ {
    pub unsafe fn new_1(_a0: *const TaggingAllocator_Tag_X_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TaggingAllocator_Tag_X_ptr_ {}
impl TaggingAllocator_Tag_X_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut Tag_X {
        return (unsafe { <std_allocator_Tag_X_ptr_>::default().allocate(n) });
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut Tag_X, mut n: usize) {
        (unsafe { <std_allocator_Tag_X_ptr_>::default().deallocate(p, n) });
    }
}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag, _a1: *mut i32, _a2: *mut i32, _a3: *mut i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_2(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_4(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
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
impl TaggingAllocator_Tag_X_ptr_ {}
pub unsafe fn make_10(mut size: i32, mut start: i32) -> Vec<Emplaceable> {
    let b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if ((start) > (0)) {
        init = ((((start) + (1)) / (b)) + (((((start) + (1)) % (b)) != (0)) as i32));
        init *= b;
        init.prefix_dec();
    }
    let mut c: Vec<Emplaceable> =
        std_deque_Emplaceable__std_allocator_Emplaceable__::new_1({ (init as u64) }, None);
    let mut i: i32 = 0;
    'loop_: while ((i) < ((init) - (start))) {
        (unsafe { c.pop_back() });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        c.push(Emplaceable::new_1());
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (start)) {
        c.remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub unsafe fn test_11(c1: *mut Vec<Emplaceable>) {
    let mut c1_osize: usize = ((unsafe { (*c1).size() }) as usize);
    let res_ref: *mut Emplaceable = (unsafe {
        let mut ___args: Emplaceable = Emplaceable::new_2({ 1 }, { 2.5_f64 });
        (*c1).emplace_front(&mut ___args)
    });
    assert!((((unsafe { (*c1).size() }) as usize) == ((c1_osize).wrapping_add(1_usize))));
    assert!(
        ((unsafe {
            let ___first: UnsafeRandomAccessIterator<Emplaceable> = (unsafe { (*c1).begin_24() });
            let ___last: UnsafeRandomAccessIterator<Emplaceable> = (unsafe { (*c1).end_26() });
            distance_12(___first, ___last)
        }) == ((unsafe { (*c1).size() }) as isize))
    );
    let mut i: UnsafeRandomAccessIterator<Emplaceable> = (unsafe { (*c1).begin_24() });
    assert!(
        (unsafe {
            let mut _x: Emplaceable = Emplaceable::new_2({ 1 }, { 2.5_f64 });
            Emplaceable::operator_eq(&(*i.iter_deref()), &mut _x)
        })
    );
    const _: () = assert!(true, "");;
    assert!(((res_ref) == (&mut (*i.iter_deref()) as *mut Emplaceable)));
}
pub unsafe fn testN_13(mut start: i32, mut N: i32) {
    let mut c1: Vec<Emplaceable> = (unsafe { make_10(N, start) });
    (unsafe { test_11(&mut c1) });
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_13(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_14(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut c: std_deque_Tag_X__TaggingAllocator_Tag_X__ =
            <std_deque_Tag_X__TaggingAllocator_Tag_X__>::default();
        (unsafe { c.emplace_front() });
        assert!(((unsafe { c.size() }) == (1_u64)));
        const _: () = assert!(true, "");;
        (unsafe {
            let mut ___args_0: i32 = 1;
            let mut ___args_1: i32 = 2;
            let mut ___args_2: i32 = 3;
            c.emplace_front(&mut ___args_0, &mut ___args_1, &mut ___args_2)
        });
        assert!(((unsafe { c.size() }) == (2_u64)));
        const _: () = assert!(true, "");;
        (unsafe { c.emplace_front() });
        assert!(((unsafe { c.size() }) == (3_u64)));
        const _: () = assert!(true, "");;
        (unsafe {
            let mut ___args_0: i32 = 1;
            let mut ___args_1: i32 = 2;
            let mut ___args_2: i32 = 3;
            c.emplace_front(&mut ___args_0, &mut ___args_1, &mut ___args_2)
        });
        assert!(((unsafe { c.size() }) == (4_u64)));
        const _: () = assert!(true, "");;
    }
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_4);
    std::cell::LazyCell::force(&*&raw const moved_value_5);
}
