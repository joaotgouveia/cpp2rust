extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn matalloc_0 ( mut n : ()  , mut p : ()  , mut e : ()  , ) -> std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__{
    let m : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { make_unique_1 ( n  , ) } )  )) ;
    let mut i: () = 0_();
    'loop_: while (i < n) {
        let __rhs = ({ make_unique_2(p) });
        (*m.borrow_mut())[i] = __rhs;
        let mut j: () = 0_();
        'loop_: while (j < p) {
            elem!(m.as_pointer().offset(i), j).write(e);
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return (*m.borrow_mut()).clone();
}pub fn matmul_3 ( m1 : std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__  , mut n1 : ()  , mut p1 : ()  , m2 : std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__  , mut n2 : ()  , mut p2 : ()  , ) -> std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__{
    let m1 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ >  = Rc::new(RefCell::new(m1)) ;
    let m2 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ >  = Rc::new(RefCell::new(m2)) ;
    let m3 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { matalloc_0 ( n1  , p2  , 0_()  , ) } )  )) ;
    let mut i: () = 0_();
    'loop_: while (i < n1) {
        let mut j: () = 0_();
        let mut sum: () = 0_();
        'loop_: while (j < p2) {
            let mut k: () = 0_();
            'loop_: while (k < p1) {
                sum += ((elem!(m1.as_pointer().offset(i), k).read())
                    * (elem!(m2.as_pointer().offset(k), j).read()));
                k.prefix_inc();
            }
            elem!(m3.as_pointer().offset(i), j).write(sum);
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
fn main_0() -> () {
    let mut n: () = 1_();
    let mut p: () = 10_();
    let m1 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { matalloc_0 ( n  , p  , 1_()  , ) } )  )) ;
    let m2 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { matalloc_0 ( p  , n  , 2_()  , ) } )  )) ;
    let m3 : Value<std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__ > = Rc::new(RefCell::new(( { let _m1: std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__   = ((*m1.borrow_mut())  ).clone() ; let _n1: ()   = n  ; let _p1: ()   = p  ; let _m2: std_unique_ptr_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__std_default_delete_std_unique_ptr_intarrarr__std_default_delete_intarrarr__arrarr__   = ((*m2.borrow_mut())  ).clone() ; let _n2: ()   = p  ; let _p2: ()   = n  ; matmul_3 ( _m1 , _n1 , _p1 , _m2 , _n2 , _p2 , ) } )  )) ;
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
