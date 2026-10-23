#include <cstddef>
#include <type_traits>
#include <utility>

template <typename T1, std::size_t T2>
std::size_t f4(const std::extent<T1, T2> &a0) {
  return a0.operator()();
}
