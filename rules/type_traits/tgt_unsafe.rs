// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1<T1>() -> T1 {
    unimplemented!()
}

fn t2<T1>() -> T1 {
    unimplemented!()
}

fn t3<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn f2<T1: libcc2rs::Conjunction>() -> bool {
    <T1 as libcc2rs::Conjunction>::VALUE
}

fn f4<T1: libcc2rs::Disjunction>() -> bool {
    <T1 as libcc2rs::Disjunction>::VALUE
}

fn t15() -> libcc2rs::FalseType {
    libcc2rs::FalseType
}

fn t28() -> libcc2rs::TrueType {
    libcc2rs::TrueType
}
