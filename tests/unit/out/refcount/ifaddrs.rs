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
    let list: Value<Ptr<ifaddrs>> = Rc::new(RefCell::new(Ptr::<ifaddrs>::null()));
    unimplemented!();
    unimplemented!();
    let mut found_loopback: () = 0_();
    let mut ifa: Ptr<ifaddrs> = Ptr::<ifaddrs>::null();
    ifa = (*list.borrow()).clone();
    'loop_: while (((!((ifa).is_null())) as ()) != 0) {
        unimplemented!();
        if ((((ifa.with(|__s| __s.ifa_addr.clone())).is_null()) as ()) != 0) {
            ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
            continue 'loop_;
        }
        if ((((ifa
            .with(|__s| __s.ifa_addr.clone())
            .with(|__s| __s.sa_family) as ())
            != libc::AF_INET) as ())
            != 0)
        {
            ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
            continue 'loop_;
        }
        let mut sin: Ptr<sockaddr_in> = ifa
            .with(|__s| __s.ifa_addr.clone())
            .reinterpret_cast::<sockaddr_in>();
        let lo_be: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([127_(), 0_(), 0_(), 1_()])));
        if (((({
            memcmp_0(
                ((field_ptr!(sin, sin_addr)) as Ptr<in_addr>).to_any(),
                ((lo_be.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
                4_(),
            )
        }) == 0_()) as ())
            != 0)
        {
            found_loopback = 1_();
            unimplemented!();
            unimplemented!();
            let mut mask: Ptr<sockaddr_in> = ifa
                .with(|__s| __s.ifa_netmask.clone())
                .reinterpret_cast::<sockaddr_in>();
            let mask_be: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([255_(), 0_(), 0_(), 0_()])));
            unimplemented!();
            unimplemented!();
        }
        ifa = { ifa.with(|__s| __s.ifa_next.clone()) };
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
