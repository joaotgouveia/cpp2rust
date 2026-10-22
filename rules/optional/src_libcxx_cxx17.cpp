#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1> std::optional<T1> f89(const std::optional<T1> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1> std::optional<T1> f90(std::optional<T1> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f91(std::optional<T1> &a0, const std::optional<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::optional<T1> &f92(std::optional<T1> &a0, std::optional<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f93(T2 a0, T3 &&...a1) {
  return std::optional<T1>(a0, std::forward<T3>(a1)...);
}
