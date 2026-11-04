extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn matalloc_0 ( mut n : i32  , mut p : i32  , mut e : i32  , ) -> std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__{
    let m : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { make_unique_1 ( ( n as u64 )  , ) } )  )) ;
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        let __rhs = ({ make_unique_2((p as u64)) });
        (*m.borrow_mut())[(i as u64)] = __rhs;
        let mut j: i32 = 0;
        'loop_: while (j < p) {
            elem!(m.as_pointer().offset((i as u64)), (j as u64)).write(e);
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return (*m.borrow_mut()).clone();
}pub fn matmul_3 ( m1 : std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__  , mut n1 : i32  , mut p1 : i32  , m2 : std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__  , mut n2 : i32  , mut p2 : i32  , ) -> std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__{
    let m1 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ >  = Rc::new(RefCell::new(m1)) ;
    let m2 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ >  = Rc::new(RefCell::new(m2)) ;
    let m3 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { matalloc_0 ( n1  , p2  , 0  , ) } )  )) ;
    let mut i: i32 = 0;
    'loop_: while (i < n1) {
        let mut j: i32 = 0;
        let mut sum: i32 = 0;
        'loop_: while (j < p2) {
            let mut k: i32 = 0;
            'loop_: while (k < p1) {
                sum += ((elem!(m1.as_pointer().offset((i as u64)), (k as u64)).read())
                    * (elem!(m2.as_pointer().offset((k as u64)), (j as u64)).read()));
                k.prefix_inc();
            }
            elem!(m3.as_pointer().offset((i as u64)), (j as u64)).write(sum);
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return (*m3.borrow_mut()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut n: i32 = 1;
    let mut p: i32 = 10;
    let m1 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { matalloc_0 ( n  , p  , 1  , ) } )  )) ;
    let m2 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { matalloc_0 ( p  , n  , 2  , ) } )  )) ;
    let m3 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { let _m1: std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__   = ((*m1.borrow_mut())  ).clone() ; let _n1: i32   = n  ; let _p1: i32   = p  ; let _m2: std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__   = ((*m2.borrow_mut())  ).clone() ; let _n2: i32   = p  ; let _p2: i32   = n  ; matmul_3 ( _m1 , _n1 , _p1 , _m2 , _n2 , _p2 , ) } )  )) ;
    ({ cpp2rust_assert_fail_4(((elem!(m3.as_pointer().offset(0_u64), 0_u64).read()) == 20)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
