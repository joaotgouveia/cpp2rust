#include <algorithm>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename T1>
using t6 = std::tuple_size<std::__enable_if_tuple_size_imp<
    const T1, std::__enable_if_t<!std::is_volatile<T1>::value>,
    decltype(std::tuple_size<T1>::value)>>;

template <typename T1>
typename std::tuple_size<const T1>::value_type
f5(const std::tuple_size<std::__enable_if_tuple_size_imp<
       const T1, std::__enable_if_t<!std::is_volatile<T1>::value>,
       decltype(std::tuple_size<T1>::value)>> &a0) {
  return static_cast<typename std::tuple_size<const T1>::value_type>(a0);
}

template <typename T1>
using t7 = std::tuple_size<std::__enable_if_tuple_size_imp<
    volatile T1, std::__enable_if_t<!std::is_const<T1>::value>,
    decltype(std::tuple_size<T1>::value)>>;

template <typename T1>
typename std::tuple_size<volatile T1>::value_type
f6(const std::tuple_size<std::__enable_if_tuple_size_imp<
       volatile T1, std::__enable_if_t<!std::is_const<T1>::value>,
       decltype(std::tuple_size<T1>::value)>> &a0) {
  return static_cast<typename std::tuple_size<volatile T1>::value_type>(a0);
}

template <typename T1>
using t8 = std::tuple_size<std::__enable_if_tuple_size_imp<
    const volatile T1, decltype(std::tuple_size<T1>::value)>>;

template <typename T1>
typename std::tuple_size<const volatile T1>::value_type
f7(const std::tuple_size<std::__enable_if_tuple_size_imp<
       const volatile T1, decltype(std::tuple_size<T1>::value)>> &a0) {
  return static_cast<typename std::tuple_size<const volatile T1>::value_type>(
      a0);
}
