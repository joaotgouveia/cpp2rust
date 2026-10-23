#include <array>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2>
const T1 &&f25(const std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2> T1 &&f26(std::tuple<T2...> &&a0) {
  return std::get<T1, T2...>(std::move(a0));
}

template <typename T1, typename... T2>
const T1 &f27(const std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}

template <typename T1, typename... T2> T1 &f28(std::tuple<T2...> &a0) {
  return std::get<T1, T2...>(a0);
}
