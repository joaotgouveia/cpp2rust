// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <ranges>

template <typename T1, typename T2>
std::ranges::in_value_result<T1, T2> f6(T1 a0, T2 a1) {
  return {a0, a1};
}

template <typename T1, typename T2>
std::ranges::elements_of<T1, T2> f11(T1 a0, T2 a1) {
  return {a0, a1};
}
