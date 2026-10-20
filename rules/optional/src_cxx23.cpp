#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> auto f75(std::optional<T1> &a0, T2 &&a1) {
  return a0.template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f76(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f77(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f78(const std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f79(std::optional<T1> &a0, T2 &&a1) {
  return a0.template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f80(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f81(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f82(const std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f83(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template or_else<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f84(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template or_else<T2>(std::forward<T2>(a1));
}
