#include <algorithm>
#include <array>
#include <cstddef>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename... T1>
std::size_t f3(const std::tuple_size<std::tuple<T1...>> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2>
unsigned long f4(const std::tuple_size<std::pair<T1, T2>> &a0) {
  return a0.operator()();
}
