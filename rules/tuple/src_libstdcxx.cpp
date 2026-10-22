#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f100(std::allocator_arg_t a0, const T2 &a1,
                       const std::tuple<T3...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1>
auto f101(T1 &&...a0)
    -> decltype(std::tuple_cat<T1...>(std::forward<T1>(a0)...)) {
  return std::tuple_cat<T1...>(std::forward<T1>(a0)...);
}
