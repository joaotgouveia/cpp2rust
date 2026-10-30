// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1<T1: Default>(a0: Ptr<::std::fs::File>) -> OstreamIterator<T1> {
    OstreamIterator::new(a0, Ptr::null())
}

fn f2<T1: Default>(a0: Ptr<::std::fs::File>, a1: Ptr<i8>) -> OstreamIterator<T1> {
    OstreamIterator::new(a0, a1)
}

fn f3<T1>(a0: OstreamIterator<T1>) -> OstreamIterator<T1> {
    a0.clone()
}

fn t1<T1: Default>() -> OstreamIterator<T1> {
    OstreamIterator::default()
}
