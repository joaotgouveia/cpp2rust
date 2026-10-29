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
pub struct Bar {
    pub w: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {
    pub x: (),
    pub y: *mut (),
    pub z: *mut (),
    pub a: [(); 3],
    pub bar: Bar,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Refs {
    pub a: *mut (),
    pub b: *mut (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x1: () = 1_();
    let mut x2: () = x1;
    x2.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut x3: () = 3_f64;
    let mut x4: () = x3;
    x4.prefix_inc();
    unimplemented!();
    unimplemented!();
    let reference: *mut () = &mut x1;
    let mut x5: () = (*reference);
    x5.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut pointer: *mut () = (&mut x1 as *mut ());
    let mut x6: () = (*pointer);
    x6.prefix_inc();
    unimplemented!();
    unimplemented!();
    let mut other_pointer: *mut () = pointer;
    unimplemented!();
    (*other_pointer).prefix_inc();
    unimplemented!();
    let mut f1: Foo = Foo {
        x: 1_(),
        y: &mut x1,
        z: (&mut x1 as *mut ()),
        a: [0_(), 1_(), 2_()],
        bar: Bar { w: 10_() },
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut f2: Foo = f1;
    f2.x.prefix_inc();
    (*f2.y).prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*f2.z).prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    f2.a[(0_()) as usize].prefix_inc();
    f2.a[(1_()) as usize].prefix_inc();
    f2.a[(2_()) as usize].prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    f2.bar.w = 20_();
    unimplemented!();
    unimplemented!();
    let mut N: () = 5_();
    let mut v1: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (unsafe { v1.push_back_57(&i) });
        i.prefix_inc();
    }
    let mut v2: std_vector_int__std_allocator_int__ = v1.clone();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        v2[(i) as usize].prefix_inc();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut m1 : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = <std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ >::default() ;
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (unsafe {
            let mut ___x: std_vector_int__std_allocator_int__ =
                std_vector_int__std_allocator_int__::new_1({ 10_() }, None);
            m1.push_back_58(&mut ___x)
        });
        i.prefix_inc();
    }
    let mut m2 : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = m1 .clone() ;
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        'loop_: while ((j) < (10_())) {
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut j: () = 0_();
        'loop_: while ((j) < (10_())) {
            m2[(i) as usize][(j) as usize].postfix_inc();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        'loop_: while ((j) < (10_())) {
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut map1: std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___ =
        <std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___>::default();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        map1[(i) as usize] = i;
        i.prefix_inc();
    }
    let mut map2: std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___ =
        map1.clone();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        map2[(i) as usize].prefix_inc();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut pair1: std_pair_int__int_ = {
        let mut __tmp_0: () = 1_();
        let mut __tmp_1: () = 2_();
        std_pair_int__int_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
    };
    let mut pair2: std_pair_int__int_ = pair1;
    pair2.first = ((pair2.first) * (10_()));
    pair2.second = ((pair2.second) * (10_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut pair3: std_pair_std_vector_int__std_allocator_int____int_ = {
        let mut __tmp_2: std_vector_int__std_allocator_int__ =
            std_vector_int__std_allocator_int__::new_1({ 0_() }, None);
        let mut __tmp_3: () = 0_();
        std_pair_std_vector_int__std_allocator_int____int_::new_1({ &mut __tmp_2 }, {
            &mut __tmp_3
        })
    };
    let mut pair4: std_pair_std_vector_int__std_allocator_int____int_ = pair3.clone();
    (unsafe {
        let mut ___x: () = 1_();
        pair4.first.push_back_58(&mut ___x)
    });
    pair4.second = 1_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s1: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { 3_() },
            { ('a' as ()) },
            None,
        );
    let mut s2: std_basic_string_char__std_char_traits_char___std_allocator_char__ = s1.clone();
    s2[(0_()) as usize] = ('b' as ());
    s2[(1_()) as usize] = ('b' as ());
    s2[(2_()) as usize] = ('b' as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b1: Bar = Bar { w: 1_() };
    let mut b2: Bar = Bar { w: 2_() };
    b2 = b1;
    b2.w.postfix_inc();
    unimplemented!();
    unimplemented!();
    let mut v4: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    v4 = (v2).clone();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        v4[(i) as usize].prefix_inc();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut ra: () = 1_();
    let mut rb: () = 2_();
    let mut r1: Refs = Refs {
        a: &mut ra,
        b: &mut rb,
    };
    let mut r2: Refs = r1;
    (*r2.a) = 10_();
    (*r2.b).prefix_inc();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
