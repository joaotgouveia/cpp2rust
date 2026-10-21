extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn early_0(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    let mut intentionally_const_var: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = unimplemented!();
            if (((n < unimplemented!()) as ()) != 0) {
                ret = -1_();
                goto!('out);
            }
            ret = 100_();
            intentionally_const_var = 22_();
        }
        'out: {
            return ((ret + intentionally_const_var) - intentionally_const_var);
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn from_loop_1(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = unimplemented!();
            let mut i: () = unimplemented!();
            'loop_: while (((i < n) as ()) != 0) {
                if (((i == 3_()) as ()) != 0) {
                    ret = 7_();
                    goto!('out);
                }
                ret += i;
                i.postfix_inc();
            }
            ret = 999_();
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn from_switch_2(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = unimplemented!();
            'switch: {
                match { n } {
                    __v if __v == 1_() => {
                        ret = 10_();
                        goto!('out);
                    }
                    _ => {
                        ret = 20_();
                        break 'switch;
                    }
                }
            };
            ret = 999_();
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct wrapper {
    #[offset(0)]
    #[byte_size(8)]
    pub item: Ptr<()>,
}
pub fn via_pointer_3(mut w: Ptr<wrapper>, mut fail: ()) -> () {
    let mut ret: () = unimplemented!();
    let mut item: Ptr<()> = Ptr::<()>::null();
    goto_block!({
        '__entry: {
            ret = unimplemented!();
            item = w.with(|__s| __s.item.clone());
            if (fail != 0) {
                ret = -1_();
                goto!('out);
            }
            ret = { (item.read()) };
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn via_arrays_4(mut fail: ()) -> () {
    let mut ret: () = unimplemented!();
    let mut remain: [(); 4] = [unimplemented!(); 4];
    let mut name: [(); 5] = [unimplemented!(); 5];
    goto_block! ( { '__entry:  { ret = unimplemented!()  ;
 ;
 ;
 remain = [ unimplemented!()  , unimplemented!()  , unimplemented!()  , unimplemented!()  , ]  ;
 ;
 ;
 name = b"wxyz\0".map(()::from_byte)  ;
 ;
 ;
 if ( fail != 0 ) { ret  = - 1_()  ;
 goto!('out) ;
 } remain [(1_() ) as usize]  = 9_()  ;
 ret  = ( ( ( remain [(unimplemented!() ) as usize] + remain [(1_() ) as usize] ) + ( ( ( ( name [(unimplemented!() ) as usize] == ( 'w' as ()  ) ) ) as () ) ) ) + ( ( ( ( name [(4_() ) as usize] == ( '\0' as ()  ) ) ) as () ) ) )  ;
 } 'out:  { return ret   ;
 } } );
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let value: Value<()> = Rc::new(RefCell::new(42_()));
    let w: Value<wrapper> = Rc::new(RefCell::new(wrapper {
        item: (value.as_pointer()),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
