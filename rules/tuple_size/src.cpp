#include <algorithm>
#include <cstddef>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename... T1> using t1 = std::tuple_size<std::tuple<T1...>>;

template <typename... T1>
std::size_t f1(const std::tuple_size<std::tuple<T1...>> &a0) {
  return static_cast<std::size_t>(a0);
}

template <typename T1, typename T2>
using t2 = std::tuple_size<std::pair<T1, T2>>;

template <typename T1, typename T2>
unsigned long f2(const std::tuple_size<std::pair<T1, T2>> &a0) {
  return static_cast<unsigned long>(a0);
}
