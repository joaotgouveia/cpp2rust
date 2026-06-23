// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Requires `-C panic=unwind` for catch_unwind to catch the panic.
// TODO: AssertUnwindSafe
fn f1<T1>(a0: T1)
where
    T1: FnOnce(),
{
    assert!(::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(a0)).is_err())
}
