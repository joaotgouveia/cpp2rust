// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <memory>

template <typename T1>
std::allocation_result<T1> f16(T1 a0, std::size_t a1) {
  return {a0, a1};
}
