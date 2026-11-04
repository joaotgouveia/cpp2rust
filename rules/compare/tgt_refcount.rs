// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn f51() -> libcc2rs::Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_less()
}

fn f52() -> libcc2rs::Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_equal()
}

fn f53() -> libcc2rs::Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_equivalent()
}

fn f54() -> libcc2rs::Ptr<std::cmp::Ordering> {
    libcc2rs::strong_ordering_greater()
}
