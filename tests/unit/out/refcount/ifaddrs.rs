extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let list: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    unimplemented!();
    unimplemented!();
    let mut found_loopback: () = unimplemented!();
    let mut ifa: Ptr<()> = Ptr::<()>::null();
    ifa = (*list.borrow()).clone();
    'loop_: while (((!((ifa).is_null())) as ()) != 0) {
        unimplemented!();
        if ((((ifa.with(|__s| __s.ifa_addr.clone())).is_null()) as ()) != 0) {
            ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
            continue 'loop_;
        }
        if (((ifa
            .with(|__s| __s.ifa_addr.clone())
            .with(|__s| __s.sa_family)
            != unimplemented!()) as ())
            != 0)
        {
            ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
            continue 'loop_;
        }
        let mut sin: Ptr<()> = ifa
            .with(|__s| __s.ifa_addr.clone())
            .reinterpret_cast::<()>();
        let lo_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            127_(),
            unimplemented!(),
            unimplemented!(),
            1_(),
        ])));
        if (((({
            memcmp_0(
                ((field_ptr!(sin, sin_addr)) as Ptr<()>).to_any(),
                ((lo_be.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
                4_(),
            )
        }) == unimplemented!()) as ())
            != 0)
        {
            found_loopback = 1_();
            unimplemented!();
            unimplemented!();
            let mut mask: Ptr<()> = ifa
                .with(|__s| __s.ifa_netmask.clone())
                .reinterpret_cast::<()>();
            let mask_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                255_(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ])));
            unimplemented!();
            unimplemented!();
        }
        ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
