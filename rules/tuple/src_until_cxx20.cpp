#include <array>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename... T1, typename... T2>
bool f20(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 != a1;
}

template <typename... T1, typename... T2>
bool f21(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 < a1;
}

template <typename... T1, typename... T2>
bool f22(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 <= a1;
}

template <typename... T1, typename... T2>
bool f23(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 > a1;
}

template <typename... T1, typename... T2>
bool f24(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return a0 >= a1;
}
