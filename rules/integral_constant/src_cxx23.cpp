// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <type_traits>
#include <utility>

template <typename T1, T1 T2> using t1 = std::integral_constant<T1, T2>;
