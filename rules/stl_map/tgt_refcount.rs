// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f3<T1: 'static, T2: 'static, T3, T4>(a0: Ptr<Map<T1, (Value<T1>, Value<T2>)>>, a1: Ptr<Map<T1, (Value<T1>, Value<T2>)>>) {
    let __src = a1.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| std::mem::take(__m));
    let __dst = a0.with_mut(|__m: &mut Map<T1, (Value<T1>, Value<T2>)>| std::mem::replace(__m, __src));
    a1.write(__dst)
}
