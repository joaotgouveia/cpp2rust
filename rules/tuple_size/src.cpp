#include <algorithm>
#include <tuple>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1> using t1 = std::tuple_size<std::tuple<T1...>>;

template <typename... T1>
typename std::tuple_size<std::tuple<T1...>>::value_type
f1(const std::tuple_size<std::tuple<T1...>> &a0) {
  return static_cast<typename std::tuple_size<std::tuple<T1...>>::value_type>(
      a0);
}

template <typename T1, typename T2>
using t2 = std::tuple_size<std::pair<T1, T2>>;

template <typename T1, typename T2>
typename std::tuple_size<std::pair<T1, T2>>::value_type
f2(const std::tuple_size<std::pair<T1, T2>> &a0) {
  return static_cast<typename std::tuple_size<std::pair<T1, T2>>::value_type>(
      a0);
}
