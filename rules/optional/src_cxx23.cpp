#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> auto f58(std::optional<T1> &a0, T2 &&a1) {
  return a0.template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f59(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f60(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f61(const std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f62(std::optional<T1> &a0, T2 &&a1) {
  return a0.template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f63(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f64(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f65(const std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f66(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template or_else<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f67(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template or_else<T2>(std::forward<T2>(a1));
}
