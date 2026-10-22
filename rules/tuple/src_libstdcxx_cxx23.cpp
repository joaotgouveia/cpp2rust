#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1, typename T2> std::tuple<T1...> f110(T2 &&a0) {
  return std::tuple<T1...>(std::forward<T2>(a0));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f111(std::allocator_arg_t a0, const T2 &a1, T3 &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2));
}

template <typename... T1, typename T2>
std::tuple<T1...> &f112(std::tuple<T1...> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2>
const std::tuple<T1...> &f113(const std::tuple<T1...> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}
