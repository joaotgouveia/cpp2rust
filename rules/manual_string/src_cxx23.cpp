// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>
#include <string>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f1(std::istreambuf_iterator<T1, T2> a0,
                                 std::istreambuf_iterator<T1, T2> a1,
                                 const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}
