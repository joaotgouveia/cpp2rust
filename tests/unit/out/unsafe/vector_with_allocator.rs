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
pub struct TestAllocator_int_ {}
impl TestAllocator_int_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut _a1: ()) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestAllocator_double_ {}
impl TestAllocator_double_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut _a1: ()) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
pub unsafe fn copy_0(mut copy_vector: std_vector_int__TestAllocator_int__) {}
pub unsafe fn fn_1(
    v: *mut std_vector_int__TestAllocator_int__,
    mut v3: std_vector_int__TestAllocator_int__,
) {
    (unsafe {
        let mut ___x: () = 20_();
        (*v).push_back_58(&mut ___x)
    });
    let mut x: () = unimplemented!();
    let mut v4: *mut std_vector_int__TestAllocator_int__ =
        (&mut v3 as *mut std_vector_int__TestAllocator_int__);
    let mut v2: std_vector_int__TestAllocator_int__ =
        <std_vector_int__TestAllocator_int__>::default();
    (unsafe {
        let mut ___x: () = 0_();
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 1_();
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 3_();
        v2.push_back_58(&mut ___x)
    });
    x = (&mut (*v))[(2_()) as usize];
    v2[(0_()) as usize] = 1_();
    (&mut (*if true {
        (&mut v3 as *mut std_vector_int__TestAllocator_int__)
    } else {
        (v)
    }))[(0_()) as usize] = 7_();
    (&mut (*v4))[(1_()) as usize] = 13_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        let mut ___x: () = 20_();
        (*v).push_back_58(&mut ___x)
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v1: std_vector_int__TestAllocator_int__ =
        <std_vector_int__TestAllocator_int__>::default();
    unimplemented!();
    unimplemented!();
    (unsafe {
        let mut ___x: () = 1_();
        v1.push_back_58(&mut ___x)
    });
    unimplemented!();
    (unsafe { v1.pop_back() });
    unimplemented!();
    let mut s1: () = (unsafe { v1.size() });
    (unsafe { v1.resize_40(100_()) });
    unimplemented!();
    unimplemented!();
    v1[(0_()) as usize] = 40_();
    v1[(99_()) as usize] = 50_();
    unimplemented!();
    unimplemented!();
    let mut v2: std_vector_int__TestAllocator_int__ =
        <std_vector_int__TestAllocator_int__>::default();
    unimplemented!();
    (unsafe {
        let mut ___x: () = 1_();
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 2_();
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 3_();
        v2.push_back_58(&mut ___x)
    });
    unimplemented!();
    (unsafe {
        let ___position: __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___  = { let mut __tmp_0 : __gnu_cxx___normal_iterator_int_ptr__std_vector_int__TestAllocator_int___  = ( unsafe { v2 . begin_26  ( ) } )  ; __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___ :: new_1 ( { & mut __tmp_0 } , )  } ;
        v2.erase_67(___position)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        let ___position: __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___  = { let mut __tmp_1 : __gnu_cxx___normal_iterator_int_ptr__std_vector_int__TestAllocator_int___  = ( unsafe { v2 . begin_26  ( ) } )  ; __gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__TestAllocator_int___ :: new_1 ( { & mut __tmp_1 } , )  } ;
        let mut ___x: () = 100_();
        v2.insert_63(___position, &mut ___x)
    });
    (unsafe { copy_0(v2.clone()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s2: () = (unsafe { v2.size() });
    let mut v3: std_vector_int__TestAllocator_int__ = {
        let mut __tmp_2: () = 1_();
        std_vector_int__TestAllocator_int__::new_1({ 100_() }, { &mut __tmp_2 }, None)
    };
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v6: std_vector_double__TestAllocator_double__ = {
        let mut __tmp_3: () = 2_f64;
        std_vector_double__TestAllocator_double__::new_1({ s2 }, { &mut __tmp_3 }, None)
    };
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (s2)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut p1: *const () = (unsafe { v6.data_55() }).cast_const();
    unimplemented!();
    let mut p2: *mut () = (unsafe { v3.data_55() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*p2) = 99_f64;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p2.prefix_inc();
    (*p2) = 98_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { v3.reserve(200_()) });
    unimplemented!();
    unimplemented!();
    (unsafe { v3.reserve(50_()) });
    unimplemented!();
    unimplemented!();
    (unsafe { v3.reserve(200_()) });
    unimplemented!();
    unimplemented!();
    (unsafe { v3.reserve(201_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let ref0: *mut () = (unsafe { v6.back_53() });
    (*ref0) = 5_f64;
    unimplemented!();
    let mut x0: () = (*(unsafe { v6.back_53() }));
    unimplemented!();
    x0 = 6_f64;
    unimplemented!();
    let mut idx: () = 0_();
    unimplemented!();
    unimplemented!();
    let ref1: *mut () = (unsafe { v6.at_49((s2).wrapping_sub(1_())) });
    (*ref1) += 1.5_f64;
    unimplemented!();
    let mut x1: () = (*(unsafe { v6.at_49((s2).wrapping_sub(1_())) }));
    unimplemented!();
    x1 -= 1.5_f64;
    unimplemented!();
    let mut v7: std_vector_int__TestAllocator_int__ =
        <std_vector_int__TestAllocator_int__>::default();
    let mut v8: std_vector_int__TestAllocator_int__ =
        <std_vector_int__TestAllocator_int__>::default();
    (unsafe {
        let mut ___x: () = 4_();
        v7.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 5_();
        v7.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 6_();
        v7.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 8_();
        v8.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 9_();
        v8.push_back_58(&mut ___x)
    });
    (unsafe { fn_1(&mut v7, v8.clone()) });
    let mut src: [(); 3] = [1_(), 2_(), 3_()];
    let mut v9: std_vector_unsigned_int__std_allocator_unsigned_int__ =
        std_vector_unsigned_int__std_allocator_unsigned_int__::new_1(
            { src.as_mut_ptr() },
            { src.as_mut_ptr().offset((3_()) as isize) },
            None,
        );
    unimplemented!();
    unimplemented!();
    let mut v10: std_vector_unsigned_long__std_allocator_unsigned_long__ =
        std_vector_unsigned_long__std_allocator_unsigned_long__::new_1(
            { src.as_mut_ptr() },
            { src.as_mut_ptr().offset((3_()) as isize) },
            None,
        );
    unimplemented!();
    unimplemented!();
    let mut v11: std_vector_int__std_allocator_int__ = std_vector_int__std_allocator_int__::new_1(
        { src.as_mut_ptr() },
        { src.as_mut_ptr().offset((3_()) as isize) },
        None,
    );
    unimplemented!();
    unimplemented!();
    let src1: [(); 3] = [1_(), 2_(), 3_()];
    let mut v12: std_vector_unsigned_int__std_allocator_unsigned_int__ =
        std_vector_unsigned_int__std_allocator_unsigned_int__::new_2(
            { src1.as_ptr() },
            { (unsafe { end_2(&src1) }) },
            None,
        );
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 5] = [10_(), 20_(), 30_(), 40_(), 50_()];
    let mut start: *const () = (buf.as_mut_ptr()).cast_const();
    let mut len: () = 5_();
    let mut v13: std_vector_unsigned_char__std_allocator_unsigned_char__ =
        std_vector_unsigned_char__std_allocator_unsigned_char__::new_1(
            { start },
            { start.offset((len) as isize) },
            None,
        );
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
