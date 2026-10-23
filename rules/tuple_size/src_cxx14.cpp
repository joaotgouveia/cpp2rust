#include <algorithm>
#include <tuple>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1>
typename std::tuple_size<std::tuple<T1...>>::value_type
f3(const std::tuple_size<std::tuple<T1...>> &a0) {
  return a0.operator()();
}

template <typename T1, typename T2>
typename std::tuple_size<std::pair<T1, T2>>::value_type
f4(const std::tuple_size<std::pair<T1, T2>> &a0) {
  return a0.operator()();
}
