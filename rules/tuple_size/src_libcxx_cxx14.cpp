#include <algorithm>
#include <cstddef>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename T1>
std::size_t f8(const std::tuple_size<std::__enable_if_tuple_size_imp<
                   const T1, std::__enable_if_t<!std::is_volatile<T1>::value>,
                   decltype(std::tuple_size<T1>::value)>> &a0) {
  return a0.operator()();
}

template <typename T1>
std::size_t f9(const std::tuple_size<std::__enable_if_tuple_size_imp<
                   volatile T1, std::__enable_if_t<!std::is_const<T1>::value>,
                   decltype(std::tuple_size<T1>::value)>> &a0) {
  return a0.operator()();
}

template <typename T1>
std::size_t
f10(const std::tuple_size<std::__enable_if_tuple_size_imp<
        const volatile T1, decltype(std::tuple_size<T1>::value)>> &a0) {
  return a0.operator()();
}
