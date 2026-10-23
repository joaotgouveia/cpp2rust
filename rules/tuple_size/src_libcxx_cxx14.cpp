#include <algorithm>
#include <tuple>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::tuple_size<const T1>::value_type
f8(const std::tuple_size<std::__enable_if_tuple_size_imp<
       const T1, std::__enable_if_t<!std::is_volatile<T1>::value>,
       decltype(std::tuple_size<T1>::value)>> &a0) {
  return a0.operator()();
}

template <typename T1>
typename std::tuple_size<volatile T1>::value_type
f9(const std::tuple_size<std::__enable_if_tuple_size_imp<
       volatile T1, std::__enable_if_t<!std::is_const<T1>::value>,
       decltype(std::tuple_size<T1>::value)>> &a0) {
  return a0.operator()();
}

template <typename T1>
typename std::tuple_size<const volatile T1>::value_type
f10(const std::tuple_size<std::__enable_if_tuple_size_imp<
        const volatile T1, decltype(std::tuple_size<T1>::value)>> &a0) {
  return a0.operator()();
}
