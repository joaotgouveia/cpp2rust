#include <algorithm>
#include <cstddef>
#include <type_traits>
#include <utility>

template <typename T1> std::size_t f2(const std::rank<T1> &a0) {
  return a0.operator()();
}
