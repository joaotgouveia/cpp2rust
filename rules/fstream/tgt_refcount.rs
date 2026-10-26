// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1() -> ::std::fs::File {
    ::std::fs::File::open("").unwrap()
}

fn t2() -> ::std::fs::File {
    ::std::fs::File::open("").unwrap()
}

fn t3() -> ::std::fs::File {
    ::std::fs::File::open("").unwrap()
}

fn t4() -> ::std::fs::File {
    ::std::fs::File::open("").unwrap()
}

fn f1(a0: Ptr<i8>) -> ::std::fs::File {
    ::std::fs::File::create(a0.to_string()).expect("Failed to open file")
}

fn f2(a0: ::std::fs::File) -> ::std::fs::File {
    a0.try_clone().unwrap()
}

fn f3(a0: &mut ::std::fs::File) -> &mut ::std::fs::File {
    a0
}

fn f4(a0: ::std::fs::File) -> ::std::fs::File {
    a0
}

fn f5(a0: Ptr<i8>) -> ::std::fs::File {
    ::std::fs::File::open(a0.to_string()).expect("Failed to open file")
}

fn f6(a0: ::std::fs::File) -> ::std::fs::File {
    a0.try_clone().unwrap()
}

fn f7(a0: ::std::fs::File) -> ::std::fs::File {
    a0.try_clone().unwrap()
}

fn f8(a0: ::std::fs::File) -> ::std::fs::File {
    a0
}
