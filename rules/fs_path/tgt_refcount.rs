// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f35(a0: Vec<i8>) -> Vec<i8> {
    a0
}

fn t2() -> Vec<i8> {
    vec![0]
}

fn t4() -> Vec<i8> {
    vec![0]
}

fn f74<T1>(a0: Ptr<T1>) -> Vec<i8>
where
    Ptr<T1>: PathSource,
{
    let mut __v: Vec<i8> = a0.path_bytes().into_iter().map(|__b| __b as i8).collect();
    __v.push(0);
    __v
}
