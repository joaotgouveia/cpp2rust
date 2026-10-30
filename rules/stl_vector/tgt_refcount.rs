// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f3<T1: ByteRepr + 'static, T2>(a0: Ptr<Vec<T1>>, a1: Ptr<Vec<T1>>) {
    let __src = a1.with_mut(|__v: &mut Vec<T1>| std::mem::take(__v));
    let __dst = a0.with_mut(|__v: &mut Vec<T1>| std::mem::replace(__v, __src));
    a1.write(__dst)
}
