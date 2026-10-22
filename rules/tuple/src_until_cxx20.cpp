#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <array>
#include <tuple>
#include <utility>

template <typename... T1, typename... T2>
bool f34(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 != a1;
}

template <typename... T1, typename... T2>
bool f35(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 < a1;
}

template <typename... T1, typename... T2>
bool f36(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 <= a1;
}

template <typename... T1, typename... T2>
bool f37(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 > a1;
}

template <typename... T1, typename... T2>
bool f38(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 >= a1;
}
