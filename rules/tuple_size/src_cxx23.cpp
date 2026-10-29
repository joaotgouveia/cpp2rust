#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <typename T1>
using t1 = std::tuple_size<const std::__enable_if_has_tuple_size<T1>>;

template <typename T1>
using t2 = std::tuple_size<volatile std::__enable_if_has_tuple_size<T1>>;

template <typename T1>
using t3 = std::tuple_size<const volatile std::__enable_if_has_tuple_size<T1>>;

template <typename... T1> using t4 = std::tuple_size<std::tuple<T1...>>;

template <typename... T1>
std::size_t f1(const std::tuple_size<std::tuple<T1...>> &a0) {
  return static_cast<std::size_t>(a0);
}

template <typename... T1>
std::size_t f2(const std::tuple_size<std::tuple<T1...>> &a0) {
  return a0.operator()();
}

template <typename T1, std::size_t T2>
using t5 = std::tuple_size<std::array<T1, T2>>;

template <typename T1, typename T2>
using t6 = std::tuple_size<std::pair<T1, T2>>;

template <typename T1, typename T2>
unsigned long f3(const std::tuple_size<std::pair<T1, T2>> &a0) {
  return static_cast<unsigned long>(a0);
}

template <typename T1, typename T2>
unsigned long f4(const std::tuple_size<std::pair<T1, T2>> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2, std::ranges::subrange_kind T3>
using t7 = std::tuple_size<std::ranges::subrange<T1, T2, T3>>;
