#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <tuple>
#include <utility>

template <typename T1, typename... T2>
const T1 &&f78(const std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2> T1 &&f79(std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
const T1 &f80(const std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename... T2> T1 &f81(std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}
